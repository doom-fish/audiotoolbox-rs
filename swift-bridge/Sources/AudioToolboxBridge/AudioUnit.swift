// swiftlint:disable function_parameter_count
import AudioToolbox
import AudioUnit
import CoreAudio
import CoreFoundation
import Foundation

private final class AudioUnitBox {
    var value: AudioUnit?

    init(_ value: AudioUnit) {
        self.value = value
    }

    deinit {
        if let value {
            AudioComponentInstanceDispose(value)
        }
    }
}

private func audioUnit(from raw: UnsafeMutableRawPointer?) -> AudioUnit {
    castOpaque(raw, to: AudioUnit.self)
}

@_cdecl("at_audio_unit_new")
public func at_audio_unit_new(
    _ componentType: UInt32,
    _ componentSubType: UInt32,
    _ componentManufacturer: UInt32,
    _ outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Int32 {
    guard let outHandle else {
        return Int32(kAudio_ParamError)
    }

    let description = AudioComponentDescription(
        componentType: componentType,
        componentSubType: componentSubType,
        componentManufacturer: componentManufacturer,
        componentFlags: 0,
        componentFlagsMask: 0
    )

    return withUnsafePointer(to: description) { descriptionPtr in
        guard let component = AudioComponentFindNext(nil, descriptionPtr) else {
            outHandle.pointee = nil
            return Int32(kAudio_ParamError)
        }

        var unit: AudioUnit?
        let status = AudioComponentInstanceNew(component, &unit)
        if status == noErr, let unit {
            outHandle.pointee = retainObject(AudioUnitBox(unit))
        } else {
            outHandle.pointee = nil
        }
        return status
    }
}

@_cdecl("at_audio_unit_raw")
public func at_audio_unit_raw(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let handle else {
        return nil
    }
    let box: AudioUnitBox = takeUnretained(handle)
    return toRawPointer(box.value)
}

@_cdecl("at_audio_unit_release")
public func at_audio_unit_release(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else {
        return
    }
    releaseObject(handle, as: AudioUnitBox.self)
}

@_cdecl("at_audio_unit_initialize")
public func at_audio_unit_initialize(_ rawUnit: UnsafeMutableRawPointer?) -> Int32 {
    AudioUnitInitialize(audioUnit(from: rawUnit))
}

@_cdecl("at_audio_unit_uninitialize")
public func at_audio_unit_uninitialize(_ rawUnit: UnsafeMutableRawPointer?) -> Int32 {
    AudioUnitUninitialize(audioUnit(from: rawUnit))
}

@_cdecl("at_audio_output_unit_start")
public func at_audio_output_unit_start(_ rawUnit: UnsafeMutableRawPointer?) -> Int32 {
    AudioOutputUnitStart(audioUnit(from: rawUnit))
}

@_cdecl("at_audio_output_unit_stop")
public func at_audio_output_unit_stop(_ rawUnit: UnsafeMutableRawPointer?) -> Int32 {
    AudioOutputUnitStop(audioUnit(from: rawUnit))
}

@_cdecl("at_audio_unit_get_property")
public func at_audio_unit_get_property(
    _ rawUnit: UnsafeMutableRawPointer?,
    _ propertyID: UInt32,
    _ scope: UInt32,
    _ element: UInt32,
    _ ioDataSize: UnsafeMutablePointer<UInt32>?,
    _ outPropertyData: UnsafeMutableRawPointer?
) -> Int32 {
    guard let ioDataSize, let outPropertyData else {
        return Int32(kAudio_ParamError)
    }
    return AudioUnitGetProperty(audioUnit(from: rawUnit), propertyID, scope, element, outPropertyData, ioDataSize)
}

@_cdecl("at_audio_unit_set_property")
public func at_audio_unit_set_property(
    _ rawUnit: UnsafeMutableRawPointer?,
    _ propertyID: UInt32,
    _ scope: UInt32,
    _ element: UInt32,
    _ dataSize: UInt32,
    _ propertyData: UnsafeRawPointer?
) -> Int32 {
    guard let propertyData else {
        return Int32(kAudio_ParamError)
    }
    return AudioUnitSetProperty(audioUnit(from: rawUnit), propertyID, scope, element, propertyData, dataSize)
}

@_cdecl("at_audio_unit_get_parameter")
public func at_audio_unit_get_parameter(
    _ rawUnit: UnsafeMutableRawPointer?,
    _ parameterID: UInt32,
    _ scope: UInt32,
    _ element: UInt32,
    _ outValue: UnsafeMutablePointer<Float>?
) -> Int32 {
    guard let outValue else {
        return Int32(kAudio_ParamError)
    }
    return AudioUnitGetParameter(audioUnit(from: rawUnit), parameterID, scope, element, outValue)
}

@_cdecl("at_audio_unit_set_parameter")
public func at_audio_unit_set_parameter(
    _ rawUnit: UnsafeMutableRawPointer?,
    _ parameterID: UInt32,
    _ scope: UInt32,
    _ element: UInt32,
    _ value: Float,
    _ bufferOffsetInFrames: UInt32
) -> Int32 {
    AudioUnitSetParameter(audioUnit(from: rawUnit), parameterID, scope, element, value, bufferOffsetInFrames)
}
