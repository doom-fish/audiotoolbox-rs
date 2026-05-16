use crate::{cf, ffi, AudioToolboxError, Result};
use std::{mem::MaybeUninit, path::Path};

pub const SYSTEM_SOUND_USER_PREFERRED_ALERT: ffi::SystemSoundId = 0x0000_1000;
pub const SYSTEM_SOUND_FLASH_SCREEN: ffi::SystemSoundId = 0x0000_0FFE;

#[derive(Debug)]
pub struct SystemSound {
    id: ffi::SystemSoundId,
    disposed: bool,
}

impl SystemSound {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        let url = cf::path_to_url(path.as_ref())?;
        let mut id = MaybeUninit::uninit();
        let status =
            unsafe { ffi::AudioServicesCreateSystemSoundID(url.as_ptr(), id.as_mut_ptr()) };
        status_to_result("AudioServicesCreateSystemSoundID", status)?;
        Ok(Self {
            id: unsafe { id.assume_init() },
            disposed: false,
        })
    }

    pub fn id(&self) -> ffi::SystemSoundId {
        self.id
    }

    pub fn play(&self) {
        unsafe { ffi::AudioServicesPlaySystemSound(self.id) };
    }

    pub fn dispose(mut self) -> Result<()> {
        self.disposed = true;
        let status = unsafe { ffi::AudioServicesDisposeSystemSoundID(self.id) };
        status_to_result("AudioServicesDisposeSystemSoundID", status)
    }
}

impl Drop for SystemSound {
    fn drop(&mut self) {
        if !self.disposed {
            let _ = unsafe { ffi::AudioServicesDisposeSystemSoundID(self.id) };
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
