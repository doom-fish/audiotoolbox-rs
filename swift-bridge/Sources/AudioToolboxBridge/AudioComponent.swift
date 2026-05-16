import AudioToolbox
import AudioUnit
import CoreAudio
import CoreFoundation
import Foundation

private final class AudioComponentBox {
    let value: AudioComponent

    init(_ value: AudioComponent) {
        self.value = value
    }
}

private final class AudioComponentInstanceBox {
    var value: AudioComponentInstance

    init(_ value: AudioComponentInstance) {
        self.value = value
    }

    deinit {
        AudioComponentInstanceDispose(value)
    }
}

private func audioComponent(from raw: UnsafeMutableRawPointer?) -> AudioComponent {
    castOpaque(raw, to: AudioComponent.self)
}

private func audioComponentInstance(from raw: UnsafeMutableRawPointer?) -> AudioComponentInstance {
    castOpaque(raw, to: AudioComponentInstance.self)
}

@_cdecl("at_audio_component_count")
public func at_audio_component_count(_ description: UnsafePointer<AudioComponentDescription>?) -> UInt32 {
    guard let description else {
        return 0
    }
    return AudioComponentCount(description)
}

@_cdecl("at_audio_component_find_next")
public func at_audio_component_find_next(
    _ previousRaw: UnsafeMutableRawPointer?,
    _ description: UnsafePointer<AudioComponentDescription>?
) -> UnsafeMutableRawPointer? {
    guard let description else {
        return nil
    }
    let previous = previousRaw.map { audioComponent(from: $0) }
    guard let component = AudioComponentFindNext(previous, description) else {
        return nil
    }
    return retainObject(AudioComponentBox(component))
}

@_cdecl("at_audio_component_raw")
public func at_audio_component_raw(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let handle else {
        return nil
    }
    let box: AudioComponentBox = takeUnretained(handle)
    return toRawPointer(box.value)
}

@_cdecl("at_audio_component_release")
public func at_audio_component_release(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else {
        return
    }
    releaseObject(handle, as: AudioComponentBox.self)
}

@_cdecl("at_audio_component_copy_name")
public func at_audio_component_copy_name(
    _ rawComponent: UnsafeMutableRawPointer?,
    _ outStatus: UnsafeMutablePointer<Int32>?
) -> UnsafeMutablePointer<CChar>? {
    guard let outStatus else {
        return nil
    }

    var cfName: Unmanaged<CFString>?
    let status = AudioComponentCopyName(audioComponent(from: rawComponent), &cfName)
    outStatus.pointee = status
    guard status == noErr, let cfName else {
        return nil
    }
    return cStringCopy(cfName.takeRetainedValue() as String)
}

@_cdecl("at_audio_component_get_description")
public func at_audio_component_get_description(
    _ rawComponent: UnsafeMutableRawPointer?,
    _ outDescription: UnsafeMutablePointer<AudioComponentDescription>?
) -> Int32 {
    guard let outDescription else {
        return Int32(kAudio_ParamError)
    }
    return AudioComponentGetDescription(audioComponent(from: rawComponent), outDescription)
}

@_cdecl("at_audio_component_get_version")
public func at_audio_component_get_version(
    _ rawComponent: UnsafeMutableRawPointer?,
    _ outVersion: UnsafeMutablePointer<UInt32>?
) -> Int32 {
    guard let outVersion else {
        return Int32(kAudio_ParamError)
    }
    return AudioComponentGetVersion(audioComponent(from: rawComponent), outVersion)
}

@_cdecl("at_audio_component_instance_new")
public func at_audio_component_instance_new(
    _ rawComponent: UnsafeMutableRawPointer?,
    _ outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Int32 {
    guard let outHandle else {
        return Int32(kAudio_ParamError)
    }

    var instance: AudioComponentInstance?
    let status = AudioComponentInstanceNew(audioComponent(from: rawComponent), &instance)
    if status == noErr, let instance {
        outHandle.pointee = retainObject(AudioComponentInstanceBox(instance))
    } else {
        outHandle.pointee = nil
    }
    return status
}

@_cdecl("at_audio_component_instance_raw")
public func at_audio_component_instance_raw(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let handle else {
        return nil
    }
    let box: AudioComponentInstanceBox = takeUnretained(handle)
    return toRawPointer(box.value)
}

@_cdecl("at_audio_component_instance_release")
public func at_audio_component_instance_release(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else {
        return
    }
    releaseObject(handle, as: AudioComponentInstanceBox.self)
}

@_cdecl("at_audio_component_instance_get_component")
public func at_audio_component_instance_get_component(
    _ rawInstance: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    let component = AudioComponentInstanceGetComponent(audioComponentInstance(from: rawInstance))
    return retainObject(AudioComponentBox(component))
}
