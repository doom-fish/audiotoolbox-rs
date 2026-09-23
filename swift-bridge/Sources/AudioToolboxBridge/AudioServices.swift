import AudioToolbox
import AudioUnit
import CoreAudio
import CoreFoundation
import Foundation

private final class SystemSoundBox {
    let id: SystemSoundID

    init(_ id: SystemSoundID) {
        self.id = id
    }

    deinit {
        AudioServicesDisposeSystemSoundID(id)
    }
}

private func systemSound(from handle: UnsafeMutableRawPointer) -> SystemSoundID {
    let box: SystemSoundBox = takeUnretained(handle, as: SystemSoundBox.self)
    return box.id
}

private final class CompletionContext {
    let context: UnsafeMutableRawPointer
    let release: ContextRelease

    init(_ context: UnsafeMutableRawPointer, _ release: @escaping ContextRelease) {
        self.context = context
        self.release = release
    }

    deinit {
        release(context)
    }
}

public typealias SystemSoundCompletionProc = @convention(c) (UnsafeMutableRawPointer?) -> Void

@_cdecl("at_system_sound_play_with_completion")
public func at_system_sound_play_with_completion(
    _ handle: UnsafeMutableRawPointer?,
    _ alert: Bool,
    _ callback: SystemSoundCompletionProc?,
    _ context: UnsafeMutableRawPointer?,
    _ release: ContextRelease?
) -> Bool {
    guard let handle, let callback, let context, let release else {
        adoptContext(into: nil, context, release)
        return false
    }
    let holder = CompletionContext(context, release)
    let completion = {
        callback(holder.context)
    }
    let sound = systemSound(from: handle)
    if alert {
        AudioServicesPlayAlertSoundWithCompletion(sound, completion)
    } else {
        AudioServicesPlaySystemSoundWithCompletion(sound, completion)
    }
    return true
}

@_cdecl("at_system_sound_create")
public func at_system_sound_create(
    _ path: UnsafePointer<CChar>?,
    _ outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Int32 {
    guard let outHandle, let url = fileURL(from: path) else {
        return Int32(kAudio_ParamError)
    }

    var soundID: SystemSoundID = 0
    let status = AudioServicesCreateSystemSoundID(url as CFURL, &soundID)
    if status == noErr {
        outHandle.pointee = retainObject(SystemSoundBox(soundID))
    } else {
        outHandle.pointee = nil
    }
    return status
}

@_cdecl("at_system_sound_release")
public func at_system_sound_release(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else {
        return
    }
    releaseObject(handle, as: SystemSoundBox.self)
}

@_cdecl("at_system_sound_id")
public func at_system_sound_id(_ handle: UnsafeMutableRawPointer?) -> UInt32 {
    guard let handle else {
        return 0
    }
    return systemSound(from: handle)
}

@_cdecl("at_system_sound_play")
public func at_system_sound_play(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else {
        return
    }
    AudioServicesPlaySystemSound(systemSound(from: handle))
}

@_cdecl("at_system_sound_play_alert")
public func at_system_sound_play_alert(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else {
        return
    }
    AudioServicesPlayAlertSound(systemSound(from: handle))
}

private func getProperty(
    _ handle: UnsafeMutableRawPointer?,
    _ propertyID: AudioServicesPropertyID,
    _ outValue: UnsafeMutablePointer<UInt32>?
) -> Int32 {
    guard let handle, let outValue else {
        return Int32(kAudio_ParamError)
    }
    var soundID = systemSound(from: handle)
    var size = UInt32(MemoryLayout<UInt32>.size)
    return AudioServicesGetProperty(
        propertyID,
        UInt32(MemoryLayout<SystemSoundID>.size),
        &soundID,
        &size,
        outValue
    )
}

private func setProperty(
    _ handle: UnsafeMutableRawPointer?,
    _ propertyID: AudioServicesPropertyID,
    _ value: UInt32
) -> Int32 {
    guard let handle else {
        return Int32(kAudio_ParamError)
    }
    var soundID = systemSound(from: handle)
    var value = value
    return AudioServicesSetProperty(
        propertyID,
        UInt32(MemoryLayout<SystemSoundID>.size),
        &soundID,
        UInt32(MemoryLayout<UInt32>.size),
        &value
    )
}

@_cdecl("at_system_sound_get_is_ui_sound")
public func at_system_sound_get_is_ui_sound(
    _ handle: UnsafeMutableRawPointer?,
    _ outValue: UnsafeMutablePointer<UInt32>?
) -> Int32 {
    getProperty(handle, kAudioServicesPropertyIsUISound, outValue)
}

@_cdecl("at_system_sound_set_is_ui_sound")
public func at_system_sound_set_is_ui_sound(
    _ handle: UnsafeMutableRawPointer?,
    _ value: UInt32
) -> Int32 {
    setProperty(handle, kAudioServicesPropertyIsUISound, value)
}

@_cdecl("at_system_sound_get_complete_playback_if_app_dies")
public func at_system_sound_get_complete_playback_if_app_dies(
    _ handle: UnsafeMutableRawPointer?,
    _ outValue: UnsafeMutablePointer<UInt32>?
) -> Int32 {
    getProperty(handle, kAudioServicesPropertyCompletePlaybackIfAppDies, outValue)
}

@_cdecl("at_system_sound_set_complete_playback_if_app_dies")
public func at_system_sound_set_complete_playback_if_app_dies(
    _ handle: UnsafeMutableRawPointer?,
    _ value: UInt32
) -> Int32 {
    setProperty(handle, kAudioServicesPropertyCompletePlaybackIfAppDies, value)
}
