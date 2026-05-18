use crate::{
    ffi,
    internal::{path_to_cstring, status_to_result, string_from_owned_ptr},
    AudioFilePermissions, AudioFilePropertyId, AudioFileTypeId, AudioStreamBasicDescription,
    AudioToolboxError, PropertyInfo, Result, AUDIO_FILE_PROPERTY_DATA_FORMAT,
    AUDIO_FILE_READ_PERMISSION,
};
use std::{fs::OpenOptions, mem::MaybeUninit, os::fd::IntoRawFd, path::Path};

#[derive(Debug)]
/// Wraps `AudioFileComponent`.
pub struct AudioFileComponent {
    handle: *mut std::ffi::c_void,
}

impl AudioFileComponent {
    /// Wraps `AudioFileComponentNew`.
    pub fn new() -> Result<Self> {
        let mut handle = std::ptr::null_mut();
        let status =
            unsafe { ffi::audio_file_component::at_audio_file_component_new_default(&mut handle) };
        status_to_result("AudioFileComponentNew", status)?;
        Self::from_handle(handle, "AudioFileComponentNew")
    }

    /// Wraps `AudioFileComponentOpen`.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let component = Self::new()?;
        component.open_with_permissions(path, AUDIO_FILE_READ_PERMISSION)?;
        Ok(component)
    }

    /// Wraps `AudioFileComponentOpenURL`.
    pub fn open_with_permissions(
        &self,
        path: impl AsRef<Path>,
        permissions: AudioFilePermissions,
    ) -> Result<()> {
        let path = path.as_ref();
        let path_cstring = path_to_cstring(path)?;
        let file = open_file_for_permissions(path, permissions, "AudioFileComponentOpenURL")?;
        let file_descriptor = file.into_raw_fd();
        let status = unsafe {
            ffi::audio_file_component::at_audio_file_component_open(
                self.handle,
                path_cstring.as_ptr(),
                permissions,
                file_descriptor,
            )
        };
        if status != crate::NO_ERR {
            unsafe { libc::close(file_descriptor) };
        }
        status_to_result("AudioFileComponentOpenURL", status)
    }

    /// Wraps `AudioFileComponentCloseFile`.
    pub fn close_file(&self) -> Result<()> {
        let status =
            unsafe { ffi::audio_file_component::at_audio_file_component_close_file(self.handle) };
        status_to_result("AudioFileComponentCloseFile", status)
    }

    /// Wraps `AudioFileComponentGetPropertyInfo`.
    pub fn property_info(&self, property_id: AudioFilePropertyId) -> Result<PropertyInfo> {
        let mut data_size = 0_u32;
        let mut writable = 0_u32;
        let status = unsafe {
            ffi::audio_file_component::at_audio_file_component_get_property_info(
                self.handle,
                property_id,
                &mut data_size,
                &mut writable,
            )
        };
        status_to_result("AudioFileComponentGetPropertyInfo", status)?;
        Ok(PropertyInfo {
            data_size,
            writable: writable != 0,
        })
    }

    /// Wraps `AudioFileComponentGetProperty`.
    pub fn data_format(&self) -> Result<AudioStreamBasicDescription> {
        self.get_property_typed(
            AUDIO_FILE_PROPERTY_DATA_FORMAT,
            "AudioFileComponentGetProperty(data format)",
        )
    }

    /// Wraps `AudioFileComponentGetGlobalInfo`.
    pub fn can_read(&self, file_type: AudioFileTypeId) -> Result<bool> {
        let mut can_read = 0_u32;
        let status = unsafe {
            ffi::audio_file_component::at_audio_file_component_can_read(
                self.handle,
                file_type,
                &mut can_read,
            )
        };
        status_to_result("AudioFileComponentGetGlobalInfo(can read)", status)?;
        Ok(can_read != 0)
    }

    /// Wraps `AudioFileComponentGetGlobalInfo`.
    pub fn file_type_name(&self, file_type: AudioFileTypeId) -> Result<String> {
        let mut status = 0;
        let ptr = unsafe {
            ffi::audio_file_component::at_audio_file_component_copy_file_type_name(
                self.handle,
                file_type,
                &mut status,
            )
        };
        status_to_result("AudioFileComponentGetGlobalInfo(file type name)", status)?;
        string_from_owned_ptr("AudioFileComponentGetGlobalInfo(file type name)", ptr)
    }

    fn get_property_typed<T: Copy>(
        &self,
        property_id: AudioFilePropertyId,
        operation: &'static str,
    ) -> Result<T> {
        let mut value = MaybeUninit::<T>::uninit();
        let mut size = u32::try_from(std::mem::size_of::<T>()).expect("typed property fits in u32");
        let status = unsafe {
            ffi::audio_file_component::at_audio_file_component_get_property(
                self.handle,
                property_id,
                &mut size,
                value.as_mut_ptr().cast(),
            )
        };
        status_to_result(operation, status)?;
        Ok(unsafe { value.assume_init() })
    }

    fn from_handle(handle: *mut std::ffi::c_void, operation: &'static str) -> Result<Self> {
        if handle.is_null() {
            Err(AudioToolboxError::message(
                operation,
                "framework returned a null AudioFileComponent",
            ))
        } else {
            Ok(Self { handle })
        }
    }

    fn release(&mut self) {
        if !self.handle.is_null() {
            unsafe { ffi::audio_file_component::at_audio_file_component_release(self.handle) };
            self.handle = std::ptr::null_mut();
        }
    }
}

fn open_file_for_permissions(
    path: &Path,
    permissions: AudioFilePermissions,
    operation: &'static str,
) -> Result<std::fs::File> {
    let read = (permissions & AUDIO_FILE_READ_PERMISSION) != 0;
    let write = (permissions & crate::AUDIO_FILE_WRITE_PERMISSION) != 0;
    if !read && !write {
        return Err(AudioToolboxError::message(
            operation,
            "permissions must include read and/or write access",
        ));
    }

    let mut options = OpenOptions::new();
    options.read(read).write(write);
    options.open(path).map_err(|error| {
        AudioToolboxError::message(
            operation,
            format!("failed to open {}: {error}", path.display()),
        )
    })
}

impl Drop for AudioFileComponent {
    fn drop(&mut self) {
        self.release();
    }
}
