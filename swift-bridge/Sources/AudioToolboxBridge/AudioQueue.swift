// swiftlint:disable function_parameter_count
import AudioToolbox
import AudioUnit
import CoreAudio
import CoreFoundation
import Foundation

private final class AudioQueueBox {
    var value: AudioQueueRef?

    init(_ value: AudioQueueRef) {
        self.value = value
    }

    deinit {
        if let value {
            AudioQueueDispose(value, true)
        }
    }
}

private final class AudioQueueBufferBox {
    let value: AudioQueueBufferRef

    init(_ value: AudioQueueBufferRef) {
        self.value = value
    }
}

private func audioQueue(from raw: UnsafeMutableRawPointer?) -> AudioQueueRef {
    castOpaque(raw, to: AudioQueueRef.self)
}

private func noopOutputCallback(
    _ inUserData: UnsafeMutableRawPointer?,
    _ inAQ: AudioQueueRef,
    _ inBuffer: AudioQueueBufferRef
) {
    _ = inUserData
    _ = inAQ
    _ = inBuffer
}

@_cdecl("at_audio_queue_new_output")
public func at_audio_queue_new_output(
    _ format: UnsafePointer<AudioStreamBasicDescription>?,
    _ outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Int32 {
    guard let format, let outHandle else {
        return Int32(kAudio_ParamError)
    }

    var queue: AudioQueueRef?
    let status = AudioQueueNewOutput(format, noopOutputCallback, nil, nil, nil, 0, &queue)
    if status == noErr, let queue {
        outHandle.pointee = retainObject(AudioQueueBox(queue))
    } else {
        outHandle.pointee = nil
    }
    return status
}

@_cdecl("at_audio_queue_raw")
public func at_audio_queue_raw(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let handle else {
        return nil
    }
    let box: AudioQueueBox = takeUnretained(handle)
    return toRawPointer(box.value)
}

@_cdecl("at_audio_queue_release")
public func at_audio_queue_release(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else {
        return
    }
    releaseObject(handle, as: AudioQueueBox.self)
}

@_cdecl("at_audio_queue_get_property")
public func at_audio_queue_get_property(
    _ rawQueue: UnsafeMutableRawPointer?,
    _ propertyID: UInt32,
    _ ioDataSize: UnsafeMutablePointer<UInt32>?,
    _ outPropertyData: UnsafeMutableRawPointer?
) -> Int32 {
    guard let ioDataSize, let outPropertyData else {
        return Int32(kAudio_ParamError)
    }
    return AudioQueueGetProperty(audioQueue(from: rawQueue), propertyID, outPropertyData, ioDataSize)
}

@_cdecl("at_audio_queue_set_property")
public func at_audio_queue_set_property(
    _ rawQueue: UnsafeMutableRawPointer?,
    _ propertyID: UInt32,
    _ dataSize: UInt32,
    _ propertyData: UnsafeRawPointer?
) -> Int32 {
    guard let propertyData else {
        return Int32(kAudio_ParamError)
    }
    return AudioQueueSetProperty(audioQueue(from: rawQueue), propertyID, propertyData, dataSize)
}

@_cdecl("at_audio_queue_get_parameter")
public func at_audio_queue_get_parameter(
    _ rawQueue: UnsafeMutableRawPointer?,
    _ parameterID: UInt32,
    _ outValue: UnsafeMutablePointer<Float>?
) -> Int32 {
    guard let outValue else {
        return Int32(kAudio_ParamError)
    }
    return AudioQueueGetParameter(audioQueue(from: rawQueue), parameterID, outValue)
}

@_cdecl("at_audio_queue_set_parameter")
public func at_audio_queue_set_parameter(
    _ rawQueue: UnsafeMutableRawPointer?,
    _ parameterID: UInt32,
    _ value: Float
) -> Int32 {
    AudioQueueSetParameter(audioQueue(from: rawQueue), parameterID, value)
}

@_cdecl("at_audio_queue_allocate_buffer")
public func at_audio_queue_allocate_buffer(
    _ rawQueue: UnsafeMutableRawPointer?,
    _ bufferByteSize: UInt32,
    _ outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Int32 {
    guard let outHandle else {
        return Int32(kAudio_ParamError)
    }

    var buffer: AudioQueueBufferRef?
    let status = AudioQueueAllocateBuffer(audioQueue(from: rawQueue), bufferByteSize, &buffer)
    if status == noErr, let buffer {
        outHandle.pointee = retainObject(AudioQueueBufferBox(buffer))
    } else {
        outHandle.pointee = nil
    }
    return status
}

@_cdecl("at_audio_queue_buffer_raw")
public func at_audio_queue_buffer_raw(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let handle else {
        return nil
    }
    let box: AudioQueueBufferBox = takeUnretained(handle)
    return toRawPointer(box.value)
}

@_cdecl("at_audio_queue_buffer_release")
public func at_audio_queue_buffer_release(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else {
        return
    }
    releaseObject(handle, as: AudioQueueBufferBox.self)
}

@_cdecl("at_audio_queue_start")
public func at_audio_queue_start(_ rawQueue: UnsafeMutableRawPointer?) -> Int32 {
    AudioQueueStart(audioQueue(from: rawQueue), nil)
}

@_cdecl("at_audio_queue_stop")
public func at_audio_queue_stop(_ rawQueue: UnsafeMutableRawPointer?, _ immediate: Bool) -> Int32 {
    AudioQueueStop(audioQueue(from: rawQueue), immediate)
}

@_cdecl("at_audio_queue_reset")
public func at_audio_queue_reset(_ rawQueue: UnsafeMutableRawPointer?) -> Int32 {
    AudioQueueReset(audioQueue(from: rawQueue))
}
