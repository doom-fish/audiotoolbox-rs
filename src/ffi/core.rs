use std::ffi::c_void;

unsafe extern "C" {
    pub fn at_free_string(ptr: *mut i8);
    pub fn CAShow(in_object: *mut c_void);
    pub fn CAShowFile(in_object: *mut c_void, in_file: *mut libc::FILE);
}
