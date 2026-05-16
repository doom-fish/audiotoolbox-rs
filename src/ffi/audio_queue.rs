use crate::{
    AudioQueueParameterId, AudioQueueParameterValue, AudioQueuePropertyId, OSStatus,
};
use std::ffi::c_void;

unsafe extern "C" {
    pub fn at_audio_queue_new_output(
        format: *const crate::AudioStreamBasicDescription,
        out_handle: *mut *mut c_void,
    ) -> OSStatus;
    pub fn at_audio_queue_raw(handle: *mut c_void) -> *mut c_void;
    pub fn at_audio_queue_release(handle: *mut c_void);
    pub fn at_audio_queue_get_property(
        raw_queue: *mut c_void,
        property_id: AudioQueuePropertyId,
        io_data_size: *mut u32,
        out_property_data: *mut c_void,
    ) -> OSStatus;
    pub fn at_audio_queue_get_parameter(
        raw_queue: *mut c_void,
        parameter_id: AudioQueueParameterId,
        out_value: *mut AudioQueueParameterValue,
    ) -> OSStatus;
    pub fn at_audio_queue_set_parameter(
        raw_queue: *mut c_void,
        parameter_id: AudioQueueParameterId,
        value: AudioQueueParameterValue,
    ) -> OSStatus;
    pub fn at_audio_queue_allocate_buffer(
        raw_queue: *mut c_void,
        buffer_byte_size: u32,
        out_handle: *mut *mut c_void,
    ) -> OSStatus;
    pub fn at_audio_queue_buffer_raw(handle: *mut c_void) -> *mut c_void;
    pub fn at_audio_queue_buffer_release(handle: *mut c_void);
    pub fn at_audio_queue_start(raw_queue: *mut c_void) -> OSStatus;
    pub fn at_audio_queue_stop(raw_queue: *mut c_void, immediate: bool) -> OSStatus;
    pub fn at_audio_queue_reset(raw_queue: *mut c_void) -> OSStatus;
}
