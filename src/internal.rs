use crate::{ffi, AudioToolboxError, Result, OSStatus, NO_ERR};
use std::{
    ffi::{CStr, CString},
    path::Path,
};

pub fn status_to_result(operation: &'static str, status: OSStatus) -> Result<()> {
    if status == NO_ERR {
        Ok(())
    } else {
        Err(AudioToolboxError::from_status(operation, status))
    }
}

pub fn path_to_cstring(path: &Path) -> Result<CString> {
    CString::new(path.to_string_lossy().as_bytes()).map_err(|_| {
        AudioToolboxError::message(
            "path_to_cstring",
            format!("path contains interior NULs: {}", path.display()),
        )
    })
}

pub fn string_from_owned_ptr(operation: &'static str, ptr: *mut i8) -> Result<String> {
    if ptr.is_null() {
        return Err(AudioToolboxError::message(operation, "framework returned a null string"));
    }

    let bytes = unsafe { CStr::from_ptr(ptr) }.to_bytes().to_vec();
    unsafe { ffi::core::at_free_string(ptr) };
    String::from_utf8(bytes)
        .map_err(|_| AudioToolboxError::message(operation, "framework returned non-UTF-8 bytes"))
}

