// swiftlint:disable function_parameter_count
import AudioToolbox
import AudioUnit
import CoreAudio
import CoreFoundation
import Foundation

private final class AudioFileBox {
    var value: AudioFileID?

    init(_ value: AudioFileID) {
        self.value = value
    }

    deinit {
        if let value {
            AudioFileClose(value)
        }
    }
}

private func audioFile(from raw: UnsafeMutableRawPointer?) -> AudioFileID {
    castOpaque(raw, to: AudioFileID.self)
}

@_cdecl("at_audio_file_open")
public func at_audio_file_open(
    _ path: UnsafePointer<CChar>?,
    _ permissions: Int8,
    _ fileTypeHint: UInt32,
    _ outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Int32 {
    guard let outHandle, let url = fileURL(from: path) else {
        return Int32(kAudio_ParamError)
    }

    var audioFile: AudioFileID?
    let status = AudioFileOpenURL(
        url as CFURL,
        AudioFilePermissions(rawValue: permissions)!,
        fileTypeHint,
        &audioFile
    )
    if status == noErr, let audioFile {
        outHandle.pointee = retainObject(AudioFileBox(audioFile))
    } else {
        outHandle.pointee = nil
    }
    return status
}

@_cdecl("at_audio_file_create")
public func at_audio_file_create(
    _ path: UnsafePointer<CChar>?,
    _ fileType: UInt32,
    _ format: UnsafePointer<AudioStreamBasicDescription>?,
    _ flags: UInt32,
    _ outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Int32 {
    guard let outHandle, let format, let url = fileURL(from: path) else {
        return Int32(kAudio_ParamError)
    }

    var audioFile: AudioFileID?
    let status = AudioFileCreateWithURL(
        url as CFURL,
        fileType,
        format,
        AudioFileFlags(rawValue: flags),
        &audioFile
    )
    if status == noErr, let audioFile {
        outHandle.pointee = retainObject(AudioFileBox(audioFile))
    } else {
        outHandle.pointee = nil
    }
    return status
}

@_cdecl("at_audio_file_raw")
public func at_audio_file_raw(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let handle else {
        return nil
    }
    let box: AudioFileBox = takeUnretained(handle)
    return toRawPointer(box.value)
}

@_cdecl("at_audio_file_release")
public func at_audio_file_release(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else {
        return
    }
    releaseObject(handle, as: AudioFileBox.self)
}

@_cdecl("at_audio_file_get_property_info")
public func at_audio_file_get_property_info(
    _ rawFile: UnsafeMutableRawPointer?,
    _ propertyID: UInt32,
    _ outDataSize: UnsafeMutablePointer<UInt32>?,
    _ outWritable: UnsafeMutablePointer<UInt32>?
) -> Int32 {
    AudioFileGetPropertyInfo(audioFile(from: rawFile), propertyID, outDataSize, outWritable)
}

@_cdecl("at_audio_file_get_property")
public func at_audio_file_get_property(
    _ rawFile: UnsafeMutableRawPointer?,
    _ propertyID: UInt32,
    _ ioDataSize: UnsafeMutablePointer<UInt32>?,
    _ outPropertyData: UnsafeMutableRawPointer?
) -> Int32 {
    guard let ioDataSize, let outPropertyData else {
        return Int32(kAudio_ParamError)
    }
    return AudioFileGetProperty(audioFile(from: rawFile), propertyID, ioDataSize, outPropertyData)
}

@_cdecl("at_audio_file_set_property")
public func at_audio_file_set_property(
    _ rawFile: UnsafeMutableRawPointer?,
    _ propertyID: UInt32,
    _ dataSize: UInt32,
    _ propertyData: UnsafeRawPointer?
) -> Int32 {
    guard let propertyData else {
        return Int32(kAudio_ParamError)
    }
    return AudioFileSetProperty(audioFile(from: rawFile), propertyID, dataSize, propertyData)
}

@_cdecl("at_audio_file_read_packet_data")
public func at_audio_file_read_packet_data(
    _ rawFile: UnsafeMutableRawPointer?,
    _ useCache: Bool,
    _ ioNumBytes: UnsafeMutablePointer<UInt32>?,
    _ outPacketDescriptions: UnsafeMutablePointer<AudioStreamPacketDescription>?,
    _ startingPacket: Int64,
    _ ioNumPackets: UnsafeMutablePointer<UInt32>?,
    _ outBuffer: UnsafeMutableRawPointer?
) -> Int32 {
    guard let ioNumBytes, let ioNumPackets, let outBuffer else {
        return Int32(kAudio_ParamError)
    }
    return AudioFileReadPacketData(
        audioFile(from: rawFile),
        useCache,
        ioNumBytes,
        outPacketDescriptions,
        startingPacket,
        ioNumPackets,
        outBuffer
    )
}

@_cdecl("at_audio_file_read_packets")
public func at_audio_file_read_packets(
    _ rawFile: UnsafeMutableRawPointer?,
    _ useCache: Bool,
    _ outNumBytes: UnsafeMutablePointer<UInt32>?,
    _ outPacketDescriptions: UnsafeMutablePointer<AudioStreamPacketDescription>?,
    _ startingPacket: Int64,
    _ ioNumPackets: UnsafeMutablePointer<UInt32>?,
    _ outBuffer: UnsafeMutableRawPointer?
) -> Int32 {
    guard let outNumBytes, let ioNumPackets, let outBuffer else {
        return Int32(kAudio_ParamError)
    }
    return AudioFileReadPackets(
        audioFile(from: rawFile),
        useCache,
        outNumBytes,
        outPacketDescriptions,
        startingPacket,
        ioNumPackets,
        outBuffer
    )
}

@_cdecl("at_audio_file_write_packets")
public func at_audio_file_write_packets(
    _ rawFile: UnsafeMutableRawPointer?,
    _ useCache: Bool,
    _ numBytes: UInt32,
    _ packetDescriptions: UnsafePointer<AudioStreamPacketDescription>?,
    _ startingPacket: Int64,
    _ ioNumPackets: UnsafeMutablePointer<UInt32>?,
    _ buffer: UnsafeRawPointer?
) -> Int32 {
    guard let ioNumPackets, let buffer else {
        return Int32(kAudio_ParamError)
    }
    return AudioFileWritePackets(
        audioFile(from: rawFile),
        useCache,
        numBytes,
        packetDescriptions,
        startingPacket,
        ioNumPackets,
        buffer
    )
}
