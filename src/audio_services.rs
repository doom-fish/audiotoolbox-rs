use crate::{
    ffi,
    internal::{path_to_cstring, status_to_result},
    AudioToolboxError, Result, SystemSoundId,
};
use doom_fish_utils::callback_context::CallbackContext;
use std::{
    ffi::c_void,
    path::Path,
    sync::{Mutex, PoisonError},
};

type Completion = Box<dyn FnOnce() + Send>;
type CompletionSlot = Mutex<Option<Completion>>;

unsafe extern "C" fn completion_trampoline(context: *mut c_void) {
    let _ = unsafe {
        CallbackContext::<CompletionSlot>::with(
            context,
            "audiotoolbox::system_sound_completion",
            |slot| {
                let completion = slot.lock().unwrap_or_else(PoisonError::into_inner).take();
                if let Some(completion) = completion {
                    completion();
                }
            },
        )
    };
}

#[derive(Debug)]
/// Owning wrapper around an AudioToolbox.framework `SystemSoundID`.
pub struct SystemSound {
    handle: *mut std::ffi::c_void,
}

impl SystemSound {
    /// Wraps `AudioServicesCreateSystemSoundID`.
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        let path = path_to_cstring(path.as_ref())?;
        let mut handle = std::ptr::null_mut();
        let status =
            unsafe { ffi::audio_services::at_system_sound_create(path.as_ptr(), &raw mut handle) };
        status_to_result("AudioServicesCreateSystemSoundID", status)?;
        if handle.is_null() {
            return Err(AudioToolboxError::message(
                "AudioServicesCreateSystemSoundID",
                "framework returned a null system sound handle",
            ));
        }
        Ok(Self { handle })
    }

    /// Wraps `SystemSoundID`.
    pub fn id(&self) -> SystemSoundId {
        unsafe { ffi::audio_services::at_system_sound_id(self.handle) }
    }

    /// Wraps `SystemSoundPlay`.
    pub fn play(&self) {
        unsafe { ffi::audio_services::at_system_sound_play(self.handle) };
    }

    /// Wraps `SystemSoundPlayAlert`.
    pub fn play_alert(&self) {
        unsafe { ffi::audio_services::at_system_sound_play_alert(self.handle) };
    }

    /// Wraps `AudioServicesGetProperty`.
    pub fn is_ui_sound(&self) -> Result<bool> {
        let mut value = 0_u32;
        let status = unsafe {
            ffi::audio_services::at_system_sound_get_is_ui_sound(self.handle, &raw mut value)
        };
        status_to_result("AudioServicesGetProperty(is UI sound)", status)?;
        Ok(value != 0)
    }

    /// Wraps `AudioServicesSetProperty`.
    pub fn set_is_ui_sound(&self, is_ui_sound: bool) -> Result<()> {
        let status = unsafe {
            ffi::audio_services::at_system_sound_set_is_ui_sound(
                self.handle,
                u32::from(is_ui_sound),
            )
        };
        status_to_result("AudioServicesSetProperty(is UI sound)", status)
    }

    /// Wraps `AudioServicesGetProperty`.
    pub fn complete_playback_if_app_dies(&self) -> Result<bool> {
        let mut value = 0_u32;
        let status = unsafe {
            ffi::audio_services::at_system_sound_get_complete_playback_if_app_dies(
                self.handle,
                &raw mut value,
            )
        };
        status_to_result(
            "AudioServicesGetProperty(complete playback if app dies)",
            status,
        )?;
        Ok(value != 0)
    }

    /// Wraps `AudioServicesSetProperty`.
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

    #[allow(clippy::missing_errors_doc)]
    pub fn play_with_completion<F: FnOnce() + Send + 'static>(&self, completion: F) -> Result<()> {
        self.play_with_completion_inner(false, Box::new(completion))
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn play_alert_with_completion<F: FnOnce() + Send + 'static>(
        &self,
        completion: F,
    ) -> Result<()> {
        self.play_with_completion_inner(true, Box::new(completion))
    }

    fn play_with_completion_inner(&self, alert: bool, completion: Completion) -> Result<()> {
        let context = CallbackContext::new(Mutex::new(Some(completion)));
        let started = unsafe {
            ffi::audio_services::at_system_sound_play_with_completion(
                self.handle,
                alert,
                completion_trampoline,
                context.retained_ptr(),
                CallbackContext::<CompletionSlot>::RELEASE,
            )
        };
        if started {
            Ok(())
        } else {
            Err(AudioToolboxError::message(
                "AudioServicesPlaySystemSoundWithCompletion",
                "the system sound could not be played",
            ))
        }
    }

    /// Wraps `SystemSoundDispose`.
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

#[cfg(test)]
mod tests {
    use super::{completion_trampoline, CompletionSlot};
    use doom_fish_utils::callback_context::CallbackContext;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};

    #[test]
    fn completion_runs_at_most_once() {
        let calls = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&calls);
        let context: CallbackContext<CompletionSlot> =
            CallbackContext::new(Mutex::new(Some(Box::new(move || {
                counter.fetch_add(1, Ordering::SeqCst);
            }))));
        let ptr = context.retained_ptr();
        unsafe {
            completion_trampoline(ptr);
            completion_trampoline(ptr);
        }
        assert_eq!(calls.load(Ordering::SeqCst), 1);
        drop(context);
        unsafe { completion_trampoline(ptr) };
        unsafe { (CallbackContext::<CompletionSlot>::RELEASE)(ptr) };
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }
}
