use crate::{AudioToolboxError, Result};
use std::{
    ffi::{c_char, c_void, CString},
    os::unix::ffi::OsStrExt,
    path::Path,
    ptr::NonNull,
};

type CFIndex = isize;
type CFStringEncoding = u32;
type Boolean = u8;

const CF_STRING_ENCODING_UTF8: CFStringEncoding = 0x0800_0100;

unsafe extern "C" {
    fn CFRetain(cf: *const c_void) -> *const c_void;
    fn CFRelease(cf: *const c_void);
    fn CFURLCreateFromFileSystemRepresentation(
        allocator: *const c_void,
        buffer: *const u8,
        buf_len: CFIndex,
        is_directory: Boolean,
    ) -> *const c_void;
    fn CFStringGetLength(the_string: *const c_void) -> CFIndex;
    fn CFStringGetMaximumSizeForEncoding(length: CFIndex, encoding: CFStringEncoding) -> CFIndex;
    fn CFStringGetCString(
        the_string: *const c_void,
        buffer: *mut c_char,
        buffer_size: CFIndex,
        encoding: CFStringEncoding,
    ) -> Boolean;
}

#[derive(Debug)]
/// Wraps `OwnedCFType`.
pub(crate) struct OwnedCFType(NonNull<c_void>);

impl OwnedCFType {
    /// Wraps `OwnedCFTypeAsPtr`.
    pub(crate) fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr().cast_const()
    }

    /// Wraps `OwnedCFTypeFromCreateRule`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `OwnedCFTypeFromCreateRule`.
    pub(crate) unsafe fn from_create_rule(raw: *const c_void) -> Option<Self> {
        NonNull::new(raw.cast_mut()).map(Self)
    }
}

impl Clone for OwnedCFType {
    fn clone(&self) -> Self {
        let retained = unsafe { CFRetain(self.as_ptr()) };
        unsafe { Self::from_create_rule(retained) }.expect("CFRetain returned null")
    }
}

impl Drop for OwnedCFType {
    fn drop(&mut self) {
        unsafe { CFRelease(self.as_ptr()) };
    }
}

/// Wraps `CFURLCreateFromFileSystemRepresentation`.
pub(crate) fn path_to_url(path: &Path) -> Result<OwnedCFType> {
    let bytes = path.as_os_str().as_bytes();
    let url = unsafe {
        CFURLCreateFromFileSystemRepresentation(
            std::ptr::null(),
            bytes.as_ptr(),
            bytes.len() as CFIndex,
            u8::from(path.is_dir()),
        )
    };
    unsafe { OwnedCFType::from_create_rule(url) }.ok_or_else(|| {
        AudioToolboxError::message(
            "CFURLCreateFromFileSystemRepresentation",
            format!("could not create CFURL for {}", path.display()),
        )
    })
}

/// Wraps `CFStringGetCString`.
pub(crate) fn cfstring_to_string(cf_string: *const c_void) -> Result<String> {
    if cf_string.is_null() {
        return Err(AudioToolboxError::message(
            "CFStringGetCString",
            "received null CFStringRef",
        ));
    }

    let length = unsafe { CFStringGetLength(cf_string) };
    let capacity =
        unsafe { CFStringGetMaximumSizeForEncoding(length, CF_STRING_ENCODING_UTF8) } + 1;
    let mut buffer = vec![0_u8; capacity as usize];

    let ok = unsafe {
        CFStringGetCString(
            cf_string,
            buffer.as_mut_ptr().cast::<c_char>(),
            capacity,
            CF_STRING_ENCODING_UTF8,
        )
    };
    if ok == 0 {
        return Err(AudioToolboxError::message(
            "CFStringGetCString",
            "CoreFoundation rejected UTF-8 conversion",
        ));
    }

    let c_string = CString::from_vec_with_nul(buffer).map_err(|_| {
        AudioToolboxError::message(
            "CFStringGetCString",
            "CoreFoundation returned a non-NUL-terminated UTF-8 buffer",
        )
    })?;

    c_string.into_string().map_err(|_| {
        AudioToolboxError::message(
            "CFStringGetCString",
            "CoreFoundation returned non-UTF-8 bytes",
        )
    })
}
