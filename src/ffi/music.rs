use crate::{
    AUNode, AUPresetEvent, Boolean, CABarBeatTime, CFDataRef, CFDictionaryRef, CFURLRef,
    ExtendedNoteOnEvent, MIDIChannelMessage, MIDIEndpointRef, MIDIMetaEvent, MIDINoteMessage,
    MIDIRawData, MusicEventIteratorRef, MusicEventType, MusicEventUserData, MusicSequenceFileFlags,
    MusicSequenceFileTypeId, MusicSequenceLoadFlags, MusicSequenceRef, MusicSequenceType,
    MusicTimeStamp, MusicTrackRef, OSStatus, ParameterEvent,
};
use std::ffi::c_void;

unsafe extern "C" {
    /// Raw binding for `MusicSequenceNew`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicSequenceNew`.
    pub fn at_music_sequence_new(out_handle: *mut *mut c_void) -> OSStatus;
    /// Raw binding for `MusicSequenceRaw`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicSequenceRaw`.
    pub fn at_music_sequence_raw(handle: *mut c_void) -> *mut c_void;
    /// Raw binding for `MusicSequenceRelease`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicSequenceRelease`.
    pub fn at_music_sequence_release(handle: *mut c_void);
    /// Raw binding for `MusicSequenceNewTrack`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicSequenceNewTrack`.
    pub fn at_music_sequence_new_track(
        raw_sequence: *mut c_void,
        out_handle: *mut *mut c_void,
    ) -> OSStatus;
    #[link_name = "MusicSequenceDisposeTrack"]
    /// Raw binding for `MusicSequenceDisposeTrack`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicSequenceDisposeTrack`.
    pub fn at_music_sequence_dispose_track(
        raw_sequence: *mut c_void,
        raw_track: *mut c_void,
    ) -> OSStatus;
    /// Raw binding for `MusicSequenceGetTrackCount`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicSequenceGetTrackCount`.
    pub fn at_music_sequence_get_track_count(
        raw_sequence: *mut c_void,
        out_track_count: *mut u32,
    ) -> OSStatus;
    #[link_name = "MusicSequenceGetIndTrack"]
    /// Raw binding for `MusicSequenceGetIndTrack`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicSequenceGetIndTrack`.
    pub fn at_music_sequence_get_ind_track(
        raw_sequence: *mut c_void,
        track_index: u32,
        out_track: *mut MusicTrackRef,
    ) -> OSStatus;
    #[link_name = "MusicSequenceGetTrackIndex"]
    /// Raw binding for `MusicSequenceGetTrackIndex`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicSequenceGetTrackIndex`.
    pub fn at_music_sequence_get_track_index(
        raw_sequence: *mut c_void,
        raw_track: *mut c_void,
        out_track_index: *mut u32,
    ) -> OSStatus;
    #[link_name = "MusicSequenceGetTempoTrack"]
    /// Raw binding for `MusicSequenceGetTempoTrack`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicSequenceGetTempoTrack`.
    pub fn at_music_sequence_get_tempo_track(
        raw_sequence: *mut c_void,
        out_track: *mut MusicTrackRef,
    ) -> OSStatus;
    #[link_name = "MusicSequenceSetAUGraph"]
    /// Raw binding for `MusicSequenceSetAUGraph`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicSequenceSetAUGraph`.
    pub fn at_music_sequence_set_au_graph(
        raw_sequence: *mut c_void,
        raw_graph: *mut c_void,
    ) -> OSStatus;
    #[link_name = "MusicSequenceGetAUGraph"]
    /// Raw binding for `MusicSequenceGetAUGraph`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicSequenceGetAUGraph`.
    pub fn at_music_sequence_get_au_graph(
        raw_sequence: *mut c_void,
        out_graph: *mut *mut c_void,
    ) -> OSStatus;
    #[link_name = "MusicSequenceSetMIDIEndpoint"]
    /// Raw binding for `MusicSequenceSetMIDIEndpoint`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicSequenceSetMIDIEndpoint`.
    pub fn at_music_sequence_set_midi_endpoint(
        raw_sequence: *mut c_void,
        endpoint: MIDIEndpointRef,
    ) -> OSStatus;
    #[link_name = "MusicSequenceSetSequenceType"]
    /// Raw binding for `MusicSequenceSetSequenceType`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicSequenceSetSequenceType`.
    pub fn at_music_sequence_set_sequence_type(
        raw_sequence: *mut c_void,
        sequence_type: MusicSequenceType,
    ) -> OSStatus;
    #[link_name = "MusicSequenceGetSequenceType"]
    /// Raw binding for `MusicSequenceGetSequenceType`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicSequenceGetSequenceType`.
    pub fn at_music_sequence_get_sequence_type(
        raw_sequence: *mut c_void,
        out_sequence_type: *mut MusicSequenceType,
    ) -> OSStatus;
    #[link_name = "MusicSequenceFileLoad"]
    /// Raw binding for `MusicSequenceFileLoad`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicSequenceFileLoad`.
    pub fn at_music_sequence_file_load(
        raw_sequence: *mut c_void,
        file_ref: CFURLRef,
        file_type_hint: MusicSequenceFileTypeId,
        flags: MusicSequenceLoadFlags,
    ) -> OSStatus;
    #[link_name = "MusicSequenceFileLoadData"]
    /// Raw binding for `MusicSequenceFileLoadData`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicSequenceFileLoadData`.
    pub fn at_music_sequence_file_load_data(
        raw_sequence: *mut c_void,
        data: CFDataRef,
        file_type_hint: MusicSequenceFileTypeId,
        flags: MusicSequenceLoadFlags,
    ) -> OSStatus;
    #[link_name = "MusicSequenceFileCreate"]
    /// Raw binding for `MusicSequenceFileCreate`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicSequenceFileCreate`.
    pub fn at_music_sequence_file_create(
        raw_sequence: *mut c_void,
        file_ref: CFURLRef,
        file_type: MusicSequenceFileTypeId,
        flags: MusicSequenceFileFlags,
        resolution: i16,
    ) -> OSStatus;
    #[link_name = "MusicSequenceFileCreateData"]
    /// Raw binding for `MusicSequenceFileCreateData`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicSequenceFileCreateData`.
    pub fn at_music_sequence_file_create_data(
        raw_sequence: *mut c_void,
        file_type: MusicSequenceFileTypeId,
        flags: MusicSequenceFileFlags,
        resolution: i16,
        out_data: *mut CFDataRef,
    ) -> OSStatus;
    #[link_name = "MusicSequenceGetSecondsForBeats"]
    /// Raw binding for `MusicSequenceGetSecondsForBeats`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicSequenceGetSecondsForBeats`.
    pub fn at_music_sequence_get_seconds_for_beats(
        raw_sequence: *mut c_void,
        beats: MusicTimeStamp,
        out_seconds: *mut f64,
    ) -> OSStatus;
    #[link_name = "MusicSequenceGetBeatsForSeconds"]
    /// Raw binding for `MusicSequenceGetBeatsForSeconds`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicSequenceGetBeatsForSeconds`.
    pub fn at_music_sequence_get_beats_for_seconds(
        raw_sequence: *mut c_void,
        seconds: f64,
        out_beats: *mut MusicTimeStamp,
    ) -> OSStatus;
    #[link_name = "MusicSequenceBeatsToBarBeatTime"]
    /// Raw binding for `MusicSequenceBeatsToBarBeatTime`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicSequenceBeatsToBarBeatTime`.
    pub fn at_music_sequence_beats_to_bar_beat_time(
        raw_sequence: *mut c_void,
        beats: MusicTimeStamp,
        subbeat_divisor: u32,
        out_bar_beat_time: *mut CABarBeatTime,
    ) -> OSStatus;
    #[link_name = "MusicSequenceBarBeatTimeToBeats"]
    /// Raw binding for `MusicSequenceBarBeatTimeToBeats`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicSequenceBarBeatTimeToBeats`.
    pub fn at_music_sequence_bar_beat_time_to_beats(
        raw_sequence: *mut c_void,
        bar_beat_time: *const CABarBeatTime,
        out_beats: *mut MusicTimeStamp,
    ) -> OSStatus;
    #[link_name = "MusicSequenceGetInfoDictionary"]
    /// Raw binding for `MusicSequenceGetInfoDictionary`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicSequenceGetInfoDictionary`.
    pub fn at_music_sequence_get_info_dictionary(raw_sequence: *mut c_void) -> CFDictionaryRef;

    /// Raw binding for `MusicTrackRaw`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicTrackRaw`.
    pub fn at_music_track_raw(handle: *mut c_void) -> *mut c_void;
    /// Raw binding for `MusicTrackRelease`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicTrackRelease`.
    pub fn at_music_track_release(handle: *mut c_void);
    #[link_name = "MusicTrackGetSequence"]
    /// Raw binding for `MusicTrackGetSequence`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicTrackGetSequence`.
    pub fn at_music_track_get_sequence(
        raw_track: *mut c_void,
        out_sequence: *mut MusicSequenceRef,
    ) -> OSStatus;
    #[link_name = "MusicTrackSetDestNode"]
    /// Raw binding for `MusicTrackSetDestNode`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicTrackSetDestNode`.
    pub fn at_music_track_set_dest_node(raw_track: *mut c_void, node: AUNode) -> OSStatus;
    #[link_name = "MusicTrackSetDestMIDIEndpoint"]
    /// Raw binding for `MusicTrackSetDestMIDIEndpoint`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicTrackSetDestMIDIEndpoint`.
    pub fn at_music_track_set_dest_midi_endpoint(
        raw_track: *mut c_void,
        endpoint: MIDIEndpointRef,
    ) -> OSStatus;
    #[link_name = "MusicTrackGetDestNode"]
    /// Raw binding for `MusicTrackGetDestNode`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicTrackGetDestNode`.
    pub fn at_music_track_get_dest_node(raw_track: *mut c_void, out_node: *mut AUNode) -> OSStatus;
    #[link_name = "MusicTrackGetDestMIDIEndpoint"]
    /// Raw binding for `MusicTrackGetDestMIDIEndpoint`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicTrackGetDestMIDIEndpoint`.
    pub fn at_music_track_get_dest_midi_endpoint(
        raw_track: *mut c_void,
        out_endpoint: *mut MIDIEndpointRef,
    ) -> OSStatus;
    #[link_name = "MusicTrackSetProperty"]
    /// Raw binding for `MusicTrackSetProperty`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicTrackSetProperty`.
    pub fn at_music_track_set_property(
        raw_track: *mut c_void,
        property_id: u32,
        data: *const c_void,
        length: u32,
    ) -> OSStatus;
    #[link_name = "MusicTrackGetProperty"]
    /// Raw binding for `MusicTrackGetProperty`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicTrackGetProperty`.
    pub fn at_music_track_get_property(
        raw_track: *mut c_void,
        property_id: u32,
        out_data: *mut c_void,
        io_length: *mut u32,
    ) -> OSStatus;
    #[link_name = "MusicTrackMoveEvents"]
    /// Raw binding for `MusicTrackMoveEvents`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicTrackMoveEvents`.
    pub fn at_music_track_move_events(
        raw_track: *mut c_void,
        start_time: MusicTimeStamp,
        end_time: MusicTimeStamp,
        move_time: MusicTimeStamp,
    ) -> OSStatus;
    #[link_name = "MusicTrackClear"]
    /// Raw binding for `MusicTrackClear`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicTrackClear`.
    pub fn at_music_track_clear(
        raw_track: *mut c_void,
        start_time: MusicTimeStamp,
        end_time: MusicTimeStamp,
    ) -> OSStatus;
    #[link_name = "MusicTrackCut"]
    /// Raw binding for `MusicTrackCut`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicTrackCut`.
    pub fn at_music_track_cut(
        raw_track: *mut c_void,
        start_time: MusicTimeStamp,
        end_time: MusicTimeStamp,
    ) -> OSStatus;
    #[link_name = "MusicTrackCopyInsert"]
    /// Raw binding for `MusicTrackCopyInsert`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicTrackCopyInsert`.
    pub fn at_music_track_copy_insert(
        source_track: *mut c_void,
        source_start_time: MusicTimeStamp,
        source_end_time: MusicTimeStamp,
        dest_track: *mut c_void,
        dest_insert_time: MusicTimeStamp,
    ) -> OSStatus;
    #[link_name = "MusicTrackMerge"]
    /// Raw binding for `MusicTrackMerge`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicTrackMerge`.
    pub fn at_music_track_merge(
        source_track: *mut c_void,
        source_start_time: MusicTimeStamp,
        source_end_time: MusicTimeStamp,
        dest_track: *mut c_void,
        dest_insert_time: MusicTimeStamp,
    ) -> OSStatus;
    /// Raw binding for `MusicTrackNewMIDINoteEvent`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicTrackNewMIDINoteEvent`.
    pub fn at_music_track_new_midi_note_event(
        raw_track: *mut c_void,
        time_stamp: f64,
        note_message: *const MIDINoteMessage,
    ) -> OSStatus;
    #[link_name = "MusicTrackNewMIDIChannelEvent"]
    /// Raw binding for `MusicTrackNewMIDIChannelEvent`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicTrackNewMIDIChannelEvent`.
    pub fn at_music_track_new_midi_channel_event(
        raw_track: *mut c_void,
        time_stamp: f64,
        message: *const MIDIChannelMessage,
    ) -> OSStatus;
    #[link_name = "MusicTrackNewMIDIRawDataEvent"]
    /// Raw binding for `MusicTrackNewMIDIRawDataEvent`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicTrackNewMIDIRawDataEvent`.
    pub fn at_music_track_new_midi_raw_data_event(
        raw_track: *mut c_void,
        time_stamp: f64,
        raw_data: *const MIDIRawData,
    ) -> OSStatus;
    #[link_name = "MusicTrackNewExtendedNoteEvent"]
    /// Raw binding for `MusicTrackNewExtendedNoteEvent`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicTrackNewExtendedNoteEvent`.
    pub fn at_music_track_new_extended_note_event(
        raw_track: *mut c_void,
        time_stamp: f64,
        note_event: *const ExtendedNoteOnEvent,
    ) -> OSStatus;
    #[link_name = "MusicTrackNewParameterEvent"]
    /// Raw binding for `MusicTrackNewParameterEvent`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicTrackNewParameterEvent`.
    pub fn at_music_track_new_parameter_event(
        raw_track: *mut c_void,
        time_stamp: f64,
        event: *const ParameterEvent,
    ) -> OSStatus;
    #[link_name = "MusicTrackNewExtendedTempoEvent"]
    /// Raw binding for `MusicTrackNewExtendedTempoEvent`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicTrackNewExtendedTempoEvent`.
    pub fn at_music_track_new_extended_tempo_event(
        raw_track: *mut c_void,
        time_stamp: f64,
        bpm: f64,
    ) -> OSStatus;
    #[link_name = "MusicTrackNewMetaEvent"]
    /// Raw binding for `MusicTrackNewMetaEvent`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicTrackNewMetaEvent`.
    pub fn at_music_track_new_meta_event(
        raw_track: *mut c_void,
        time_stamp: f64,
        event: *const MIDIMetaEvent,
    ) -> OSStatus;
    #[link_name = "MusicTrackNewUserEvent"]
    /// Raw binding for `MusicTrackNewUserEvent`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicTrackNewUserEvent`.
    pub fn at_music_track_new_user_event(
        raw_track: *mut c_void,
        time_stamp: f64,
        event: *const MusicEventUserData,
    ) -> OSStatus;
    #[link_name = "MusicTrackNewAUPresetEvent"]
    /// Raw binding for `MusicTrackNewAUPresetEvent`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicTrackNewAUPresetEvent`.
    pub fn at_music_track_new_au_preset_event(
        raw_track: *mut c_void,
        time_stamp: f64,
        event: *const AUPresetEvent,
    ) -> OSStatus;

    #[link_name = "NewMusicEventIterator"]
    /// Raw binding for `MusicEventIteratorNew`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicEventIteratorNew`.
    pub fn at_music_event_iterator_new(
        raw_track: *mut c_void,
        out_iterator: *mut MusicEventIteratorRef,
    ) -> OSStatus;
    #[link_name = "DisposeMusicEventIterator"]
    /// Raw binding for `MusicEventIteratorDispose`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicEventIteratorDispose`.
    pub fn at_music_event_iterator_dispose(iterator: MusicEventIteratorRef) -> OSStatus;
    #[link_name = "MusicEventIteratorSeek"]
    /// Raw binding for `MusicEventIteratorSeek`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicEventIteratorSeek`.
    pub fn at_music_event_iterator_seek(
        iterator: MusicEventIteratorRef,
        time_stamp: MusicTimeStamp,
    ) -> OSStatus;
    #[link_name = "MusicEventIteratorNextEvent"]
    /// Raw binding for `MusicEventIteratorNextEvent`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicEventIteratorNextEvent`.
    pub fn at_music_event_iterator_next_event(iterator: MusicEventIteratorRef) -> OSStatus;
    #[link_name = "MusicEventIteratorPreviousEvent"]
    /// Raw binding for `MusicEventIteratorPreviousEvent`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicEventIteratorPreviousEvent`.
    pub fn at_music_event_iterator_previous_event(iterator: MusicEventIteratorRef) -> OSStatus;
    #[link_name = "MusicEventIteratorGetEventInfo"]
    /// Raw binding for `MusicEventIteratorGetEventInfo`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicEventIteratorGetEventInfo`.
    pub fn at_music_event_iterator_get_event_info(
        iterator: MusicEventIteratorRef,
        out_time_stamp: *mut MusicTimeStamp,
        out_event_type: *mut MusicEventType,
        out_event_data: *mut *const c_void,
        out_event_data_size: *mut u32,
    ) -> OSStatus;
    #[link_name = "MusicEventIteratorSetEventInfo"]
    /// Raw binding for `MusicEventIteratorSetEventInfo`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicEventIteratorSetEventInfo`.
    pub fn at_music_event_iterator_set_event_info(
        iterator: MusicEventIteratorRef,
        event_type: MusicEventType,
        event_data: *const c_void,
    ) -> OSStatus;
    #[link_name = "MusicEventIteratorSetEventTime"]
    /// Raw binding for `MusicEventIteratorSetEventTime`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicEventIteratorSetEventTime`.
    pub fn at_music_event_iterator_set_event_time(
        iterator: MusicEventIteratorRef,
        time_stamp: MusicTimeStamp,
    ) -> OSStatus;
    #[link_name = "MusicEventIteratorDeleteEvent"]
    /// Raw binding for `MusicEventIteratorDeleteEvent`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicEventIteratorDeleteEvent`.
    pub fn at_music_event_iterator_delete_event(iterator: MusicEventIteratorRef) -> OSStatus;
    #[link_name = "MusicEventIteratorHasPreviousEvent"]
    /// Raw binding for `MusicEventIteratorHasPreviousEvent`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicEventIteratorHasPreviousEvent`.
    pub fn at_music_event_iterator_has_previous_event(
        iterator: MusicEventIteratorRef,
        out_has_previous_event: *mut Boolean,
    ) -> OSStatus;
    #[link_name = "MusicEventIteratorHasNextEvent"]
    /// Raw binding for `MusicEventIteratorHasNextEvent`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicEventIteratorHasNextEvent`.
    pub fn at_music_event_iterator_has_next_event(
        iterator: MusicEventIteratorRef,
        out_has_next_event: *mut Boolean,
    ) -> OSStatus;
    #[link_name = "MusicEventIteratorHasCurrentEvent"]
    /// Raw binding for `MusicEventIteratorHasCurrentEvent`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicEventIteratorHasCurrentEvent`.
    pub fn at_music_event_iterator_has_current_event(
        iterator: MusicEventIteratorRef,
        out_has_current_event: *mut Boolean,
    ) -> OSStatus;

    /// Raw binding for `MusicPlayerNew`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicPlayerNew`.
    pub fn at_music_player_new(out_handle: *mut *mut c_void) -> OSStatus;
    /// Raw binding for `MusicPlayerRaw`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicPlayerRaw`.
    pub fn at_music_player_raw(handle: *mut c_void) -> *mut c_void;
    /// Raw binding for `MusicPlayerRelease`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicPlayerRelease`.
    pub fn at_music_player_release(handle: *mut c_void);
    /// Raw binding for `MusicPlayerSetSequence`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicPlayerSetSequence`.
    pub fn at_music_player_set_sequence(
        raw_player: *mut c_void,
        raw_sequence: *mut c_void,
    ) -> OSStatus;
    #[link_name = "MusicPlayerGetSequence"]
    /// Raw binding for `MusicPlayerGetSequence`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicPlayerGetSequence`.
    pub fn at_music_player_get_sequence(
        raw_player: *mut c_void,
        out_sequence: *mut MusicSequenceRef,
    ) -> OSStatus;
    #[link_name = "MusicPlayerSetTime"]
    /// Raw binding for `MusicPlayerSetTime`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicPlayerSetTime`.
    pub fn at_music_player_set_time(raw_player: *mut c_void, time: MusicTimeStamp) -> OSStatus;
    #[link_name = "MusicPlayerGetTime"]
    /// Raw binding for `MusicPlayerGetTime`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicPlayerGetTime`.
    pub fn at_music_player_get_time(
        raw_player: *mut c_void,
        out_time: *mut MusicTimeStamp,
    ) -> OSStatus;
    #[link_name = "MusicPlayerGetHostTimeForBeats"]
    /// Raw binding for `MusicPlayerGetHostTimeForBeats`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicPlayerGetHostTimeForBeats`.
    pub fn at_music_player_get_host_time_for_beats(
        raw_player: *mut c_void,
        beats: MusicTimeStamp,
        out_host_time: *mut u64,
    ) -> OSStatus;
    #[link_name = "MusicPlayerGetBeatsForHostTime"]
    /// Raw binding for `MusicPlayerGetBeatsForHostTime`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicPlayerGetBeatsForHostTime`.
    pub fn at_music_player_get_beats_for_host_time(
        raw_player: *mut c_void,
        host_time: u64,
        out_beats: *mut MusicTimeStamp,
    ) -> OSStatus;
    /// Raw binding for `MusicPlayerPreroll`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicPlayerPreroll`.
    pub fn at_music_player_preroll(raw_player: *mut c_void) -> OSStatus;
    /// Raw binding for `MusicPlayerStart`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicPlayerStart`.
    pub fn at_music_player_start(raw_player: *mut c_void) -> OSStatus;
    /// Raw binding for `MusicPlayerStop`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicPlayerStop`.
    pub fn at_music_player_stop(raw_player: *mut c_void) -> OSStatus;
    /// Raw binding for `MusicPlayerIsPlaying`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicPlayerIsPlaying`.
    pub fn at_music_player_is_playing(
        raw_player: *mut c_void,
        out_is_playing: *mut u32,
    ) -> OSStatus;
    #[link_name = "MusicPlayerSetPlayRateScalar"]
    /// Raw binding for `MusicPlayerSetPlayRateScalar`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicPlayerSetPlayRateScalar`.
    pub fn at_music_player_set_play_rate_scalar(
        raw_player: *mut c_void,
        scale_rate: f64,
    ) -> OSStatus;
    #[link_name = "MusicPlayerGetPlayRateScalar"]
    /// Raw binding for `MusicPlayerGetPlayRateScalar`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `MusicPlayerGetPlayRateScalar`.
    pub fn at_music_player_get_play_rate_scalar(
        raw_player: *mut c_void,
        out_scale_rate: *mut f64,
    ) -> OSStatus;
}
