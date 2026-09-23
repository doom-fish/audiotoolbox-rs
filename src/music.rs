use crate::{
    ffi,
    internal::{
        cf_data_from_bytes, cf_data_to_vec, cf_release, cf_url_from_path, lifecycle_lock,
        status_to_result,
    },
    property::{property_byte_size, read_property, AudioProperty},
    AUGraph, AUNode, AUPresetEvent, AudioToolboxError, AudioUnitElement, AudioUnitScope,
    CABarBeatTime, ExtendedControlEvent, ExtendedNoteOnEvent, ExtendedTempoEvent,
    MIDIChannelMessage, MIDIEndpointRef, MIDIMetaEvent, MIDINoteMessage, MIDIRawData,
    MusicDeviceGroupId, MusicDeviceInstrumentId, MusicEventIteratorRef, MusicEventType,
    MusicEventUserData, MusicPlayerRef, MusicSequenceFileFlags, MusicSequenceFileTypeId,
    MusicSequenceLoadFlags, MusicSequenceRef, MusicSequenceType, MusicTimeStamp,
    MusicTrackLoopInfo, MusicTrackRef, NoteParamsControlValue, ParameterEvent, Result,
    MUSIC_EVENT_TYPE_AU_PRESET, MUSIC_EVENT_TYPE_EXTENDED_CONTROL, MUSIC_EVENT_TYPE_EXTENDED_NOTE,
    MUSIC_EVENT_TYPE_EXTENDED_TEMPO, MUSIC_EVENT_TYPE_META, MUSIC_EVENT_TYPE_MIDI_CHANNEL_MESSAGE,
    MUSIC_EVENT_TYPE_MIDI_NOTE_MESSAGE, MUSIC_EVENT_TYPE_MIDI_RAW_DATA, MUSIC_EVENT_TYPE_PARAMETER,
    MUSIC_EVENT_TYPE_USER, SEQUENCE_TRACK_PROPERTY_LOOP_INFO, SEQUENCE_TRACK_PROPERTY_MUTE_STATUS,
    SEQUENCE_TRACK_PROPERTY_OFFSET_TIME, SEQUENCE_TRACK_PROPERTY_SOLO_STATUS,
    SEQUENCE_TRACK_PROPERTY_TIME_RESOLUTION, SEQUENCE_TRACK_PROPERTY_TRACK_LENGTH,
};
use apple_cf::cf::CFDictionary;
use std::{
    ffi::c_void,
    marker::PhantomData,
    mem::{size_of, MaybeUninit},
    path::Path,
};

#[derive(Debug)]
/// Owning wrapper around an AudioToolbox.framework `MusicSequence`.
pub struct MusicSequence {
    handle: *mut c_void,
    raw: MusicSequenceRef,
}

#[derive(Debug)]
/// Wrapper around an AudioToolbox.framework `MusicTrack`.
pub struct MusicTrack<'a> {
    raw: MusicTrackRef,
    _sequence: PhantomData<&'a MusicSequence>,
}

#[derive(Debug)]
/// Owning wrapper around an AudioToolbox.framework `MusicPlayer`.
pub struct MusicPlayer {
    handle: *mut c_void,
    raw: MusicPlayerRef,
}

#[derive(Debug)]
/// Owning wrapper around an AudioToolbox.framework `MusicEventIterator`.
pub struct MusicEventIterator<'a> {
    raw: MusicEventIteratorRef,
    _sequence: PhantomData<&'a MusicSequence>,
}

#[derive(Debug, Clone)]
/// Event snapshot returned by `MusicEventIteratorGetEventInfo`.
pub struct MusicEventInfo {
    /// Wraps `kCVoid`.
    pub time_stamp: MusicTimeStamp,
    /// Wraps `kCVoid`.
    pub event_type: MusicEventType,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Copy)]
pub struct ExtendedNote<'a> {
    pub instrument_id: MusicDeviceInstrumentId,
    pub group_id: MusicDeviceGroupId,
    pub duration: f32,
    pub pitch: f32,
    pub velocity: f32,
    pub controls: &'a [NoteParamsControlValue],
}

#[derive(Debug, Clone, Copy)]
pub enum MusicEvent<'a> {
    MidiNoteMessage(MIDINoteMessage),
    MidiChannelMessage(MIDIChannelMessage),
    MidiRawData(&'a [u8]),
    Meta {
        meta_event_type: u8,
        data: &'a [u8],
    },
    User(&'a [u8]),
    ExtendedNote(ExtendedNote<'a>),
    ExtendedControl(ExtendedControlEvent),
    ExtendedTempo(ExtendedTempoEvent),
    Parameter(ParameterEvent),
    AuPreset {
        scope: AudioUnitScope,
        element: AudioUnitElement,
        preset: &'a CFDictionary,
    },
}

struct EventBytes {
    words: Vec<u32>,
}

impl EventBytes {
    fn new(
        operation: &'static str,
        fixed_size: usize,
        header: &[u8],
        payload: &[u8],
    ) -> Result<Self> {
        let used = header.len() + payload.len();
        let total = fixed_size
            .checked_add(payload.len())
            .filter(|total| u32::try_from(*total).is_ok())
            .ok_or_else(|| {
                AudioToolboxError::message(operation, "event payload exceeds UInt32::MAX bytes")
            })?
            .max(used);
        let mut words = vec![0_u32; total.div_ceil(4)];
        let bytes = unsafe {
            std::slice::from_raw_parts_mut(words.as_mut_ptr().cast::<u8>(), words.len() * 4)
        };
        bytes[..header.len()].copy_from_slice(header);
        bytes[header.len()..used].copy_from_slice(payload);
        Ok(Self { words })
    }

    fn from_value<T: Copy>(value: &T) -> Self {
        let mut words = vec![0_u32; size_of::<T>().div_ceil(4)];
        unsafe { words.as_mut_ptr().cast::<T>().write_unaligned(*value) };
        Self { words }
    }

    fn as_ptr<T>(&self) -> *const T {
        self.words.as_ptr().cast()
    }

    #[cfg(test)]
    fn bytes(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.words.as_ptr().cast(), self.words.len() * 4) }
    }
}

fn payload_length(operation: &'static str, data: &[u8]) -> Result<[u8; 4]> {
    u32::try_from(data.len())
        .map(u32::to_ne_bytes)
        .map_err(|_| {
            AudioToolboxError::message(operation, "event payload exceeds UInt32::MAX bytes")
        })
}

impl MusicEvent<'_> {
    fn encode(&self, operation: &'static str) -> Result<(MusicEventType, EventBytes)> {
        Ok(match self {
            Self::MidiNoteMessage(message) => (
                MUSIC_EVENT_TYPE_MIDI_NOTE_MESSAGE,
                EventBytes::from_value(message),
            ),
            Self::MidiChannelMessage(message) => (
                MUSIC_EVENT_TYPE_MIDI_CHANNEL_MESSAGE,
                EventBytes::from_value(message),
            ),
            Self::MidiRawData(data) => (
                MUSIC_EVENT_TYPE_MIDI_RAW_DATA,
                EventBytes::new(
                    operation,
                    size_of::<MIDIRawData>(),
                    &payload_length(operation, data)?,
                    data,
                )?,
            ),
            Self::Meta {
                meta_event_type,
                data,
            } => {
                let mut header = [0_u8; 8];
                header[0] = *meta_event_type;
                header[4..].copy_from_slice(&payload_length(operation, data)?);
                (
                    MUSIC_EVENT_TYPE_META,
                    EventBytes::new(operation, size_of::<MIDIMetaEvent>(), &header, data)?,
                )
            }
            Self::User(data) => (
                MUSIC_EVENT_TYPE_USER,
                EventBytes::new(
                    operation,
                    size_of::<MusicEventUserData>(),
                    &payload_length(operation, data)?,
                    data,
                )?,
            ),
            Self::ExtendedNote(note) => {
                let arg_count = u32::try_from(note.controls.len())
                    .ok()
                    .and_then(|controls| controls.checked_add(2))
                    .ok_or_else(|| {
                        AudioToolboxError::message(operation, "too many note controls")
                    })?;
                let mut header = Vec::with_capacity(24);
                header.extend_from_slice(&note.instrument_id.to_ne_bytes());
                header.extend_from_slice(&note.group_id.to_ne_bytes());
                header.extend_from_slice(&note.duration.to_ne_bytes());
                header.extend_from_slice(&arg_count.to_ne_bytes());
                header.extend_from_slice(&note.pitch.to_ne_bytes());
                header.extend_from_slice(&note.velocity.to_ne_bytes());
                let controls: Vec<u8> = note
                    .controls
                    .iter()
                    .flat_map(|control| {
                        let mut bytes = [0_u8; 8];
                        bytes[..4].copy_from_slice(&control.mID.to_ne_bytes());
                        bytes[4..].copy_from_slice(&control.mValue.to_ne_bytes());
                        bytes
                    })
                    .collect();
                (
                    MUSIC_EVENT_TYPE_EXTENDED_NOTE,
                    EventBytes::new(
                        operation,
                        size_of::<ExtendedNoteOnEvent>(),
                        &header,
                        &controls,
                    )?,
                )
            }
            Self::ExtendedControl(event) => (
                MUSIC_EVENT_TYPE_EXTENDED_CONTROL,
                EventBytes::from_value(event),
            ),
            Self::ExtendedTempo(event) => (
                MUSIC_EVENT_TYPE_EXTENDED_TEMPO,
                EventBytes::from_value(event),
            ),
            Self::Parameter(event) => (MUSIC_EVENT_TYPE_PARAMETER, EventBytes::from_value(event)),
            Self::AuPreset {
                scope,
                element,
                preset,
            } => (
                MUSIC_EVENT_TYPE_AU_PRESET,
                EventBytes::from_value(&AUPresetEvent {
                    scope: *scope,
                    element: *element,
                    preset: preset.as_ptr().cast_const(),
                }),
            ),
        })
    }
}

impl MusicSequence {
    /// Wraps `NewMusicSequence`.
    ///
    /// The returned wrapper owns the underlying AudioToolbox.framework handle and releases it on drop.
    pub fn new() -> Result<Self> {
        let mut handle = std::ptr::null_mut();
        let status = unsafe { ffi::music::at_music_sequence_new(&raw mut handle) };
        status_to_result("NewMusicSequence", status)?;
        let raw: MusicSequenceRef = unsafe { ffi::music::at_music_sequence_raw(handle) }.cast();
        if raw.is_null() {
            return Err(AudioToolboxError::message(
                "NewMusicSequence",
                "framework returned a null MusicSequence",
            ));
        }
        Ok(Self { handle, raw })
    }

    /// Returns the wrapped `MusicSequenceRef`.
    pub fn as_raw(&self) -> MusicSequenceRef {
        self.raw
    }

    /// Wraps `MusicSequenceNewTrack`.
    pub fn new_track(&self) -> Result<MusicTrack<'_>> {
        let mut handle = std::ptr::null_mut();
        let status =
            unsafe { ffi::music::at_music_sequence_new_track(self.raw.cast(), &raw mut handle) };
        status_to_result("MusicSequenceNewTrack", status)?;
        let raw: MusicTrackRef = unsafe { ffi::music::at_music_track_raw(handle) }.cast();
        unsafe { ffi::music::at_music_track_release(handle) };
        MusicTrack::from_raw(raw, "MusicSequenceNewTrack")
    }

    /// Wraps `MusicSequenceDisposeTrack`.
    pub fn dispose_track(&mut self, index: u32) -> Result<()> {
        let track = self.track(index)?.raw;
        let status =
            unsafe { ffi::music::at_music_sequence_dispose_track(self.raw.cast(), track.cast()) };
        status_to_result("MusicSequenceDisposeTrack", status)
    }

    /// Wraps `MusicSequenceGetTrackCount`.
    pub fn track_count(&self) -> Result<u32> {
        let mut track_count = 0_u32;
        let status = unsafe {
            ffi::music::at_music_sequence_get_track_count(self.raw.cast(), &raw mut track_count)
        };
        status_to_result("MusicSequenceGetTrackCount", status)?;
        Ok(track_count)
    }

    /// Wraps `MusicSequenceGetIndTrack`.
    pub fn track(&self, index: u32) -> Result<MusicTrack<'_>> {
        let mut raw: MusicTrackRef = std::ptr::null_mut();
        let status = unsafe {
            ffi::music::at_music_sequence_get_ind_track(self.raw.cast(), index, &raw mut raw)
        };
        status_to_result("MusicSequenceGetIndTrack", status)?;
        MusicTrack::from_raw(raw, "MusicSequenceGetIndTrack")
    }

    /// Wraps `MusicSequenceGetTrackIndex`.
    pub fn track_index(&self, track: &MusicTrack<'_>) -> Result<u32> {
        let mut track_index = 0_u32;
        let status = unsafe {
            ffi::music::at_music_sequence_get_track_index(
                self.raw.cast(),
                track.raw.cast(),
                &raw mut track_index,
            )
        };
        status_to_result("MusicSequenceGetTrackIndex", status)?;
        Ok(track_index)
    }

    /// Wraps `MusicSequenceGetTempoTrack`.
    pub fn tempo_track(&self) -> Result<MusicTrack<'_>> {
        let mut raw: MusicTrackRef = std::ptr::null_mut();
        let status =
            unsafe { ffi::music::at_music_sequence_get_tempo_track(self.raw.cast(), &raw mut raw) };
        status_to_result("MusicSequenceGetTempoTrack", status)?;
        MusicTrack::from_raw(raw, "MusicSequenceGetTempoTrack")
    }

    /// Wraps `MusicSequenceSetAUGraph`.
    pub fn set_au_graph(&self, graph: &AUGraph) -> Result<()> {
        let _lifecycle = lifecycle_lock();
        let status = unsafe {
            ffi::music::at_music_sequence_set_au_graph(self.handle, graph.bridge_handle())
        };
        status_to_result("MusicSequenceSetAUGraph", status)
    }

    /// Wraps `MusicSequenceGetAUGraph`.
    pub fn au_graph_raw(&self) -> Result<*mut c_void> {
        let mut graph = std::ptr::null_mut();
        let status =
            unsafe { ffi::music::at_music_sequence_get_au_graph(self.raw.cast(), &raw mut graph) };
        status_to_result("MusicSequenceGetAUGraph", status)?;
        Ok(graph)
    }

    /// Wraps `MusicSequenceSetMIDIEndpoint`.
    pub fn set_midi_endpoint(&self, endpoint: MIDIEndpointRef) -> Result<()> {
        let status =
            unsafe { ffi::music::at_music_sequence_set_midi_endpoint(self.raw.cast(), endpoint) };
        status_to_result("MusicSequenceSetMIDIEndpoint", status)
    }

    /// Wraps `MusicSequenceSetSequenceType`.
    pub fn set_sequence_type(&self, sequence_type: MusicSequenceType) -> Result<()> {
        let status = unsafe {
            ffi::music::at_music_sequence_set_sequence_type(self.raw.cast(), sequence_type)
        };
        status_to_result("MusicSequenceSetSequenceType", status)
    }

    /// Wraps `MusicSequenceGetSequenceType`.
    pub fn sequence_type(&self) -> Result<MusicSequenceType> {
        let mut sequence_type = 0_u32;
        let status = unsafe {
            ffi::music::at_music_sequence_get_sequence_type(self.raw.cast(), &raw mut sequence_type)
        };
        status_to_result("MusicSequenceGetSequenceType", status)?;
        Ok(sequence_type)
    }

    /// Wraps `MusicSequenceFileLoad`.
    pub fn load_file(
        &self,
        path: impl AsRef<Path>,
        file_type_hint: MusicSequenceFileTypeId,
        flags: MusicSequenceLoadFlags,
    ) -> Result<()> {
        let url = cf_url_from_path("MusicSequenceFileLoad", path.as_ref())?;
        let status = unsafe {
            ffi::music::at_music_sequence_file_load(self.raw.cast(), url, file_type_hint, flags)
        };
        cf_release(url.cast());
        status_to_result("MusicSequenceFileLoad", status)
    }

    /// Wraps `MusicSequenceFileLoadData`.
    pub fn load_data(
        &self,
        data: &[u8],
        file_type_hint: MusicSequenceFileTypeId,
        flags: MusicSequenceLoadFlags,
    ) -> Result<()> {
        let data = cf_data_from_bytes("MusicSequenceFileLoadData", data)?;
        let status = unsafe {
            ffi::music::at_music_sequence_file_load_data(
                self.raw.cast(),
                data,
                file_type_hint,
                flags,
            )
        };
        cf_release(data.cast());
        status_to_result("MusicSequenceFileLoadData", status)
    }

    /// Wraps `MusicSequenceFileCreate`.
    pub fn create_file(
        &self,
        path: impl AsRef<Path>,
        file_type: MusicSequenceFileTypeId,
        flags: MusicSequenceFileFlags,
        resolution: i16,
    ) -> Result<()> {
        let url = cf_url_from_path("MusicSequenceFileCreate", path.as_ref())?;
        let status = unsafe {
            ffi::music::at_music_sequence_file_create(
                self.raw.cast(),
                url,
                file_type,
                flags,
                resolution,
            )
        };
        cf_release(url.cast());
        status_to_result("MusicSequenceFileCreate", status)
    }

    /// Wraps `MusicSequenceFileCreateData`.
    pub fn create_data(
        &self,
        file_type: MusicSequenceFileTypeId,
        flags: MusicSequenceFileFlags,
        resolution: i16,
    ) -> Result<Vec<u8>> {
        let mut data = std::ptr::null();
        let status = unsafe {
            ffi::music::at_music_sequence_file_create_data(
                self.raw.cast(),
                file_type,
                flags,
                resolution,
                &raw mut data,
            )
        };
        status_to_result("MusicSequenceFileCreateData", status)?;
        cf_data_to_vec("MusicSequenceFileCreateData", data)
    }

    /// Wraps `MusicSequenceGetSecondsForBeats`.
    pub fn seconds_for_beats(&self, beats: MusicTimeStamp) -> Result<f64> {
        let mut seconds = 0.0_f64;
        let status = unsafe {
            ffi::music::at_music_sequence_get_seconds_for_beats(
                self.raw.cast(),
                beats,
                &raw mut seconds,
            )
        };
        status_to_result("MusicSequenceGetSecondsForBeats", status)?;
        Ok(seconds)
    }

    /// Wraps `MusicSequenceGetBeatsForSeconds`.
    pub fn beats_for_seconds(&self, seconds: f64) -> Result<MusicTimeStamp> {
        let mut beats = 0.0_f64;
        let status = unsafe {
            ffi::music::at_music_sequence_get_beats_for_seconds(
                self.raw.cast(),
                seconds,
                &raw mut beats,
            )
        };
        status_to_result("MusicSequenceGetBeatsForSeconds", status)?;
        Ok(beats)
    }

    /// Wraps `MusicSequenceBeatsToBarBeatTime`.
    pub fn beats_to_bar_beat_time(
        &self,
        beats: MusicTimeStamp,
        subbeat_divisor: u32,
    ) -> Result<CABarBeatTime> {
        let mut bar_beat_time = MaybeUninit::<CABarBeatTime>::uninit();
        let status = unsafe {
            ffi::music::at_music_sequence_beats_to_bar_beat_time(
                self.raw.cast(),
                beats,
                subbeat_divisor,
                bar_beat_time.as_mut_ptr(),
            )
        };
        status_to_result("MusicSequenceBeatsToBarBeatTime", status)?;
        Ok(unsafe { bar_beat_time.assume_init() })
    }

    /// Wraps `MusicSequenceBarBeatTimeToBeats`.
    pub fn bar_beat_time_to_beats(&self, bar_beat_time: &CABarBeatTime) -> Result<MusicTimeStamp> {
        let mut beats = 0.0_f64;
        let status = unsafe {
            ffi::music::at_music_sequence_bar_beat_time_to_beats(
                self.raw.cast(),
                bar_beat_time,
                &raw mut beats,
            )
        };
        status_to_result("MusicSequenceBarBeatTimeToBeats", status)?;
        Ok(beats)
    }

    /// Wraps `MusicSequenceGetInfoDictionary`.
    pub fn info_dictionary(&self) -> Result<CFDictionary> {
        let dictionary =
            unsafe { ffi::music::at_music_sequence_get_info_dictionary(self.raw.cast()) };
        unsafe { CFDictionary::from_raw(dictionary.cast_mut().cast()) }.ok_or_else(|| {
            AudioToolboxError::message(
                "MusicSequenceGetInfoDictionary",
                "framework returned a null CFDictionaryRef",
            )
        })
    }

    /// Wraps `MusicSequenceClose`.
    pub fn close(mut self) -> Result<()> {
        self.release();
        Ok(())
    }

    fn release(&mut self) {
        if !self.handle.is_null() {
            let _lifecycle = lifecycle_lock();
            unsafe { ffi::music::at_music_sequence_release(self.handle) };
            self.handle = std::ptr::null_mut();
            self.raw = std::ptr::null_mut();
        }
    }
}

impl Drop for MusicSequence {
    fn drop(&mut self) {
        self.release();
    }
}

impl<'a> MusicTrack<'a> {
    fn from_raw(raw: MusicTrackRef, operation: &'static str) -> Result<Self> {
        if raw.is_null() {
            return Err(AudioToolboxError::message(
                operation,
                "framework returned a null MusicTrack",
            ));
        }
        Ok(Self {
            raw,
            _sequence: PhantomData,
        })
    }

    /// Returns the wrapped `MusicTrackRef`.
    pub fn as_raw(&self) -> MusicTrackRef {
        self.raw
    }

    /// Wraps `MusicTrackGetSequence`.
    pub fn sequence_raw(&self) -> Result<MusicSequenceRef> {
        let mut sequence = std::ptr::null_mut();
        let status =
            unsafe { ffi::music::at_music_track_get_sequence(self.raw.cast(), &raw mut sequence) };
        status_to_result("MusicTrackGetSequence", status)?;
        Ok(sequence)
    }

    /// Wraps `MusicTrackSetDestNode`.
    pub fn set_dest_node(&self, node: AUNode) -> Result<()> {
        let status = unsafe { ffi::music::at_music_track_set_dest_node(self.raw.cast(), node) };
        status_to_result("MusicTrackSetDestNode", status)
    }

    /// Wraps `MusicTrackSetDestMIDIEndpoint`.
    pub fn set_dest_midi_endpoint(&self, endpoint: MIDIEndpointRef) -> Result<()> {
        let status =
            unsafe { ffi::music::at_music_track_set_dest_midi_endpoint(self.raw.cast(), endpoint) };
        status_to_result("MusicTrackSetDestMIDIEndpoint", status)
    }

    /// Wraps `MusicTrackGetDestNode`.
    pub fn dest_node(&self) -> Result<AUNode> {
        let mut node = 0_i32;
        let status =
            unsafe { ffi::music::at_music_track_get_dest_node(self.raw.cast(), &raw mut node) };
        status_to_result("MusicTrackGetDestNode", status)?;
        Ok(node)
    }

    /// Wraps `MusicTrackGetDestMIDIEndpoint`.
    pub fn dest_midi_endpoint(&self) -> Result<MIDIEndpointRef> {
        let mut endpoint = 0_u32;
        let status = unsafe {
            ffi::music::at_music_track_get_dest_midi_endpoint(self.raw.cast(), &raw mut endpoint)
        };
        status_to_result("MusicTrackGetDestMIDIEndpoint", status)?;
        Ok(endpoint)
    }

    /// Wraps `MusicTrackGetProperty`.
    pub fn get_property_typed<T: AudioProperty>(&self, property_id: u32) -> Result<T> {
        read_property("MusicTrackGetProperty", |data, size| unsafe {
            ffi::music::at_music_track_get_property(self.raw.cast(), property_id, data, size)
        })
    }

    /// Wraps `MusicTrackSetProperty`.
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn set_property_typed<T: Copy>(&self, property_id: u32, value: &T) -> Result<()> {
        let length = property_byte_size::<T>("MusicTrackSetProperty")?;
        let status = unsafe {
            ffi::music::at_music_track_set_property(
                self.raw.cast(),
                property_id,
                std::ptr::from_ref(value).cast(),
                length,
            )
        };
        status_to_result("MusicTrackSetProperty", status)
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn loop_info(&self) -> Result<MusicTrackLoopInfo> {
        self.get_property_typed(SEQUENCE_TRACK_PROPERTY_LOOP_INFO)
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn set_loop_info(&self, loop_info: MusicTrackLoopInfo) -> Result<()> {
        unsafe { self.set_property_typed(SEQUENCE_TRACK_PROPERTY_LOOP_INFO, &loop_info) }
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn offset_time(&self) -> Result<MusicTimeStamp> {
        self.get_property_typed(SEQUENCE_TRACK_PROPERTY_OFFSET_TIME)
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn set_offset_time(&self, offset: MusicTimeStamp) -> Result<()> {
        unsafe { self.set_property_typed(SEQUENCE_TRACK_PROPERTY_OFFSET_TIME, &offset) }
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn is_muted(&self) -> Result<bool> {
        Ok(self.get_property_typed::<u8>(SEQUENCE_TRACK_PROPERTY_MUTE_STATUS)? != 0)
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn set_muted(&self, muted: bool) -> Result<()> {
        unsafe { self.set_property_typed(SEQUENCE_TRACK_PROPERTY_MUTE_STATUS, &u8::from(muted)) }
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn is_solo(&self) -> Result<bool> {
        Ok(self.get_property_typed::<u8>(SEQUENCE_TRACK_PROPERTY_SOLO_STATUS)? != 0)
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn set_solo(&self, solo: bool) -> Result<()> {
        unsafe { self.set_property_typed(SEQUENCE_TRACK_PROPERTY_SOLO_STATUS, &u8::from(solo)) }
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn track_length(&self) -> Result<MusicTimeStamp> {
        self.get_property_typed(SEQUENCE_TRACK_PROPERTY_TRACK_LENGTH)
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn set_track_length(&self, length: MusicTimeStamp) -> Result<()> {
        unsafe { self.set_property_typed(SEQUENCE_TRACK_PROPERTY_TRACK_LENGTH, &length) }
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn time_resolution(&self) -> Result<i16> {
        self.get_property_typed(SEQUENCE_TRACK_PROPERTY_TIME_RESOLUTION)
    }

    /// Wraps `MusicTrackMoveEvents`.
    pub fn move_events(
        &self,
        start_time: MusicTimeStamp,
        end_time: MusicTimeStamp,
        move_time: MusicTimeStamp,
    ) -> Result<()> {
        let status = unsafe {
            ffi::music::at_music_track_move_events(self.raw.cast(), start_time, end_time, move_time)
        };
        status_to_result("MusicTrackMoveEvents", status)
    }

    /// Wraps `MusicTrackClear`.
    pub fn clear(&self, start_time: MusicTimeStamp, end_time: MusicTimeStamp) -> Result<()> {
        let status =
            unsafe { ffi::music::at_music_track_clear(self.raw.cast(), start_time, end_time) };
        status_to_result("MusicTrackClear", status)
    }

    /// Wraps `MusicTrackCut`.
    pub fn cut(&self, start_time: MusicTimeStamp, end_time: MusicTimeStamp) -> Result<()> {
        let status =
            unsafe { ffi::music::at_music_track_cut(self.raw.cast(), start_time, end_time) };
        status_to_result("MusicTrackCut", status)
    }

    /// Wraps `MusicTrackCopyInsert`.
    pub fn copy_insert(
        &self,
        source_start_time: MusicTimeStamp,
        source_end_time: MusicTimeStamp,
        dest_track: &MusicTrack<'_>,
        dest_insert_time: MusicTimeStamp,
    ) -> Result<()> {
        let status = unsafe {
            ffi::music::at_music_track_copy_insert(
                self.raw.cast(),
                source_start_time,
                source_end_time,
                dest_track.raw.cast(),
                dest_insert_time,
            )
        };
        status_to_result("MusicTrackCopyInsert", status)
    }

    /// Wraps `MusicTrackMerge`.
    pub fn merge(
        &self,
        source_start_time: MusicTimeStamp,
        source_end_time: MusicTimeStamp,
        dest_track: &MusicTrack<'_>,
        dest_insert_time: MusicTimeStamp,
    ) -> Result<()> {
        let status = unsafe {
            ffi::music::at_music_track_merge(
                self.raw.cast(),
                source_start_time,
                source_end_time,
                dest_track.raw.cast(),
                dest_insert_time,
            )
        };
        status_to_result("MusicTrackMerge", status)
    }

    /// Wraps `MusicTrackNewMIDINoteEvent`.
    pub fn new_midi_note_event(
        &self,
        time_stamp: f64,
        note_message: MIDINoteMessage,
    ) -> Result<()> {
        let status = unsafe {
            ffi::music::at_music_track_new_midi_note_event(
                self.raw.cast(),
                time_stamp,
                &raw const note_message,
            )
        };
        status_to_result("MusicTrackNewMIDINoteEvent", status)
    }

    /// Wraps `MusicTrackNewMIDIChannelEvent`.
    pub fn new_midi_channel_event(
        &self,
        time_stamp: f64,
        message: &MIDIChannelMessage,
    ) -> Result<()> {
        let status = unsafe {
            ffi::music::at_music_track_new_midi_channel_event(
                self.raw.cast(),
                time_stamp,
                std::ptr::from_ref(message),
            )
        };
        status_to_result("MusicTrackNewMIDIChannelEvent", status)
    }

    /// Wraps `MusicTrackNewMIDIRawDataEvent`.
    pub fn new_midi_raw_data_event(&self, time_stamp: f64, data: &[u8]) -> Result<()> {
        let operation = "MusicTrackNewMIDIRawDataEvent";
        let (_, event) = MusicEvent::MidiRawData(data).encode(operation)?;
        let status = unsafe {
            ffi::music::at_music_track_new_midi_raw_data_event(
                self.raw.cast(),
                time_stamp,
                event.as_ptr(),
            )
        };
        status_to_result(operation, status)
    }

    /// Wraps `MusicTrackNewExtendedNoteEvent`.
    pub fn new_extended_note_event(&self, time_stamp: f64, note: &ExtendedNote<'_>) -> Result<()> {
        let operation = "MusicTrackNewExtendedNoteEvent";
        let (_, event) = MusicEvent::ExtendedNote(*note).encode(operation)?;
        let status = unsafe {
            ffi::music::at_music_track_new_extended_note_event(
                self.raw.cast(),
                time_stamp,
                event.as_ptr(),
            )
        };
        status_to_result(operation, status)
    }

    /// Wraps `MusicTrackNewParameterEvent`.
    pub fn new_parameter_event(&self, time_stamp: f64, event: &ParameterEvent) -> Result<()> {
        let status = unsafe {
            ffi::music::at_music_track_new_parameter_event(
                self.raw.cast(),
                time_stamp,
                std::ptr::from_ref(event),
            )
        };
        status_to_result("MusicTrackNewParameterEvent", status)
    }

    /// Wraps `MusicTrackNewExtendedTempoEvent`.
    pub fn new_extended_tempo_event(&self, time_stamp: f64, bpm: f64) -> Result<()> {
        let status = unsafe {
            ffi::music::at_music_track_new_extended_tempo_event(self.raw.cast(), time_stamp, bpm)
        };
        status_to_result("MusicTrackNewExtendedTempoEvent", status)
    }

    /// Wraps `MusicTrackNewMetaEvent`.
    pub fn new_meta_event(&self, time_stamp: f64, meta_event_type: u8, data: &[u8]) -> Result<()> {
        let operation = "MusicTrackNewMetaEvent";
        let (_, event) = MusicEvent::Meta {
            meta_event_type,
            data,
        }
        .encode(operation)?;
        let status = unsafe {
            ffi::music::at_music_track_new_meta_event(self.raw.cast(), time_stamp, event.as_ptr())
        };
        status_to_result(operation, status)
    }

    /// Wraps `MusicTrackNewUserEvent`.
    pub fn new_user_event(&self, time_stamp: f64, data: &[u8]) -> Result<()> {
        let operation = "MusicTrackNewUserEvent";
        let (_, event) = MusicEvent::User(data).encode(operation)?;
        let status = unsafe {
            ffi::music::at_music_track_new_user_event(self.raw.cast(), time_stamp, event.as_ptr())
        };
        status_to_result(operation, status)
    }

    /// Wraps `MusicTrackNewAUPresetEvent`.
    pub fn new_au_preset_event(
        &self,
        time_stamp: f64,
        scope: AudioUnitScope,
        element: AudioUnitElement,
        preset: &CFDictionary,
    ) -> Result<()> {
        let event = AUPresetEvent {
            scope,
            element,
            preset: preset.as_ptr().cast_const(),
        };
        let status = unsafe {
            ffi::music::at_music_track_new_au_preset_event(
                self.raw.cast(),
                time_stamp,
                &raw const event,
            )
        };
        status_to_result("MusicTrackNewAUPresetEvent", status)
    }

    /// Wraps `MusicTrackEventIterator`.
    pub fn event_iterator(&self) -> Result<MusicEventIterator<'a>> {
        MusicEventIterator::new(self)
    }
}

impl MusicPlayer {
    /// Wraps `NewMusicPlayer`.
    ///
    /// The returned wrapper owns the underlying AudioToolbox.framework handle and releases it on drop.
    pub fn new() -> Result<Self> {
        let mut handle = std::ptr::null_mut();
        let status = unsafe { ffi::music::at_music_player_new(&raw mut handle) };
        status_to_result("NewMusicPlayer", status)?;
        let raw: MusicPlayerRef = unsafe { ffi::music::at_music_player_raw(handle) }.cast();
        if raw.is_null() {
            return Err(AudioToolboxError::message(
                "NewMusicPlayer",
                "framework returned a null MusicPlayer",
            ));
        }
        Ok(Self { handle, raw })
    }

    /// Returns the wrapped `MusicPlayerRef`.
    pub fn as_raw(&self) -> MusicPlayerRef {
        self.raw
    }

    /// Wraps `MusicPlayerSetSequence`.
    pub fn set_sequence(&self, sequence: &MusicSequence) -> Result<()> {
        let _lifecycle = lifecycle_lock();
        let status =
            unsafe { ffi::music::at_music_player_set_sequence(self.handle, sequence.handle) };
        status_to_result("MusicPlayerSetSequence", status)
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn clear_sequence(&self) -> Result<()> {
        let _lifecycle = lifecycle_lock();
        let status =
            unsafe { ffi::music::at_music_player_set_sequence(self.handle, std::ptr::null_mut()) };
        status_to_result("MusicPlayerSetSequence", status)
    }

    /// Wraps `MusicPlayerGetSequence`.
    pub fn sequence_raw(&self) -> Result<MusicSequenceRef> {
        let mut sequence = std::ptr::null_mut();
        let status =
            unsafe { ffi::music::at_music_player_get_sequence(self.raw.cast(), &raw mut sequence) };
        status_to_result("MusicPlayerGetSequence", status)?;
        Ok(sequence)
    }

    /// Wraps `MusicPlayerSetTime`.
    pub fn set_time(&self, time: MusicTimeStamp) -> Result<()> {
        let status = unsafe { ffi::music::at_music_player_set_time(self.raw.cast(), time) };
        status_to_result("MusicPlayerSetTime", status)
    }

    /// Wraps `MusicPlayerGetTime`.
    pub fn time(&self) -> Result<MusicTimeStamp> {
        let mut time = 0.0_f64;
        let status =
            unsafe { ffi::music::at_music_player_get_time(self.raw.cast(), &raw mut time) };
        status_to_result("MusicPlayerGetTime", status)?;
        Ok(time)
    }

    /// Wraps `MusicPlayerGetHostTimeForBeats`.
    pub fn host_time_for_beats(&self, beats: MusicTimeStamp) -> Result<u64> {
        let mut host_time = 0_u64;
        let status = unsafe {
            ffi::music::at_music_player_get_host_time_for_beats(
                self.raw.cast(),
                beats,
                &raw mut host_time,
            )
        };
        status_to_result("MusicPlayerGetHostTimeForBeats", status)?;
        Ok(host_time)
    }

    /// Wraps `MusicPlayerGetBeatsForHostTime`.
    pub fn beats_for_host_time(&self, host_time: u64) -> Result<MusicTimeStamp> {
        let mut beats = 0.0_f64;
        let status = unsafe {
            ffi::music::at_music_player_get_beats_for_host_time(
                self.raw.cast(),
                host_time,
                &raw mut beats,
            )
        };
        status_to_result("MusicPlayerGetBeatsForHostTime", status)?;
        Ok(beats)
    }

    /// Wraps `MusicPlayerPreroll`.
    pub fn preroll(&self) -> Result<()> {
        let _lifecycle = lifecycle_lock();
        let status = unsafe { ffi::music::at_music_player_preroll(self.raw.cast()) };
        status_to_result("MusicPlayerPreroll", status)
    }

    /// Wraps `MusicPlayerStart`.
    pub fn start(&self) -> Result<()> {
        let _lifecycle = lifecycle_lock();
        let status = unsafe { ffi::music::at_music_player_start(self.raw.cast()) };
        status_to_result("MusicPlayerStart", status)
    }

    /// Wraps `MusicPlayerStop`.
    pub fn stop(&self) -> Result<()> {
        let _lifecycle = lifecycle_lock();
        let status = unsafe { ffi::music::at_music_player_stop(self.raw.cast()) };
        status_to_result("MusicPlayerStop", status)
    }

    /// Wraps `MusicPlayerIsPlaying`.
    pub fn is_playing(&self) -> Result<bool> {
        let mut is_playing = 0_u32;
        let status =
            unsafe { ffi::music::at_music_player_is_playing(self.raw.cast(), &raw mut is_playing) };
        status_to_result("MusicPlayerIsPlaying", status)?;
        Ok(is_playing != 0)
    }

    /// Wraps `MusicPlayerSetPlayRateScalar`.
    pub fn set_play_rate_scalar(&self, scale_rate: f64) -> Result<()> {
        let status = unsafe {
            ffi::music::at_music_player_set_play_rate_scalar(self.raw.cast(), scale_rate)
        };
        status_to_result("MusicPlayerSetPlayRateScalar", status)
    }

    /// Wraps `MusicPlayerGetPlayRateScalar`.
    pub fn play_rate_scalar(&self) -> Result<f64> {
        let mut scale_rate = 0.0_f64;
        let status = unsafe {
            ffi::music::at_music_player_get_play_rate_scalar(self.raw.cast(), &raw mut scale_rate)
        };
        status_to_result("MusicPlayerGetPlayRateScalar", status)?;
        Ok(scale_rate)
    }

    /// Wraps `MusicPlayerClose`.
    pub fn close(mut self) -> Result<()> {
        self.release();
        Ok(())
    }

    fn release(&mut self) {
        if !self.handle.is_null() {
            let _lifecycle = lifecycle_lock();
            unsafe { ffi::music::at_music_player_release(self.handle) };
            self.handle = std::ptr::null_mut();
            self.raw = std::ptr::null_mut();
        }
    }
}

impl Drop for MusicPlayer {
    fn drop(&mut self) {
        self.release();
    }
}

impl<'a> MusicEventIterator<'a> {
    /// Wraps `NewMusicEventIterator`.
    ///
    /// The returned wrapper owns the underlying AudioToolbox.framework handle and releases it on drop.
    pub fn new(track: &MusicTrack<'a>) -> Result<Self> {
        let mut raw = std::ptr::null_mut();
        let status =
            unsafe { ffi::music::at_music_event_iterator_new(track.raw.cast(), &raw mut raw) };
        status_to_result("NewMusicEventIterator", status)?;
        if raw.is_null() {
            return Err(AudioToolboxError::message(
                "NewMusicEventIterator",
                "framework returned a null MusicEventIterator",
            ));
        }
        Ok(Self {
            raw,
            _sequence: PhantomData,
        })
    }

    /// Returns the wrapped `MusicEventIteratorRef`.
    pub fn as_raw(&self) -> MusicEventIteratorRef {
        self.raw
    }

    /// Wraps `MusicEventIteratorSeek`.
    pub fn seek(&self, time_stamp: MusicTimeStamp) -> Result<()> {
        let status = unsafe { ffi::music::at_music_event_iterator_seek(self.raw, time_stamp) };
        status_to_result("MusicEventIteratorSeek", status)
    }

    /// Wraps `MusicEventIteratorNextEvent`.
    pub fn next_event(&self) -> Result<()> {
        let status = unsafe { ffi::music::at_music_event_iterator_next_event(self.raw) };
        status_to_result("MusicEventIteratorNextEvent", status)
    }

    /// Wraps `MusicEventIteratorPreviousEvent`.
    pub fn previous_event(&self) -> Result<()> {
        let status = unsafe { ffi::music::at_music_event_iterator_previous_event(self.raw) };
        status_to_result("MusicEventIteratorPreviousEvent", status)
    }

    /// Wraps `MusicEventIteratorGetEventInfo`.
    pub fn event_info(&self) -> Result<MusicEventInfo> {
        let mut time_stamp = 0.0_f64;
        let mut event_type = 0_u32;
        let mut event_data: *const c_void = std::ptr::null();
        let mut event_data_size = 0_u32;
        let status = unsafe {
            ffi::music::at_music_event_iterator_get_event_info(
                self.raw,
                &raw mut time_stamp,
                &raw mut event_type,
                &raw mut event_data,
                &raw mut event_data_size,
            )
        };
        status_to_result("MusicEventIteratorGetEventInfo", status)?;
        let data = if event_data.is_null() || event_data_size == 0 {
            Vec::new()
        } else {
            unsafe { std::slice::from_raw_parts(event_data.cast::<u8>(), event_data_size as usize) }
                .to_vec()
        };
        Ok(MusicEventInfo {
            time_stamp,
            event_type,
            data,
        })
    }

    /// Wraps `MusicEventIteratorSetEventInfo`.
    pub fn set_event_info(&self, event: &MusicEvent<'_>) -> Result<()> {
        let operation = "MusicEventIteratorSetEventInfo";
        let (event_type, bytes) = event.encode(operation)?;
        let status = unsafe {
            ffi::music::at_music_event_iterator_set_event_info(self.raw, event_type, bytes.as_ptr())
        };
        status_to_result(operation, status)
    }

    /// Wraps `MusicEventIteratorSetEventTime`.
    pub fn set_event_time(&self, time_stamp: MusicTimeStamp) -> Result<()> {
        let status =
            unsafe { ffi::music::at_music_event_iterator_set_event_time(self.raw, time_stamp) };
        status_to_result("MusicEventIteratorSetEventTime", status)
    }

    /// Wraps `MusicEventIteratorDeleteEvent`.
    pub fn delete_event(&self) -> Result<()> {
        let status = unsafe { ffi::music::at_music_event_iterator_delete_event(self.raw) };
        status_to_result("MusicEventIteratorDeleteEvent", status)
    }

    /// Wraps `MusicEventIteratorHasPreviousEvent`.
    pub fn has_previous_event(&self) -> Result<bool> {
        let mut has_event = 0_u8;
        let status = unsafe {
            ffi::music::at_music_event_iterator_has_previous_event(self.raw, &raw mut has_event)
        };
        status_to_result("MusicEventIteratorHasPreviousEvent", status)?;
        Ok(has_event != 0)
    }

    /// Wraps `MusicEventIteratorHasNextEvent`.
    pub fn has_next_event(&self) -> Result<bool> {
        let mut has_event = 0_u8;
        let status = unsafe {
            ffi::music::at_music_event_iterator_has_next_event(self.raw, &raw mut has_event)
        };
        status_to_result("MusicEventIteratorHasNextEvent", status)?;
        Ok(has_event != 0)
    }

    /// Wraps `MusicEventIteratorHasCurrentEvent`.
    pub fn has_current_event(&self) -> Result<bool> {
        let mut has_event = 0_u8;
        let status = unsafe {
            ffi::music::at_music_event_iterator_has_current_event(self.raw, &raw mut has_event)
        };
        status_to_result("MusicEventIteratorHasCurrentEvent", status)?;
        Ok(has_event != 0)
    }

    /// Wraps `MusicEventIteratorDispose`.
    pub fn dispose(mut self) -> Result<()> {
        self.release();
        Ok(())
    }

    fn release(&mut self) {
        if !self.raw.is_null() {
            let _ = unsafe { ffi::music::at_music_event_iterator_dispose(self.raw) };
            self.raw = std::ptr::null_mut();
        }
    }
}

impl Drop for MusicEventIterator<'_> {
    fn drop(&mut self) {
        self.release();
    }
}

#[cfg(test)]
mod tests {
    use super::{ExtendedNote, MusicEvent};
    use crate::{
        NoteParamsControlValue, MUSIC_EVENT_TYPE_EXTENDED_NOTE, MUSIC_EVENT_TYPE_META,
        MUSIC_EVENT_TYPE_MIDI_RAW_DATA, MUSIC_EVENT_TYPE_USER,
    };

    #[test]
    fn raw_data_event_carries_its_length_and_every_byte() {
        let payload = [0xF0, 0x7E, 0x7F, 0x09, 0x01, 0xF7];
        let (event_type, bytes) = MusicEvent::MidiRawData(&payload)
            .encode("test")
            .expect("encode");
        assert_eq!(event_type, MUSIC_EVENT_TYPE_MIDI_RAW_DATA);
        let bytes = bytes.bytes();
        assert_eq!(&bytes[..4], &6_u32.to_ne_bytes());
        assert_eq!(&bytes[4..10], &payload);
        assert!(bytes.len() >= std::mem::size_of::<crate::MIDIRawData>() + payload.len());
    }

    #[test]
    fn meta_event_places_the_length_after_the_type_byte() {
        let text = b"a longer meta text than one byte";
        let (event_type, bytes) = MusicEvent::Meta {
            meta_event_type: 0x01,
            data: text,
        }
        .encode("test")
        .expect("encode");
        assert_eq!(event_type, MUSIC_EVENT_TYPE_META);
        let bytes = bytes.bytes();
        assert_eq!(bytes[0], 0x01);
        assert_eq!(
            &bytes[4..8],
            &u32::try_from(text.len()).unwrap().to_ne_bytes()
        );
        assert_eq!(&bytes[8..8 + text.len()], text);
    }

    #[test]
    fn user_event_is_sized_from_the_slice() {
        let (event_type, bytes) = MusicEvent::User(&[]).encode("test").expect("encode");
        assert_eq!(event_type, MUSIC_EVENT_TYPE_USER);
        assert_eq!(&bytes.bytes()[..4], &0_u32.to_ne_bytes());
    }

    #[test]
    fn extended_note_arg_count_follows_the_controls() {
        let controls = [
            NoteParamsControlValue {
                mID: 7,
                mValue: 0.5,
            },
            NoteParamsControlValue {
                mID: 10,
                mValue: 0.25,
            },
        ];
        let (event_type, bytes) = MusicEvent::ExtendedNote(ExtendedNote {
            instrument_id: 1,
            group_id: 2,
            duration: 0.5,
            pitch: 60.0,
            velocity: 100.0,
            controls: &controls,
        })
        .encode("test")
        .expect("encode");
        assert_eq!(event_type, MUSIC_EVENT_TYPE_EXTENDED_NOTE);
        let bytes = bytes.bytes();
        assert_eq!(&bytes[12..16], &4_u32.to_ne_bytes());
        assert_eq!(&bytes[24..28], &7_u32.to_ne_bytes());
        assert_eq!(&bytes[32..36], &10_u32.to_ne_bytes());
        assert!(bytes.len() >= std::mem::size_of::<crate::ExtendedNoteOnEvent>() + 16);
    }
}
