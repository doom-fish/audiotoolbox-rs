use crate::{ffi, AudioToolboxError, CFDataRef, CFURLRef, OSStatus, Result, NO_ERR};
use std::{
    ffi::{CStr, CString},
    path::Path,
};

/// Converts an AudioToolbox.framework `OSStatus` into `Result<()>`.
pub fn status_to_result(operation: &'static str, status: OSStatus) -> Result<()> {
    if status == NO_ERR {
        Ok(())
    } else {
        Err(AudioToolboxError::from_status(operation, status))
    }
}

/// Converts a filesystem path for AudioToolbox.framework path-based APIs.
pub fn path_to_cstring(path: &Path) -> Result<CString> {
    CString::new(path.to_string_lossy().as_bytes()).map_err(|_| {
        AudioToolboxError::message(
            "path_to_cstring",
            format!("path contains interior NULs: {}", path.display()),
        )
    })
}

/// Converts an owned C string returned by AudioToolbox.framework into `String`.
pub fn string_from_owned_ptr(operation: &'static str, ptr: *mut i8) -> Result<String> {
    if ptr.is_null() {
        return Err(AudioToolboxError::message(
            operation,
            "framework returned a null string",
        ));
    }

    let bytes = unsafe { CStr::from_ptr(ptr) }.to_bytes().to_vec();
    unsafe { ffi::core::at_free_string(ptr) };
    String::from_utf8(bytes)
        .map_err(|_| AudioToolboxError::message(operation, "framework returned non-UTF-8 bytes"))
}

/// Builds an `AudioToolboxError` from an owned C string returned by AudioToolbox.framework.
pub fn error_from_owned_ptr(operation: &'static str, ptr: *mut i8) -> AudioToolboxError {
    if ptr.is_null() {
        AudioToolboxError::message(operation, "framework returned an unknown error")
    } else {
        string_from_owned_ptr(operation, ptr).map_or_else(
            |error| error,
            |message| AudioToolboxError::message(operation, message),
        )
    }
}

/// Creates a `CFDataRef` for AudioToolbox.framework APIs from a Rust byte slice.
pub fn cf_data_from_bytes(operation: &'static str, bytes: &[u8]) -> Result<CFDataRef> {
    let length = isize::try_from(bytes.len())
        .map_err(|_| AudioToolboxError::message(operation, "payload exceeds isize::MAX bytes"))?;
    let data = unsafe { ffi::core::at_cf_data_create(std::ptr::null(), bytes.as_ptr(), length) };
    if data.is_null() {
        Err(AudioToolboxError::message(
            operation,
            "framework returned a null CFDataRef",
        ))
    } else {
        Ok(data)
    }
}

/// Copies bytes out of a `CFDataRef` returned by AudioToolbox.framework.
pub fn cf_data_to_vec(operation: &'static str, data: CFDataRef) -> Result<Vec<u8>> {
    if data.is_null() {
        return Err(AudioToolboxError::message(
            operation,
            "framework returned a null CFDataRef",
        ));
    }
    let length = unsafe { ffi::core::at_cf_data_get_length(data) };
    let length = usize::try_from(length).map_err(|_| {
        AudioToolboxError::message(operation, "framework returned an invalid CFData length")
    })?;
    let bytes = unsafe { ffi::core::at_cf_data_get_byte_ptr(data) };
    if bytes.is_null() && length != 0 {
        return Err(AudioToolboxError::message(
            operation,
            "framework returned a null CFData byte pointer",
        ));
    }
    let vec = unsafe { std::slice::from_raw_parts(bytes, length) }.to_vec();
    unsafe { ffi::core::at_cf_release(data.cast()) };
    Ok(vec)
}

/// Creates a `CFURLRef` for AudioToolbox.framework file APIs from a filesystem path.
pub fn cf_url_from_path(operation: &'static str, path: &Path) -> Result<CFURLRef> {
    let path = path_to_cstring(path)?;
    let url = unsafe {
        ffi::core::at_cf_url_create_from_file_system_representation(
            std::ptr::null(),
            path.as_ptr().cast(),
            isize::try_from(path.as_bytes().len()).map_err(|_| {
                AudioToolboxError::message(operation, "path exceeds isize::MAX bytes")
            })?,
            path.to_bytes().ends_with(b"/"),
        )
    };
    if url.is_null() {
        Err(AudioToolboxError::message(
            operation,
            "framework returned a null CFURLRef",
        ))
    } else {
        Ok(url)
    }
}

/// Wraps `CFRelease` for Core Foundation objects used by AudioToolbox.framework.
pub fn cf_release(object: *const std::ffi::c_void) {
    if !object.is_null() {
        unsafe { ffi::core::at_cf_release(object) };
    }
}
