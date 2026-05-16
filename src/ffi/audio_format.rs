use crate::{AudioFormatPropertyId, OSStatus};
use std::ffi::c_void;

unsafe extern "C" {
    pub fn at_audio_format_get_property_info(
        property_id: AudioFormatPropertyId,
        specifier_size: u32,
        specifier: *const c_void,
        out_size: *mut u32,
    ) -> OSStatus;
    pub fn at_audio_format_get_property(
        property_id: AudioFormatPropertyId,
        specifier_size: u32,
        specifier: *const c_void,
        io_size: *mut u32,
        out_data: *mut c_void,
    ) -> OSStatus;
}
