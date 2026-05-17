use crate::{
    ffi, internal::status_to_result, AudioQueueBufferRef, AudioQueueParameterId,
    AudioQueueParameterValue, AudioQueuePropertyId, AudioQueueRef, AudioStreamBasicDescription,
    AudioToolboxError, Result, AUDIO_QUEUE_PARAM_VOLUME, AUDIO_QUEUE_PROPERTY_IS_RUNNING,
    AUDIO_QUEUE_PROPERTY_STREAM_DESCRIPTION,
};
use std::mem::MaybeUninit;

#[derive(Debug)]
pub struct AudioQueue {
    handle: *mut std::ffi::c_void,
    raw: AudioQueueRef,
}

#[derive(Debug)]
pub struct AudioQueueBufferHandle {
    handle: *mut std::ffi::c_void,
    raw: AudioQueueBufferRef,
}

impl AudioQueue {
    pub fn new_output(format: &AudioStreamBasicDescription) -> Result<Self> {
        let mut handle = std::ptr::null_mut();
        let status = unsafe { ffi::audio_queue::at_audio_queue_new_output(format, &mut handle) };
        status_to_result("AudioQueueNewOutput", status)?;
        let raw: AudioQueueRef = unsafe { ffi::audio_queue::at_audio_queue_raw(handle) }.cast();
        if raw.is_null() {
            return Err(AudioToolboxError::message(
                "AudioQueueNewOutput",
                "framework returned a null AudioQueueRef",
            ));
        }
        Ok(Self { handle, raw })
    }

    pub fn as_raw(&self) -> AudioQueueRef {
        self.raw
    }

    pub fn stream_description(&self) -> Result<AudioStreamBasicDescription> {
        self.get_property_typed(
            AUDIO_QUEUE_PROPERTY_STREAM_DESCRIPTION,
            "AudioQueueGetProperty(stream description)",
        )
    }

    pub fn is_running(&self) -> Result<bool> {
        Ok(self.get_property_typed::<u32>(
            AUDIO_QUEUE_PROPERTY_IS_RUNNING,
            "AudioQueueGetProperty(is running)",
        )? != 0)
    }

    pub fn get_parameter(
        &self,
        parameter_id: AudioQueueParameterId,
    ) -> Result<AudioQueueParameterValue> {
        let mut value = 0.0_f32;
        let status = unsafe {
            ffi::audio_queue::at_audio_queue_get_parameter(
                self.raw.cast(),
                parameter_id,
                &mut value,
            )
        };
        status_to_result("AudioQueueGetParameter", status)?;
        Ok(value)
    }

    pub fn set_parameter(
        &self,
        parameter_id: AudioQueueParameterId,
        value: AudioQueueParameterValue,
    ) -> Result<()> {
        let status = unsafe {
            ffi::audio_queue::at_audio_queue_set_parameter(self.raw.cast(), parameter_id, value)
        };
        status_to_result("AudioQueueSetParameter", status)
    }

    pub fn volume(&self) -> Result<f32> {
        self.get_parameter(AUDIO_QUEUE_PARAM_VOLUME)
    }

    pub fn set_volume(&self, value: f32) -> Result<()> {
        self.set_parameter(AUDIO_QUEUE_PARAM_VOLUME, value)
    }

    pub fn allocate_buffer(&self, byte_size: u32) -> Result<AudioQueueBufferHandle> {
        let mut handle = std::ptr::null_mut();
        let status = unsafe {
            ffi::audio_queue::at_audio_queue_allocate_buffer(
                self.raw.cast(),
                byte_size,
                &mut handle,
            )
        };
        status_to_result("AudioQueueAllocateBuffer", status)?;
        let raw: AudioQueueBufferRef =
            unsafe { ffi::audio_queue::at_audio_queue_buffer_raw(handle) }.cast();
        if raw.is_null() {
            return Err(AudioToolboxError::message(
                "AudioQueueAllocateBuffer",
                "framework returned a null AudioQueueBufferRef",
            ));
        }
        Ok(AudioQueueBufferHandle { handle, raw })
    }

    pub fn start(&self) -> Result<()> {
        let status = unsafe { ffi::audio_queue::at_audio_queue_start(self.raw.cast()) };
        status_to_result("AudioQueueStart", status)
    }

    pub fn stop(&self, immediate: bool) -> Result<()> {
        let status = unsafe { ffi::audio_queue::at_audio_queue_stop(self.raw.cast(), immediate) };
        status_to_result("AudioQueueStop", status)
    }

    pub fn reset(&self) -> Result<()> {
        let status = unsafe { ffi::audio_queue::at_audio_queue_reset(self.raw.cast()) };
        status_to_result("AudioQueueReset", status)
    }

    pub fn close(mut self) -> Result<()> {
        self.release();
        Ok(())
    }

    fn get_property_typed<T: Copy>(
        &self,
        property_id: AudioQueuePropertyId,
        operation: &'static str,
    ) -> Result<T> {
        let mut value = MaybeUninit::<T>::uninit();
        let mut size = u32::try_from(std::mem::size_of::<T>()).expect("typed property fits in u32");
        let status = unsafe {
            ffi::audio_queue::at_audio_queue_get_property(
                self.raw.cast(),
                property_id,
                &mut size,
                value.as_mut_ptr().cast(),
            )
        };
        status_to_result(operation, status)?;
        Ok(unsafe { value.assume_init() })
    }

    fn release(&mut self) {
        if !self.handle.is_null() {
            unsafe { ffi::audio_queue::at_audio_queue_release(self.handle) };
            self.handle = std::ptr::null_mut();
            self.raw = std::ptr::null_mut();
        }
    }
}

impl Drop for AudioQueue {
    fn drop(&mut self) {
        self.release();
    }
}

impl AudioQueueBufferHandle {
    pub fn as_raw(&self) -> AudioQueueBufferRef {
        self.raw
    }

    pub fn audio_data_bytes_capacity(&self) -> u32 {
        unsafe { (*self.raw).mAudioDataBytesCapacity }
    }

    fn release(&mut self) {
        if !self.handle.is_null() {
            unsafe { ffi::audio_queue::at_audio_queue_buffer_release(self.handle) };
            self.handle = std::ptr::null_mut();
            self.raw = std::ptr::null_mut();
        }
    }
}

impl Drop for AudioQueueBufferHandle {
    fn drop(&mut self) {
        self.release();
    }
}
