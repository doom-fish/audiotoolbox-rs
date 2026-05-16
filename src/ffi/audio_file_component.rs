use crate::{AudioFilePermissions, AudioFilePropertyId, AudioFileTypeId, OSStatus};
use std::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn at_audio_file_component_new_default(out_handle: *mut *mut c_void) -> OSStatus;
    pub fn at_audio_file_component_release(handle: *mut c_void);
    pub fn at_audio_file_component_open(
        handle: *mut c_void,
        path: *const c_char,
        permissions: AudioFilePermissions,
        file_descriptor: i32,
    ) -> OSStatus;
    pub fn at_audio_file_component_close_file(handle: *mut c_void) -> OSStatus;
    pub fn at_audio_file_component_get_property_info(
        handle: *mut c_void,
        property_id: AudioFilePropertyId,
        out_data_size: *mut u32,
        out_writable: *mut u32,
    ) -> OSStatus;
    pub fn at_audio_file_component_get_property(
        handle: *mut c_void,
        property_id: AudioFilePropertyId,
        io_data_size: *mut u32,
        out_property_data: *mut c_void,
    ) -> OSStatus;
    pub fn at_audio_file_component_can_read(
        handle: *mut c_void,
        file_type: AudioFileTypeId,
        out_can_read: *mut u32,
    ) -> OSStatus;
    pub fn at_audio_file_component_copy_file_type_name(
        handle: *mut c_void,
        file_type: AudioFileTypeId,
        out_status: *mut OSStatus,
    ) -> *mut c_char;
}
