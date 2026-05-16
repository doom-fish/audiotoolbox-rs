import AudioToolbox
import AudioUnit
import CoreAudio
import CoreFoundation
import Darwin
import Foundation

private final class AudioFileComponentBox {
    var value: AudioFileComponent?

    init(_ value: AudioFileComponent) {
        self.value = value
    }

    deinit {
        if let value {
            _ = AudioFileComponentCloseFile(value)
            AudioComponentInstanceDispose(value)
        }
    }
}

private func audioFileComponent(from handle: UnsafeMutableRawPointer) -> AudioFileComponent {
    let box: AudioFileComponentBox = takeUnretained(handle)
    return box.value!
}

@_cdecl("at_audio_file_component_new_default")
public func at_audio_file_component_new_default(
    _ outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Int32 {
    guard let outHandle else {
        return Int32(kAudio_ParamError)
    }

    var description = AudioComponentDescription(
        componentType: 0x6166696C,
        componentSubType: 0,
        componentManufacturer: 0,
        componentFlags: 0,
        componentFlagsMask: 0
    )
    guard let component = AudioComponentFindNext(nil, &description) else {
        outHandle.pointee = nil
        return Int32(kAudio_ParamError)
    }

    var instance: AudioFileComponent?
    let status = AudioComponentInstanceNew(component, &instance)
    if status == noErr, let instance {
        outHandle.pointee = retainObject(AudioFileComponentBox(instance))
    } else {
        outHandle.pointee = nil
    }
    return status
}

@_cdecl("at_audio_file_component_release")
public func at_audio_file_component_release(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else {
        return
    }
    releaseObject(handle, as: AudioFileComponentBox.self)
}

@_cdecl("at_audio_file_component_open")
public func at_audio_file_component_open(
    _ handle: UnsafeMutableRawPointer?,
    _ path: UnsafePointer<CChar>?,
    _ permissions: Int8,
    _ fileDescriptor: Int32
) -> Int32 {
    guard let handle, let url = fileURL(from: path) else {
        return Int32(kAudio_ParamError)
    }
    return AudioFileComponentOpenURL(
        audioFileComponent(from: handle),
        url as CFURL,
        permissions,
        fileDescriptor
    )
}

@_cdecl("at_audio_file_component_close_file")
public func at_audio_file_component_close_file(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let handle else {
        return Int32(kAudio_ParamError)
    }
    return AudioFileComponentCloseFile(audioFileComponent(from: handle))
}

@_cdecl("at_audio_file_component_get_property_info")
public func at_audio_file_component_get_property_info(
    _ handle: UnsafeMutableRawPointer?,
    _ propertyID: UInt32,
    _ outDataSize: UnsafeMutablePointer<UInt32>?,
    _ outWritable: UnsafeMutablePointer<UInt32>?
) -> Int32 {
    guard let handle, let outDataSize, let outWritable else {
        return Int32(kAudio_ParamError)
    }
    return AudioFileComponentGetPropertyInfo(
        audioFileComponent(from: handle),
        propertyID,
        outDataSize,
        outWritable
    )
}

@_cdecl("at_audio_file_component_get_property")
public func at_audio_file_component_get_property(
    _ handle: UnsafeMutableRawPointer?,
    _ propertyID: UInt32,
    _ ioDataSize: UnsafeMutablePointer<UInt32>?,
    _ outPropertyData: UnsafeMutableRawPointer?
) -> Int32 {
    guard let handle, let ioDataSize, let outPropertyData else {
        return Int32(kAudio_ParamError)
    }
    return AudioFileComponentGetProperty(
        audioFileComponent(from: handle),
        propertyID,
        ioDataSize,
        outPropertyData
    )
}

@_cdecl("at_audio_file_component_can_read")
public func at_audio_file_component_can_read(
    _ handle: UnsafeMutableRawPointer?,
    _ fileType: UInt32,
    _ outCanRead: UnsafeMutablePointer<UInt32>?
) -> Int32 {
    guard let handle, let outCanRead else {
        return Int32(kAudio_ParamError)
    }

    var typeSpecifier = fileType
    var propertySize = UInt32(MemoryLayout<UInt32>.size)
    return withUnsafePointer(to: &typeSpecifier) { specifier in
        AudioFileComponentGetGlobalInfo(
            audioFileComponent(from: handle),
            kAudioFileComponent_CanRead,
            UInt32(MemoryLayout<UInt32>.size),
            specifier,
            &propertySize,
            outCanRead
        )
    }
}

@_cdecl("at_audio_file_component_copy_file_type_name")
public func at_audio_file_component_copy_file_type_name(
    _ handle: UnsafeMutableRawPointer?,
    _ fileType: UInt32,
    _ outStatus: UnsafeMutablePointer<Int32>?
) -> UnsafeMutablePointer<CChar>? {
    guard let handle, let outStatus else {
        return nil
    }

    var typeSpecifier = fileType
    var propertySize = UInt32(MemoryLayout<CFString?>.size)
    var name: CFString?
    let status = withUnsafePointer(to: &typeSpecifier) { specifier in
        withUnsafeMutablePointer(to: &name) { outName in
            AudioFileComponentGetGlobalInfo(
                audioFileComponent(from: handle),
                kAudioFileComponent_FileTypeName,
                UInt32(MemoryLayout<UInt32>.size),
                specifier,
                &propertySize,
                outName
            )
        }
    }
    outStatus.pointee = status
    guard status == noErr, let name else {
        return nil
    }
    return cStringCopy(name as String)
}
