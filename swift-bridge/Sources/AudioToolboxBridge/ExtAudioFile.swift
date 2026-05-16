import AudioToolbox
import AudioUnit
import CoreAudio
import CoreFoundation
import Foundation

private final class ExtAudioFileBox {
    var value: ExtAudioFileRef?

    init(_ value: ExtAudioFileRef) {
        self.value = value
    }

    deinit {
        if let value {
            ExtAudioFileDispose(value)
        }
    }
}

private final class BorrowedAudioConverterBox {
    let value: AudioConverterRef

    init(_ value: AudioConverterRef) {
        self.value = value
    }
}

private func extAudioFile(from raw: UnsafeMutableRawPointer?) -> ExtAudioFileRef {
    castOpaque(raw, to: ExtAudioFileRef.self)
}

@_cdecl("at_ext_audio_file_open")
public func at_ext_audio_file_open(
    _ path: UnsafePointer<CChar>?,
    _ outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Int32 {
    guard let outHandle, let url = fileURL(from: path) else {
        return Int32(kAudio_ParamError)
    }

    var extAudioFile: ExtAudioFileRef?
    let status = ExtAudioFileOpenURL(url as CFURL, &extAudioFile)
    if status == noErr, let extAudioFile {
        outHandle.pointee = retainObject(ExtAudioFileBox(extAudioFile))
    } else {
        outHandle.pointee = nil
    }
    return status
}

@_cdecl("at_ext_audio_file_create")
public func at_ext_audio_file_create(
    _ path: UnsafePointer<CChar>?,
    _ fileType: UInt32,
    _ format: UnsafePointer<AudioStreamBasicDescription>?,
    _ flags: UInt32,
    _ outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Int32 {
    guard let outHandle, let format, let url = fileURL(from: path) else {
        return Int32(kAudio_ParamError)
    }

    var extAudioFile: ExtAudioFileRef?
    let status = ExtAudioFileCreateWithURL(
        url as CFURL,
        fileType,
        format,
        nil,
        flags,
        &extAudioFile
    )
    if status == noErr, let extAudioFile {
        outHandle.pointee = retainObject(ExtAudioFileBox(extAudioFile))
    } else {
        outHandle.pointee = nil
    }
    return status
}

@_cdecl("at_ext_audio_file_raw")
public func at_ext_audio_file_raw(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let handle else {
        return nil
    }
    let box: ExtAudioFileBox = takeUnretained(handle)
    return toRawPointer(box.value)
}

@_cdecl("at_ext_audio_file_release")
public func at_ext_audio_file_release(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else {
        return
    }
    releaseObject(handle, as: ExtAudioFileBox.self)
}

@_cdecl("at_ext_audio_file_get_property")
public func at_ext_audio_file_get_property(
    _ rawFile: UnsafeMutableRawPointer?,
    _ propertyID: UInt32,
    _ ioPropertyDataSize: UnsafeMutablePointer<UInt32>?,
    _ outPropertyData: UnsafeMutableRawPointer?
) -> Int32 {
    guard let ioPropertyDataSize, let outPropertyData else {
        return Int32(kAudio_ParamError)
    }
    return ExtAudioFileGetProperty(extAudioFile(from: rawFile), propertyID, ioPropertyDataSize, outPropertyData)
}

@_cdecl("at_ext_audio_file_set_property")
public func at_ext_audio_file_set_property(
    _ rawFile: UnsafeMutableRawPointer?,
    _ propertyID: UInt32,
    _ propertyDataSize: UInt32,
    _ propertyData: UnsafeRawPointer?
) -> Int32 {
    guard let propertyData else {
        return Int32(kAudio_ParamError)
    }
    return ExtAudioFileSetProperty(extAudioFile(from: rawFile), propertyID, propertyDataSize, propertyData)
}

@_cdecl("at_ext_audio_file_read")
public func at_ext_audio_file_read(
    _ rawFile: UnsafeMutableRawPointer?,
    _ ioNumberFrames: UnsafeMutablePointer<UInt32>?,
    _ ioData: UnsafeMutablePointer<AudioBufferList>?
) -> Int32 {
    guard let ioNumberFrames, let ioData else {
        return Int32(kAudio_ParamError)
    }
    return ExtAudioFileRead(extAudioFile(from: rawFile), ioNumberFrames, ioData)
}

@_cdecl("at_ext_audio_file_write")
public func at_ext_audio_file_write(
    _ rawFile: UnsafeMutableRawPointer?,
    _ numberFrames: UInt32,
    _ ioData: UnsafePointer<AudioBufferList>?
) -> Int32 {
    guard let ioData else {
        return Int32(kAudio_ParamError)
    }
    return ExtAudioFileWrite(extAudioFile(from: rawFile), numberFrames, ioData)
}

@_cdecl("at_ext_audio_file_copy_audio_converter")
public func at_ext_audio_file_copy_audio_converter(
    _ rawFile: UnsafeMutableRawPointer?,
    _ outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Int32 {
    guard let outHandle else {
        return Int32(kAudio_ParamError)
    }

    var converter: AudioConverterRef?
    var size = UInt32(MemoryLayout<AudioConverterRef?>.size)
    let status = ExtAudioFileGetProperty(
        extAudioFile(from: rawFile),
        kExtAudioFileProperty_AudioConverter,
        &size,
        &converter
    )
    if status == noErr, let converter {
        outHandle.pointee = retainObject(BorrowedAudioConverterBox(converter))
    } else {
        outHandle.pointee = nil
    }
    return status
}

@_cdecl("at_borrowed_audio_converter_raw")
public func at_borrowed_audio_converter_raw(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let handle else {
        return nil
    }
    let box: BorrowedAudioConverterBox = takeUnretained(handle)
    return toRawPointer(box.value)
}

@_cdecl("at_borrowed_audio_converter_release")
public func at_borrowed_audio_converter_release(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else {
        return
    }
    releaseObject(handle, as: BorrowedAudioConverterBox.self)
}
