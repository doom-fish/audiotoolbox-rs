use crate::{cf, ffi, AudioComponentDescription, AudioToolboxError, Result};
use std::{mem::MaybeUninit, ptr::NonNull};

pub const AUDIO_COMPONENT_TYPE_OUTPUT: u32 = ffi::fourcc(*b"auou");
pub const AUDIO_COMPONENT_TYPE_MUSIC_DEVICE: u32 = ffi::fourcc(*b"aumu");
pub const AUDIO_COMPONENT_TYPE_MUSIC_EFFECT: u32 = ffi::fourcc(*b"aumf");
pub const AUDIO_COMPONENT_TYPE_FORMAT_CONVERTER: u32 = ffi::fourcc(*b"aufc");
pub const AUDIO_COMPONENT_TYPE_EFFECT: u32 = ffi::fourcc(*b"aufx");
pub const AUDIO_COMPONENT_TYPE_MIXER: u32 = ffi::fourcc(*b"aumx");
pub const AUDIO_COMPONENT_TYPE_PANNER: u32 = ffi::fourcc(*b"aupn");
pub const AUDIO_COMPONENT_TYPE_GENERATOR: u32 = ffi::fourcc(*b"augn");
pub const AUDIO_COMPONENT_TYPE_OFFLINE_EFFECT: u32 = ffi::fourcc(*b"auol");
pub const AUDIO_COMPONENT_TYPE_MIDI_PROCESSOR: u32 = ffi::fourcc(*b"aumi");
pub const AUDIO_COMPONENT_MANUFACTURER_APPLE: u32 = ffi::fourcc(*b"appl");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioComponent(NonNull<std::ffi::c_void>);

#[derive(Debug, Clone)]
pub struct AudioComponentIter {
    previous: Option<AudioComponent>,
    description: AudioComponentDescription,
}

#[derive(Debug)]
pub struct AudioComponentInstance {
    raw: ffi::AudioComponentInstanceRef,
}

impl AudioComponent {
    pub fn count(description: AudioComponentDescription) -> u32 {
        unsafe { ffi::AudioComponentCount(std::ptr::from_ref(&description)) }
    }

    pub fn find_next(
        previous: Option<Self>,
        description: AudioComponentDescription,
    ) -> Option<Self> {
        let previous_raw = previous.map_or(std::ptr::null_mut(), |component| component.as_raw());
        let raw =
            unsafe { ffi::AudioComponentFindNext(previous_raw, std::ptr::from_ref(&description)) };
        NonNull::new(raw).map(Self)
    }

    pub fn iter(description: AudioComponentDescription) -> AudioComponentIter {
        AudioComponentIter {
            previous: None,
            description,
        }
    }

    pub fn as_raw(&self) -> ffi::AudioComponentRef {
        self.0.as_ptr()
    }

    pub fn copy_name(&self) -> Result<String> {
        let mut cf_name = MaybeUninit::uninit();
        let status = unsafe { ffi::AudioComponentCopyName(self.as_raw(), cf_name.as_mut_ptr()) };
        status_to_result("AudioComponentCopyName", status)?;
        let cf_name = unsafe { cf_name.assume_init() };
        let owned =
            unsafe { cf::OwnedCFType::from_create_rule(cf_name.cast()) }.ok_or_else(|| {
                AudioToolboxError::message(
                    "AudioComponentCopyName",
                    "framework returned a null name",
                )
            })?;
        cf::cfstring_to_string(owned.as_ptr())
    }

    pub fn description(&self) -> Result<AudioComponentDescription> {
        let mut description = MaybeUninit::uninit();
        let status =
            unsafe { ffi::AudioComponentGetDescription(self.as_raw(), description.as_mut_ptr()) };
        status_to_result("AudioComponentGetDescription", status)?;
        Ok(unsafe { description.assume_init() })
    }

    pub fn version(&self) -> Result<u32> {
        let mut version = 0_u32;
        let status = unsafe { ffi::AudioComponentGetVersion(self.as_raw(), &mut version) };
        status_to_result("AudioComponentGetVersion", status)?;
        Ok(version)
    }

    pub fn new_instance(&self) -> Result<AudioComponentInstance> {
        let mut instance = MaybeUninit::uninit();
        let status =
            unsafe { ffi::AudioComponentInstanceNew(self.as_raw(), instance.as_mut_ptr()) };
        status_to_result("AudioComponentInstanceNew", status)?;
        Ok(AudioComponentInstance {
            raw: unsafe { instance.assume_init() },
        })
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
    pub fn as_raw(&self) -> ffi::AudioComponentInstanceRef {
        self.raw
    }

    pub fn component(&self) -> Result<AudioComponent> {
        NonNull::new(unsafe { ffi::AudioComponentInstanceGetComponent(self.raw) })
            .map(AudioComponent)
            .ok_or_else(|| {
                AudioToolboxError::message(
                    "AudioComponentInstanceGetComponent",
                    "framework returned a null component",
                )
            })
    }

    pub fn dispose(mut self) -> Result<()> {
        let raw = self.raw;
        self.raw = std::ptr::null_mut();
        let status = unsafe { ffi::AudioComponentInstanceDispose(raw) };
        status_to_result("AudioComponentInstanceDispose", status)
    }
}

impl Drop for AudioComponentInstance {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            let _ = unsafe { ffi::AudioComponentInstanceDispose(self.raw) };
        }
    }
}

fn status_to_result(operation: &'static str, status: ffi::OSStatus) -> Result<()> {
    if status == ffi::NO_ERR {
        Ok(())
    } else {
        Err(AudioToolboxError::from_status(operation, status))
    }
}
