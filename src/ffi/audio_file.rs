use crate::{
    AudioFileFlags, AudioFileId, AudioFilePermissions, AudioFilePropertyId, AudioFileTypeId,
    AudioStreamBasicDescription, AudioStreamPacketDescription, OSStatus,
};
use std::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn at_audio_file_open(
        path: *const c_char,
        permissions: AudioFilePermissions,
        file_type_hint: AudioFileTypeId,
        out_handle: *mut *mut c_void,
    ) -> OSStatus;
    pub fn at_audio_file_create(
        path: *const c_char,
        file_type: AudioFileTypeId,
        format: *const AudioStreamBasicDescription,
        flags: AudioFileFlags,
        out_handle: *mut *mut c_void,
    ) -> OSStatus;
    pub fn at_audio_file_raw(handle: *mut c_void) -> *mut c_void;
    pub fn at_audio_file_release(handle: *mut c_void);
    pub fn at_audio_file_get_property_info(
        raw_file: *mut c_void,
        property_id: AudioFilePropertyId,
        out_data_size: *mut u32,
        out_writable: *mut u32,
    ) -> OSStatus;
    pub fn at_audio_file_get_property(
        raw_file: *mut c_void,
        property_id: AudioFilePropertyId,
        io_data_size: *mut u32,
        out_property_data: *mut c_void,
    ) -> OSStatus;
    pub fn at_audio_file_set_property(
        raw_file: *mut c_void,
        property_id: AudioFilePropertyId,
        data_size: u32,
        property_data: *const c_void,
    ) -> OSStatus;
    pub fn at_audio_file_read_packet_data(
        raw_file: *mut c_void,
        use_cache: bool,
        io_num_bytes: *mut u32,
        out_packet_descriptions: *mut AudioStreamPacketDescription,
        starting_packet: i64,
        io_num_packets: *mut u32,
        out_buffer: *mut c_void,
    ) -> OSStatus;
    pub fn at_audio_file_read_packets(
        raw_file: *mut c_void,
        use_cache: bool,
        out_num_bytes: *mut u32,
        out_packet_descriptions: *mut AudioStreamPacketDescription,
        starting_packet: i64,
        io_num_packets: *mut u32,
        out_buffer: *mut c_void,
    ) -> OSStatus;
    pub fn at_audio_file_write_packets(
        raw_file: *mut c_void,
        use_cache: bool,
        num_bytes: u32,
        packet_descriptions: *const AudioStreamPacketDescription,
        starting_packet: i64,
        io_num_packets: *mut u32,
        buffer: *const c_void,
    ) -> OSStatus;
}

pub(crate) fn cast_audio_file_id(raw: *mut c_void) -> AudioFileId {
    raw.cast()
}
