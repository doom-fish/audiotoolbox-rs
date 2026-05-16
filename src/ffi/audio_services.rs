use crate::OSStatus;
use std::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn at_system_sound_create(path: *const c_char, out_handle: *mut *mut c_void) -> OSStatus;
    pub fn at_system_sound_release(handle: *mut c_void);
    pub fn at_system_sound_id(handle: *mut c_void) -> u32;
    pub fn at_system_sound_play(handle: *mut c_void);
    pub fn at_system_sound_play_alert(handle: *mut c_void);
    pub fn at_system_sound_get_is_ui_sound(
        handle: *mut c_void,
        out_value: *mut u32,
    ) -> OSStatus;
    pub fn at_system_sound_set_is_ui_sound(handle: *mut c_void, value: u32) -> OSStatus;
    pub fn at_system_sound_get_complete_playback_if_app_dies(
        handle: *mut c_void,
        out_value: *mut u32,
    ) -> OSStatus;
    pub fn at_system_sound_set_complete_playback_if_app_dies(
        handle: *mut c_void,
        value: u32,
    ) -> OSStatus;
}
