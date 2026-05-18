use crate::{CFDataRef, CFURLRef};
use apple_cf::raw::CFAllocatorRef;
use std::ffi::c_void;

unsafe extern "C" {
    /// Raw binding for `FreeString`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `FreeString`.
    pub fn at_free_string(ptr: *mut i8);
    /// Raw binding for `CoreAudioCashow`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `CoreAudioCashow`.
    pub fn CAShow(in_object: *mut c_void);
    /// Raw binding for `CoreAudioCashowfile`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `CoreAudioCashowfile`.
    pub fn CAShowFile(in_object: *mut c_void, in_file: *mut libc::FILE);
    #[link_name = "CFRelease"]
    /// Raw binding for `CFRelease`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `CFRelease`.
    pub fn at_cf_release(object: *const c_void);
    #[link_name = "CFDataCreate"]
    /// Raw binding for `CFDataCreate`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `CFDataCreate`.
    pub fn at_cf_data_create(
        allocator: CFAllocatorRef,
        bytes: *const u8,
        length: isize,
    ) -> CFDataRef;
    #[link_name = "CFDataGetLength"]
    /// Raw binding for `CFDataGetLength`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `CFDataGetLength`.
    pub fn at_cf_data_get_length(data: CFDataRef) -> isize;
    #[link_name = "CFDataGetBytePtr"]
    /// Raw binding for `CFDataGetBytePtr`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `CFDataGetBytePtr`.
    pub fn at_cf_data_get_byte_ptr(data: CFDataRef) -> *const u8;
    #[link_name = "CFURLCreateFromFileSystemRepresentation"]
    /// Raw binding for `CFURLCreateFromFileSystemRepresentation`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `CFURLCreateFromFileSystemRepresentation`.
    pub fn at_cf_url_create_from_file_system_representation(
        allocator: CFAllocatorRef,
        buffer: *const u8,
        buf_len: isize,
        is_directory: bool,
    ) -> CFURLRef;
}
