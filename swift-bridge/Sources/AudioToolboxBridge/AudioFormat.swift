import AudioToolbox
import AudioUnit
import CoreAudio
import CoreFoundation
import Foundation

@_cdecl("at_audio_format_get_property_info")
public func at_audio_format_get_property_info(
    _ propertyID: UInt32,
    _ specifierSize: UInt32,
    _ specifier: UnsafeRawPointer?,
    _ outSize: UnsafeMutablePointer<UInt32>?
) -> Int32 {
    guard let outSize else {
        return Int32(kAudio_ParamError)
    }
    return AudioFormatGetPropertyInfo(propertyID, specifierSize, specifier, outSize)
}

@_cdecl("at_audio_format_get_property")
public func at_audio_format_get_property(
    _ propertyID: UInt32,
    _ specifierSize: UInt32,
    _ specifier: UnsafeRawPointer?,
    _ ioSize: UnsafeMutablePointer<UInt32>?,
    _ outData: UnsafeMutableRawPointer?
) -> Int32 {
    AudioFormatGetProperty(propertyID, specifierSize, specifier, ioSize, outData)
}
