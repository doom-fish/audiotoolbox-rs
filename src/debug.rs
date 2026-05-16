use crate::{
    ffi, AudioComponent, AudioComponentInstance, AudioConverter, AudioFile, AudioToolboxError,
    BorrowedAudioConverter, ExtAudioFile, Result,
};
use std::{
    ffi::{c_void, CString},
    io::Error,
};

pub trait AudioToolboxDebugObject {
    fn debug_ptr(&self) -> *mut c_void;
}

impl AudioToolboxDebugObject for AudioFile {
    fn debug_ptr(&self) -> *mut c_void {
        self.as_raw().cast()
    }
}

impl AudioToolboxDebugObject for ExtAudioFile {
    fn debug_ptr(&self) -> *mut c_void {
        self.as_raw().cast()
    }
}

impl AudioToolboxDebugObject for AudioConverter {
    fn debug_ptr(&self) -> *mut c_void {
        self.as_raw().cast()
    }
}

impl AudioToolboxDebugObject for BorrowedAudioConverter<'_> {
    fn debug_ptr(&self) -> *mut c_void {
        self.as_raw().cast()
    }
}

impl AudioToolboxDebugObject for AudioComponent {
    fn debug_ptr(&self) -> *mut c_void {
        self.as_raw().cast()
    }
}

impl AudioToolboxDebugObject for AudioComponentInstance {
    fn debug_ptr(&self) -> *mut c_void {
        self.as_raw().cast()
    }
}

pub fn ca_show(object: &impl AudioToolboxDebugObject) {
    unsafe { ffi::CAShow(object.debug_ptr()) };
}

pub fn ca_show_to_stdout(object: &impl AudioToolboxDebugObject) -> Result<()> {
    ca_show_to_fd(object.debug_ptr(), libc::STDOUT_FILENO, "ca_show_to_stdout")
}

pub fn ca_show_to_stderr(object: &impl AudioToolboxDebugObject) -> Result<()> {
    ca_show_to_fd(object.debug_ptr(), libc::STDERR_FILENO, "ca_show_to_stderr")
}

pub fn flush_debug_output() -> Result<()> {
    let rc = unsafe { libc::fflush(std::ptr::null_mut()) };
    if rc == 0 {
        Ok(())
    } else {
        Err(AudioToolboxError::message(
            "flush_debug_output",
            format!("fflush failed: {}", Error::last_os_error()),
        ))
    }
}

fn ca_show_to_fd(object: *mut c_void, fd: i32, operation: &'static str) -> Result<()> {
    let mode = CString::new("w").expect("CString::new on constant string cannot fail");
    let duplicated_fd = unsafe { libc::dup(fd) };
    if duplicated_fd < 0 {
        return Err(AudioToolboxError::message(
            operation,
            format!("dup failed: {}", Error::last_os_error()),
        ));
    }

    let file = unsafe { libc::fdopen(duplicated_fd, mode.as_ptr()) };
    if file.is_null() {
        let _ = unsafe { libc::close(duplicated_fd) };
        return Err(AudioToolboxError::message(
            operation,
            format!("fdopen failed: {}", Error::last_os_error()),
        ));
    }

    unsafe { ffi::CAShowFile(object, file) };
    let flush_status = unsafe { libc::fflush(file) };
    let close_status = unsafe { libc::fclose(file) };

    if flush_status != 0 {
        return Err(AudioToolboxError::message(
            operation,
            format!("fflush failed: {}", Error::last_os_error()),
        ));
    }
    if close_status != 0 {
        return Err(AudioToolboxError::message(
            operation,
            format!("fclose failed: {}", Error::last_os_error()),
        ));
    }

    Ok(())
}
