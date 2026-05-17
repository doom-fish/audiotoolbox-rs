use crate::{AudioFileStreamParseFlags, AudioFileStreamPropertyId, Boolean, OSStatus};
use std::ffi::c_void;

unsafe extern "C" {
    pub fn at_audio_file_stream_open(file_type_hint: u32, out_handle: *mut *mut c_void)
        -> OSStatus;
    pub fn at_audio_file_stream_raw(handle: *mut c_void) -> *mut c_void;
    pub fn at_audio_file_stream_release(handle: *mut c_void);
    pub fn at_audio_file_stream_parse_bytes(
        raw_stream: *mut c_void,
        data: *const c_void,
        data_byte_size: u32,
        parse_flags: AudioFileStreamParseFlags,
    ) -> OSStatus;
    pub fn at_audio_file_stream_get_property_info(
        raw_stream: *mut c_void,
        property_id: AudioFileStreamPropertyId,
        out_property_data_size: *mut u32,
        out_writable: *mut Boolean,
    ) -> OSStatus;
    pub fn at_audio_file_stream_get_property(
        raw_stream: *mut c_void,
        property_id: AudioFileStreamPropertyId,
        io_property_data_size: *mut u32,
        out_property_data: *mut c_void,
    ) -> OSStatus;
    pub fn at_audio_file_stream_ready_to_produce_packets(handle: *mut c_void) -> u32;
    pub fn at_audio_file_stream_packet_count_seen(handle: *mut c_void) -> u64;
}
