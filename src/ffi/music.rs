use crate::{MIDINoteMessage, OSStatus};
use std::ffi::c_void;

unsafe extern "C" {
    pub fn at_music_sequence_new(out_handle: *mut *mut c_void) -> OSStatus;
    pub fn at_music_sequence_raw(handle: *mut c_void) -> *mut c_void;
    pub fn at_music_sequence_release(handle: *mut c_void);
    pub fn at_music_sequence_new_track(
        raw_sequence: *mut c_void,
        out_handle: *mut *mut c_void,
    ) -> OSStatus;
    pub fn at_music_sequence_get_track_count(
        raw_sequence: *mut c_void,
        out_track_count: *mut u32,
    ) -> OSStatus;
    pub fn at_music_track_raw(handle: *mut c_void) -> *mut c_void;
    pub fn at_music_track_release(handle: *mut c_void);
    pub fn at_music_track_new_midi_note_event(
        raw_track: *mut c_void,
        time_stamp: f64,
        note_message: *const MIDINoteMessage,
    ) -> OSStatus;
    pub fn at_music_player_new(out_handle: *mut *mut c_void) -> OSStatus;
    pub fn at_music_player_raw(handle: *mut c_void) -> *mut c_void;
    pub fn at_music_player_release(handle: *mut c_void);
    pub fn at_music_player_set_sequence(
        raw_player: *mut c_void,
        raw_sequence: *mut c_void,
    ) -> OSStatus;
    pub fn at_music_player_preroll(raw_player: *mut c_void) -> OSStatus;
    pub fn at_music_player_start(raw_player: *mut c_void) -> OSStatus;
    pub fn at_music_player_stop(raw_player: *mut c_void) -> OSStatus;
    pub fn at_music_player_is_playing(
        raw_player: *mut c_void,
        out_is_playing: *mut u32,
    ) -> OSStatus;
}
