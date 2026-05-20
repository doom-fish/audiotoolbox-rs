#![allow(non_camel_case_types, non_snake_case)]

use crate::{
    AVAudio3DPoint, AudioFileSmpteTime, AudioFileTypeAndFormatId, CFStringRef, MusicDeviceGroupId,
    MusicDeviceInstrumentId, NoteInstanceId, OSStatus,
};
use std::ffi::c_void;

/// Wraps `AudioFileTypeAndFormatID`.
pub type AudioFileTypeAndFormatID = AudioFileTypeAndFormatId;
/// Wraps `AudioFile_SMPTE_Time`.
pub type AudioFile_SMPTE_Time = AudioFileSmpteTime;
/// Wraps `MusicDeviceGroupID`.
pub type MusicDeviceGroupID = MusicDeviceGroupId;
/// Wraps `MusicDeviceInstrumentID`.
pub type MusicDeviceInstrumentID = MusicDeviceInstrumentId;
/// Wraps `NoteInstanceID`.
pub type NoteInstanceID = NoteInstanceId;
/// Wraps `AudioUnitParameterOptions`.
pub type AudioUnitParameterOptions = crate::generated_c_types::AudioUnitParameterOptions;
/// Wraps `MIDIEventList`.
pub type MIDIEventList = crate::generated_c_types::MIDIEventList;

/// Wraps `AUValue`.
pub type AUValue = f32;
/// Wraps `AUEventSampleTime`.
pub type AUEventSampleTime = i64;
/// Wraps `AUParameterAddress`.
pub type AUParameterAddress = u64;
/// Wraps `AUParameterObserverToken`.
pub type AUParameterObserverToken = *mut c_void;
/// Wraps `AUParameterAutomationEventType`.
pub type AUParameterAutomationEventType = u32;
/// Wraps `AUAudioUnitBusType`.
pub type AUAudioUnitBusType = i64;
/// Wraps `AUAudioUnitStatus`.
pub type AUAudioUnitStatus = OSStatus;
/// Wraps `AUAudioObjectID`.
pub type AUAudioObjectID = u32;
/// Wraps `MIDIChannelNumber`.
pub type MIDIChannelNumber = u8;
/// Wraps `AUMIDIEventListBlock`.
pub type AUMIDIEventListBlock = *mut c_void;
/// Wraps `AUMIDIOutputEventBlock`.
pub type AUMIDIOutputEventBlock = *mut c_void;
/// Wraps `AURenderPullInputBlock`.
pub type AURenderPullInputBlock = *mut c_void;
/// Wraps `kAudioUnitParameterFlagDisplayMask`.
pub const AUDIO_UNIT_PARAMETER_FLAG_DISPLAY_MASK: AudioUnitParameterOptions =
    (7_u32 << 16) | (1_u32 << 22);

#[must_use]
/// Extracts the display-type bits used with `AudioUnitParameterOptions`.
pub const fn get_audio_unit_parameter_display_type(
    flags: AudioUnitParameterOptions,
) -> AudioUnitParameterOptions {
    flags & AUDIO_UNIT_PARAMETER_FLAG_DISPLAY_MASK
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
/// Wraps `AURecordedParameterEvent`.
pub struct AURecordedParameterEvent {
    pub hostTime: u64,
    pub address: AUParameterAddress,
    pub value: AUValue,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
/// Wraps `AUParameterAutomationEvent`.
pub struct AUParameterAutomationEvent {
    pub hostTime: u64,
    pub address: AUParameterAddress,
    pub value: AUValue,
    pub eventType: AUParameterAutomationEventType,
    pub reserved: u64,
}

impl Default for AUParameterAutomationEvent {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// Wraps `AURenderEventType`.
pub type AURenderEventType = u8;

#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
/// Wraps `AURenderEventHeader`.
pub struct AURenderEventHeader {
    pub next: *mut AURenderEvent,
    pub eventSampleTime: AUEventSampleTime,
    pub eventType: AURenderEventType,
    pub reserved: u8,
}

impl Default for AURenderEventHeader {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
/// Wraps `AUParameterEvent`.
pub struct AUParameterEvent {
    pub next: *mut AURenderEvent,
    pub eventSampleTime: AUEventSampleTime,
    pub eventType: AURenderEventType,
    pub reserved: [u8; 3],
    pub rampDurationSampleFrames: crate::AUAudioFrameCount,
    pub parameterAddress: AUParameterAddress,
    pub value: AUValue,
}

impl Default for AUParameterEvent {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
/// Wraps `AUMIDIEvent`.
pub struct AUMIDIEvent {
    pub next: *mut AURenderEvent,
    pub eventSampleTime: AUEventSampleTime,
    pub eventType: AURenderEventType,
    pub reserved: u8,
    pub length: u16,
    pub cable: u8,
    pub data: [u8; 3],
}

impl Default for AUMIDIEvent {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
/// Wraps `AUMIDIEventList`.
pub struct AUMIDIEventList {
    pub next: *mut AURenderEvent,
    pub eventSampleTime: AUEventSampleTime,
    pub eventType: AURenderEventType,
    pub reserved: u8,
    pub cable: u8,
    pub eventList: MIDIEventList,
}

impl Default for AUMIDIEventList {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
/// Wraps `AURenderEvent`.
pub union AURenderEvent {
    pub head: AURenderEventHeader,
    pub parameter: AUParameterEvent,
    pub MIDI: AUMIDIEvent,
    pub MIDIEventsList: AUMIDIEventList,
}

impl Default for AURenderEvent {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// Wraps `AVAudioSequencerInfoDictionaryKey`.
pub type AVAudioSequencerInfoDictionaryKey = CFStringRef;
/// Wraps `AVAudioSessionPolarPattern`.
pub type AVAudioSessionPolarPattern = CFStringRef;
/// Wraps `AVAudioSessionOrientation`.
pub type AVAudioSessionOrientation = CFStringRef;
/// Wraps `AVAudioSessionLocation`.
pub type AVAudioSessionLocation = CFStringRef;
/// Wraps `AVAudio3DVector`.
pub type AVAudio3DVector = AVAudio3DPoint;
/// Wraps `AVAudioPlayerNodeCompletionCallbackType`.
pub type AVAudioPlayerNodeCompletionCallbackType = i64;
/// Wraps `AVAudioSessionCategoryOptions`.
pub type AVAudioSessionCategoryOptions = u64;
/// Wraps `AVAudioSessionSetActiveOptions`.
pub type AVAudioSessionSetActiveOptions = u64;
/// Wraps `AVAudioStereoOrientation`.
pub type AVAudioStereoOrientation = i64;
/// Wraps `AVAudioSessionMicrophoneInjectionMode`.
pub type AVAudioSessionMicrophoneInjectionMode = i64;
/// Wraps `AVAudioUnitReverbPreset`.
pub type AVAudioUnitReverbPreset = i64;
/// Wraps `AVMIDIControlChangeMessageType`.
pub type AVMIDIControlChangeMessageType = i64;
/// Wraps `AVMIDIMetaEventType`.
pub type AVMIDIMetaEventType = i64;
/// Wraps `AVAudioApplicationMicrophoneInjectionPermission`.
pub type AVAudioApplicationMicrophoneInjectionPermission = i64;
/// Wraps `AVAudioNodeCompletionHandler`.
pub type AVAudioNodeCompletionHandler = *mut c_void;
/// Wraps `AVAudioPlayerNodeCompletionHandler`.
pub type AVAudioPlayerNodeCompletionHandler = *mut c_void;
/// Wraps `AVSpeechSynthesizerBufferCallback`.
pub type AVSpeechSynthesizerBufferCallback = *mut c_void;
/// Wraps `AVAudioVoiceProcessingOtherAudioDuckingLevel`.
pub type AVAudioVoiceProcessingOtherAudioDuckingLevel = i64;

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
/// Wraps `AVAudioVoiceProcessingOtherAudioDuckingConfiguration`.
pub struct AVAudioVoiceProcessingOtherAudioDuckingConfiguration {
    pub enableAdvancedDucking: bool,
    pub duckingLevel: AVAudioVoiceProcessingOtherAudioDuckingLevel,
}

macro_rules! opaque_objc_handle {
    ($($name:ident),+ $(,)?) => {
        $(
            #[repr(transparent)]
            #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
            #[doc = concat!("Wraps `", stringify!($name), "`.")]
            pub struct $name {
                handle: *mut c_void,
            }

            impl $name {
                #[must_use]
                #[doc = concat!("Wraps an existing `", stringify!($name), "` handle without changing ownership.")]
                pub fn from_raw(handle: *mut c_void) -> Self {
                    Self { handle }
                }

                #[must_use]
                #[doc = concat!("Returns the wrapped `", stringify!($name), "` handle.")]
                pub fn as_raw(self) -> *mut c_void {
                    self.handle
                }

                #[must_use]
                /// Returns whether the wrapped AudioToolbox.framework handle is null.
                pub fn is_null(self) -> bool {
                    self.handle.is_null()
                }
            }
        )+
    };
}

opaque_objc_handle!(
    AVAudioTime,
    AVAudioUnit,
    AVMusicEvent,
    AVSpeechSynthesizer,
    AVSpeechUtterance,
    AUParameter,
    AVAudioBuffer,
    AVAudioUnitEffect,
    AVMusicTrack,
    AUParameterNode,
    AUAudioUnitPreset,
    AVAudioConnectionPoint,
    AVAudioPlayer,
    AVAudioRecorder,
    AVSpeechSynthesisMarker,
    AUAudioUnitBus,
    AVAudioUnitComponent,
    AUParameterGroup,
    AVAudioUnitTimeEffect,
    AVMIDIChannelEvent,
    AVSpeechSynthesisVoice,
    AUAudioUnitBusArray,
    AVAudioIONode,
    AVAudioMixerNode,
    AVSpeechSynthesisProviderRequest,
    AVSpeechSynthesisProviderVoice,
    AUParameterTree,
    AVAudioUnitMIDIInstrument,
    AVAudioConverter,
    AVAudioSessionCapability,
    AVAudioUnitComponentManager,
    AVAudioUnitEQ,
    AVAudioUnitReverb,
    AVAudioEnvironmentNode,
    AVAudioPlayerNode,
    AVAudioUnitDelay,
    AVAudioUnitDistortion,
    AVAudioUnitGenerator,
    AVAudioUnitSampler,
    AVAudioUnitTimePitch,
);

/// Marker trait mirroring `AVAudioMixing`.
pub trait AVAudioMixing {}
/// Marker trait mirroring `AUMessageChannel`.
pub trait AUMessageChannel {}
/// Marker trait mirroring `AVAudio3DMixing`.
pub trait AVAudio3DMixing: AVAudioMixing {}
/// Marker trait mirroring `AVAudioPlayerDelegate`.
pub trait AVAudioPlayerDelegate {}
/// Marker trait mirroring `AVAudioRecorderDelegate`.
pub trait AVAudioRecorderDelegate {}
/// Marker trait mirroring `AVAudioStereoMixing`.
pub trait AVAudioStereoMixing: AVAudioMixing {}
/// Marker trait mirroring `AVSpeechSynthesizerDelegate`.
pub trait AVSpeechSynthesizerDelegate {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::c_void;

    #[test]
    fn display_type_masks_unrelated_bits() {
        let flags = AUDIO_UNIT_PARAMETER_FLAG_DISPLAY_MASK | 0b1010;

        assert_eq!(
            get_audio_unit_parameter_display_type(flags),
            AUDIO_UNIT_PARAMETER_FLAG_DISPLAY_MASK,
        );
    }

    #[test]
    fn parameter_automation_event_default_zeroes_fields() {
        let event = AUParameterAutomationEvent::default();

        assert_eq!(event.hostTime, 0);
        assert_eq!(event.address, 0);
        assert!(event.value.abs() < f32::EPSILON);
        assert_eq!(event.eventType, 0);
        assert_eq!(event.reserved, 0);
    }

    #[test]
    fn render_event_header_default_zeroes_fields() {
        let header = AURenderEventHeader::default();
        let next = header.next;
        let event_sample_time = header.eventSampleTime;
        let event_type = header.eventType;
        let reserved = header.reserved;

        assert!(next.is_null());
        assert_eq!(event_sample_time, 0);
        assert_eq!(event_type, 0);
        assert_eq!(reserved, 0);
    }

    #[test]
    fn voice_processing_ducking_configuration_default_is_disabled() {
        let configuration = AVAudioVoiceProcessingOtherAudioDuckingConfiguration::default();

        assert!(!configuration.enableAdvancedDucking);
        assert_eq!(configuration.duckingLevel, 0);
    }

    #[test]
    fn opaque_handle_round_trips_raw_pointer() {
        let raw = std::ptr::NonNull::<u8>::dangling().as_ptr().cast::<c_void>();
        let handle = AVAudioTime::from_raw(raw);

        assert_eq!(handle.as_raw(), raw);
        assert!(!handle.is_null());
    }

    #[test]
    fn opaque_handle_reports_null_pointer() {
        let handle = AVAudioTime::from_raw(std::ptr::null_mut());

        assert!(handle.is_null());
    }
}
