use crate::{AudioFormatPropertyId, OSStatus};
use std::ffi::c_void;

unsafe extern "C" {
    /// Raw binding for `AudioFormatGetPropertyInfo`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioFormatGetPropertyInfo`.
    pub fn at_audio_format_get_property_info(
        property_id: AudioFormatPropertyId,
        specifier_size: u32,
        specifier: *const c_void,
        out_size: *mut u32,
    ) -> OSStatus;
    /// Raw binding for `AudioFormatGetProperty`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioFormatGetProperty`.
    pub fn at_audio_format_get_property(
        property_id: AudioFormatPropertyId,
        specifier_size: u32,
        specifier: *const c_void,
        io_size: *mut u32,
        out_data: *mut c_void,
    ) -> OSStatus;
}
