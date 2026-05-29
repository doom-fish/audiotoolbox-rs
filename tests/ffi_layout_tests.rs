//! Cross-language ABI check for the `#[repr(C)]` structs shared with the Swift
//! `AudioToolbox` bridge.
//!
//! The exhaustive compile-time size/alignment/offset assertions live in
//! `src/ffi_layout.rs`; if any of them drift the crate fails to compile. This
//! integration test exercises the runtime half: it asks the Swift bridge to
//! confirm that *its* `MemoryLayout` for the `CoreAudio` structs that cross the
//! FFI boundary matches the values pinned on the Rust side. A `false` return
//! means the Rust and Swift layouts genuinely disagree, which is a real ABI bug.

#[test]
fn ffi_layout_matches_swift() {
    assert!(
        audiotoolbox::verify_ffi_layout(),
        "Swift FFI struct layout disagrees with Rust layout (ABI mismatch)"
    );
}
