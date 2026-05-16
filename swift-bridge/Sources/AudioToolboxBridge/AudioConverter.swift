// swiftlint:disable function_parameter_count line_length
import AudioToolbox
import AudioUnit
import CoreAudio
import CoreFoundation
import Foundation

private final class AudioConverterBox {
    var value: AudioConverterRef?

    init(_ value: AudioConverterRef) {
        self.value = value
    }

    deinit {
        if let value {
            AudioConverterDispose(value)
        }
    }
}

private struct OneShotInputState {
    let data: UnsafePointer<UInt8>?
    let dataLen: UInt32
    let packetCount: UInt32
    let packetDescriptions: UnsafePointer<AudioStreamPacketDescription>?
    let channels: UInt32
    var provided: Bool
}

private func audioConverter(from raw: UnsafeMutableRawPointer?) -> AudioConverterRef {
    castOpaque(raw, to: AudioConverterRef.self)
}

private func oneShotInputProc(
    _ inAudioConverter: AudioConverterRef,
    _ ioNumberDataPackets: UnsafeMutablePointer<UInt32>,
    _ ioData: UnsafeMutablePointer<AudioBufferList>,
    _ outDataPacketDescription: UnsafeMutablePointer<UnsafeMutablePointer<AudioStreamPacketDescription>?>?,
    _ inUserData: UnsafeMutableRawPointer?
) -> OSStatus {
    guard let inUserData else {
        ioNumberDataPackets.pointee = 0
        return noErr
    }

    let state = inUserData.assumingMemoryBound(to: OneShotInputState.self)
    if state.pointee.provided || state.pointee.dataLen == 0 {
        ioNumberDataPackets.pointee = 0
        return noErr
    }

    state.pointee.provided = true
    ioNumberDataPackets.pointee = state.pointee.packetCount

    let buffers = UnsafeMutableAudioBufferListPointer(ioData)
    guard !buffers.isEmpty else {
        return Int32(kAudio_ParamError)
    }
    buffers[0].mNumberChannels = state.pointee.channels
    buffers[0].mDataByteSize = state.pointee.dataLen
    buffers[0].mData = UnsafeMutableRawPointer(mutating: state.pointee.data)

    if let outDataPacketDescription {
        outDataPacketDescription.pointee = UnsafeMutablePointer(mutating: state.pointee.packetDescriptions)
    }

    return noErr
}

@_cdecl("at_audio_converter_new")
public func at_audio_converter_new(
    _ sourceFormat: UnsafePointer<AudioStreamBasicDescription>?,
    _ destinationFormat: UnsafePointer<AudioStreamBasicDescription>?,
    _ outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Int32 {
    guard let outHandle, let sourceFormat, let destinationFormat else {
        return Int32(kAudio_ParamError)
    }

    var converter: AudioConverterRef?
    let status = AudioConverterNew(sourceFormat, destinationFormat, &converter)
    if status == noErr, let converter {
        outHandle.pointee = retainObject(AudioConverterBox(converter))
    } else {
        outHandle.pointee = nil
    }
    return status
}

@_cdecl("at_audio_converter_new_specific")
public func at_audio_converter_new_specific(
    _ sourceFormat: UnsafePointer<AudioStreamBasicDescription>?,
    _ destinationFormat: UnsafePointer<AudioStreamBasicDescription>?,
    _ classDescriptions: UnsafePointer<AudioClassDescription>?,
    _ classDescriptionCount: UInt32,
    _ outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Int32 {
    guard let outHandle, let sourceFormat, let destinationFormat else {
        return Int32(kAudio_ParamError)
    }

    var converter: AudioConverterRef?
    var emptyDescription = AudioClassDescription(mType: 0, mSubType: 0, mManufacturer: 0)
    let descriptionPointer = classDescriptions ?? withUnsafePointer(to: &emptyDescription) { $0 }
    let status = AudioConverterNewSpecific(
        sourceFormat,
        destinationFormat,
        classDescriptionCount,
        descriptionPointer,
        &converter
    )
    if status == noErr, let converter {
        outHandle.pointee = retainObject(AudioConverterBox(converter))
    } else {
        outHandle.pointee = nil
    }
    return status
}

@_cdecl("at_audio_converter_raw")
public func at_audio_converter_raw(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let handle else {
        return nil
    }
    let box: AudioConverterBox = takeUnretained(handle)
    return toRawPointer(box.value)
}

@_cdecl("at_audio_converter_release")
public func at_audio_converter_release(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else {
        return
    }
    releaseObject(handle, as: AudioConverterBox.self)
}

@_cdecl("at_audio_converter_reset")
public func at_audio_converter_reset(_ rawConverter: UnsafeMutableRawPointer?) -> Int32 {
    AudioConverterReset(audioConverter(from: rawConverter))
}

@_cdecl("at_audio_converter_get_property_info")
public func at_audio_converter_get_property_info(
    _ rawConverter: UnsafeMutableRawPointer?,
    _ propertyID: UInt32,
    _ outSize: UnsafeMutablePointer<UInt32>?,
    _ outWritable: UnsafeMutablePointer<DarwinBoolean>?
) -> Int32 {
    AudioConverterGetPropertyInfo(audioConverter(from: rawConverter), propertyID, outSize, outWritable)
}

@_cdecl("at_audio_converter_get_property")
public func at_audio_converter_get_property(
    _ rawConverter: UnsafeMutableRawPointer?,
    _ propertyID: UInt32,
    _ ioPropertyDataSize: UnsafeMutablePointer<UInt32>?,
    _ outPropertyData: UnsafeMutableRawPointer?
) -> Int32 {
    guard let ioPropertyDataSize, let outPropertyData else {
        return Int32(kAudio_ParamError)
    }
    return AudioConverterGetProperty(audioConverter(from: rawConverter), propertyID, ioPropertyDataSize, outPropertyData)
}

@_cdecl("at_audio_converter_set_property")
public func at_audio_converter_set_property(
    _ rawConverter: UnsafeMutableRawPointer?,
    _ propertyID: UInt32,
    _ propertyDataSize: UInt32,
    _ propertyData: UnsafeRawPointer?
) -> Int32 {
    guard let propertyData else {
        return Int32(kAudio_ParamError)
    }
    return AudioConverterSetProperty(audioConverter(from: rawConverter), propertyID, propertyDataSize, propertyData)
}

@_cdecl("at_audio_converter_fill_complex_buffer_once")
public func at_audio_converter_fill_complex_buffer_once(
    _ rawConverter: UnsafeMutableRawPointer?,
    _ inputData: UnsafePointer<UInt8>?,
    _ inputLen: UInt32,
    _ packetCount: UInt32,
    _ packetDescriptions: UnsafePointer<AudioStreamPacketDescription>?,
    _ channels: UInt32,
    _ ioOutputPacketSize: UnsafeMutablePointer<UInt32>?,
    _ outOutputData: UnsafeMutablePointer<AudioBufferList>?,
    _ outPacketDescriptions: UnsafeMutablePointer<AudioStreamPacketDescription>?
) -> Int32 {
    guard let ioOutputPacketSize, let outOutputData else {
        return Int32(kAudio_ParamError)
    }

    var state = OneShotInputState(
        data: inputData,
        dataLen: inputLen,
        packetCount: packetCount,
        packetDescriptions: packetDescriptions,
        channels: channels,
        provided: false
    )

    return withUnsafeMutablePointer(to: &state) { statePtr in
        AudioConverterFillComplexBuffer(
            audioConverter(from: rawConverter),
            oneShotInputProc,
            statePtr,
            ioOutputPacketSize,
            outOutputData,
            outPacketDescriptions
        )
    }
}
