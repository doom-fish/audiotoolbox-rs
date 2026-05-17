use std::ffi::c_void;

unsafe extern "C" {
    pub fn at_free_string(ptr: *mut i8);
    pub fn CAShow(in_object: *mut c_void);
    pub fn CAShowFile(in_object: *mut c_void, in_file: *mut libc::FILE);
    #[link_name = "CFRelease"]
    pub fn at_cf_release(object: *const c_void);
    #[link_name = "CFDataCreate"]
    pub fn at_cf_data_create(
        allocator: *const c_void,
        bytes: *const u8,
        length: isize,
    ) -> *const c_void;
    #[link_name = "CFDataGetLength"]
    pub fn at_cf_data_get_length(data: *const c_void) -> isize;
    #[link_name = "CFDataGetBytePtr"]
    pub fn at_cf_data_get_byte_ptr(data: *const c_void) -> *const u8;
    #[link_name = "CFURLCreateFromFileSystemRepresentation"]
    pub fn at_cf_url_create_from_file_system_representation(
        allocator: *const c_void,
        buffer: *const u8,
        buf_len: isize,
        is_directory: bool,
    ) -> *const c_void;
}
