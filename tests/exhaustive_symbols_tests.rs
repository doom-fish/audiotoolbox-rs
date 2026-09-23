#![allow(clippy::too_many_lines)]

use audiotoolbox::{generated_c_types, *};
use std::mem::size_of;

macro_rules! sdk_sizes {
    ($($ty:ty => $size:expr),+ $(,)?) => {
        vec![$((stringify!($ty), size_of::<$ty>(), $size)),+]
    };
}

#[test]
fn every_audit_symbol_matches_the_sdk_size() {
    let checked = sdk_sizes!(
        AVAudioSequencerInfoDictionaryKey => 8,
        AVAudioTime => 8,
        AVMusicTimeStamp => 8,
        AUValue => 4,
        AudioUnitParameter => 24,
        AVAudioPCMBuffer => 8,
        AudioUnitRenderActionFlags => 4,
        AVAudioUnit => 8,
        AVMusicEvent => 8,
        MusicEventIterator => 8,
        AVSpeechSynthesizer => 8,
        AVSpeechUtterance => 8,
        AudioFileMarker => 40,
        AUEventSampleTime => 8,
        AVAudioFramePosition => 8,
        AUParameter => 8,
        AVAudioBuffer => 8,
        AVAudioUnitEffect => 8,
        AVMusicTrack => 8,
        AudioFileRegion => 64,
        AUParameterAddress => 8,
        AUParameterNode => 8,
        AudioFileMarkerList => 48,
        AVAudioSessionPolarPattern => 8,
        AudioUnitParameterOptions => 4,
        AUAudioUnitPreset => 8,
        AVAudioConnectionPoint => 8,
        AVAudioPlayer => 8,
        AVAudioRecorder => 8,
        AVSpeechSynthesisMarker => 8,
        AVAudio3DPoint => 12,
        AUParameterObserverToken => 8,
        AURenderEvent => 296,
        AVAudioSessionOrientation => 8,
        AURenderCallback => 8,
        AUAudioUnitBus => 8,
        AVAudioUnitComponent => 8,
        MIDIEventList => 276,
        AVAudio3DVector => 12,
        MIDIEndpointRef => 4,
        MusicDeviceGroupID => 4,
        AudioUnitPropertyListenerProc => 8,
        AUParameterGroup => 8,
        AVAudioUnitTimeEffect => 8,
        AVMIDIChannelEvent => 8,
        AVSpeechSynthesisVoice => 8,
        AVAudio3DAngularOrientation => 12,
        AVAudio3DVectorOrientation => 24,
        MusicDeviceNoteParams => 20,
        AVAudioPacketCount => 4,
        AVAudioPlayerNodeCompletionCallbackType => 8,
        MusicDeviceInstrumentID => 4,
        AUAudioUnitBusArray => 8,
        AVAudioIONode => 8,
        AVAudioMixerNode => 8,
        AVSpeechSynthesisProviderRequest => 8,
        AVSpeechSynthesisProviderVoice => 8,
        AUNodeInteraction => 32,
        AURenderCallbackStruct => 16,
        AudioUnitNodeConnection => 16,
        AudioUnitParameterEvent => 32,
        AUAudioUnitBusType => 8,
        AURenderEventType => 1,
        AVAudioSessionCategoryOptions => 8,
        MIDIChannelNumber => 1,
        NoteInstanceID => 4,
        AUMIDIEventListBlock => 8,
        AVAudioNodeCompletionHandler => 8,
        AVAudioPlayerNodeCompletionHandler => 8,
        AUParameterTree => 8,
        AVAudioSequencer => 8,
        AVAudioUnitMIDIInstrument => 8,
        AUNodeRenderCallback => 24,
        AVAudioVoiceProcessingOtherAudioDuckingConfiguration => 16,
        AudioFile_SMPTE_Time => 8,
        AudioUnitProperty => 24,
        MusicEventUserData => 8,
        NoteParamsControlValue => 8,
        AUAudioUnitStatus => 4,
        AVAudioSessionLocation => 8,
        AVAudioStereoOrientation => 8,
        AUMIDIOutputEventBlock => 8,
        AURenderPullInputBlock => 8,
        AVAudioConverter => 8,
        AVAudioSessionCapability => 8,
        AVAudioUnitComponentManager => 8,
        AVAudioUnitEQ => 8,
        AVAudioUnitReverb => 8,
        AUMIDIEvent => 24,
        AUMIDIEventList => 296,
        AUParameterAutomationEvent => 32,
        AUParameterEvent => 36,
        AUPresetEvent => 16,
        AURecordedParameterEvent => 24,
        AURenderEventHeader => 20,
        AVAudioConverterPrimeInfo => 8,
        AudioBalanceFade => 24,
        AudioBytePacketTranslation => 24,
        AudioConverterPrimeInfo => 8,
        AudioFilePacketTableInfo => 16,
        AudioFileRegionList => 72,
        AudioFileTypeAndFormatID => 8,
        AudioFormatInfo => 56,
        AudioFramePacketTranslation => 24,
        AudioIndependentPacketTranslation => 16,
        AudioPacketDependencyInfoTranslation => 16,
        AudioPacketRangeByteCountTranslation => 24,
        AudioPacketRollDistanceTranslation => 16,
        AudioPanningInfo => 32,
        ExtendedAudioFormatInfo => 64,
        ExtendedControlEvent => 12,
        ExtendedNoteOnEvent => 32,
        MIDIChannelMessage => 4,
        MIDIMetaEvent => 12,
        MIDIRawData => 8,
        MusicDeviceStdNoteParams => 12,
        ParameterEvent => 16,
        AUAudioObjectID => 4,
        AUParameterAutomationEventType => 4,
        AVAudioApplicationMicrophoneInjectionPermission => 8,
        AVAudioEngineManualRenderingMode => 8,
        AVAudioEngineManualRenderingStatus => 8,
        AVAudioPlayerNodeBufferOptions => 8,
        AVAudioSessionMicrophoneInjectionMode => 8,
        AVAudioSessionSetActiveOptions => 8,
        AVAudioUnitReverbPreset => 8,
        AVMIDIControlChangeMessageType => 8,
        AVMIDIMetaEventType => 8,
        AVMusicSequenceLoadOptions => 8,
        AVSpeechBoundary => 8,
        AVSpeechSynthesisMarkerMark => 8,
        AVSpeechSynthesisPersonalVoiceAuthorizationStatus => 8,
        AVSpeechSynthesisVoiceGender => 8,
        AudioComponentValidationResult => 4,
        MusicEventType => 4,
        AVSpeechSynthesizerBufferCallback => 8,
        AVAudioEnvironmentNode => 8,
        AVAudioPlayerNode => 8,
        AVAudioUnitDelay => 8,
        AVAudioUnitDistortion => 8,
        AVAudioUnitGenerator => 8,
        AVAudioUnitSampler => 8,
        AVAudioUnitTimePitch => 8,
        generated_c_types::CAClockRef => 8,
        generated_c_types::AudioCodec => 8,
        generated_c_types::AudioCodecPropertyID => 4,
        generated_c_types::CAClockTime => 32,
        generated_c_types::AUParameterListenerRef => 8,
        generated_c_types::AudioUnitEvent => 32,
        generated_c_types::AudioSessionPropertyID => 4,
        generated_c_types::MusicDeviceComponent => 8,
        generated_c_types::AudioFile_GetSizeProc => 8,
        generated_c_types::AudioFile_ReadProc => 8,
        generated_c_types::AudioFile_SetSizeProc => 8,
        generated_c_types::AudioFile_WriteProc => 8,
        generated_c_types::CAFMarker => 28,
        generated_c_types::CAFRegion => 40,
        generated_c_types::AUEventListenerRef => 8,
        generated_c_types::AudioQueueProcessingTapRef => 8,
        generated_c_types::CAClockBeats => 8,
        generated_c_types::AUNumVersion => 4,
        generated_c_types::CAClockTimeFormat => 4,
        generated_c_types::MIDIPacketList => 272,
        generated_c_types::AudioCodecMagicCookieInfo => 16,
        generated_c_types::AudioComponentPlugInInterface => 32,
        generated_c_types::AudioQueueParameterEvent => 8,
        generated_c_types::AudioUnitOtherPluginDesc => 16,
        generated_c_types::AudioUnitParameterNameInfo => 16,
        generated_c_types::AudioUnitPresetMAS_SettingData => 16,
        generated_c_types::AudioUnitRenderContext => 32,
        generated_c_types::CAFAudioDescription => 32,
        generated_c_types::CAFMarkerChunk => 36,
        generated_c_types::CAFOverviewSample => 4,
        generated_c_types::CAFPositionPeak => 12,
        generated_c_types::CAFRegionChunk => 48,
        generated_c_types::CAFStringID => 12,
        generated_c_types::CAF_SMPTE_Time => 8,
        generated_c_types::ScheduledAudioFileRegion => 112,
        generated_c_types::ScheduledAudioSlice => 112,
        generated_c_types::AudioQueueProcessingTapFlags => 4,
        generated_c_types::AudioUnitParameterUnit => 4,
        generated_c_types::CAClockPropertyID => 4,
        generated_c_types::CAClockSeconds => 8,
        generated_c_types::CAClockTempo => 8,
        generated_c_types::AUChannelInfo => 4,
        generated_c_types::AUDependentParameter => 8,
        generated_c_types::AUHostIdentifier => 16,
        generated_c_types::AUHostVersionIdentifier => 16,
        generated_c_types::AUInputSamplesInOutputCallbackStruct => 16,
        generated_c_types::AUMIDIOutputCallbackStruct => 16,
        generated_c_types::AUParameterMIDIMapping => 32,
        generated_c_types::AUPreset => 16,
        generated_c_types::AUSamplerInstrumentData => 16,
        generated_c_types::AUVoiceIOOtherAudioDuckingConfiguration => 8,
        generated_c_types::AudioOutputUnitMIDICallbacks => 24,
        generated_c_types::AudioOutputUnitStartAtTimeParams => 72,
        generated_c_types::AudioUnitCocoaViewInfo => 16,
        generated_c_types::AudioUnitConnection => 16,
        generated_c_types::AudioUnitExternalBuffer => 16,
        generated_c_types::AudioUnitFrequencyResponseBin => 16,
        generated_c_types::AudioUnitMeterClipping => 8,
        generated_c_types::AudioUnitParameterHistoryInfo => 8,
        generated_c_types::AudioUnitParameterInfo => 104,
        generated_c_types::AudioUnitParameterStringFromValue => 24,
        generated_c_types::AudioUnitParameterValueFromString => 24,
        generated_c_types::AudioUnitParameterValueTranslation => 32,
        generated_c_types::AudioUnitPresetMAS_Settings => 36,
        generated_c_types::CAFDataChunk => 5,
        generated_c_types::CAFInfoStrings => 4,
        generated_c_types::CAFInstrumentChunk => 28,
        generated_c_types::CAFOverviewChunk => 12,
        generated_c_types::CAFPacketTableHeader => 25,
        generated_c_types::CAFPeakChunk => 16,
        generated_c_types::CAFStrings => 16,
        generated_c_types::CAFUMIDChunk => 64,
        generated_c_types::CAF_UUID_ChunkHeader => 28,
        generated_c_types::CAMeterTrackEntry => 16,
        generated_c_types::CATempoMapEntry => 16,
        generated_c_types::HostCallbackInfo => 40,
        generated_c_types::MixerDistanceParams => 12,
        generated_c_types::AudioConverterComplexInputDataProc => 8,
        generated_c_types::AudioQueuePropertyListenerProc => 8,
        generated_c_types::AudioSessionPropertyListener => 8,
        generated_c_types::CAClockListenerProc => 8,
        generated_c_types::CountUserDataFDF => 8,
        generated_c_types::GetPropertyFDF => 8,
        generated_c_types::GetPropertyInfoFDF => 8,
        generated_c_types::GetUserDataFDF => 8,
        generated_c_types::GetUserDataSizeFDF => 8,
        generated_c_types::ReadBytesFDF => 8,
        generated_c_types::ReadPacketsFDF => 8,
        generated_c_types::SetPropertyFDF => 8,
        generated_c_types::SetUserDataFDF => 8,
        generated_c_types::WriteBytesFDF => 8,
        generated_c_types::WritePacketsFDF => 8,
    );
    let mismatched: Vec<_> = checked
        .iter()
        .filter(|(_, rust, sdk)| rust != sdk)
        .collect();
    assert!(
        mismatched.is_empty(),
        "size differs from the SDK: {mismatched:?}"
    );
}

#[test]
fn objective_c_handles_are_null_by_default_and_round_trip() {
    assert!(AVAudioTime::default().is_null());
    assert!(AVAudioUnit::default().is_null());
    assert!(AVMusicEvent::default().is_null());
    assert!(AVSpeechSynthesizer::default().is_null());
    assert!(AVSpeechUtterance::default().is_null());
    assert!(AUParameter::default().is_null());
    assert!(AVAudioBuffer::default().is_null());
    assert!(AVAudioUnitEffect::default().is_null());
    assert!(AVMusicTrack::default().is_null());
    assert!(AUParameterNode::default().is_null());
    assert!(AUAudioUnitPreset::default().is_null());
    assert!(AVAudioConnectionPoint::default().is_null());
    assert!(AVAudioPlayer::default().is_null());
    assert!(AVAudioRecorder::default().is_null());
    assert!(AVSpeechSynthesisMarker::default().is_null());
    assert!(AUAudioUnitBus::default().is_null());
    assert!(AVAudioUnitComponent::default().is_null());
    assert!(AUParameterGroup::default().is_null());
    assert!(AVAudioUnitTimeEffect::default().is_null());
    assert!(AVMIDIChannelEvent::default().is_null());
    assert!(AVSpeechSynthesisVoice::default().is_null());
    assert!(AUAudioUnitBusArray::default().is_null());
    assert!(AVAudioIONode::default().is_null());
    assert!(AVAudioMixerNode::default().is_null());
    assert!(AVSpeechSynthesisProviderRequest::default().is_null());
    assert!(AVSpeechSynthesisProviderVoice::default().is_null());
    assert!(AUParameterTree::default().is_null());
    assert!(AVAudioUnitMIDIInstrument::default().is_null());
    assert!(AVAudioConverter::default().is_null());
    assert!(AVAudioSessionCapability::default().is_null());
    assert!(AVAudioUnitComponentManager::default().is_null());
    assert!(AVAudioUnitEQ::default().is_null());
    assert!(AVAudioUnitReverb::default().is_null());
    assert!(AVAudioEnvironmentNode::default().is_null());
    assert!(AVAudioPlayerNode::default().is_null());
    assert!(AVAudioUnitDelay::default().is_null());
    assert!(AVAudioUnitDistortion::default().is_null());
    assert!(AVAudioUnitGenerator::default().is_null());
    assert!(AVAudioUnitSampler::default().is_null());
    assert!(AVAudioUnitTimePitch::default().is_null());
    let raw = std::ptr::NonNull::<u8>::dangling().as_ptr().cast();
    assert_eq!(AVAudioTime::from_raw(raw).as_raw(), raw);
}

#[test]
fn protocol_markers_are_object_safe() {
    let _: Option<&dyn AVAudioMixing> = None;
    let _: Option<&dyn AUMessageChannel> = None;
    let _: Option<&dyn AVAudio3DMixing> = None;
    let _: Option<&dyn AVAudioPlayerDelegate> = None;
    let _: Option<&dyn AVAudioRecorderDelegate> = None;
    let _: Option<&dyn AVAudioStereoMixing> = None;
    let _: Option<&dyn AVSpeechSynthesizerDelegate> = None;

    assert_eq!(
        get_audio_unit_parameter_display_type(AUDIO_UNIT_PARAMETER_FLAG_DISPLAY_MASK),
        AUDIO_UNIT_PARAMETER_FLAG_DISPLAY_MASK
    );
}
