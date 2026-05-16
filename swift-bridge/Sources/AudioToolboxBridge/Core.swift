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
