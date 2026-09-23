use crate::{
    AudioQueueBufferRef, AudioQueueParameterId, AudioQueueParameterValue, AudioQueuePropertyId,
    AudioStreamBasicDescription, AudioStreamPacketDescription, AudioTimeStamp, OSStatus,
};
use std::ffi::c_void;

pub type AudioQueueOutputProc = unsafe extern "C" fn(*mut c_void, *mut c_void, AudioQueueBufferRef);
pub type AudioQueueInputProc = unsafe extern "C" fn(
    *mut c_void,
    *mut c_void,
    AudioQueueBufferRef,
    *const AudioTimeStamp,
    u32,
    *const AudioStreamPacketDescription,
);

unsafe extern "C" {
    /// Raw binding for `AudioQueueNewOutput`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioQueueNewOutput`.
    pub fn at_audio_queue_new_output(
        format: *const crate::AudioStreamBasicDescription,
        out_handle: *mut *mut c_void,
    ) -> OSStatus;
    /// Raw binding for `AudioQueueRaw`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioQueueRaw`.
    pub fn at_audio_queue_raw(handle: *mut c_void) -> *mut c_void;
    /// Raw binding for `AudioQueueRelease`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioQueueRelease`.
    pub fn at_audio_queue_release(handle: *mut c_void);
    /// Raw binding for `AudioQueueGetProperty`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioQueueGetProperty`.
    pub fn at_audio_queue_get_property(
        raw_queue: *mut c_void,
        property_id: AudioQueuePropertyId,
        io_data_size: *mut u32,
        out_property_data: *mut c_void,
    ) -> OSStatus;
    /// Raw binding for `AudioQueueGetParameter`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioQueueGetParameter`.
    pub fn at_audio_queue_get_parameter(
        raw_queue: *mut c_void,
        parameter_id: AudioQueueParameterId,
        out_value: *mut AudioQueueParameterValue,
    ) -> OSStatus;
    /// Raw binding for `AudioQueueSetParameter`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioQueueSetParameter`.
    pub fn at_audio_queue_set_parameter(
        raw_queue: *mut c_void,
        parameter_id: AudioQueueParameterId,
        value: AudioQueueParameterValue,
    ) -> OSStatus;
    pub fn at_audio_queue_new_output_with_callback(
        format: *const AudioStreamBasicDescription,
        callback: AudioQueueOutputProc,
        context: *mut c_void,
        release: unsafe extern "C" fn(*mut c_void),
        out_handle: *mut *mut c_void,
    ) -> OSStatus;
    pub fn at_audio_queue_new_input_with_callback(
        format: *const AudioStreamBasicDescription,
        callback: AudioQueueInputProc,
        context: *mut c_void,
        release: unsafe extern "C" fn(*mut c_void),
        out_handle: *mut *mut c_void,
    ) -> OSStatus;
    #[link_name = "AudioQueueAllocateBuffer"]
    pub fn at_audio_queue_allocate_buffer(
        raw_queue: *mut c_void,
        buffer_byte_size: u32,
        out_buffer: *mut AudioQueueBufferRef,
    ) -> OSStatus;
    #[link_name = "AudioQueueAllocateBufferWithPacketDescriptions"]
    pub fn at_audio_queue_allocate_buffer_with_packet_descriptions(
        raw_queue: *mut c_void,
        buffer_byte_size: u32,
        number_packet_descriptions: u32,
        out_buffer: *mut AudioQueueBufferRef,
    ) -> OSStatus;
    #[link_name = "AudioQueueFreeBuffer"]
    pub fn at_audio_queue_free_buffer(
        raw_queue: *mut c_void,
        buffer: AudioQueueBufferRef,
    ) -> OSStatus;
    #[link_name = "AudioQueueEnqueueBuffer"]
    pub fn at_audio_queue_enqueue_buffer(
        raw_queue: *mut c_void,
        buffer: AudioQueueBufferRef,
        number_packet_descriptions: u32,
        packet_descriptions: *const AudioStreamPacketDescription,
    ) -> OSStatus;
    #[link_name = "AudioQueueSetOfflineRenderFormat"]
    pub fn at_audio_queue_set_offline_render_format(
        raw_queue: *mut c_void,
        format: *const AudioStreamBasicDescription,
        layout: *const c_void,
    ) -> OSStatus;
    #[link_name = "AudioQueueOfflineRender"]
    pub fn at_audio_queue_offline_render(
        raw_queue: *mut c_void,
        time_stamp: *const AudioTimeStamp,
        buffer: AudioQueueBufferRef,
        number_frames: u32,
    ) -> OSStatus;
    /// Raw binding for `AudioQueueStart`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioQueueStart`.
    pub fn at_audio_queue_start(raw_queue: *mut c_void) -> OSStatus;
    /// Raw binding for `AudioQueueStop`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioQueueStop`.
    pub fn at_audio_queue_stop(raw_queue: *mut c_void, immediate: bool) -> OSStatus;
    /// Raw binding for `AudioQueueReset`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioQueueReset`.
    pub fn at_audio_queue_reset(raw_queue: *mut c_void) -> OSStatus;
}
