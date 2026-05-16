import AVFAudio
import AudioToolbox
import AudioUnit
import CoreAudio
import CoreFoundation
import Foundation

private final class AUAudioUnitBox {
    let value: AUAudioUnit

    init(_ value: AUAudioUnit) {
        self.value = value
    }
}

private func auAudioUnitBox(from handle: UnsafeMutableRawPointer) -> AUAudioUnitBox {
    takeUnretained(handle)
}

@_cdecl("at_au_audio_unit_new")
public func at_au_audio_unit_new(
    _ description: UnsafePointer<AudioComponentDescription>?,
    _ options: UInt32,
    _ outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
    guard let description, let outHandle else {
        outError?.pointee = cStringCopy("AUAudioUnit parameters were null")
        return false
    }
    do {
        let unit = try AUAudioUnit(
            componentDescription: description.pointee,
            options: AudioComponentInstantiationOptions(rawValue: options)
        )
        outHandle.pointee = retainObject(AUAudioUnitBox(unit))
        outError?.pointee = nil
        return true
    } catch {
        outHandle.pointee = nil
        outError?.pointee = copyErrorDescription(error)
        return false
    }
}

@_cdecl("at_au_audio_unit_release")
public func at_au_audio_unit_release(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else {
        return
    }
    releaseObject(handle, as: AUAudioUnitBox.self)
}

@_cdecl("at_au_audio_unit_component_description")
public func at_au_audio_unit_component_description(
    _ handle: UnsafeMutableRawPointer?,
    _ outDescription: UnsafeMutablePointer<AudioComponentDescription>?
) -> Bool {
    guard let handle, let outDescription else {
        return false
    }
    outDescription.pointee = auAudioUnitBox(from: handle).value.componentDescription
    return true
}

@_cdecl("at_au_audio_unit_copy_component_name")
public func at_au_audio_unit_copy_component_name(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let handle else {
        return nil
    }
    return copyOptionalString(auAudioUnitBox(from: handle).value.componentName)
}

@_cdecl("at_au_audio_unit_copy_audio_unit_name")
public func at_au_audio_unit_copy_audio_unit_name(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let handle else {
        return nil
    }
    return copyOptionalString(auAudioUnitBox(from: handle).value.audioUnitName)
}

@_cdecl("at_au_audio_unit_copy_manufacturer_name")
public func at_au_audio_unit_copy_manufacturer_name(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let handle else {
        return nil
    }
    return copyOptionalString(auAudioUnitBox(from: handle).value.manufacturerName)
}

@_cdecl("at_au_audio_unit_input_bus_count")
public func at_au_audio_unit_input_bus_count(_ handle: UnsafeMutableRawPointer?) -> UInt64 {
    guard let handle else {
        return 0
    }
    return UInt64(auAudioUnitBox(from: handle).value.inputBusses.count)
}

@_cdecl("at_au_audio_unit_output_bus_count")
public func at_au_audio_unit_output_bus_count(_ handle: UnsafeMutableRawPointer?) -> UInt64 {
    guard let handle else {
        return 0
    }
    return UInt64(auAudioUnitBox(from: handle).value.outputBusses.count)
}

@_cdecl("at_au_audio_unit_allocate_render_resources")
public func at_au_audio_unit_allocate_render_resources(
    _ handle: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
    guard let handle else {
        outError?.pointee = cStringCopy("AUAudioUnit handle was null")
        return false
    }
    do {
        try auAudioUnitBox(from: handle).value.allocateRenderResources()
        outError?.pointee = nil
        return true
    } catch {
        outError?.pointee = copyErrorDescription(error)
        return false
    }
}

@_cdecl("at_au_audio_unit_deallocate_render_resources")
public func at_au_audio_unit_deallocate_render_resources(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else {
        return
    }
    auAudioUnitBox(from: handle).value.deallocateRenderResources()
}

@_cdecl("at_au_audio_unit_render_resources_allocated")
public func at_au_audio_unit_render_resources_allocated(_ handle: UnsafeMutableRawPointer?) -> Bool {
    guard let handle else {
        return false
    }
    return auAudioUnitBox(from: handle).value.renderResourcesAllocated
}

@_cdecl("at_au_audio_unit_reset")
public func at_au_audio_unit_reset(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else {
        return
    }
    auAudioUnitBox(from: handle).value.reset()
}

@_cdecl("at_au_audio_unit_maximum_frames_to_render")
public func at_au_audio_unit_maximum_frames_to_render(_ handle: UnsafeMutableRawPointer?) -> UInt32 {
    guard let handle else {
        return 0
    }
    return auAudioUnitBox(from: handle).value.maximumFramesToRender
}

@_cdecl("at_au_audio_unit_set_maximum_frames_to_render")
public func at_au_audio_unit_set_maximum_frames_to_render(
    _ handle: UnsafeMutableRawPointer?,
    _ maximumFrames: UInt32
) {
    guard let handle else {
        return
    }
    auAudioUnitBox(from: handle).value.maximumFramesToRender = maximumFrames
}
