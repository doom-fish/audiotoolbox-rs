use crate::{
    ffi, internal::status_to_result, AURenderCallback, AudioBufferList1,
    AudioStreamBasicDescription, AudioTimeStamp, AudioToolboxError, AudioUnitElement,
    AudioUnitParameterEvent, AudioUnitParameterId, AudioUnitParameterValue, AudioUnitPropertyId,
    AudioUnitPropertyListenerProc, AudioUnitRef, AudioUnitRenderActionFlags, AudioUnitScope,
    Result, AUDIO_COMPONENT_MANUFACTURER_APPLE, AUDIO_UNIT_PROPERTY_CPULOAD,
    AUDIO_UNIT_PROPERTY_ELEMENT_COUNT, AUDIO_UNIT_PROPERTY_LAST_RENDER_ERROR,
    AUDIO_UNIT_PROPERTY_LATENCY, AUDIO_UNIT_PROPERTY_PRESENTATION_LATENCY,
    AUDIO_UNIT_PROPERTY_SAMPLE_RATE, AUDIO_UNIT_PROPERTY_STREAM_FORMAT,
};
use std::mem::MaybeUninit;

#[derive(Debug)]
/// Wraps `AudioUnit`.
pub struct AudioUnit {
    handle: *mut std::ffi::c_void,
    raw: AudioUnitRef,
}

impl AudioUnit {
    /// Wraps `AudioUnitNew`.
    ///
    /// The returned wrapper owns the underlying AudioToolbox.framework handle and releases it on drop.
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

    /// Wraps `AudioUnitNewApple`.
    ///
    /// The returned wrapper owns the underlying AudioToolbox.framework handle and releases it on drop.
    pub fn new_apple(component_type: u32, component_sub_type: u32) -> Result<Self> {
        Self::new(
            component_type,
            component_sub_type,
            AUDIO_COMPONENT_MANUFACTURER_APPLE,
        )
    }

    /// Returns the wrapped `AudioUnitRef`.
    pub fn as_raw(&self) -> AudioUnitRef {
        self.raw
    }

    pub(crate) fn retained(&self) -> Result<Self> {
        let handle = unsafe { ffi::audio_unit::at_audio_unit_retain(self.handle) };
        if handle.is_null() {
            return Err(AudioToolboxError::message(
                "AudioUnitRetain",
                "framework returned a null AudioUnit handle",
            ));
        }

        let raw: AudioUnitRef = unsafe { ffi::audio_unit::at_audio_unit_raw(handle) }.cast();
        if raw.is_null() {
            unsafe { ffi::audio_unit::at_audio_unit_release(handle) };
            return Err(AudioToolboxError::message(
                "AudioUnitRetain",
                "framework returned a null AudioUnit",
            ));
        }

        Ok(Self { handle, raw })
    }

    /// Wraps `AudioUnitInitialize`.
    pub fn initialize(&self) -> Result<()> {
        let status = unsafe { ffi::audio_unit::at_audio_unit_initialize(self.raw.cast()) };
        status_to_result("AudioUnitInitialize", status)
    }

    /// Wraps `AudioUnitUninitialize`.
    pub fn uninitialize(&self) -> Result<()> {
        let status = unsafe { ffi::audio_unit::at_audio_unit_uninitialize(self.raw.cast()) };
        status_to_result("AudioUnitUninitialize", status)
    }

    /// Wraps `AudioOutputUnitStart`.
    pub fn start(&self) -> Result<()> {
        let status = unsafe { ffi::audio_unit::at_audio_output_unit_start(self.raw.cast()) };
        status_to_result("AudioOutputUnitStart", status)
    }

    /// Wraps `AudioOutputUnitStop`.
    pub fn stop(&self) -> Result<()> {
        let status = unsafe { ffi::audio_unit::at_audio_output_unit_stop(self.raw.cast()) };
        status_to_result("AudioOutputUnitStop", status)
    }

    /// Wraps `AudioUnitGetPropertyInfo`.
    pub fn property_info(
        &self,
        property_id: AudioUnitPropertyId,
        scope: AudioUnitScope,
        element: AudioUnitElement,
    ) -> Result<(u32, bool)> {
        let mut data_size = 0_u32;
        let mut writable = 0_u8;
        let status = unsafe {
            ffi::audio_unit::at_audio_unit_get_property_info(
                self.raw.cast(),
                property_id,
                scope,
                element,
                &mut data_size,
                &mut writable,
            )
        };
        status_to_result("AudioUnitGetPropertyInfo", status)?;
        Ok((data_size, writable != 0))
    }

    /// Wraps `AudioUnitGetProperty`.
    pub fn sample_rate(&self, scope: AudioUnitScope, element: AudioUnitElement) -> Result<f64> {
        self.get_property_typed(
            AUDIO_UNIT_PROPERTY_SAMPLE_RATE,
            scope,
            element,
            "AudioUnitGetProperty(sample rate)",
        )
    }

    /// Wraps `AudioUnitGetProperty`.
    pub fn latency(&self) -> Result<f64> {
        self.get_property_typed(
            AUDIO_UNIT_PROPERTY_LATENCY,
            crate::AUDIO_UNIT_SCOPE_GLOBAL,
            0,
            "AudioUnitGetProperty(latency)",
        )
    }

    /// Wraps `AudioUnitGetProperty`.
    pub fn presentation_latency(&self) -> Result<f64> {
        self.get_property_typed(
            AUDIO_UNIT_PROPERTY_PRESENTATION_LATENCY,
            crate::AUDIO_UNIT_SCOPE_GLOBAL,
            0,
            "AudioUnitGetProperty(presentation latency)",
        )
    }

    /// Wraps `AudioUnitGetProperty`.
    pub fn cpu_load(&self) -> Result<f32> {
        self.get_property_typed(
            AUDIO_UNIT_PROPERTY_CPULOAD,
            crate::AUDIO_UNIT_SCOPE_GLOBAL,
            0,
            "AudioUnitGetProperty(CPU load)",
        )
    }

    /// Wraps `AudioUnitGetProperty`.
    pub fn last_render_error(&self) -> Result<crate::OSStatus> {
        self.get_property_typed(
            AUDIO_UNIT_PROPERTY_LAST_RENDER_ERROR,
            crate::AUDIO_UNIT_SCOPE_GLOBAL,
            0,
            "AudioUnitGetProperty(last render error)",
        )
    }

    /// Wraps `AudioUnitGetProperty`.
    pub fn element_count(&self, scope: AudioUnitScope) -> Result<u32> {
        self.get_property_typed(
            AUDIO_UNIT_PROPERTY_ELEMENT_COUNT,
            scope,
            0,
            "AudioUnitGetProperty(element count)",
        )
    }

    /// Wraps `AudioUnitGetProperty`.
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

    /// Wraps `AudioUnitSetProperty`.
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

    /// Wraps `AudioUnitGetParameter`.
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

    /// Wraps `AudioUnitSetParameter`.
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

    /// Wraps `AudioUnitAddPropertyListener`.
    ///
    /// # Safety
    ///
    /// `user_data` must remain valid for the lifetime of the registered listener.
    pub unsafe fn add_property_listener(
        &self,
        property_id: AudioUnitPropertyId,
        proc: AudioUnitPropertyListenerProc,
        user_data: *mut std::ffi::c_void,
    ) -> Result<()> {
        let status = unsafe {
            ffi::audio_unit::at_audio_unit_add_property_listener(
                self.raw.cast(),
                property_id,
                proc,
                user_data,
            )
        };
        status_to_result("AudioUnitAddPropertyListener", status)
    }

    /// Wraps `AudioUnitRemovePropertyListenerWithUserData`.
    ///
    /// # Safety
    ///
    /// `user_data` must match the pointer used when the listener was registered.
    pub unsafe fn remove_property_listener_with_user_data(
        &self,
        property_id: AudioUnitPropertyId,
        proc: AudioUnitPropertyListenerProc,
        user_data: *mut std::ffi::c_void,
    ) -> Result<()> {
        let status = unsafe {
            ffi::audio_unit::at_audio_unit_remove_property_listener_with_user_data(
                self.raw.cast(),
                property_id,
                proc,
                user_data,
            )
        };
        status_to_result("AudioUnitRemovePropertyListenerWithUserData", status)
    }

    /// Wraps `AudioUnitAddRenderNotify`.
    ///
    /// # Safety
    ///
    /// `user_data` must remain valid for as long as the render notify callback can be invoked.
    pub unsafe fn add_render_notify(
        &self,
        proc: AURenderCallback,
        user_data: *mut std::ffi::c_void,
    ) -> Result<()> {
        let status = unsafe {
            ffi::audio_unit::at_audio_unit_add_render_notify(self.raw.cast(), proc, user_data)
        };
        status_to_result("AudioUnitAddRenderNotify", status)
    }

    /// Wraps `AudioUnitRemoveRenderNotify`.
    ///
    /// # Safety
    ///
    /// `user_data` must match the pointer used when the render notify callback was registered.
    pub unsafe fn remove_render_notify(
        &self,
        proc: AURenderCallback,
        user_data: *mut std::ffi::c_void,
    ) -> Result<()> {
        let status = unsafe {
            ffi::audio_unit::at_audio_unit_remove_render_notify(self.raw.cast(), proc, user_data)
        };
        status_to_result("AudioUnitRemoveRenderNotify", status)
    }

    /// Subscribe to async property-listener events for `property_id`.
    #[cfg(feature = "async")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async")))]
    pub fn property_events(
        &self,
        property_id: AudioUnitPropertyId,
        capacity: usize,
    ) -> Result<crate::async_api::AudioUnitPropertyStream> {
        crate::async_api::AudioUnitPropertyStream::subscribe(self, property_id, capacity)
    }

    /// Subscribe to async render-notify events.
    #[cfg(feature = "async")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async")))]
    pub fn render_notify_stream(
        &self,
        capacity: usize,
    ) -> Result<crate::async_api::AudioUnitRenderNotifyStream> {
        crate::async_api::AudioUnitRenderNotifyStream::subscribe(self, capacity)
    }

    /// Wraps `AudioUnitScheduleParameters`.
    pub fn schedule_parameters(&self, events: &[AudioUnitParameterEvent]) -> Result<()> {
        let event_count = u32::try_from(events.len()).map_err(|_| {
            AudioToolboxError::message("AudioUnitScheduleParameters", "too many parameter events")
        })?;
        let status = unsafe {
            ffi::audio_unit::at_audio_unit_schedule_parameters(
                self.raw.cast(),
                event_count,
                events.as_ptr(),
            )
        };
        status_to_result("AudioUnitScheduleParameters", status)
    }

    /// Wraps `AudioUnitRender`.
    pub fn render(
        &self,
        io_action_flags: &mut AudioUnitRenderActionFlags,
        time_stamp: &AudioTimeStamp,
        output_bus_number: u32,
        number_frames: u32,
        io_data: &mut AudioBufferList1,
    ) -> Result<()> {
        let status = unsafe {
            ffi::audio_unit::at_audio_unit_render(
                self.raw.cast(),
                io_action_flags,
                time_stamp,
                output_bus_number,
                number_frames,
                io_data,
            )
        };
        status_to_result("AudioUnitRender", status)
    }

    /// Wraps `AudioUnitClose`.
    pub fn close(mut self) -> Result<()> {
        self.release();
        Ok(())
    }

    /// Wraps `AudioUnitGetProperty`.
    pub fn get_property_typed<T: Copy>(
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

    /// Wraps `AudioUnitSetProperty`.
    pub fn set_property_typed<T: Copy>(
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
