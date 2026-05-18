use crate::{
    ffi,
    internal::{status_to_result, string_from_owned_ptr},
    AudioComponentDescription, AudioComponentInstanceRef, AudioComponentRef,
    AudioComponentValidationResult, AudioToolboxError, CFDictionaryRef, Result,
};
use std::{mem::MaybeUninit, ptr::NonNull};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Wrapper around an AudioToolbox.framework `AudioComponent`.
pub struct AudioComponent(NonNull<std::ffi::c_void>);

#[derive(Debug, Clone)]
/// Iterator over components returned by `AudioComponentFindNext`.
pub struct AudioComponentIter {
    previous: Option<AudioComponent>,
    description: AudioComponentDescription,
}

#[derive(Debug)]
/// Owning wrapper around an AudioToolbox.framework `AudioComponentInstance`.
pub struct AudioComponentInstance {
    handle: *mut std::ffi::c_void,
    raw: AudioComponentInstanceRef,
}

impl AudioComponent {
    /// Wraps `AudioComponentCount`.
    pub fn count(description: AudioComponentDescription) -> u32 {
        unsafe { ffi::audio_component::at_audio_component_count(std::ptr::from_ref(&description)) }
    }

    /// Wraps `AudioComponentFindNext`.
    pub fn find_next(
        previous: Option<Self>,
        description: AudioComponentDescription,
    ) -> Option<Self> {
        let previous_raw = previous.map_or(std::ptr::null_mut(), |component| component.as_raw());
        let handle = unsafe {
            ffi::audio_component::at_audio_component_find_next(
                previous_raw.cast(),
                std::ptr::from_ref(&description),
            )
        };
        let raw = NonNull::new(unsafe { ffi::audio_component::at_audio_component_raw(handle) })?;
        unsafe { ffi::audio_component::at_audio_component_release(handle) };
        Some(Self(raw))
    }

    /// Returns an iterator that walks `AudioComponentFindNext`.
    pub fn iter(description: AudioComponentDescription) -> AudioComponentIter {
        AudioComponentIter {
            previous: None,
            description,
        }
    }

    /// Returns the wrapped `AudioComponentRef`.
    pub fn as_raw(&self) -> AudioComponentRef {
        self.0.as_ptr()
    }

    /// Wraps `AudioComponentCopyName`.
    pub fn copy_name(&self) -> Result<String> {
        let mut status = 0;
        let ptr = unsafe {
            ffi::audio_component::at_audio_component_copy_name(self.as_raw().cast(), &mut status)
        };
        status_to_result("AudioComponentCopyName", status)?;
        string_from_owned_ptr("AudioComponentCopyName", ptr)
    }

    /// Wraps `AudioComponentGetDescription`.
    pub fn description(&self) -> Result<AudioComponentDescription> {
        let mut description = MaybeUninit::uninit();
        let status = unsafe {
            ffi::audio_component::at_audio_component_get_description(
                self.as_raw().cast(),
                description.as_mut_ptr(),
            )
        };
        status_to_result("AudioComponentGetDescription", status)?;
        Ok(unsafe { description.assume_init() })
    }

    /// Wraps `AudioComponentGetVersion`.
    pub fn version(&self) -> Result<u32> {
        let mut version = 0_u32;
        let status = unsafe {
            ffi::audio_component::at_audio_component_get_version(self.as_raw().cast(), &mut version)
        };
        status_to_result("AudioComponentGetVersion", status)?;
        Ok(version)
    }

    /// Wraps `AudioComponentCopyConfigurationInfo`.
    pub fn copy_configuration_info_raw(&self) -> Result<CFDictionaryRef> {
        let mut configuration_info = std::ptr::null();
        let status = unsafe {
            ffi::audio_component::at_audio_component_copy_configuration_info(
                self.as_raw(),
                &mut configuration_info,
            )
        };
        status_to_result("AudioComponentCopyConfigurationInfo", status)?;
        Ok(configuration_info)
    }

    /// Wraps `AudioComponentValidate`.
    pub fn validate_raw(
        &self,
        validation_parameters: Option<CFDictionaryRef>,
    ) -> Result<AudioComponentValidationResult> {
        let mut validation_result = 0_u32;
        let status = unsafe {
            ffi::audio_component::at_audio_component_validate(
                self.as_raw(),
                validation_parameters.unwrap_or(std::ptr::null()),
                &mut validation_result,
            )
        };
        status_to_result("AudioComponentValidate", status)?;
        Ok(validation_result)
    }

    /// Wraps `AudioComponentInstanceNew`.
    ///
    /// The returned wrapper owns the underlying AudioToolbox.framework handle and releases it on drop.
    pub fn new_instance(&self) -> Result<AudioComponentInstance> {
        let mut handle = std::ptr::null_mut();
        let status = unsafe {
            ffi::audio_component::at_audio_component_instance_new(self.as_raw().cast(), &mut handle)
        };
        status_to_result("AudioComponentInstanceNew", status)?;
        let raw: AudioComponentInstanceRef =
            unsafe { ffi::audio_component::at_audio_component_instance_raw(handle) }.cast();
        if raw.is_null() {
            return Err(AudioToolboxError::message(
                "AudioComponentInstanceNew",
                "framework returned a null AudioComponentInstance",
            ));
        }
        Ok(AudioComponentInstance { handle, raw })
    }
}

impl Iterator for AudioComponentIter {
    type Item = AudioComponent;

    fn next(&mut self) -> Option<Self::Item> {
        let next = AudioComponent::find_next(self.previous, self.description);
        self.previous = next;
        next
    }
}

impl AudioComponentInstance {
    /// Returns the wrapped `AudioComponentInstanceRef`.
    pub fn as_raw(&self) -> AudioComponentInstanceRef {
        self.raw
    }

    /// Wraps `AudioComponentInstanceGetComponent`.
    pub fn component(&self) -> Result<AudioComponent> {
        let handle = unsafe {
            ffi::audio_component::at_audio_component_instance_get_component(self.raw.cast())
        };
        let raw = NonNull::new(unsafe { ffi::audio_component::at_audio_component_raw(handle) })
            .ok_or_else(|| {
                AudioToolboxError::message(
                    "AudioComponentInstanceGetComponent",
                    "framework returned a null component",
                )
            })?;
        unsafe { ffi::audio_component::at_audio_component_release(handle) };
        Ok(AudioComponent(raw))
    }

    /// Wraps `AudioComponentInstanceCanDo`.
    pub fn can_do(&self, selector_id: i16) -> bool {
        unsafe {
            ffi::audio_component::at_audio_component_instance_can_do(self.raw, selector_id) != 0
        }
    }

    /// Wraps `AudioComponentInstanceDispose`.
    pub fn dispose(mut self) -> Result<()> {
        self.release();
        Ok(())
    }

    fn release(&mut self) {
        if !self.handle.is_null() {
            unsafe { ffi::audio_component::at_audio_component_instance_release(self.handle) };
            self.handle = std::ptr::null_mut();
            self.raw = std::ptr::null_mut();
        }
    }
}

impl Drop for AudioComponentInstance {
    fn drop(&mut self) {
        self.release();
    }
}
