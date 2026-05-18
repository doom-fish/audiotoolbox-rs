use crate::{
    ffi,
    internal::{error_from_owned_ptr, string_from_owned_ptr},
    AUAudioFrameCount, AudioComponentDescription, AudioComponentInstantiationOptions,
    AudioToolboxError, Result, AUDIO_COMPONENT_INSTANTIATION_LOAD_IN_PROCESS,
    AUDIO_COMPONENT_MANUFACTURER_APPLE,
};
use std::{ffi::c_void, mem::MaybeUninit};

#[derive(Debug)]
/// Wraps `AUAudioUnit`.
pub struct AUAudioUnit {
    handle: *mut c_void,
}

impl AUAudioUnit {
    /// Wraps `AUAudioUnitInit`.
    ///
    /// The returned wrapper owns the underlying AudioToolbox.framework handle and releases it on drop.
    pub fn new(
        description: AudioComponentDescription,
        options: AudioComponentInstantiationOptions,
    ) -> Result<Self> {
        let mut handle = std::ptr::null_mut();
        let mut error = std::ptr::null_mut();
        let ok = unsafe {
            ffi::au_audio_unit::at_au_audio_unit_new(
                std::ptr::from_ref(&description),
                options,
                &mut handle,
                &mut error,
            )
        };
        if ok {
            Self::from_handle(handle, "AUAudioUnitInit")
        } else {
            Err(error_from_owned_ptr("AUAudioUnitInit", error))
        }
    }

    /// Wraps `AUAudioUnitNewInProcess`.
    ///
    /// The returned wrapper owns the underlying AudioToolbox.framework handle and releases it on drop.
    pub fn new_in_process(description: AudioComponentDescription) -> Result<Self> {
        Self::new(description, AUDIO_COMPONENT_INSTANTIATION_LOAD_IN_PROCESS)
    }

    /// Wraps `AUAudioUnitNewApple`.
    ///
    /// The returned wrapper owns the underlying AudioToolbox.framework handle and releases it on drop.
    pub fn new_apple(component_type: u32, component_sub_type: u32) -> Result<Self> {
        Self::new_in_process(AudioComponentDescription::new(
            component_type,
            component_sub_type,
            AUDIO_COMPONENT_MANUFACTURER_APPLE,
        ))
    }

    /// Wraps `AUAudioUnitComponentDescription`.
    pub fn component_description(&self) -> Result<AudioComponentDescription> {
        let mut description = MaybeUninit::<AudioComponentDescription>::uninit();
        let ok = unsafe {
            ffi::au_audio_unit::at_au_audio_unit_component_description(
                self.handle,
                description.as_mut_ptr(),
            )
        };
        if ok {
            Ok(unsafe { description.assume_init() })
        } else {
            Err(AudioToolboxError::message(
                "AUAudioUnitComponentDescription",
                "framework returned a null AudioComponentDescription",
            ))
        }
    }

    /// Wraps `AUAudioUnitComponentName`.
    pub fn component_name(&self) -> Result<Option<String>> {
        optional_string_from_owned_ptr("AUAudioUnitComponentName", unsafe {
            ffi::au_audio_unit::at_au_audio_unit_copy_component_name(self.handle)
        })
    }

    /// Wraps `AUAudioUnitAudioUnitName`.
    pub fn audio_unit_name(&self) -> Result<Option<String>> {
        optional_string_from_owned_ptr("AUAudioUnitAudioUnitName", unsafe {
            ffi::au_audio_unit::at_au_audio_unit_copy_audio_unit_name(self.handle)
        })
    }

    /// Wraps `AUAudioUnitManufacturerName`.
    pub fn manufacturer_name(&self) -> Result<Option<String>> {
        optional_string_from_owned_ptr("AUAudioUnitManufacturerName", unsafe {
            ffi::au_audio_unit::at_au_audio_unit_copy_manufacturer_name(self.handle)
        })
    }

    /// Wraps `AUAudioUnitInputBusCount`.
    pub fn input_bus_count(&self) -> u64 {
        unsafe { ffi::au_audio_unit::at_au_audio_unit_input_bus_count(self.handle) }
    }

    /// Wraps `AUAudioUnitOutputBusCount`.
    pub fn output_bus_count(&self) -> u64 {
        unsafe { ffi::au_audio_unit::at_au_audio_unit_output_bus_count(self.handle) }
    }

    /// Wraps `AUAudioUnitAllocateRenderResources`.
    pub fn allocate_render_resources(&self) -> Result<()> {
        let mut error = std::ptr::null_mut();
        if unsafe {
            ffi::au_audio_unit::at_au_audio_unit_allocate_render_resources(self.handle, &mut error)
        } {
            Ok(())
        } else {
            Err(error_from_owned_ptr(
                "AUAudioUnitAllocateRenderResources",
                error,
            ))
        }
    }

    /// Wraps `AUAudioUnitDeallocateRenderResources`.
    pub fn deallocate_render_resources(&self) {
        unsafe { ffi::au_audio_unit::at_au_audio_unit_deallocate_render_resources(self.handle) };
    }

    /// Wraps `AUAudioUnitRenderResourcesAllocated`.
    pub fn render_resources_allocated(&self) -> bool {
        unsafe { ffi::au_audio_unit::at_au_audio_unit_render_resources_allocated(self.handle) }
    }

    /// Wraps `AUAudioUnitReset`.
    pub fn reset(&self) {
        unsafe { ffi::au_audio_unit::at_au_audio_unit_reset(self.handle) };
    }

    /// Wraps `AUAudioUnitMaximumFramesToRender`.
    pub fn maximum_frames_to_render(&self) -> AUAudioFrameCount {
        unsafe { ffi::au_audio_unit::at_au_audio_unit_maximum_frames_to_render(self.handle) }
    }

    /// Wraps `AUAudioUnitSetMaximumFramesToRender`.
    pub fn set_maximum_frames_to_render(&self, maximum_frames: AUAudioFrameCount) {
        unsafe {
            ffi::au_audio_unit::at_au_audio_unit_set_maximum_frames_to_render(
                self.handle,
                maximum_frames,
            );
        }
    }

    fn from_handle(handle: *mut c_void, operation: &'static str) -> Result<Self> {
        if handle.is_null() {
            Err(AudioToolboxError::message(
                operation,
                "framework returned a null AUAudioUnit",
            ))
        } else {
            Ok(Self { handle })
        }
    }

    fn release(&mut self) {
        if !self.handle.is_null() {
            unsafe { ffi::au_audio_unit::at_au_audio_unit_release(self.handle) };
            self.handle = std::ptr::null_mut();
        }
    }
}

fn optional_string_from_owned_ptr(operation: &'static str, ptr: *mut i8) -> Result<Option<String>> {
    if ptr.is_null() {
        Ok(None)
    } else {
        string_from_owned_ptr(operation, ptr).map(Some)
    }
}

impl Drop for AUAudioUnit {
    fn drop(&mut self) {
        self.release();
    }
}
