// swiftlint:disable function_parameter_count
import AudioToolbox
import AudioUnit
import CoreAudio
import CoreFoundation
import Foundation

private final class AudioFileStreamBox {
    var value: AudioFileStreamID?
    var readyToProducePackets = false
    var packetCountSeen: UInt64 = 0

    deinit {
        if let value {
            AudioFileStreamClose(value)
        }
    }
}

private func audioFileStream(from raw: UnsafeMutableRawPointer?) -> AudioFileStreamID {
    castOpaque(raw, to: AudioFileStreamID.self)
}

private func fileStreamBox(from handle: UnsafeMutableRawPointer) -> AudioFileStreamBox {
    takeUnretained(handle, as: AudioFileStreamBox.self)
}

private func propertyListenerProc(
    _ inClientData: UnsafeMutableRawPointer,
    _ inAudioFileStream: AudioFileStreamID,
    _ inPropertyID: AudioFileStreamPropertyID,
    _ ioFlags: UnsafeMutablePointer<AudioFileStreamPropertyFlags>
) {
    _ = inAudioFileStream
    _ = ioFlags
    let box = fileStreamBox(from: inClientData)
    if inPropertyID == kAudioFileStreamProperty_ReadyToProducePackets {
        box.readyToProducePackets = true
    }
}

private func packetsProc(
    _ inClientData: UnsafeMutableRawPointer,
    _ inNumberBytes: UInt32,
    _ inNumberPackets: UInt32,
    _ inInputData: UnsafeRawPointer,
    _ inPacketDescriptions: UnsafeMutablePointer<AudioStreamPacketDescription>?
) {
    _ = inNumberBytes
    _ = inInputData
    _ = inPacketDescriptions
    let box = fileStreamBox(from: inClientData)
    box.packetCountSeen += UInt64(inNumberPackets)
}

@_cdecl("at_audio_file_stream_open")
public func at_audio_file_stream_open(
    _ fileTypeHint: UInt32,
    _ outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Int32 {
    guard let outHandle else {
        return Int32(kAudio_ParamError)
    }

    let box = AudioFileStreamBox()
    let handle = retainObject(box)
    var stream: AudioFileStreamID?
    let status = AudioFileStreamOpen(handle, propertyListenerProc, packetsProc, fileTypeHint, &stream)
    if status == noErr, let stream {
        box.value = stream
        outHandle.pointee = handle
    } else {
        outHandle.pointee = nil
        releaseObject(handle, as: AudioFileStreamBox.self)
    }
    return status
}

@_cdecl("at_audio_file_stream_raw")
public func at_audio_file_stream_raw(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let handle else {
        return nil
    }
    let box = fileStreamBox(from: handle)
    return toRawPointer(box.value)
}

@_cdecl("at_audio_file_stream_release")
public func at_audio_file_stream_release(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else {
        return
    }
    releaseObject(handle, as: AudioFileStreamBox.self)
}

@_cdecl("at_audio_file_stream_parse_bytes")
public func at_audio_file_stream_parse_bytes(
    _ rawStream: UnsafeMutableRawPointer?,
    _ data: UnsafeRawPointer?,
    _ dataByteSize: UInt32,
    _ parseFlags: UInt32
) -> Int32 {
    guard let data else {
        return Int32(kAudio_ParamError)
    }
    return AudioFileStreamParseBytes(
        audioFileStream(from: rawStream),
        dataByteSize,
        data,
        AudioFileStreamParseFlags(rawValue: parseFlags)
    )
}

@_cdecl("at_audio_file_stream_get_property_info")
public func at_audio_file_stream_get_property_info(
    _ rawStream: UnsafeMutableRawPointer?,
    _ propertyID: UInt32,
    _ outPropertyDataSize: UnsafeMutablePointer<UInt32>?,
    _ outWritable: UnsafeMutablePointer<DarwinBoolean>?
) -> Int32 {
    AudioFileStreamGetPropertyInfo(audioFileStream(from: rawStream), propertyID, outPropertyDataSize, outWritable)
}

@_cdecl("at_audio_file_stream_get_property")
public func at_audio_file_stream_get_property(
    _ rawStream: UnsafeMutableRawPointer?,
    _ propertyID: UInt32,
    _ ioPropertyDataSize: UnsafeMutablePointer<UInt32>?,
    _ outPropertyData: UnsafeMutableRawPointer?
) -> Int32 {
    guard let ioPropertyDataSize, let outPropertyData else {
        return Int32(kAudio_ParamError)
    }
    return AudioFileStreamGetProperty(audioFileStream(from: rawStream), propertyID, ioPropertyDataSize, outPropertyData)
}

@_cdecl("at_audio_file_stream_ready_to_produce_packets")
public func at_audio_file_stream_ready_to_produce_packets(_ handle: UnsafeMutableRawPointer?) -> UInt32 {
    guard let handle else {
        return 0
    }
    return fileStreamBox(from: handle).readyToProducePackets ? 1 : 0
}

@_cdecl("at_audio_file_stream_packet_count_seen")
public func at_audio_file_stream_packet_count_seen(_ handle: UnsafeMutableRawPointer?) -> UInt64 {
    guard let handle else {
        return 0
    }
    return fileStreamBox(from: handle).packetCountSeen
}
