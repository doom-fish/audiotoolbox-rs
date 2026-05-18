unsafe extern "C" {
    /// Raw binding for `CAFIsCAFFile`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `CAFIsCAFFile`.
    pub fn at_caf_is_caf_file(data: *const u8, data_len: u32) -> u32;
}
