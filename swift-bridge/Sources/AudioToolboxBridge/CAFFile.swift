import AudioToolbox
import AudioUnit
import CoreAudio
import CoreFoundation
import Foundation

@_cdecl("at_caf_is_caf_file")
public func at_caf_is_caf_file(_ data: UnsafePointer<UInt8>?, _ dataLen: UInt32) -> UInt32 {
    guard let data, dataLen >= 4 else {
        return 0
    }
    let bytes = UnsafeBufferPointer(start: data, count: 4)
    return bytes[0] == 0x63 && bytes[1] == 0x61 && bytes[2] == 0x66 && bytes[3] == 0x66 ? 1 : 0
}
