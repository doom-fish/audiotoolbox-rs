#![allow(non_camel_case_types, non_snake_case)]

use crate::{
    AVAudio3DPoint, AudioFileSmpteTime, AudioFileTypeAndFormatId, CFStringRef, MusicDeviceGroupId,
    MusicDeviceInstrumentId, NoteInstanceId, OSStatus,
};
use std::ffi::c_void;

pub type AudioFileTypeAndFormatID = AudioFileTypeAndFormatId;
pub type AudioFile_SMPTE_Time = AudioFileSmpteTime;
pub type MusicDeviceGroupID = MusicDeviceGroupId;
pub type MusicDeviceInstrumentID = MusicDeviceInstrumentId;
pub type NoteInstanceID = NoteInstanceId;
pub type AudioUnitParameterOptions = crate::generated_c_types::AudioUnitParameterOptions;
pub type MIDIEventList = crate::generated_c_types::MIDIEventList;

pub type AUValue = f32;
pub type AUEventSampleTime = i64;
pub type AUParameterAddress = u64;
pub type AUParameterObserverToken = *mut c_void;
pub type AUParameterAutomationEventType = u32;
pub type AUAudioUnitBusType = i64;
pub type AUAudioUnitStatus = OSStatus;
pub type AUAudioObjectID = u32;
pub type MIDIChannelNumber = u8;
pub type AUMIDIEventListBlock = *mut c_void;
pub type AUMIDIOutputEventBlock = *mut c_void;
pub type AURenderPullInputBlock = *mut c_void;
pub const AUDIO_UNIT_PARAMETER_FLAG_DISPLAY_MASK: AudioUnitParameterOptions =
    (7_u32 << 16) | (1_u32 << 22);

#[must_use]
pub const fn get_audio_unit_parameter_display_type(
    flags: AudioUnitParameterOptions,
) -> AudioUnitParameterOptions {
    flags & AUDIO_UNIT_PARAMETER_FLAG_DISPLAY_MASK
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct AURecordedParameterEvent {
    pub hostTime: u64,
    pub address: AUParameterAddress,
    pub value: AUValue,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
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

pub type AURenderEventType = u8;

#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
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

pub type AVAudioSequencerInfoDictionaryKey = CFStringRef;
pub type AVAudioSessionPolarPattern = CFStringRef;
pub type AVAudioSessionOrientation = CFStringRef;
pub type AVAudioSessionLocation = CFStringRef;
pub type AVAudio3DVector = AVAudio3DPoint;
pub type AVAudioPlayerNodeCompletionCallbackType = i64;
pub type AVAudioSessionCategoryOptions = u64;
pub type AVAudioSessionSetActiveOptions = u64;
pub type AVAudioStereoOrientation = i64;
pub type AVAudioSessionMicrophoneInjectionMode = i64;
pub type AVAudioUnitReverbPreset = i64;
pub type AVMIDIControlChangeMessageType = i64;
pub type AVMIDIMetaEventType = i64;
pub type AVAudioApplicationMicrophoneInjectionPermission = i64;
pub type AVAudioNodeCompletionHandler = *mut c_void;
pub type AVAudioPlayerNodeCompletionHandler = *mut c_void;
pub type AVSpeechSynthesizerBufferCallback = *mut c_void;
pub type AVAudioVoiceProcessingOtherAudioDuckingLevel = i64;

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct AVAudioVoiceProcessingOtherAudioDuckingConfiguration {
    pub enableAdvancedDucking: bool,
    pub duckingLevel: AVAudioVoiceProcessingOtherAudioDuckingLevel,
}

macro_rules! opaque_objc_handle {
    ($($name:ident),+ $(,)?) => {
        $(
            #[repr(transparent)]
            #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
            pub struct $name {
                handle: *mut c_void,
            }

            impl $name {
                #[must_use]
                pub fn from_raw(handle: *mut c_void) -> Self {
                    Self { handle }
                }

                #[must_use]
                pub fn as_raw(self) -> *mut c_void {
                    self.handle
                }

                #[must_use]
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

pub trait AVAudioMixing {}
pub trait AUMessageChannel {}
pub trait AVAudio3DMixing: AVAudioMixing {}
pub trait AVAudioPlayerDelegate {}
pub trait AVAudioRecorderDelegate {}
pub trait AVAudioStereoMixing: AVAudioMixing {}
pub trait AVSpeechSynthesizerDelegate {}
