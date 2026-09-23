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
    var pendingData: [UInt8] = []
    var pendingDescriptions: [AudioStreamPacketDescription] = []
    var pendingPackets: UInt64 = 0

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
    let box = fileStreamBox(from: inClientData)
    box.packetCountSeen &+= UInt64(inNumberPackets)
    box.pendingPackets &+= UInt64(inNumberPackets)
    let base = Int64(box.pendingData.count)
    box.pendingData.append(
        contentsOf: UnsafeRawBufferPointer(start: inInputData, count: Int(inNumberBytes))
    )
    guard let inPacketDescriptions else {
        return
    }
    for index in 0 ..< Int(inNumberPackets) {
        var description = inPacketDescriptions[index]
        description.mStartOffset += base
        box.pendingDescriptions.append(description)
    }
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

@_cdecl("at_audio_file_stream_pending_sizes")
public func at_audio_file_stream_pending_sizes(
    _ handle: UnsafeMutableRawPointer?,
    _ outByteCount: UnsafeMutablePointer<UInt64>?,
    _ outDescriptionCount: UnsafeMutablePointer<UInt64>?,
    _ outPacketCount: UnsafeMutablePointer<UInt64>?
) {
    guard let handle else {
        return
    }
    let box = fileStreamBox(from: handle)
    outByteCount?.pointee = UInt64(box.pendingData.count)
    outDescriptionCount?.pointee = UInt64(box.pendingDescriptions.count)
    outPacketCount?.pointee = box.pendingPackets
}

@_cdecl("at_audio_file_stream_take_pending")
public func at_audio_file_stream_take_pending(
    _ handle: UnsafeMutableRawPointer?,
    _ data: UnsafeMutableRawPointer?,
    _ dataCapacity: UInt64,
    _ descriptions: UnsafeMutablePointer<AudioStreamPacketDescription>?,
    _ descriptionCapacity: UInt64
) -> Bool {
    guard let handle else {
        return false
    }
    let box = fileStreamBox(from: handle)
    guard UInt64(box.pendingData.count) <= dataCapacity,
          UInt64(box.pendingDescriptions.count) <= descriptionCapacity
    else {
        return false
    }
    if let data, !box.pendingData.isEmpty {
        box.pendingData.withUnsafeBytes { bytes in
            if let base = bytes.baseAddress {
                data.copyMemory(from: base, byteCount: bytes.count)
            }
        }
    }
    if let descriptions, !box.pendingDescriptions.isEmpty {
        box.pendingDescriptions.withUnsafeBufferPointer { source in
            if let base = source.baseAddress {
                descriptions.update(from: base, count: source.count)
            }
        }
    }
    box.pendingData.removeAll(keepingCapacity: true)
    box.pendingDescriptions.removeAll(keepingCapacity: true)
    box.pendingPackets = 0
    return true
}
