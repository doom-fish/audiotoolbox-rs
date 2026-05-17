use crate::{
    ffi,
    internal::{path_to_cstring, status_to_result},
    AudioToolboxError, Result, SystemSoundId,
};
use std::path::Path;

#[derive(Debug)]
pub struct SystemSound {
    handle: *mut std::ffi::c_void,
}

impl SystemSound {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        let path = path_to_cstring(path.as_ref())?;
        let mut handle = std::ptr::null_mut();
        let status =
            unsafe { ffi::audio_services::at_system_sound_create(path.as_ptr(), &mut handle) };
        status_to_result("AudioServicesCreateSystemSoundID", status)?;
        if handle.is_null() {
            return Err(AudioToolboxError::message(
                "AudioServicesCreateSystemSoundID",
                "framework returned a null system sound handle",
            ));
        }
        Ok(Self { handle })
    }

    pub fn id(&self) -> SystemSoundId {
        unsafe { ffi::audio_services::at_system_sound_id(self.handle) }
    }

    pub fn play(&self) {
        unsafe { ffi::audio_services::at_system_sound_play(self.handle) };
    }

    pub fn play_alert(&self) {
        unsafe { ffi::audio_services::at_system_sound_play_alert(self.handle) };
    }

    pub fn is_ui_sound(&self) -> Result<bool> {
        let mut value = 0_u32;
        let status = unsafe {
            ffi::audio_services::at_system_sound_get_is_ui_sound(self.handle, &mut value)
        };
        status_to_result("AudioServicesGetProperty(is UI sound)", status)?;
        Ok(value != 0)
    }

    pub fn set_is_ui_sound(&self, is_ui_sound: bool) -> Result<()> {
        let status = unsafe {
            ffi::audio_services::at_system_sound_set_is_ui_sound(
                self.handle,
                u32::from(is_ui_sound),
            )
        };
        status_to_result("AudioServicesSetProperty(is UI sound)", status)
    }

    pub fn complete_playback_if_app_dies(&self) -> Result<bool> {
        let mut value = 0_u32;
        let status = unsafe {
            ffi::audio_services::at_system_sound_get_complete_playback_if_app_dies(
                self.handle,
                &mut value,
            )
        };
        status_to_result(
            "AudioServicesGetProperty(complete playback if app dies)",
            status,
        )?;
        Ok(value != 0)
    }

    pub fn set_complete_playback_if_app_dies(&self, enabled: bool) -> Result<()> {
        let status = unsafe {
            ffi::audio_services::at_system_sound_set_complete_playback_if_app_dies(
                self.handle,
                u32::from(enabled),
            )
        };
        status_to_result(
            "AudioServicesSetProperty(complete playback if app dies)",
            status,
        )
    }

    pub fn dispose(mut self) -> Result<()> {
        self.release();
        Ok(())
    }

    fn release(&mut self) {
        if !self.handle.is_null() {
            unsafe { ffi::audio_services::at_system_sound_release(self.handle) };
            self.handle = std::ptr::null_mut();
        }
    }
}

impl Drop for SystemSound {
    fn drop(&mut self) {
        self.release();
    }
}
