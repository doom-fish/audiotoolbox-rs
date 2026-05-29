import AudioToolbox
import AudioUnit
import CoreAudio
import CoreFoundation
import Foundation

func retainObject(_ object: some AnyObject) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(object).toOpaque()
}

func takeUnretained<T: AnyObject>(_ ptr: UnsafeMutableRawPointer, as _: T.Type = T.self) -> T {
    let typed = ptr.assumingMemoryBound(to: T.self)
    return Unmanaged<T>.fromOpaque(UnsafeRawPointer(typed)).takeUnretainedValue()
}

func releaseObject<T: AnyObject>(_ ptr: UnsafeMutableRawPointer, as _: T.Type = T.self) {
    let typed = ptr.assumingMemoryBound(to: T.self)
    Unmanaged<T>.fromOpaque(UnsafeRawPointer(typed)).release()
}

func fileURL(from path: UnsafePointer<CChar>?) -> URL? {
    guard let path else {
        return nil
    }
    return URL(fileURLWithPath: String(cString: path))
}

func cStringCopy(_ string: String) -> UnsafeMutablePointer<CChar>? {
    strdup(string)
}

func copyOptionalString(_ string: String?) -> UnsafeMutablePointer<CChar>? {
    guard let string else {
        return nil
    }
    return cStringCopy(string)
}

func copyErrorDescription(_ error: Error?) -> UnsafeMutablePointer<CChar>? {
    guard let error else {
        return nil
    }
    return cStringCopy((error as NSError).localizedDescription)
}

func castOpaque<T>(_ raw: UnsafeMutableRawPointer?, to _: T.Type = T.self) -> T {
    unsafeBitCast(raw, to: T.self)
}

func toRawPointer<T>(_ value: T) -> UnsafeMutableRawPointer? {
    unsafeBitCast(value, to: UnsafeMutableRawPointer?.self)
}

@_cdecl("at_free_string")
public func at_free_string(_ ptr: UnsafeMutablePointer<CChar>?) {
    free(ptr)
}

// MARK: - FFI Layout Verification

/// Cross-language ABI check called from Rust's `tests/ffi_layout_tests.rs`.
///
/// Returns `true` only if the Swift `MemoryLayout` (size, stride and alignment)
/// of the CoreAudio structs that cross the FFI boundary matches the values
/// pinned on the Rust side via the `const _: () = assert!(...)` checks in
/// `src/ffi_layout.rs`. A `false` return flags a genuine Rust <-> Swift ABI
/// mismatch and fails the Rust test.
@_cdecl("at_verify_ffi_layout")
public func at_verify_ffi_layout() -> Bool {
    MemoryLayout<AudioBuffer>.size == 16
        && MemoryLayout<AudioBuffer>.stride == 16
        && MemoryLayout<AudioBuffer>.alignment == 8
        && MemoryLayout<AudioBufferList>.size == 24
        && MemoryLayout<AudioBufferList>.stride == 24
        && MemoryLayout<AudioBufferList>.alignment == 8
        && MemoryLayout<AudioStreamPacketDescription>.size == 16
        && MemoryLayout<AudioStreamPacketDescription>.stride == 16
        && MemoryLayout<AudioStreamPacketDescription>.alignment == 8
        && MemoryLayout<SMPTETime>.size == 24
        && MemoryLayout<SMPTETime>.stride == 24
        && MemoryLayout<SMPTETime>.alignment == 4
        && MemoryLayout<AudioTimeStamp>.size == 64
        && MemoryLayout<AudioTimeStamp>.stride == 64
        && MemoryLayout<AudioTimeStamp>.alignment == 8
        && MemoryLayout<AudioClassDescription>.size == 12
        && MemoryLayout<AudioClassDescription>.stride == 12
        && MemoryLayout<AudioClassDescription>.alignment == 4
}
