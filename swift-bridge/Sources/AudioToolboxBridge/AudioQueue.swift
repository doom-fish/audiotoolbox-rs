// swiftlint:disable function_parameter_count
import AudioToolbox
import AudioUnit
import CoreAudio
import CoreFoundation
import Foundation

private final class AudioQueueBox {
    let value: AudioQueueRef
    let contexts = AdoptedContexts()

    init(_ value: AudioQueueRef) {
        self.value = value
    }

    deinit {
        AudioQueueDispose(value, true)
        contexts.releaseAll()
    }
}

public typealias AudioQueueOutputProc = @convention(c) (
    UnsafeMutableRawPointer?,
    AudioQueueRef,
    AudioQueueBufferRef
) -> Void

public typealias AudioQueueInputProc = @convention(c) (
    UnsafeMutableRawPointer?,
    AudioQueueRef,
    AudioQueueBufferRef,
    UnsafePointer<AudioTimeStamp>,
    UInt32,
    UnsafePointer<AudioStreamPacketDescription>?
) -> Void

private func finishQueueCreation(
    _ status: OSStatus,
    _ queue: AudioQueueRef?,
    _ context: UnsafeMutableRawPointer?,
    _ release: ContextRelease?,
    _ outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>
) -> Int32 {
    if status == noErr, let queue {
        let box = AudioQueueBox(queue)
        adoptContext(into: box.contexts, context, release)
        outHandle.pointee = retainObject(box)
    } else {
        outHandle.pointee = nil
        adoptContext(into: nil, context, release)
    }
    return status
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

@_cdecl("at_audio_queue_new_output_with_callback")
public func at_audio_queue_new_output_with_callback(
    _ format: UnsafePointer<AudioStreamBasicDescription>?,
    _ callback: AudioQueueOutputProc?,
    _ context: UnsafeMutableRawPointer?,
    _ release: ContextRelease?,
    _ outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Int32 {
    guard let format, let callback, let outHandle else {
        adoptContext(into: nil, context, release)
        return Int32(kAudio_ParamError)
    }
    var queue: AudioQueueRef?
    let status = AudioQueueNewOutput(format, callback, context, nil, nil, 0, &queue)
    return finishQueueCreation(status, queue, context, release, outHandle)
}

@_cdecl("at_audio_queue_new_input_with_callback")
public func at_audio_queue_new_input_with_callback(
    _ format: UnsafePointer<AudioStreamBasicDescription>?,
    _ callback: AudioQueueInputProc?,
    _ context: UnsafeMutableRawPointer?,
    _ release: ContextRelease?,
    _ outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Int32 {
    guard let format, let callback, let outHandle else {
        adoptContext(into: nil, context, release)
        return Int32(kAudio_ParamError)
    }
    var queue: AudioQueueRef?
    let status = AudioQueueNewInput(format, callback, context, nil, nil, 0, &queue)
    return finishQueueCreation(status, queue, context, release, outHandle)
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
