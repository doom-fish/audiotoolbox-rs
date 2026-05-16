use crate::{
    ffi,
    internal::status_to_result,
    AudioStreamBasicDescription,
    AudioToolboxError,
    AudioUnitElement,
    AudioUnitParameterId,
    AudioUnitParameterValue,
    AudioUnitPropertyId,
    AudioUnitRef,
    AudioUnitScope,
    Result,
    AUDIO_COMPONENT_MANUFACTURER_APPLE,
    AUDIO_UNIT_PROPERTY_STREAM_FORMAT,
};
use std::mem::MaybeUninit;

#[derive(Debug)]
pub struct AudioUnit {
    handle: *mut std::ffi::c_void,
    raw: AudioUnitRef,
}

impl AudioUnit {
    pub fn new(component_type: u32, component_sub_type: u32, manufacturer: u32) -> Result<Self> {
        let mut handle = std::ptr::null_mut();
        let status = unsafe {
            ffi::audio_unit::at_audio_unit_new(
                component_type,
                component_sub_type,
                manufacturer,
                &mut handle,
            )
        };
        status_to_result("AudioUnitNew", status)?;
        let raw: AudioUnitRef = unsafe { ffi::audio_unit::at_audio_unit_raw(handle) }.cast();
        if raw.is_null() {
            return Err(AudioToolboxError::message(
                "AudioUnitNew",
                "framework returned a null AudioUnit",
            ));
        }
        Ok(Self { handle, raw })
    }

    pub fn new_apple(component_type: u32, component_sub_type: u32) -> Result<Self> {
        Self::new(component_type, component_sub_type, AUDIO_COMPONENT_MANUFACTURER_APPLE)
    }

    pub fn as_raw(&self) -> AudioUnitRef {
        self.raw
    }

    pub fn initialize(&self) -> Result<()> {
        let status = unsafe { ffi::audio_unit::at_audio_unit_initialize(self.raw.cast()) };
        status_to_result("AudioUnitInitialize", status)
    }

    pub fn uninitialize(&self) -> Result<()> {
        let status = unsafe { ffi::audio_unit::at_audio_unit_uninitialize(self.raw.cast()) };
        status_to_result("AudioUnitUninitialize", status)
    }

    pub fn start(&self) -> Result<()> {
        let status = unsafe { ffi::audio_unit::at_audio_output_unit_start(self.raw.cast()) };
        status_to_result("AudioOutputUnitStart", status)
    }

    pub fn stop(&self) -> Result<()> {
        let status = unsafe { ffi::audio_unit::at_audio_output_unit_stop(self.raw.cast()) };
        status_to_result("AudioOutputUnitStop", status)
    }

    pub fn stream_format(
        &self,
        scope: AudioUnitScope,
        element: AudioUnitElement,
    ) -> Result<AudioStreamBasicDescription> {
        self.get_property_typed(
            AUDIO_UNIT_PROPERTY_STREAM_FORMAT,
            scope,
            element,
            "AudioUnitGetProperty(stream format)",
        )
    }

    pub fn set_stream_format(
        &self,
        scope: AudioUnitScope,
        element: AudioUnitElement,
        format: &AudioStreamBasicDescription,
    ) -> Result<()> {
        self.set_property_typed(
            AUDIO_UNIT_PROPERTY_STREAM_FORMAT,
            scope,
            element,
            format,
            "AudioUnitSetProperty(stream format)",
        )
    }

    pub fn get_parameter(
        &self,
        parameter_id: AudioUnitParameterId,
        scope: AudioUnitScope,
        element: AudioUnitElement,
    ) -> Result<AudioUnitParameterValue> {
        let mut value = 0.0_f32;
        let status = unsafe {
            ffi::audio_unit::at_audio_unit_get_parameter(
                self.raw.cast(),
                parameter_id,
                scope,
                element,
                &mut value,
            )
        };
        status_to_result("AudioUnitGetParameter", status)?;
        Ok(value)
    }

    pub fn set_parameter(
        &self,
        parameter_id: AudioUnitParameterId,
        scope: AudioUnitScope,
        element: AudioUnitElement,
        value: AudioUnitParameterValue,
        buffer_offset_in_frames: u32,
    ) -> Result<()> {
        let status = unsafe {
            ffi::audio_unit::at_audio_unit_set_parameter(
                self.raw.cast(),
                parameter_id,
                scope,
                element,
                value,
                buffer_offset_in_frames,
            )
        };
        status_to_result("AudioUnitSetParameter", status)
    }

    pub fn close(mut self) -> Result<()> {
        self.release();
        Ok(())
    }

    fn get_property_typed<T: Copy>(
        &self,
        property_id: AudioUnitPropertyId,
        scope: AudioUnitScope,
        element: AudioUnitElement,
        operation: &'static str,
    ) -> Result<T> {
        let mut value = MaybeUninit::<T>::uninit();
        let mut size = u32::try_from(std::mem::size_of::<T>()).expect("typed property fits in u32");
        let status = unsafe {
            ffi::audio_unit::at_audio_unit_get_property(
                self.raw.cast(),
                property_id,
                scope,
                element,
                &mut size,
                value.as_mut_ptr().cast(),
            )
        };
        status_to_result(operation, status)?;
        Ok(unsafe { value.assume_init() })
    }

    fn set_property_typed<T: Copy>(
        &self,
        property_id: AudioUnitPropertyId,
        scope: AudioUnitScope,
        element: AudioUnitElement,
        value: &T,
        operation: &'static str,
    ) -> Result<()> {
        let size = u32::try_from(std::mem::size_of::<T>()).expect("typed property fits in u32");
        let status = unsafe {
            ffi::audio_unit::at_audio_unit_set_property(
                self.raw.cast(),
                property_id,
                scope,
                element,
                size,
                std::ptr::from_ref(value).cast(),
            )
        };
        status_to_result(operation, status)
    }

    fn release(&mut self) {
        if !self.handle.is_null() {
            unsafe { ffi::audio_unit::at_audio_unit_release(self.handle) };
            self.handle = std::ptr::null_mut();
            self.raw = std::ptr::null_mut();
        }
    }
}

impl Drop for AudioUnit {
    fn drop(&mut self) {
        self.release();
    }
}
