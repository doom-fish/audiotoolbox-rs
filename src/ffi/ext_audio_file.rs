use crate::{
    AudioBufferList1, AudioConverterRef, AudioFileTypeId, AudioStreamBasicDescription,
    ExtAudioFilePropertyId, OSStatus,
};
use std::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn at_ext_audio_file_open(path: *const c_char, out_handle: *mut *mut c_void) -> OSStatus;
    pub fn at_ext_audio_file_create(
        path: *const c_char,
        file_type: AudioFileTypeId,
        format: *const AudioStreamBasicDescription,
        flags: u32,
        out_handle: *mut *mut c_void,
    ) -> OSStatus;
    pub fn at_ext_audio_file_raw(handle: *mut c_void) -> *mut c_void;
    pub fn at_ext_audio_file_release(handle: *mut c_void);
    pub fn at_ext_audio_file_get_property(
        raw_file: *mut c_void,
        property_id: ExtAudioFilePropertyId,
        io_property_data_size: *mut u32,
        out_property_data: *mut c_void,
    ) -> OSStatus;
    pub fn at_ext_audio_file_set_property(
        raw_file: *mut c_void,
        property_id: ExtAudioFilePropertyId,
        property_data_size: u32,
        property_data: *const c_void,
    ) -> OSStatus;
    pub fn at_ext_audio_file_read(
        raw_file: *mut c_void,
        io_number_frames: *mut u32,
        io_data: *mut AudioBufferList1,
    ) -> OSStatus;
    pub fn at_ext_audio_file_write(
        raw_file: *mut c_void,
        number_frames: u32,
        io_data: *const AudioBufferList1,
    ) -> OSStatus;
    pub fn at_ext_audio_file_copy_audio_converter(
        raw_file: *mut c_void,
        out_handle: *mut *mut c_void,
    ) -> OSStatus;
    pub fn at_borrowed_audio_converter_raw(handle: *mut c_void) -> *mut c_void;
    pub fn at_borrowed_audio_converter_release(handle: *mut c_void);
}

pub(crate) fn cast_audio_converter_ref(raw: *mut c_void) -> AudioConverterRef {
    raw.cast()
}
