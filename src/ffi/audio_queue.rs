use crate::{AudioQueueParameterId, AudioQueueParameterValue, AudioQueuePropertyId, OSStatus};
use std::ffi::c_void;

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
    /// Raw binding for `AudioQueueAllocateBuffer`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioQueueAllocateBuffer`.
    pub fn at_audio_queue_allocate_buffer(
        raw_queue: *mut c_void,
        buffer_byte_size: u32,
        out_handle: *mut *mut c_void,
    ) -> OSStatus;
    /// Raw binding for `AudioQueueBufferRaw`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioQueueBufferRaw`.
    pub fn at_audio_queue_buffer_raw(handle: *mut c_void) -> *mut c_void;
    /// Raw binding for `AudioQueueBufferRelease`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioQueueBufferRelease`.
    pub fn at_audio_queue_buffer_release(handle: *mut c_void);
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
