//! ABI layout assertions for the `#[repr(C)]` structs shared with the Swift
//! `AudioToolbox` bridge.
//!
//! Every struct generated in [`crate::generated_c_types`] is passed by value
//! (or through packed buffers) across the Rust <-> Swift `@_cdecl` FFI
//! boundary. If a field type, field order, or the resulting padding ever drifts
//! from what the Swift side (and the system `AudioToolbox` headers) expect, the
//! marshalled data silently corrupts.
//!
//! The compile-time `const _: () = assert!(...)` checks below pin the exact
//! size and alignment of each `repr(C)` struct so any accidental change fails
//! the build immediately instead of producing runtime garbage. The hottest
//! structs that actually cross the boundary additionally pin field offsets.
//!
//! The runtime [`verify_ffi_layout`] call asks the Swift bridge to confirm that
//! *its* `MemoryLayout` for the structs that cross FFI matches these values; a
//! `false` return is a genuine Rust <-> Swift ABI mismatch. It is exercised by
//! `tests/ffi_layout_tests.rs`.

use core::mem::{align_of, offset_of, size_of};

use crate::generated_c_types as g;

// MARK: - Size / alignment assertions for the full generated set
const _: () = assert!(size_of::<g::AudioBuffer>() == 16);
const _: () = assert!(align_of::<g::AudioBuffer>() == 8);
const _: () = assert!(size_of::<g::AudioBufferList>() == 24);
const _: () = assert!(align_of::<g::AudioBufferList>() == 8);
const _: () = assert!(size_of::<g::AudioStreamPacketDescription>() == 16);
const _: () = assert!(align_of::<g::AudioStreamPacketDescription>() == 8);
const _: () = assert!(size_of::<g::SMPTETime>() == 24);
const _: () = assert!(align_of::<g::SMPTETime>() == 4);
const _: () = assert!(size_of::<g::AudioTimeStamp>() == 64);
const _: () = assert!(align_of::<g::AudioTimeStamp>() == 8);
const _: () = assert!(size_of::<g::AudioClassDescription>() == 12);
const _: () = assert!(align_of::<g::AudioClassDescription>() == 4);
const _: () = assert!(size_of::<g::os_workgroup_s>() == 0);
const _: () = assert!(align_of::<g::os_workgroup_s>() == 1);
const _: () = assert!(size_of::<g::ComponentInstanceRecord>() == 0);
const _: () = assert!(align_of::<g::ComponentInstanceRecord>() == 1);
const _: () = assert!(size_of::<g::AudioComponentPlugInInterface>() == 32);
const _: () = assert!(align_of::<g::AudioComponentPlugInInterface>() == 8);
const _: () = assert!(size_of::<g::AudioCodecMagicCookieInfo>() == 16);
const _: () = assert!(align_of::<g::AudioCodecMagicCookieInfo>() == 8);
const _: () = assert!(size_of::<g::OpaqueAudioFileID>() == 0);
const _: () = assert!(align_of::<g::OpaqueAudioFileID>() == 1);
const _: () = assert!(size_of::<g::OpaqueAudioQueue>() == 0);
const _: () = assert!(align_of::<g::OpaqueAudioQueue>() == 1);
const _: () = assert!(size_of::<g::AudioQueueParameterEvent>() == 8);
const _: () = assert!(align_of::<g::AudioQueueParameterEvent>() == 4);
const _: () = assert!(size_of::<g::OpaqueAudioQueueProcessingTap>() == 0);
const _: () = assert!(align_of::<g::OpaqueAudioQueueProcessingTap>() == 1);
const _: () = assert!(size_of::<g::AudioUnitParameter>() == 24);
const _: () = assert!(align_of::<g::AudioUnitParameter>() == 8);
const _: () = assert!(size_of::<g::AudioUnitProperty>() == 24);
const _: () = assert!(align_of::<g::AudioUnitProperty>() == 8);
const _: () = assert!(size_of::<g::AudioUnitConnection>() == 16);
const _: () = assert!(align_of::<g::AudioUnitConnection>() == 8);
const _: () = assert!(size_of::<g::AUChannelInfo>() == 4);
const _: () = assert!(align_of::<g::AUChannelInfo>() == 2);
const _: () = assert!(size_of::<g::AudioUnitExternalBuffer>() == 16);
const _: () = assert!(align_of::<g::AudioUnitExternalBuffer>() == 8);
const _: () = assert!(size_of::<g::AUPreset>() == 16);
const _: () = assert!(align_of::<g::AUPreset>() == 8);
const _: () = assert!(size_of::<g::AudioUnitFrequencyResponseBin>() == 16);
const _: () = assert!(align_of::<g::AudioUnitFrequencyResponseBin>() == 8);
const _: () = assert!(size_of::<g::HostCallbackInfo>() == 40);
const _: () = assert!(align_of::<g::HostCallbackInfo>() == 8);
const _: () = assert!(size_of::<g::AUDependentParameter>() == 8);
const _: () = assert!(align_of::<g::AUDependentParameter>() == 4);
const _: () = assert!(size_of::<g::AudioUnitCocoaViewInfo>() == 16);
const _: () = assert!(align_of::<g::AudioUnitCocoaViewInfo>() == 8);
const _: () = assert!(size_of::<g::AUHostVersionIdentifier>() == 16);
const _: () = assert!(align_of::<g::AUHostVersionIdentifier>() == 8);
const _: () = assert!(size_of::<g::AUMIDIOutputCallbackStruct>() == 16);
const _: () = assert!(align_of::<g::AUMIDIOutputCallbackStruct>() == 8);
const _: () = assert!(size_of::<g::AUInputSamplesInOutputCallbackStruct>() == 16);
const _: () = assert!(align_of::<g::AUInputSamplesInOutputCallbackStruct>() == 8);
const _: () = assert!(size_of::<g::AudioUnitParameterHistoryInfo>() == 8);
const _: () = assert!(align_of::<g::AudioUnitParameterHistoryInfo>() == 4);
const _: () = assert!(size_of::<g::AudioUnitRenderContext>() == 32);
const _: () = assert!(align_of::<g::AudioUnitRenderContext>() == 8);
const _: () = assert!(size_of::<g::AudioUnitParameterInfo>() == 104);
const _: () = assert!(align_of::<g::AudioUnitParameterInfo>() == 8);
const _: () = assert!(size_of::<g::AudioUnitParameterNameInfo>() == 16);
const _: () = assert!(align_of::<g::AudioUnitParameterNameInfo>() == 8);
const _: () = assert!(size_of::<g::AudioUnitParameterStringFromValue>() == 24);
const _: () = assert!(align_of::<g::AudioUnitParameterStringFromValue>() == 8);
const _: () = assert!(size_of::<g::AudioUnitParameterValueFromString>() == 24);
const _: () = assert!(align_of::<g::AudioUnitParameterValueFromString>() == 8);
const _: () = assert!(size_of::<g::AUParameterMIDIMapping>() == 32);
const _: () = assert!(align_of::<g::AUParameterMIDIMapping>() == 4);
const _: () = assert!(size_of::<g::AudioUnitOtherPluginDesc>() == 16);
const _: () = assert!(align_of::<g::AudioUnitOtherPluginDesc>() == 4);
const _: () = assert!(size_of::<g::AudioUnitParameterValueTranslation>() == 32);
const _: () = assert!(align_of::<g::AudioUnitParameterValueTranslation>() == 4);
const _: () = assert!(size_of::<g::AudioUnitPresetMAS_SettingData>() == 16);
const _: () = assert!(align_of::<g::AudioUnitPresetMAS_SettingData>() == 4);
const _: () = assert!(size_of::<g::AudioUnitPresetMAS_Settings>() == 36);
const _: () = assert!(align_of::<g::AudioUnitPresetMAS_Settings>() == 4);
const _: () = assert!(size_of::<g::AudioOutputUnitMIDICallbacks>() == 24);
const _: () = assert!(align_of::<g::AudioOutputUnitMIDICallbacks>() == 8);
const _: () = assert!(size_of::<g::AudioOutputUnitStartAtTimeParams>() == 72);
const _: () = assert!(align_of::<g::AudioOutputUnitStartAtTimeParams>() == 8);
const _: () = assert!(size_of::<g::AUVoiceIOOtherAudioDuckingConfiguration>() == 8);
const _: () = assert!(align_of::<g::AUVoiceIOOtherAudioDuckingConfiguration>() == 4);
const _: () = assert!(size_of::<g::AudioUnitMeterClipping>() == 8);
const _: () = assert!(align_of::<g::AudioUnitMeterClipping>() == 4);
const _: () = assert!(size_of::<g::MixerDistanceParams>() == 12);
const _: () = assert!(align_of::<g::MixerDistanceParams>() == 4);
const _: () = assert!(size_of::<g::ScheduledAudioSlice>() == 112);
const _: () = assert!(align_of::<g::ScheduledAudioSlice>() == 8);
const _: () = assert!(size_of::<g::ScheduledAudioFileRegion>() == 112);
const _: () = assert!(align_of::<g::ScheduledAudioFileRegion>() == 8);
const _: () = assert!(size_of::<g::AUSamplerInstrumentData>() == 16);
const _: () = assert!(align_of::<g::AUSamplerInstrumentData>() == 8);
const _: () = assert!(size_of::<g::AUNumVersion>() == 4);
const _: () = assert!(align_of::<g::AUNumVersion>() == 1);
const _: () = assert!(size_of::<g::AUHostIdentifier>() == 16);
const _: () = assert!(align_of::<g::AUHostIdentifier>() == 8);
const _: () = assert!(size_of::<g::AUListenerBase>() == 0);
const _: () = assert!(align_of::<g::AUListenerBase>() == 1);
const _: () = assert!(size_of::<g::AudioUnitEvent>() == 32);
const _: () = assert!(align_of::<g::AudioUnitEvent>() == 8);
const _: () = assert!(size_of::<g::AudioUnitEvent__bindgen_ty_1>() == 24);
const _: () = assert!(align_of::<g::AudioUnitEvent__bindgen_ty_1>() == 8);
const _: () = assert!(size_of::<g::MIDIEventList>() == 276);
const _: () = assert!(align_of::<g::MIDIEventList>() == 4);
const _: () = assert!(size_of::<g::MIDIPacketList>() == 272);
const _: () = assert!(align_of::<g::MIDIPacketList>() == 4);
const _: () = assert!(size_of::<g::OpaqueCAClock>() == 0);
const _: () = assert!(align_of::<g::OpaqueCAClock>() == 1);
const _: () = assert!(size_of::<g::CAClockTime>() == 32);
const _: () = assert!(align_of::<g::CAClockTime>() == 8);
const _: () = assert!(size_of::<g::CAClockTime__bindgen_ty_1>() == 24);
const _: () = assert!(align_of::<g::CAClockTime__bindgen_ty_1>() == 8);
const _: () = assert!(size_of::<g::CATempoMapEntry>() == 16);
const _: () = assert!(align_of::<g::CATempoMapEntry>() == 8);
const _: () = assert!(size_of::<g::CAMeterTrackEntry>() == 16);
const _: () = assert!(align_of::<g::CAMeterTrackEntry>() == 8);
const _: () = assert!(size_of::<g::OpaqueAudioConverter>() == 0);
const _: () = assert!(align_of::<g::OpaqueAudioConverter>() == 1);

// MARK: - Field-offset assertions for the structs that cross FFI by value
//
// These are the `CoreAudio` types passed by value to/from the Swift bridge.
// Pinning offsets (not just size/align) guards against field reordering that
// keeps the overall size identical but scrambles the marshalled fields.
const _: () = assert!(offset_of!(g::AudioBuffer, mNumberChannels) == 0);
const _: () = assert!(offset_of!(g::AudioBuffer, mDataByteSize) == 4);
const _: () = assert!(offset_of!(g::AudioBuffer, mData) == 8);

const _: () = assert!(offset_of!(g::AudioBufferList, mNumberBuffers) == 0);
const _: () = assert!(offset_of!(g::AudioBufferList, mBuffers) == 8);

const _: () = assert!(offset_of!(g::AudioStreamPacketDescription, mStartOffset) == 0);
const _: () = assert!(offset_of!(g::AudioStreamPacketDescription, mVariableFramesInPacket) == 8);
const _: () = assert!(offset_of!(g::AudioStreamPacketDescription, mDataByteSize) == 12);

const _: () = assert!(offset_of!(g::AudioTimeStamp, mSampleTime) == 0);
const _: () = assert!(offset_of!(g::AudioTimeStamp, mHostTime) == 8);
const _: () = assert!(offset_of!(g::AudioTimeStamp, mRateScalar) == 16);
const _: () = assert!(offset_of!(g::AudioTimeStamp, mWordClockTime) == 24);
const _: () = assert!(offset_of!(g::AudioTimeStamp, mSMPTETime) == 32);
const _: () = assert!(offset_of!(g::AudioTimeStamp, mFlags) == 56);

const _: () = assert!(offset_of!(g::AudioClassDescription, mType) == 0);
const _: () = assert!(offset_of!(g::AudioClassDescription, mSubType) == 4);
const _: () = assert!(offset_of!(g::AudioClassDescription, mManufacturer) == 8);

// MARK: - Cross-language verification

extern "C" {
    /// Implemented in `swift-bridge/Sources/AudioToolboxBridge/Core.swift`.
    ///
    /// Returns `true` only if the Swift `MemoryLayout` (size, stride and
    /// alignment) of the `CoreAudio` structs that cross the FFI boundary matches
    /// the values pinned above on the Rust side.
    fn at_verify_ffi_layout() -> bool;
}

/// Ask the Swift bridge to confirm its view of the shared FFI struct layout
/// matches the Rust side. Returns `false` on a genuine ABI mismatch.
#[must_use]
pub fn verify_ffi_layout() -> bool {
    // SAFETY: `at_verify_ffi_layout` takes no arguments and only reads
    // compile-time `MemoryLayout` constants in the Swift bridge.
    unsafe { at_verify_ffi_layout() }
}
