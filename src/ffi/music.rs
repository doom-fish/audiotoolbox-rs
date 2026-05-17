use crate::{
    AUNode, AUPresetEvent, Boolean, CABarBeatTime, CFDataRef, CFDictionaryRef, CFURLRef,
    ExtendedNoteOnEvent, MIDIChannelMessage, MIDIEndpointRef, MIDIMetaEvent, MIDINoteMessage,
    MIDIRawData, MusicEventIteratorRef, MusicEventType, MusicEventUserData, MusicSequenceFileFlags,
    MusicSequenceFileTypeId, MusicSequenceLoadFlags, MusicSequenceRef, MusicSequenceType,
    MusicTimeStamp, MusicTrackRef, OSStatus, ParameterEvent,
};
use std::ffi::c_void;

unsafe extern "C" {
    pub fn at_music_sequence_new(out_handle: *mut *mut c_void) -> OSStatus;
    pub fn at_music_sequence_raw(handle: *mut c_void) -> *mut c_void;
    pub fn at_music_sequence_release(handle: *mut c_void);
    pub fn at_music_sequence_new_track(
        raw_sequence: *mut c_void,
        out_handle: *mut *mut c_void,
    ) -> OSStatus;
    #[link_name = "MusicSequenceDisposeTrack"]
    pub fn at_music_sequence_dispose_track(
        raw_sequence: *mut c_void,
        raw_track: *mut c_void,
    ) -> OSStatus;
    pub fn at_music_sequence_get_track_count(
        raw_sequence: *mut c_void,
        out_track_count: *mut u32,
    ) -> OSStatus;
    #[link_name = "MusicSequenceGetIndTrack"]
    pub fn at_music_sequence_get_ind_track(
        raw_sequence: *mut c_void,
        track_index: u32,
        out_track: *mut MusicTrackRef,
    ) -> OSStatus;
    #[link_name = "MusicSequenceGetTrackIndex"]
    pub fn at_music_sequence_get_track_index(
        raw_sequence: *mut c_void,
        raw_track: *mut c_void,
        out_track_index: *mut u32,
    ) -> OSStatus;
    #[link_name = "MusicSequenceGetTempoTrack"]
    pub fn at_music_sequence_get_tempo_track(
        raw_sequence: *mut c_void,
        out_track: *mut MusicTrackRef,
    ) -> OSStatus;
    #[link_name = "MusicSequenceSetAUGraph"]
    pub fn at_music_sequence_set_au_graph(
        raw_sequence: *mut c_void,
        raw_graph: *mut c_void,
    ) -> OSStatus;
    #[link_name = "MusicSequenceGetAUGraph"]
    pub fn at_music_sequence_get_au_graph(
        raw_sequence: *mut c_void,
        out_graph: *mut *mut c_void,
    ) -> OSStatus;
    #[link_name = "MusicSequenceSetMIDIEndpoint"]
    pub fn at_music_sequence_set_midi_endpoint(
        raw_sequence: *mut c_void,
        endpoint: MIDIEndpointRef,
    ) -> OSStatus;
    #[link_name = "MusicSequenceSetSequenceType"]
    pub fn at_music_sequence_set_sequence_type(
        raw_sequence: *mut c_void,
        sequence_type: MusicSequenceType,
    ) -> OSStatus;
    #[link_name = "MusicSequenceGetSequenceType"]
    pub fn at_music_sequence_get_sequence_type(
        raw_sequence: *mut c_void,
        out_sequence_type: *mut MusicSequenceType,
    ) -> OSStatus;
    #[link_name = "MusicSequenceFileLoad"]
    pub fn at_music_sequence_file_load(
        raw_sequence: *mut c_void,
        file_ref: CFURLRef,
        file_type_hint: MusicSequenceFileTypeId,
        flags: MusicSequenceLoadFlags,
    ) -> OSStatus;
    #[link_name = "MusicSequenceFileLoadData"]
    pub fn at_music_sequence_file_load_data(
        raw_sequence: *mut c_void,
        data: CFDataRef,
        file_type_hint: MusicSequenceFileTypeId,
        flags: MusicSequenceLoadFlags,
    ) -> OSStatus;
    #[link_name = "MusicSequenceFileCreate"]
    pub fn at_music_sequence_file_create(
        raw_sequence: *mut c_void,
        file_ref: CFURLRef,
        file_type: MusicSequenceFileTypeId,
        flags: MusicSequenceFileFlags,
        resolution: i16,
    ) -> OSStatus;
    #[link_name = "MusicSequenceFileCreateData"]
    pub fn at_music_sequence_file_create_data(
        raw_sequence: *mut c_void,
        file_type: MusicSequenceFileTypeId,
        flags: MusicSequenceFileFlags,
        resolution: i16,
        out_data: *mut CFDataRef,
    ) -> OSStatus;
    #[link_name = "MusicSequenceGetSecondsForBeats"]
    pub fn at_music_sequence_get_seconds_for_beats(
        raw_sequence: *mut c_void,
        beats: MusicTimeStamp,
        out_seconds: *mut f64,
    ) -> OSStatus;
    #[link_name = "MusicSequenceGetBeatsForSeconds"]
    pub fn at_music_sequence_get_beats_for_seconds(
        raw_sequence: *mut c_void,
        seconds: f64,
        out_beats: *mut MusicTimeStamp,
    ) -> OSStatus;
    #[link_name = "MusicSequenceBeatsToBarBeatTime"]
    pub fn at_music_sequence_beats_to_bar_beat_time(
        raw_sequence: *mut c_void,
        beats: MusicTimeStamp,
        subbeat_divisor: u32,
        out_bar_beat_time: *mut CABarBeatTime,
    ) -> OSStatus;
    #[link_name = "MusicSequenceBarBeatTimeToBeats"]
    pub fn at_music_sequence_bar_beat_time_to_beats(
        raw_sequence: *mut c_void,
        bar_beat_time: *const CABarBeatTime,
        out_beats: *mut MusicTimeStamp,
    ) -> OSStatus;
    #[link_name = "MusicSequenceGetInfoDictionary"]
    pub fn at_music_sequence_get_info_dictionary(raw_sequence: *mut c_void) -> CFDictionaryRef;

    pub fn at_music_track_raw(handle: *mut c_void) -> *mut c_void;
    pub fn at_music_track_release(handle: *mut c_void);
    #[link_name = "MusicTrackGetSequence"]
    pub fn at_music_track_get_sequence(
        raw_track: *mut c_void,
        out_sequence: *mut MusicSequenceRef,
    ) -> OSStatus;
    #[link_name = "MusicTrackSetDestNode"]
    pub fn at_music_track_set_dest_node(raw_track: *mut c_void, node: AUNode) -> OSStatus;
    #[link_name = "MusicTrackSetDestMIDIEndpoint"]
    pub fn at_music_track_set_dest_midi_endpoint(
        raw_track: *mut c_void,
        endpoint: MIDIEndpointRef,
    ) -> OSStatus;
    #[link_name = "MusicTrackGetDestNode"]
    pub fn at_music_track_get_dest_node(raw_track: *mut c_void, out_node: *mut AUNode) -> OSStatus;
    #[link_name = "MusicTrackGetDestMIDIEndpoint"]
    pub fn at_music_track_get_dest_midi_endpoint(
        raw_track: *mut c_void,
        out_endpoint: *mut MIDIEndpointRef,
    ) -> OSStatus;
    #[link_name = "MusicTrackSetProperty"]
    pub fn at_music_track_set_property(
        raw_track: *mut c_void,
        property_id: u32,
        data: *const c_void,
        length: u32,
    ) -> OSStatus;
    #[link_name = "MusicTrackGetProperty"]
    pub fn at_music_track_get_property(
        raw_track: *mut c_void,
        property_id: u32,
        out_data: *mut c_void,
        io_length: *mut u32,
    ) -> OSStatus;
    #[link_name = "MusicTrackMoveEvents"]
    pub fn at_music_track_move_events(
        raw_track: *mut c_void,
        start_time: MusicTimeStamp,
        end_time: MusicTimeStamp,
        move_time: MusicTimeStamp,
    ) -> OSStatus;
    #[link_name = "MusicTrackClear"]
    pub fn at_music_track_clear(
        raw_track: *mut c_void,
        start_time: MusicTimeStamp,
        end_time: MusicTimeStamp,
    ) -> OSStatus;
    #[link_name = "MusicTrackCut"]
    pub fn at_music_track_cut(
        raw_track: *mut c_void,
        start_time: MusicTimeStamp,
        end_time: MusicTimeStamp,
    ) -> OSStatus;
    #[link_name = "MusicTrackCopyInsert"]
    pub fn at_music_track_copy_insert(
        source_track: *mut c_void,
        source_start_time: MusicTimeStamp,
        source_end_time: MusicTimeStamp,
        dest_track: *mut c_void,
        dest_insert_time: MusicTimeStamp,
    ) -> OSStatus;
    #[link_name = "MusicTrackMerge"]
    pub fn at_music_track_merge(
        source_track: *mut c_void,
        source_start_time: MusicTimeStamp,
        source_end_time: MusicTimeStamp,
        dest_track: *mut c_void,
        dest_insert_time: MusicTimeStamp,
    ) -> OSStatus;
    pub fn at_music_track_new_midi_note_event(
        raw_track: *mut c_void,
        time_stamp: f64,
        note_message: *const MIDINoteMessage,
    ) -> OSStatus;
    #[link_name = "MusicTrackNewMIDIChannelEvent"]
    pub fn at_music_track_new_midi_channel_event(
        raw_track: *mut c_void,
        time_stamp: f64,
        message: *const MIDIChannelMessage,
    ) -> OSStatus;
    #[link_name = "MusicTrackNewMIDIRawDataEvent"]
    pub fn at_music_track_new_midi_raw_data_event(
        raw_track: *mut c_void,
        time_stamp: f64,
        raw_data: *const MIDIRawData,
    ) -> OSStatus;
    #[link_name = "MusicTrackNewExtendedNoteEvent"]
    pub fn at_music_track_new_extended_note_event(
        raw_track: *mut c_void,
        time_stamp: f64,
        note_event: *const ExtendedNoteOnEvent,
    ) -> OSStatus;
    #[link_name = "MusicTrackNewParameterEvent"]
    pub fn at_music_track_new_parameter_event(
        raw_track: *mut c_void,
        time_stamp: f64,
        event: *const ParameterEvent,
    ) -> OSStatus;
    #[link_name = "MusicTrackNewExtendedTempoEvent"]
    pub fn at_music_track_new_extended_tempo_event(
        raw_track: *mut c_void,
        time_stamp: f64,
        bpm: f64,
    ) -> OSStatus;
    #[link_name = "MusicTrackNewMetaEvent"]
    pub fn at_music_track_new_meta_event(
        raw_track: *mut c_void,
        time_stamp: f64,
        event: *const MIDIMetaEvent,
    ) -> OSStatus;
    #[link_name = "MusicTrackNewUserEvent"]
    pub fn at_music_track_new_user_event(
        raw_track: *mut c_void,
        time_stamp: f64,
        event: *const MusicEventUserData,
    ) -> OSStatus;
    #[link_name = "MusicTrackNewAUPresetEvent"]
    pub fn at_music_track_new_au_preset_event(
        raw_track: *mut c_void,
        time_stamp: f64,
        event: *const AUPresetEvent,
    ) -> OSStatus;

    #[link_name = "NewMusicEventIterator"]
    pub fn at_music_event_iterator_new(
        raw_track: *mut c_void,
        out_iterator: *mut MusicEventIteratorRef,
    ) -> OSStatus;
    #[link_name = "DisposeMusicEventIterator"]
    pub fn at_music_event_iterator_dispose(iterator: MusicEventIteratorRef) -> OSStatus;
    #[link_name = "MusicEventIteratorSeek"]
    pub fn at_music_event_iterator_seek(
        iterator: MusicEventIteratorRef,
        time_stamp: MusicTimeStamp,
    ) -> OSStatus;
    #[link_name = "MusicEventIteratorNextEvent"]
    pub fn at_music_event_iterator_next_event(iterator: MusicEventIteratorRef) -> OSStatus;
    #[link_name = "MusicEventIteratorPreviousEvent"]
    pub fn at_music_event_iterator_previous_event(iterator: MusicEventIteratorRef) -> OSStatus;
    #[link_name = "MusicEventIteratorGetEventInfo"]
    pub fn at_music_event_iterator_get_event_info(
        iterator: MusicEventIteratorRef,
        out_time_stamp: *mut MusicTimeStamp,
        out_event_type: *mut MusicEventType,
        out_event_data: *mut *const c_void,
        out_event_data_size: *mut u32,
    ) -> OSStatus;
    #[link_name = "MusicEventIteratorSetEventInfo"]
    pub fn at_music_event_iterator_set_event_info(
        iterator: MusicEventIteratorRef,
        event_type: MusicEventType,
        event_data: *const c_void,
    ) -> OSStatus;
    #[link_name = "MusicEventIteratorSetEventTime"]
    pub fn at_music_event_iterator_set_event_time(
        iterator: MusicEventIteratorRef,
        time_stamp: MusicTimeStamp,
    ) -> OSStatus;
    #[link_name = "MusicEventIteratorDeleteEvent"]
    pub fn at_music_event_iterator_delete_event(iterator: MusicEventIteratorRef) -> OSStatus;
    #[link_name = "MusicEventIteratorHasPreviousEvent"]
    pub fn at_music_event_iterator_has_previous_event(
        iterator: MusicEventIteratorRef,
        out_has_previous_event: *mut Boolean,
    ) -> OSStatus;
    #[link_name = "MusicEventIteratorHasNextEvent"]
    pub fn at_music_event_iterator_has_next_event(
        iterator: MusicEventIteratorRef,
        out_has_next_event: *mut Boolean,
    ) -> OSStatus;
    #[link_name = "MusicEventIteratorHasCurrentEvent"]
    pub fn at_music_event_iterator_has_current_event(
        iterator: MusicEventIteratorRef,
        out_has_current_event: *mut Boolean,
    ) -> OSStatus;

    pub fn at_music_player_new(out_handle: *mut *mut c_void) -> OSStatus;
    pub fn at_music_player_raw(handle: *mut c_void) -> *mut c_void;
    pub fn at_music_player_release(handle: *mut c_void);
    pub fn at_music_player_set_sequence(
        raw_player: *mut c_void,
        raw_sequence: *mut c_void,
    ) -> OSStatus;
    #[link_name = "MusicPlayerGetSequence"]
    pub fn at_music_player_get_sequence(
        raw_player: *mut c_void,
        out_sequence: *mut MusicSequenceRef,
    ) -> OSStatus;
    #[link_name = "MusicPlayerSetTime"]
    pub fn at_music_player_set_time(raw_player: *mut c_void, time: MusicTimeStamp) -> OSStatus;
    #[link_name = "MusicPlayerGetTime"]
    pub fn at_music_player_get_time(
        raw_player: *mut c_void,
        out_time: *mut MusicTimeStamp,
    ) -> OSStatus;
    #[link_name = "MusicPlayerGetHostTimeForBeats"]
    pub fn at_music_player_get_host_time_for_beats(
        raw_player: *mut c_void,
        beats: MusicTimeStamp,
        out_host_time: *mut u64,
    ) -> OSStatus;
    #[link_name = "MusicPlayerGetBeatsForHostTime"]
    pub fn at_music_player_get_beats_for_host_time(
        raw_player: *mut c_void,
        host_time: u64,
        out_beats: *mut MusicTimeStamp,
    ) -> OSStatus;
    pub fn at_music_player_preroll(raw_player: *mut c_void) -> OSStatus;
    pub fn at_music_player_start(raw_player: *mut c_void) -> OSStatus;
    pub fn at_music_player_stop(raw_player: *mut c_void) -> OSStatus;
    pub fn at_music_player_is_playing(
        raw_player: *mut c_void,
        out_is_playing: *mut u32,
    ) -> OSStatus;
    #[link_name = "MusicPlayerSetPlayRateScalar"]
    pub fn at_music_player_set_play_rate_scalar(
        raw_player: *mut c_void,
        scale_rate: f64,
    ) -> OSStatus;
    #[link_name = "MusicPlayerGetPlayRateScalar"]
    pub fn at_music_player_get_play_rate_scalar(
        raw_player: *mut c_void,
        out_scale_rate: *mut f64,
    ) -> OSStatus;
}
