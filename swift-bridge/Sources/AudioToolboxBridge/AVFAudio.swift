import AVFAudio
import AudioToolbox
import AudioUnit
import CoreAudio
import CoreFoundation
import Foundation

private final class AVAudioEngineBox {
    let value: AVAudioEngine

    init(_ value: AVAudioEngine) {
        self.value = value
    }
}

private final class AVAudioNodeBox {
    let value: AVAudioNode

    init(_ value: AVAudioNode) {
        self.value = value
    }
}

private final class AVAudioFormatBox {
    let value: AVAudioFormat

    init(_ value: AVAudioFormat) {
        self.value = value
    }
}

private func avAudioEngineBox(from handle: UnsafeMutableRawPointer) -> AVAudioEngineBox {
    takeUnretained(handle)
}

private func avAudioNodeBox(from handle: UnsafeMutableRawPointer) -> AVAudioNodeBox {
    takeUnretained(handle)
}

private func avAudioFormatBox(from handle: UnsafeMutableRawPointer) -> AVAudioFormatBox {
    takeUnretained(handle)
}

@_cdecl("at_av_audio_engine_new")
public func at_av_audio_engine_new(
    _ outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Bool {
    guard let outHandle else {
        return false
    }
    outHandle.pointee = retainObject(AVAudioEngineBox(AVAudioEngine()))
    return true
}

@_cdecl("at_av_audio_engine_release")
public func at_av_audio_engine_release(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else {
        return
    }
    releaseObject(handle, as: AVAudioEngineBox.self)
}

@_cdecl("at_av_audio_engine_prepare")
public func at_av_audio_engine_prepare(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else {
        return
    }
    avAudioEngineBox(from: handle).value.prepare()
}

@_cdecl("at_av_audio_engine_start")
public func at_av_audio_engine_start(
    _ handle: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
    guard let handle else {
        outError?.pointee = cStringCopy("AVAudioEngine handle was null")
        return false
    }
    do {
        try avAudioEngineBox(from: handle).value.start()
        outError?.pointee = nil
        return true
    } catch {
        outError?.pointee = copyErrorDescription(error)
        return false
    }
}

@_cdecl("at_av_audio_engine_stop")
public func at_av_audio_engine_stop(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else {
        return
    }
    avAudioEngineBox(from: handle).value.stop()
}

@_cdecl("at_av_audio_engine_reset")
public func at_av_audio_engine_reset(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else {
        return
    }
    avAudioEngineBox(from: handle).value.reset()
}

@_cdecl("at_av_audio_engine_is_running")
public func at_av_audio_engine_is_running(_ handle: UnsafeMutableRawPointer?) -> Bool {
    guard let handle else {
        return false
    }
    return avAudioEngineBox(from: handle).value.isRunning
}

@_cdecl("at_av_audio_engine_output_node")
public func at_av_audio_engine_output_node(
    _ handle: UnsafeMutableRawPointer?,
    _ outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Bool {
    guard let handle, let outHandle else {
        return false
    }
    outHandle.pointee = retainObject(AVAudioNodeBox(avAudioEngineBox(from: handle).value.outputNode))
    return true
}

@_cdecl("at_av_audio_engine_main_mixer_node")
public func at_av_audio_engine_main_mixer_node(
    _ handle: UnsafeMutableRawPointer?,
    _ outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Bool {
    guard let handle, let outHandle else {
        return false
    }
    outHandle.pointee = retainObject(AVAudioNodeBox(avAudioEngineBox(from: handle).value.mainMixerNode))
    return true
}

@_cdecl("at_av_audio_node_release")
public func at_av_audio_node_release(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else {
        return
    }
    releaseObject(handle, as: AVAudioNodeBox.self)
}

@_cdecl("at_av_audio_node_number_of_inputs")
public func at_av_audio_node_number_of_inputs(_ handle: UnsafeMutableRawPointer?) -> UInt64 {
    guard let handle else {
        return 0
    }
    return UInt64(avAudioNodeBox(from: handle).value.numberOfInputs)
}

@_cdecl("at_av_audio_node_number_of_outputs")
public func at_av_audio_node_number_of_outputs(_ handle: UnsafeMutableRawPointer?) -> UInt64 {
    guard let handle else {
        return 0
    }
    return UInt64(avAudioNodeBox(from: handle).value.numberOfOutputs)
}

@_cdecl("at_av_audio_node_input_format")
public func at_av_audio_node_input_format(
    _ handle: UnsafeMutableRawPointer?,
    _ bus: UInt64,
    _ outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Bool {
    guard let handle, let outHandle else {
        return false
    }
    let format = avAudioNodeBox(from: handle).value.inputFormat(forBus: AVAudioNodeBus(bus))
    outHandle.pointee = retainObject(AVAudioFormatBox(format))
    return true
}

@_cdecl("at_av_audio_node_output_format")
public func at_av_audio_node_output_format(
    _ handle: UnsafeMutableRawPointer?,
    _ bus: UInt64,
    _ outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Bool {
    guard let handle, let outHandle else {
        return false
    }
    let format = avAudioNodeBox(from: handle).value.outputFormat(forBus: AVAudioNodeBus(bus))
    outHandle.pointee = retainObject(AVAudioFormatBox(format))
    return true
}

@_cdecl("at_av_audio_node_reset")
public func at_av_audio_node_reset(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else {
        return
    }
    avAudioNodeBox(from: handle).value.reset()
}

@_cdecl("at_av_audio_node_latency")
public func at_av_audio_node_latency(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard let handle else {
        return 0
    }
    return avAudioNodeBox(from: handle).value.latency
}

@_cdecl("at_av_audio_node_output_presentation_latency")
public func at_av_audio_node_output_presentation_latency(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard let handle else {
        return 0
    }
    return avAudioNodeBox(from: handle).value.outputPresentationLatency
}

@_cdecl("at_av_audio_format_new_standard")
public func at_av_audio_format_new_standard(
    _ sampleRate: Double,
    _ channels: UInt32,
    _ outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Bool {
    guard let outHandle,
          let format = AVAudioFormat(standardFormatWithSampleRate: sampleRate, channels: channels)
    else {
        return false
    }
    outHandle.pointee = retainObject(AVAudioFormatBox(format))
    return true
}

@_cdecl("at_av_audio_format_new_common")
public func at_av_audio_format_new_common(
    _ commonFormat: UInt64,
    _ sampleRate: Double,
    _ channels: UInt32,
    _ interleaved: Bool,
    _ outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Bool {
    guard let outHandle,
          let commonFormat = AVAudioCommonFormat(rawValue: UInt(commonFormat)),
          let format = AVAudioFormat(
              commonFormat: commonFormat,
              sampleRate: sampleRate,
              channels: channels,
              interleaved: interleaved
          )
    else {
        return false
    }
    outHandle.pointee = retainObject(AVAudioFormatBox(format))
    return true
}

@_cdecl("at_av_audio_format_new_stream_description")
public func at_av_audio_format_new_stream_description(
    _ description: UnsafePointer<AudioStreamBasicDescription>?,
    _ outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Bool {
    guard let description, let outHandle,
          let format = AVAudioFormat(streamDescription: description)
    else {
        return false
    }
    outHandle.pointee = retainObject(AVAudioFormatBox(format))
    return true
}

@_cdecl("at_av_audio_format_release")
public func at_av_audio_format_release(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else {
        return
    }
    releaseObject(handle, as: AVAudioFormatBox.self)
}

@_cdecl("at_av_audio_format_sample_rate")
public func at_av_audio_format_sample_rate(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard let handle else {
        return 0
    }
    return avAudioFormatBox(from: handle).value.sampleRate
}

@_cdecl("at_av_audio_format_channel_count")
public func at_av_audio_format_channel_count(_ handle: UnsafeMutableRawPointer?) -> UInt32 {
    guard let handle else {
        return 0
    }
    return avAudioFormatBox(from: handle).value.channelCount
}

@_cdecl("at_av_audio_format_common_format")
public func at_av_audio_format_common_format(_ handle: UnsafeMutableRawPointer?) -> UInt64 {
    guard let handle else {
        return 0
    }
    return UInt64(avAudioFormatBox(from: handle).value.commonFormat.rawValue)
}

@_cdecl("at_av_audio_format_is_standard")
public func at_av_audio_format_is_standard(_ handle: UnsafeMutableRawPointer?) -> Bool {
    guard let handle else {
        return false
    }
    return avAudioFormatBox(from: handle).value.isStandard
}

@_cdecl("at_av_audio_format_is_interleaved")
public func at_av_audio_format_is_interleaved(_ handle: UnsafeMutableRawPointer?) -> Bool {
    guard let handle else {
        return false
    }
    return avAudioFormatBox(from: handle).value.isInterleaved
}

@_cdecl("at_av_audio_format_copy_stream_description")
public func at_av_audio_format_copy_stream_description(
    _ handle: UnsafeMutableRawPointer?,
    _ outDescription: UnsafeMutablePointer<AudioStreamBasicDescription>?
) -> Bool {
    guard let handle, let outDescription else {
        return false
    }
    outDescription.pointee = avAudioFormatBox(from: handle).value.streamDescription.pointee
    return true
}
