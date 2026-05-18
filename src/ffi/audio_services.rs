use crate::OSStatus;
use std::ffi::{c_char, c_void};

unsafe extern "C" {
    /// Raw binding for `SystemSoundCreate`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `SystemSoundCreate`.
    pub fn at_system_sound_create(path: *const c_char, out_handle: *mut *mut c_void) -> OSStatus;
    /// Raw binding for `SystemSoundRelease`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `SystemSoundRelease`.
    pub fn at_system_sound_release(handle: *mut c_void);
    /// Raw binding for `SystemSoundID`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `SystemSoundID`.
    pub fn at_system_sound_id(handle: *mut c_void) -> u32;
    /// Raw binding for `SystemSoundPlay`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `SystemSoundPlay`.
    pub fn at_system_sound_play(handle: *mut c_void);
    /// Raw binding for `SystemSoundPlayAlert`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `SystemSoundPlayAlert`.
    pub fn at_system_sound_play_alert(handle: *mut c_void);
    /// Raw binding for `SystemSoundGetIsUISound`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `SystemSoundGetIsUISound`.
    pub fn at_system_sound_get_is_ui_sound(handle: *mut c_void, out_value: *mut u32) -> OSStatus;
    /// Raw binding for `SystemSoundSetIsUISound`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `SystemSoundSetIsUISound`.
    pub fn at_system_sound_set_is_ui_sound(handle: *mut c_void, value: u32) -> OSStatus;
    /// Raw binding for `SystemSoundGetCompletePlaybackIfAppDies`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `SystemSoundGetCompletePlaybackIfAppDies`.
    pub fn at_system_sound_get_complete_playback_if_app_dies(
        handle: *mut c_void,
        out_value: *mut u32,
    ) -> OSStatus;
    /// Raw binding for `SystemSoundSetCompletePlaybackIfAppDies`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `SystemSoundSetCompletePlaybackIfAppDies`.
    pub fn at_system_sound_set_complete_playback_if_app_dies(
        handle: *mut c_void,
        value: u32,
    ) -> OSStatus;
}
