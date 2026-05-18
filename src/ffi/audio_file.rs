use crate::{
    AudioFileFlags, AudioFileId, AudioFilePermissions, AudioFilePropertyId, AudioFileTypeId,
    AudioStreamBasicDescription, AudioStreamPacketDescription, Boolean, OSStatus,
};
use std::ffi::{c_char, c_void};

unsafe extern "C" {
    /// Raw binding for `AudioFileOpenURL`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioFileOpenURL`.
    pub fn at_audio_file_open(
        path: *const c_char,
        permissions: AudioFilePermissions,
        file_type_hint: AudioFileTypeId,
        out_handle: *mut *mut c_void,
    ) -> OSStatus;
    /// Raw binding for `AudioFileCreate`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioFileCreate`.
    pub fn at_audio_file_create(
        path: *const c_char,
        file_type: AudioFileTypeId,
        format: *const AudioStreamBasicDescription,
        flags: AudioFileFlags,
        out_handle: *mut *mut c_void,
    ) -> OSStatus;
    /// Raw binding for `AudioFileRaw`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioFileRaw`.
    pub fn at_audio_file_raw(handle: *mut c_void) -> *mut c_void;
    /// Raw binding for `AudioFileRelease`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioFileRelease`.
    pub fn at_audio_file_release(handle: *mut c_void);
    /// Raw binding for `AudioFileGetPropertyInfo`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioFileGetPropertyInfo`.
    pub fn at_audio_file_get_property_info(
        raw_file: *mut c_void,
        property_id: AudioFilePropertyId,
        out_data_size: *mut u32,
        out_writable: *mut u32,
    ) -> OSStatus;
    /// Raw binding for `AudioFileGetProperty`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioFileGetProperty`.
    pub fn at_audio_file_get_property(
        raw_file: *mut c_void,
        property_id: AudioFilePropertyId,
        io_data_size: *mut u32,
        out_property_data: *mut c_void,
    ) -> OSStatus;
    /// Raw binding for `AudioFileSetProperty`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioFileSetProperty`.
    pub fn at_audio_file_set_property(
        raw_file: *mut c_void,
        property_id: AudioFilePropertyId,
        data_size: u32,
        property_data: *const c_void,
    ) -> OSStatus;
    #[link_name = "AudioFileOptimize"]
    /// Raw binding for `AudioFileOptimize`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioFileOptimize`.
    pub fn at_audio_file_optimize(raw_file: AudioFileId) -> OSStatus;
    #[link_name = "AudioFileReadBytes"]
    /// Raw binding for `AudioFileReadBytes`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioFileReadBytes`.
    pub fn at_audio_file_read_bytes(
        raw_file: AudioFileId,
        use_cache: Boolean,
        starting_byte: i64,
        io_num_bytes: *mut u32,
        out_buffer: *mut c_void,
    ) -> OSStatus;
    #[link_name = "AudioFileWriteBytes"]
    /// Raw binding for `AudioFileWriteBytes`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioFileWriteBytes`.
    pub fn at_audio_file_write_bytes(
        raw_file: AudioFileId,
        use_cache: Boolean,
        starting_byte: i64,
        io_num_bytes: *mut u32,
        in_buffer: *const c_void,
    ) -> OSStatus;
    /// Raw binding for `AudioFileReadPacketData`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioFileReadPacketData`.
    pub fn at_audio_file_read_packet_data(
        raw_file: *mut c_void,
        use_cache: bool,
        io_num_bytes: *mut u32,
        out_packet_descriptions: *mut AudioStreamPacketDescription,
        starting_packet: i64,
        io_num_packets: *mut u32,
        out_buffer: *mut c_void,
    ) -> OSStatus;
    /// Raw binding for `AudioFileReadPackets`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioFileReadPackets`.
    pub fn at_audio_file_read_packets(
        raw_file: *mut c_void,
        use_cache: bool,
        out_num_bytes: *mut u32,
        out_packet_descriptions: *mut AudioStreamPacketDescription,
        starting_packet: i64,
        io_num_packets: *mut u32,
        out_buffer: *mut c_void,
    ) -> OSStatus;
    /// Raw binding for `AudioFileWritePackets`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioFileWritePackets`.
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
    /// Raw binding for `AudioFileCountUserData`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioFileCountUserData`.
    pub fn at_audio_file_count_user_data(
        raw_file: AudioFileId,
        user_data_id: u32,
        out_number_items: *mut u32,
    ) -> OSStatus;
    #[link_name = "AudioFileGetUserDataSize"]
    /// Raw binding for `AudioFileGetUserDataSize`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioFileGetUserDataSize`.
    pub fn at_audio_file_get_user_data_size(
        raw_file: AudioFileId,
        user_data_id: u32,
        index: u32,
        out_user_data_size: *mut u32,
    ) -> OSStatus;
    #[link_name = "AudioFileGetUserDataSize64"]
    /// Raw binding for `AudioFileGetUserDataSize64`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioFileGetUserDataSize64`.
    pub fn at_audio_file_get_user_data_size64(
        raw_file: AudioFileId,
        user_data_id: u32,
        index: u32,
        out_user_data_size: *mut u64,
    ) -> OSStatus;
    #[link_name = "AudioFileGetUserData"]
    /// Raw binding for `AudioFileGetUserData`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioFileGetUserData`.
    pub fn at_audio_file_get_user_data(
        raw_file: AudioFileId,
        user_data_id: u32,
        index: u32,
        io_user_data_size: *mut u32,
        out_user_data: *mut c_void,
    ) -> OSStatus;
    #[link_name = "AudioFileGetUserDataAtOffset"]
    /// Raw binding for `AudioFileGetUserDataAtOffset`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioFileGetUserDataAtOffset`.
    pub fn at_audio_file_get_user_data_at_offset(
        raw_file: AudioFileId,
        user_data_id: u32,
        index: u32,
        offset: i64,
        io_user_data_size: *mut u32,
        out_user_data: *mut c_void,
    ) -> OSStatus;
    #[link_name = "AudioFileSetUserData"]
    /// Raw binding for `AudioFileSetUserData`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioFileSetUserData`.
    pub fn at_audio_file_set_user_data(
        raw_file: AudioFileId,
        user_data_id: u32,
        index: u32,
        user_data_size: u32,
        user_data: *const c_void,
    ) -> OSStatus;
    #[link_name = "AudioFileRemoveUserData"]
    /// Raw binding for `AudioFileRemoveUserData`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioFileRemoveUserData`.
    pub fn at_audio_file_remove_user_data(
        raw_file: AudioFileId,
        user_data_id: u32,
        index: u32,
    ) -> OSStatus;
    #[link_name = "AudioFileGetGlobalInfoSize"]
    /// Raw binding for `AudioFileGetGlobalInfoSize`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioFileGetGlobalInfoSize`.
    pub fn at_audio_file_get_global_info_size(
        property_id: AudioFilePropertyId,
        specifier_size: u32,
        specifier: *const c_void,
        out_data_size: *mut u32,
    ) -> OSStatus;
    #[link_name = "AudioFileGetGlobalInfo"]
    /// Raw binding for `AudioFileGetGlobalInfo`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioFileGetGlobalInfo`.
    pub fn at_audio_file_get_global_info(
        property_id: AudioFilePropertyId,
        specifier_size: u32,
        specifier: *const c_void,
        io_data_size: *mut u32,
        out_property_data: *mut c_void,
    ) -> OSStatus;
}

/// Wraps `AudioFileCastAudioFileID`.
pub(crate) fn cast_audio_file_id(raw: *mut c_void) -> AudioFileId {
    raw.cast()
}
