use crate::{
    AudioFileFlags, AudioFileId, AudioFilePermissions, AudioFilePropertyId, AudioFileTypeId,
    AudioStreamBasicDescription, AudioStreamPacketDescription, Boolean, OSStatus,
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
    #[link_name = "AudioFileOptimize"]
    pub fn at_audio_file_optimize(raw_file: AudioFileId) -> OSStatus;
    #[link_name = "AudioFileReadBytes"]
    pub fn at_audio_file_read_bytes(
        raw_file: AudioFileId,
        use_cache: Boolean,
        starting_byte: i64,
        io_num_bytes: *mut u32,
        out_buffer: *mut c_void,
    ) -> OSStatus;
    #[link_name = "AudioFileWriteBytes"]
    pub fn at_audio_file_write_bytes(
        raw_file: AudioFileId,
        use_cache: Boolean,
        starting_byte: i64,
        io_num_bytes: *mut u32,
        in_buffer: *const c_void,
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
    #[link_name = "AudioFileCountUserData"]
    pub fn at_audio_file_count_user_data(
        raw_file: AudioFileId,
        user_data_id: u32,
        out_number_items: *mut u32,
    ) -> OSStatus;
    #[link_name = "AudioFileGetUserDataSize"]
    pub fn at_audio_file_get_user_data_size(
        raw_file: AudioFileId,
        user_data_id: u32,
        index: u32,
        out_user_data_size: *mut u32,
    ) -> OSStatus;
    #[link_name = "AudioFileGetUserDataSize64"]
    pub fn at_audio_file_get_user_data_size64(
        raw_file: AudioFileId,
        user_data_id: u32,
        index: u32,
        out_user_data_size: *mut u64,
    ) -> OSStatus;
    #[link_name = "AudioFileGetUserData"]
    pub fn at_audio_file_get_user_data(
        raw_file: AudioFileId,
        user_data_id: u32,
        index: u32,
        io_user_data_size: *mut u32,
        out_user_data: *mut c_void,
    ) -> OSStatus;
    #[link_name = "AudioFileGetUserDataAtOffset"]
    pub fn at_audio_file_get_user_data_at_offset(
        raw_file: AudioFileId,
        user_data_id: u32,
        index: u32,
        offset: i64,
        io_user_data_size: *mut u32,
        out_user_data: *mut c_void,
    ) -> OSStatus;
    #[link_name = "AudioFileSetUserData"]
    pub fn at_audio_file_set_user_data(
        raw_file: AudioFileId,
        user_data_id: u32,
        index: u32,
        user_data_size: u32,
        user_data: *const c_void,
    ) -> OSStatus;
    #[link_name = "AudioFileRemoveUserData"]
    pub fn at_audio_file_remove_user_data(
        raw_file: AudioFileId,
        user_data_id: u32,
        index: u32,
    ) -> OSStatus;
    #[link_name = "AudioFileGetGlobalInfoSize"]
    pub fn at_audio_file_get_global_info_size(
        property_id: AudioFilePropertyId,
        specifier_size: u32,
        specifier: *const c_void,
        out_data_size: *mut u32,
    ) -> OSStatus;
    #[link_name = "AudioFileGetGlobalInfo"]
    pub fn at_audio_file_get_global_info(
        property_id: AudioFilePropertyId,
        specifier_size: u32,
        specifier: *const c_void,
        io_data_size: *mut u32,
        out_property_data: *mut c_void,
    ) -> OSStatus;
}

pub(crate) fn cast_audio_file_id(raw: *mut c_void) -> AudioFileId {
    raw.cast()
}
