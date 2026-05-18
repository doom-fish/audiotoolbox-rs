use crate::{
    ffi,
    internal::{
        cf_data_from_bytes, cf_data_to_vec, cf_release, cf_url_from_path, status_to_result,
    },
    AUGraph, AUNode, AUPresetEvent, AudioToolboxError, CABarBeatTime, CFDictionaryRef,
    ExtendedNoteOnEvent, MIDIChannelMessage, MIDIEndpointRef, MIDIMetaEvent, MIDINoteMessage,
    MIDIRawData, MusicEventIteratorRef, MusicEventType, MusicEventUserData, MusicPlayerRef,
    MusicSequenceFileFlags, MusicSequenceFileTypeId, MusicSequenceLoadFlags, MusicSequenceRef,
    MusicSequenceType, MusicTimeStamp, MusicTrackRef, ParameterEvent, Result,
};
use std::{ffi::c_void, mem::MaybeUninit, path::Path};

#[derive(Debug)]
/// Owning wrapper around an AudioToolbox.framework `MusicSequence`.
pub struct MusicSequence {
    handle: *mut c_void,
    raw: MusicSequenceRef,
}

#[derive(Debug, Clone, Copy)]
/// Wrapper around an AudioToolbox.framework `MusicTrack`.
pub struct MusicTrack {
    raw: MusicTrackRef,
}

#[derive(Debug)]
/// Owning wrapper around an AudioToolbox.framework `MusicPlayer`.
pub struct MusicPlayer {
    handle: *mut c_void,
    raw: MusicPlayerRef,
}

#[derive(Debug)]
/// Owning wrapper around an AudioToolbox.framework `MusicEventIterator`.
pub struct MusicEventIterator {
    raw: MusicEventIteratorRef,
}

#[derive(Debug, Clone, Copy)]
/// Event snapshot returned by `MusicEventIteratorGetEventInfo`.
pub struct MusicEventInfo {
    /// Wraps `kCVoid`.
    pub time_stamp: MusicTimeStamp,
    /// Wraps `kCVoid`.
    pub event_type: MusicEventType,
    /// Wraps `kCVoid`.
    pub event_data: *const c_void,
    pub event_data_size: u32,
}

impl MusicSequence {
    /// Wraps `NewMusicSequence`.
    ///
    /// The returned wrapper owns the underlying AudioToolbox.framework handle and releases it on drop.
    pub fn new() -> Result<Self> {
        let mut handle = std::ptr::null_mut();
        let status = unsafe { ffi::music::at_music_sequence_new(&mut handle) };
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
    pub fn new_track(&self) -> Result<MusicTrack> {
        let mut handle = std::ptr::null_mut();
        let status =
            unsafe { ffi::music::at_music_sequence_new_track(self.raw.cast(), &mut handle) };
        status_to_result("MusicSequenceNewTrack", status)?;
        let raw: MusicTrackRef = unsafe { ffi::music::at_music_track_raw(handle) }.cast();
        unsafe { ffi::music::at_music_track_release(handle) };
        if raw.is_null() {
            return Err(AudioToolboxError::message(
                "MusicSequenceNewTrack",
                "framework returned a null MusicTrack",
            ));
        }
        Ok(MusicTrack { raw })
    }

    /// Wraps `MusicSequenceDisposeTrack`.
    pub fn dispose_track(&self, track: MusicTrack) -> Result<()> {
        let status = unsafe {
            ffi::music::at_music_sequence_dispose_track(self.raw.cast(), track.raw.cast())
        };
        status_to_result("MusicSequenceDisposeTrack", status)
    }

    /// Wraps `MusicSequenceGetTrackCount`.
    pub fn track_count(&self) -> Result<u32> {
        let mut track_count = 0_u32;
        let status = unsafe {
            ffi::music::at_music_sequence_get_track_count(self.raw.cast(), &mut track_count)
        };
        status_to_result("MusicSequenceGetTrackCount", status)?;
        Ok(track_count)
    }

    /// Wraps `MusicSequenceGetIndTrack`.
    pub fn track(&self, index: u32) -> Result<MusicTrack> {
        let mut raw = MaybeUninit::<MusicTrackRef>::uninit();
        let status = unsafe {
            ffi::music::at_music_sequence_get_ind_track(
                self.raw.cast(),
                index,
                raw.as_mut_ptr().cast(),
            )
        };
        status_to_result("MusicSequenceGetIndTrack", status)?;
        let raw = unsafe { raw.assume_init() };
        if raw.is_null() {
            return Err(AudioToolboxError::message(
                "MusicSequenceGetIndTrack",
                "framework returned a null MusicTrack",
            ));
        }
        Ok(MusicTrack { raw })
    }

    /// Wraps `MusicSequenceGetTrackIndex`.
    pub fn track_index(&self, track: MusicTrack) -> Result<u32> {
        let mut track_index = 0_u32;
        let status = unsafe {
            ffi::music::at_music_sequence_get_track_index(
                self.raw.cast(),
                track.raw.cast(),
                &mut track_index,
            )
        };
        status_to_result("MusicSequenceGetTrackIndex", status)?;
        Ok(track_index)
    }

    /// Wraps `MusicSequenceGetTempoTrack`.
    pub fn tempo_track(&self) -> Result<MusicTrack> {
        let mut raw = MaybeUninit::<MusicTrackRef>::uninit();
        let status = unsafe {
            ffi::music::at_music_sequence_get_tempo_track(self.raw.cast(), raw.as_mut_ptr().cast())
        };
        status_to_result("MusicSequenceGetTempoTrack", status)?;
        let raw = unsafe { raw.assume_init() };
        if raw.is_null() {
            return Err(AudioToolboxError::message(
                "MusicSequenceGetTempoTrack",
                "framework returned a null MusicTrack",
            ));
        }
        Ok(MusicTrack { raw })
    }

    /// Wraps `MusicSequenceSetAUGraph`.
    pub fn set_au_graph(&self, graph: &AUGraph) -> Result<()> {
        let status =
            unsafe { ffi::music::at_music_sequence_set_au_graph(self.raw.cast(), graph.as_raw()) };
        status_to_result("MusicSequenceSetAUGraph", status)
    }

    /// Wraps `MusicSequenceGetAUGraph`.
    pub fn au_graph_raw(&self) -> Result<*mut c_void> {
        let mut graph = std::ptr::null_mut();
        let status =
            unsafe { ffi::music::at_music_sequence_get_au_graph(self.raw.cast(), &mut graph) };
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
            ffi::music::at_music_sequence_get_sequence_type(self.raw.cast(), &mut sequence_type)
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
                &mut data,
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
                &mut seconds,
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
                &mut beats,
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
                &mut beats,
            )
        };
        status_to_result("MusicSequenceBarBeatTimeToBeats", status)?;
        Ok(beats)
    }

    /// Wraps `MusicSequenceGetInfoDictionary`.
    pub fn info_dictionary_raw(&self) -> CFDictionaryRef {
        unsafe { ffi::music::at_music_sequence_get_info_dictionary(self.raw.cast()) }
    }

    /// Wraps `MusicSequenceClose`.
    pub fn close(mut self) -> Result<()> {
        self.release();
        Ok(())
    }

    fn release(&mut self) {
        if !self.handle.is_null() {
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

impl MusicTrack {
    /// Returns the wrapped `MusicTrackRef`.
    pub fn as_raw(&self) -> MusicTrackRef {
        self.raw
    }

    /// Wraps `MusicTrackGetSequence`.
    pub fn sequence_raw(&self) -> Result<MusicSequenceRef> {
        let mut sequence = std::ptr::null_mut();
        let status =
            unsafe { ffi::music::at_music_track_get_sequence(self.raw.cast(), &mut sequence) };
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
            unsafe { ffi::music::at_music_track_get_dest_node(self.raw.cast(), &mut node) };
        status_to_result("MusicTrackGetDestNode", status)?;
        Ok(node)
    }

    /// Wraps `MusicTrackGetDestMIDIEndpoint`.
    pub fn dest_midi_endpoint(&self) -> Result<MIDIEndpointRef> {
        let mut endpoint = 0_u32;
        let status = unsafe {
            ffi::music::at_music_track_get_dest_midi_endpoint(self.raw.cast(), &mut endpoint)
        };
        status_to_result("MusicTrackGetDestMIDIEndpoint", status)?;
        Ok(endpoint)
    }

    /// Wraps `MusicTrackGetProperty`.
    pub fn get_property_typed<T: Copy>(&self, property_id: u32) -> Result<T> {
        let mut value = MaybeUninit::<T>::uninit();
        let mut length =
            u32::try_from(std::mem::size_of::<T>()).expect("typed property fits in u32");
        let status = unsafe {
            ffi::music::at_music_track_get_property(
                self.raw.cast(),
                property_id,
                value.as_mut_ptr().cast(),
                &mut length,
            )
        };
        status_to_result("MusicTrackGetProperty", status)?;
        Ok(unsafe { value.assume_init() })
    }

    /// Wraps `MusicTrackSetProperty`.
    pub fn set_property_typed<T: Copy>(&self, property_id: u32, value: &T) -> Result<()> {
        let length = u32::try_from(std::mem::size_of::<T>()).expect("typed property fits in u32");
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
        dest_track: MusicTrack,
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
        dest_track: MusicTrack,
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
                std::ptr::from_ref(&note_message),
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
    pub fn new_midi_raw_data_event(&self, time_stamp: f64, raw_data: &MIDIRawData) -> Result<()> {
        let status = unsafe {
            ffi::music::at_music_track_new_midi_raw_data_event(
                self.raw.cast(),
                time_stamp,
                std::ptr::from_ref(raw_data),
            )
        };
        status_to_result("MusicTrackNewMIDIRawDataEvent", status)
    }

    /// Wraps `MusicTrackNewExtendedNoteEvent`.
    pub fn new_extended_note_event(
        &self,
        time_stamp: f64,
        event: &ExtendedNoteOnEvent,
    ) -> Result<()> {
        let status = unsafe {
            ffi::music::at_music_track_new_extended_note_event(
                self.raw.cast(),
                time_stamp,
                std::ptr::from_ref(event),
            )
        };
        status_to_result("MusicTrackNewExtendedNoteEvent", status)
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
    pub fn new_meta_event(&self, time_stamp: f64, event: &MIDIMetaEvent) -> Result<()> {
        let status = unsafe {
            ffi::music::at_music_track_new_meta_event(
                self.raw.cast(),
                time_stamp,
                std::ptr::from_ref(event),
            )
        };
        status_to_result("MusicTrackNewMetaEvent", status)
    }

    /// Wraps `MusicTrackNewUserEvent`.
    pub fn new_user_event(&self, time_stamp: f64, event: &MusicEventUserData) -> Result<()> {
        let status = unsafe {
            ffi::music::at_music_track_new_user_event(
                self.raw.cast(),
                time_stamp,
                std::ptr::from_ref(event),
            )
        };
        status_to_result("MusicTrackNewUserEvent", status)
    }

    /// Wraps `MusicTrackNewAUPresetEvent`.
    pub fn new_au_preset_event(&self, time_stamp: f64, event: &AUPresetEvent) -> Result<()> {
        let status = unsafe {
            ffi::music::at_music_track_new_au_preset_event(
                self.raw.cast(),
                time_stamp,
                std::ptr::from_ref(event),
            )
        };
        status_to_result("MusicTrackNewAUPresetEvent", status)
    }

    /// Wraps `MusicTrackEventIterator`.
    pub fn event_iterator(&self) -> Result<MusicEventIterator> {
        MusicEventIterator::new(*self)
    }
}

impl MusicPlayer {
    /// Wraps `NewMusicPlayer`.
    ///
    /// The returned wrapper owns the underlying AudioToolbox.framework handle and releases it on drop.
    pub fn new() -> Result<Self> {
        let mut handle = std::ptr::null_mut();
        let status = unsafe { ffi::music::at_music_player_new(&mut handle) };
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
        let status = unsafe {
            ffi::music::at_music_player_set_sequence(self.raw.cast(), sequence.raw.cast())
        };
        status_to_result("MusicPlayerSetSequence", status)
    }

    /// Wraps `MusicPlayerGetSequence`.
    pub fn sequence_raw(&self) -> Result<MusicSequenceRef> {
        let mut sequence = std::ptr::null_mut();
        let status =
            unsafe { ffi::music::at_music_player_get_sequence(self.raw.cast(), &mut sequence) };
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
        let status = unsafe { ffi::music::at_music_player_get_time(self.raw.cast(), &mut time) };
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
                &mut host_time,
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
                &mut beats,
            )
        };
        status_to_result("MusicPlayerGetBeatsForHostTime", status)?;
        Ok(beats)
    }

    /// Wraps `MusicPlayerPreroll`.
    pub fn preroll(&self) -> Result<()> {
        let status = unsafe { ffi::music::at_music_player_preroll(self.raw.cast()) };
        status_to_result("MusicPlayerPreroll", status)
    }

    /// Wraps `MusicPlayerStart`.
    pub fn start(&self) -> Result<()> {
        let status = unsafe { ffi::music::at_music_player_start(self.raw.cast()) };
        status_to_result("MusicPlayerStart", status)
    }

    /// Wraps `MusicPlayerStop`.
    pub fn stop(&self) -> Result<()> {
        let status = unsafe { ffi::music::at_music_player_stop(self.raw.cast()) };
        status_to_result("MusicPlayerStop", status)
    }

    /// Wraps `MusicPlayerIsPlaying`.
    pub fn is_playing(&self) -> Result<bool> {
        let mut is_playing = 0_u32;
        let status =
            unsafe { ffi::music::at_music_player_is_playing(self.raw.cast(), &mut is_playing) };
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
            ffi::music::at_music_player_get_play_rate_scalar(self.raw.cast(), &mut scale_rate)
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

impl MusicEventIterator {
    /// Wraps `NewMusicEventIterator`.
    ///
    /// The returned wrapper owns the underlying AudioToolbox.framework handle and releases it on drop.
    pub fn new(track: MusicTrack) -> Result<Self> {
        let mut raw = std::ptr::null_mut();
        let status = unsafe { ffi::music::at_music_event_iterator_new(track.raw.cast(), &mut raw) };
        status_to_result("NewMusicEventIterator", status)?;
        if raw.is_null() {
            return Err(AudioToolboxError::message(
                "NewMusicEventIterator",
                "framework returned a null MusicEventIterator",
            ));
        }
        Ok(Self { raw })
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
        let mut event_data = std::ptr::null();
        let mut event_data_size = 0_u32;
        let status = unsafe {
            ffi::music::at_music_event_iterator_get_event_info(
                self.raw,
                &mut time_stamp,
                &mut event_type,
                &mut event_data,
                &mut event_data_size,
            )
        };
        status_to_result("MusicEventIteratorGetEventInfo", status)?;
        Ok(MusicEventInfo {
            time_stamp,
            event_type,
            event_data,
            event_data_size,
        })
    }

    /// Wraps `MusicEventIteratorSetEventInfo`.
    pub fn set_event_info<T>(&self, event_type: MusicEventType, event_data: &T) -> Result<()> {
        let status = unsafe {
            ffi::music::at_music_event_iterator_set_event_info(
                self.raw,
                event_type,
                std::ptr::from_ref(event_data).cast(),
            )
        };
        status_to_result("MusicEventIteratorSetEventInfo", status)
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
            ffi::music::at_music_event_iterator_has_previous_event(self.raw, &mut has_event)
        };
        status_to_result("MusicEventIteratorHasPreviousEvent", status)?;
        Ok(has_event != 0)
    }

    /// Wraps `MusicEventIteratorHasNextEvent`.
    pub fn has_next_event(&self) -> Result<bool> {
        let mut has_event = 0_u8;
        let status =
            unsafe { ffi::music::at_music_event_iterator_has_next_event(self.raw, &mut has_event) };
        status_to_result("MusicEventIteratorHasNextEvent", status)?;
        Ok(has_event != 0)
    }

    /// Wraps `MusicEventIteratorHasCurrentEvent`.
    pub fn has_current_event(&self) -> Result<bool> {
        let mut has_event = 0_u8;
        let status = unsafe {
            ffi::music::at_music_event_iterator_has_current_event(self.raw, &mut has_event)
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

impl Drop for MusicEventIterator {
    fn drop(&mut self) {
        self.release();
    }
}
