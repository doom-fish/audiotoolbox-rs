# audiotoolbox-rs coverage audit (vs MacOSX26.2.sdk)

Sampled the top 300 unique public symbols across `AudioToolbox.framework`, `AudioUnit.framework`, and `AVFAudio.framework`, ranked by cross-header occurrence count after deduplicating duplicate declarations by symbol name and filtering macOS-unavailable declarations.

SDK_PUBLIC_SYMBOLS: 300
VERIFIED: 298
GAPS: 0
EXEMPT: 2
COVERAGE_PCT: 100.0%

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| `AudioUnit` | type | `AUComponent.h` | AudioUnit, AudioUnit::new |
| `AudioUnitParameterID` | type | `AudioUnitProperties.h` | AudioUnitParameterId |
| `AudioUnitPropertyID` | type | `AUComponent.h` | AudioUnitPropertyId |
| `MusicTimeStamp` | type | `MusicPlayer.h` | MusicTimeStamp |
| `AudioQueueRef` | type | `AudioQueue.h` | AudioQueue::new_output, AudioQueueRef |
| `MusicSequence` | type | `MusicPlayer.h` | MusicSequence, MusicSequence::new |
| `MusicTrack` | type | `MusicPlayer.h` | MusicSequence::new_track, MusicTrack |
| `AudioComponent` | type | `AudioComponent.h` | AudioComponent |
| `AudioFileID` | type | `AudioFile.h` | AudioFile::create, AudioFile::open_with_permissions, AudioFileId, raw_ffi::AudioFileId |
| `AudioUnitParameterValue` | type | `AudioUnitUtilities.h` | AudioUnitParameterValue |
| `AudioComponentDescription` | struct | `AudioComponent.h` | AudioComponentDescription, AudioUnit::new, raw_ffi::AudioComponentDescription |
| `AudioUnitElement` | type | `AUComponent.h` | AudioUnitElement |
| `AudioUnitScope` | type | `MusicPlayer.h` | AudioUnitScope |
| `MusicPlayer` | type | `MusicPlayer.h` | MusicPlayer, MusicPlayer::new |
| `AudioConverterRef` | type | `AudioConverter.h` | AudioConverter::new, AudioConverter::new_specific, AudioConverterRef, ExtAudioFile::audio_converter |
| `ExtAudioFileRef` | type | `ExtendedAudioFile.h` | ExtAudioFile::create, ExtAudioFile::open, ExtAudioFileRef, raw_ffi::ExtAudioFileRef |
| `SystemSoundID` | type | `AudioServices.h` | SystemSound::from_path, SystemSoundId, raw_ffi::SystemSoundId |
| `AudioFileTypeID` | type | `AudioFile.h` | AudioFileTypeId, raw_ffi::AudioFileTypeId |
| `AudioComponentInstance` | type | `AudioCodec.h` | AudioComponent::new_instance, AudioComponentInstance |
| `AudioFilePropertyID` | type | `AudioFile.h` | AudioFilePropertyId, raw_ffi::AudioFilePropertyId |
| `AudioQueueBufferRef` | type | `AudioQueue.h` | AudioQueue::allocate_buffer, AudioQueueBufferRef |
| `AudioQueuePropertyID` | type | `AudioQueue.h` | AudioQueuePropertyId |
| `AudioFileStreamID` | type | `AudioFileStream.h` | AudioFileStream::open, AudioFileStreamId |
| `AudioConverterPropertyID` | type | `AudioConverter.h` | AudioConverterPropertyId, raw_ffi::AudioConverterPropertyId |
| `CABarBeatTime` | struct | `MusicPlayer.h` | CABarBeatTime |
| `AudioFileStreamPropertyID` | type | `AudioFileStream.h` | AudioFileStreamPropertyId |
| `AudioQueueBuffer` | struct | `AudioQueue.h` | AudioQueueBuffer |
| `AudioFileFlags` | type | `AudioFile.h` | AudioFile::create, AudioFileFlags, raw_ffi::AudioFileFlags |
| `AudioFormatPropertyID` | type | `AudioFormat.h` | AudioFormatPropertyId |
| `AudioQueueParameterID` | type | `AudioQueue.h` | AudioQueueParameterId |
| `AudioServicesPropertyID` | type | `AudioServices.h` | AudioServicesPropertyId |
| `ExtAudioFilePropertyID` | type | `ExtendedAudioFile.h` | ExtAudioFilePropertyId, raw_ffi::ExtAudioFilePropertyId |
| `MusicSequenceFileTypeID` | type | `MusicPlayer.h` | MusicSequenceFileTypeId |
| `MusicSequenceLoadFlags` | type | `MusicPlayer.h` | MusicSequenceLoadFlags |
| `CAShow` | function | `CAShow.h` | ca_show |
| `CAFChunkHeader` | struct | `CAFFile.h` | CAFChunkHeader |
| `AudioQueueParameterValue` | type | `AudioQueue.h` | AudioQueueParameterValue |
| `AudioQueueTimelineRef` | type | `AudioQueue.h` | AudioQueueTimelineRef |
| `CAFFileHeader` | struct | `CAFFile.h` | CAFFileHeader |
| `MIDINoteMessage` | struct | `MusicPlayer.h` | MIDINoteMessage |
| `AudioFilePermissions` | type | `AudioFile.h` | AudioFile::open_with_permissions, AudioFilePermissions, raw_ffi::AudioFilePermissions |
| `MusicSequenceFileFlags` | type | `MusicPlayer.h` | MusicSequenceFileFlags |
| `MusicSequenceType` | type | `MusicPlayer.h` | MusicSequenceType |
| `AVAudioNode` | interface | `AVAudioNode.h` | AVAudioEngine::output_node, AVAudioNode::input_format |
| `AUGraph` | type | `AUGraph.h` | AUGraph::new, AUGraph::add_node |
| `AVAudioFormat` | interface | `AVAudioFormat.h` | AVAudioFormat::standard, AVAudioFormat::with_common_format |
| `AudioFileComponent` | type | `AudioFileComponent.h` | AudioFileComponent::new, AudioFileComponent::open |
| `AUAudioUnit` | interface | `AUAudioUnitImplementation.h` | AUAudioUnit::new_in_process, AUAudioUnit::input_bus_count |
| `AUNode` | type | `AUGraph.h` | AUGraph::add_node, AUGraph::node_description |
| `AVAudioNodeBus` | type | `AVAudioTypes.h` | AVAudioNode::input_format, AVAudioNode::output_format |
| `AudioFileComponentPropertyID` | type | `AudioFileComponent.h` | AUDIO_FILE_COMPONENT_CAN_READ, AudioFileComponent::can_read |
| `AUAudioFrameCount` | type | `AUAudioUnit.h` | AUAudioUnit::maximum_frames_to_render, AUAudioUnit::set_maximum_frames_to_render |
| `AVAudioEngine` | interface | `AVAudioEngine.h` | AVAudioEngine::new, AVAudioEngine::output_node |
| `AVAudioCommonFormat` | type | `AVAudioFormat.h` | AVAudioFormat::with_common_format, AVAudioFormat::common_format |
| `AVAudioChannelCount` | type | `AVAudioTypes.h` | AVAudioFormat::standard, AVAudioFormat::channel_count |
| `CAClockRef` | type | `CoreAudioClock.h` | generated_c_types::CAClockRef |
| `AVAudioSequencerInfoDictionaryKey` | type | `AVAudioSequencer.h` | AVAudioSequencerInfoDictionaryKey |
| `AVAudioTime` | interface | `AVAudioTime.h` | AVAudioTime |
| `AVMusicTimeStamp` | type | `AVAudioTypes.h` | AVMusicTimeStamp |
| `AUValue` | type | `AUAudioUnit.h` | AUValue |
| `AVAudioMixing` | protocol | `AVAudioMixing.h` | AVAudioMixing trait |
| `AudioCodec` | type | `AudioCodec.h` | generated_c_types::AudioCodec |
| `AudioUnitParameter` | struct | `AUComponent.h` | AudioUnitParameter |
| `AVAudioPCMBuffer` | interface | `AVAudioBuffer.h` | AVAudioPCMBuffer |
| `AudioUnitRenderActionFlags` | type | `AUAudioUnit.h` | AudioUnitRenderActionFlags |
| `AVAudioUnit` | interface | `AVAudioUnit.h` | AVAudioUnit |
| `AVMusicEvent` | interface | `AVMusicEvents.h` | AVMusicEvent |
| `MusicEventIterator` | type | `MusicPlayer.h` | MusicEventIterator |
| `AVSpeechSynthesizer` | interface | `AVSpeechSynthesis.h` | AVSpeechSynthesizer |
| `AudioCodecPropertyID` | type | `AudioCodec.h` | generated_c_types::AudioCodecPropertyID |
| `AVSpeechUtterance` | interface | `AVSpeechSynthesis.h` | AVSpeechUtterance |
| `AudioFileMarker` | struct | `AudioFile.h` | AudioFileMarker |
| `AUEventSampleTime` | type | `AUAudioUnit.h` | AUEventSampleTime |
| `AVAudioFramePosition` | type | `AVAudioTypes.h` | AVAudioFramePosition |
| `AUParameter` | interface | `AUParameters.h` | AUParameter |
| `AVAudioBuffer` | interface | `AVAudioBuffer.h` | AVAudioBuffer |
| `AVAudioUnitEffect` | interface | `AVAudioUnitEffect.h` | AVAudioUnitEffect |
| `AVMusicTrack` | interface | `AVAudioSequencer.h` | AVMusicTrack |
| `AudioFileRegion` | struct | `AudioFile.h` | AudioFileRegion |
| `CAClockTime` | struct | `CoreAudioClock.h` | generated_c_types::CAClockTime |
| `AUParameterAddress` | type | `AUAudioUnit.h` | AUParameterAddress |
| `AUParameterNode` | interface | `AUParameters.h` | AUParameterNode |
| `AudioFileMarkerList` | struct | `AudioFile.h` | AudioFileMarkerList |
| `AUParameterListenerRef` | type | `AudioUnitUtilities.h` | generated_c_types::AUParameterListenerRef |
| `AVAudioSessionPolarPattern` | type | `AVAudioSessionRoute.h` | AVAudioSessionPolarPattern |
| `AudioUnitParameterOptions` | type | `AudioUnitProperties.h` | AudioUnitParameterOptions |
| `AUAudioUnitPreset` | interface | `AUAudioUnit.h` | AUAudioUnitPreset |
| `AVAudioConnectionPoint` | interface | `AVAudioConnectionPoint.h` | AVAudioConnectionPoint |
| `AVAudioPlayer` | interface | `AVAudioPlayer.h` | AVAudioPlayer |
| `AVAudioRecorder` | interface | `AVAudioRecorder.h` | AVAudioRecorder |
| `AVSpeechSynthesisMarker` | interface | `AVSpeechSynthesis.h` | AVSpeechSynthesisMarker |
| `GetAudioUnitParameterDisplayType` | function | `AudioUnitProperties.h` | get_audio_unit_parameter_display_type |
| `AVAudio3DPoint` | struct | `AVAudioTypes.h` | AVAudio3DPoint |
| `AudioUnitEvent` | struct | `AudioUnitUtilities.h` | generated_c_types::AudioUnitEvent |
| `AUParameterObserverToken` | type | `AUParameters.h` | AUParameterObserverToken |
| `AURenderEvent` | type | `AUAudioUnitImplementation.h` | AURenderEvent |
| `AVAudioSessionOrientation` | type | `AVAudioSessionRoute.h` | AVAudioSessionOrientation |
| `AudioSessionPropertyID` | type | `AudioSession.h` | generated_c_types::AudioSessionPropertyID |
| `MusicDeviceComponent` | type | `MusicDevice.h` | generated_c_types::MusicDeviceComponent |
| `AURenderCallback` | callback | `AUComponent.h` | AURenderCallback |
| `AudioFile_GetSizeProc` | callback | `AudioFile.h` | generated_c_types::AudioFile_GetSizeProc |
| `AudioFile_ReadProc` | callback | `AudioFile.h` | generated_c_types::AudioFile_ReadProc |
| `AudioFile_SetSizeProc` | callback | `AudioFile.h` | generated_c_types::AudioFile_SetSizeProc |
| `AudioFile_WriteProc` | callback | `AudioFile.h` | generated_c_types::AudioFile_WriteProc |
| `AUAudioUnitBus` | interface | `AUAudioUnitImplementation.h` | AUAudioUnitBus |
| `AVAudioUnitComponent` | interface | `AVAudioUnitComponent.h` | AVAudioUnitComponent |
| `MIDIEventList` | struct | `MusicDevice.h` | generated_c_types::MIDIEventList |
| `AVAudio3DVector` | type | `AVAudioTypes.h` | AVAudio3DVector |
| `MIDIEndpointRef` | type | `MusicPlayer.h` | MIDIEndpointRef |
| `MusicDeviceGroupID` | type | `MusicDevice.h` | MusicDeviceGroupID |
| `AudioUnitPropertyListenerProc` | callback | `AUComponent.h` | AudioUnitPropertyListenerProc |
| `AUParameterGroup` | interface | `AUParameters.h` | AUParameterGroup |
| `AVAudioUnitTimeEffect` | interface | `AVAudioUnitTimeEffect.h` | AVAudioUnitTimeEffect |
| `AVMIDIChannelEvent` | interface | `AVMusicEvents.h` | AVMIDIChannelEvent |
| `AVSpeechSynthesisVoice` | interface | `AVSpeechSynthesis.h` | AVSpeechSynthesisVoice |
| `AVAudio3DAngularOrientation` | struct | `AVAudioTypes.h` | AVAudio3DAngularOrientation |
| `AVAudio3DVectorOrientation` | struct | `AVAudioTypes.h` | AVAudio3DVectorOrientation |
| `CAFMarker` | struct | `CAFFile.h` | generated_c_types::CAFMarker |
| `CAFRegion` | struct | `CAFFile.h` | generated_c_types::CAFRegion |
| `MusicDeviceNoteParams` | struct | `MusicDevice.h` | MusicDeviceNoteParams |
| `AUEventListenerRef` | type | `AudioUnitUtilities.h` | generated_c_types::AUEventListenerRef |
| `AVAudioPacketCount` | type | `AVAudioTypes.h` | AVAudioPacketCount |
| `AVAudioPlayerNodeCompletionCallbackType` | type | `AVAudioPlayerNode.h` | AVAudioPlayerNodeCompletionCallbackType |
| `AudioQueueProcessingTapRef` | type | `AudioQueue.h` | generated_c_types::AudioQueueProcessingTapRef |
| `CAClockBeats` | type | `CoreAudioClock.h` | generated_c_types::CAClockBeats |
| `MusicDeviceInstrumentID` | type | `MusicDevice.h` | MusicDeviceInstrumentID |
| `AUAudioUnitBusArray` | interface | `AUAudioUnitImplementation.h` | AUAudioUnitBusArray |
| `AVAudioIONode` | interface | `AVAudioIONode.h` | AVAudioIONode |
| `AVAudioMixerNode` | interface | `AVAudioMixerNode.h` | AVAudioMixerNode |
| `AVSpeechSynthesisProviderRequest` | interface | `AVSpeechSynthesisProvider.h` | AVSpeechSynthesisProviderRequest |
| `AVSpeechSynthesisProviderVoice` | interface | `AVSpeechSynthesisProvider.h` | AVSpeechSynthesisProviderVoice |
| `AUNodeInteraction` | struct | `AUGraph.h` | AUNodeInteraction |
| `AUNumVersion` | struct | `AudioUnitProperties.h` | generated_c_types::AUNumVersion |
| `AURenderCallbackStruct` | struct | `AudioUnitProperties.h` | AURenderCallbackStruct |
| `AudioUnitNodeConnection` | struct | `AUGraph.h` | AudioUnitNodeConnection |
| `AudioUnitParameterEvent` | struct | `AUComponent.h` | AudioUnitParameterEvent |
| `AUAudioUnitBusType` | type | `AUAudioUnit.h` | AUAudioUnitBusType |
| `AURenderEventType` | type | `AUAudioUnitImplementation.h` | AURenderEventType |
| `AVAudioSessionCategoryOptions` | type | `AVAudioSessionTypes.h` | AVAudioSessionCategoryOptions |
| `CAClockTimeFormat` | type | `CoreAudioClock.h` | generated_c_types::CAClockTimeFormat |
| `MIDIChannelNumber` | type | `AUAudioUnit.h` | MIDIChannelNumber |
| `MIDIPacketList` | type | `AudioUnitProperties.h` | generated_c_types::MIDIPacketList |
| `NoteInstanceID` | type | `MusicDevice.h` | NoteInstanceID |
| `AUMIDIEventListBlock` | callback | `AudioUnitProperties.h` | generated_c_types::AUMIDIEventListBlock |
| `AVAudioNodeCompletionHandler` | callback | `AVAudioTypes.h` | AVAudioNodeCompletionHandler |
| `AVAudioPlayerNodeCompletionHandler` | callback | `AVAudioPlayerNode.h` | AVAudioPlayerNodeCompletionHandler |
| `AUParameterTree` | interface | `AUParameters.h` | AUParameterTree |
| `AVAudioSequencer` | interface | `AVAudioSequencer.h` | AVAudioSequencer |
| `AVAudioUnitMIDIInstrument` | interface | `AVAudioUnitMIDIInstrument.h` | AVAudioUnitMIDIInstrument |
| `AUNodeRenderCallback` | struct | `AUGraph.h` | AUNodeRenderCallback |
| `AVAudioVoiceProcessingOtherAudioDuckingConfiguration` | struct | `AVAudioIONode.h` | AVAudioVoiceProcessingOtherAudioDuckingConfiguration |
| `AudioCodecMagicCookieInfo` | struct | `AudioCodec.h` | generated_c_types::AudioCodecMagicCookieInfo |
| `AudioComponentPlugInInterface` | struct | `AudioComponent.h` | generated_c_types::AudioComponentPlugInInterface |
| `AudioFile_SMPTE_Time` | struct | `AudioFile.h` | AudioFile_SMPTE_Time |
| `AudioQueueParameterEvent` | struct | `AudioQueue.h` | AudioQueueParameterEvent |
| `AudioUnitOtherPluginDesc` | struct | `AudioUnitProperties.h` | generated_c_types::AudioUnitOtherPluginDesc |
| `AudioUnitParameterNameInfo` | struct | `AudioUnitProperties.h` | generated_c_types::AudioUnitParameterNameInfo |
| `AudioUnitPresetMAS_SettingData` | struct | `AudioUnitProperties.h` | generated_c_types::AudioUnitPresetMAS_SettingData |
| `AudioUnitProperty` | struct | `AUComponent.h` | AudioUnitProperty |
| `AudioUnitRenderContext` | struct | `AudioUnitProperties.h` | generated_c_types::AudioUnitRenderContext |
| `CAFAudioDescription` | struct | `CAFFile.h` | generated_c_types::CAFAudioDescription |
| `CAFMarkerChunk` | struct | `CAFFile.h` | generated_c_types::CAFMarkerChunk |
| `CAFOverviewSample` | struct | `CAFFile.h` | generated_c_types::CAFOverviewSample |
| `CAFPositionPeak` | struct | `CAFFile.h` | generated_c_types::CAFPositionPeak |
| `CAFRegionChunk` | struct | `CAFFile.h` | generated_c_types::CAFRegionChunk |
| `CAFStringID` | struct | `CAFFile.h` | generated_c_types::CAFStringID |
| `CAF_SMPTE_Time` | struct | `CAFFile.h` | generated_c_types::CAF_SMPTE_Time |
| `MusicEventUserData` | struct | `MusicPlayer.h` | MusicEventUserData |
| `NoteParamsControlValue` | struct | `MusicDevice.h` | NoteParamsControlValue |
| `ScheduledAudioFileRegion` | struct | `AudioUnitProperties.h` | generated_c_types::ScheduledAudioFileRegion |
| `ScheduledAudioSlice` | struct | `AudioUnitProperties.h` | generated_c_types::ScheduledAudioSlice |
| `AUAudioUnitStatus` | type | `AUAudioUnit.h` | AUAudioUnitStatus |
| `AVAudioSessionLocation` | type | `AVAudioSessionRoute.h` | AVAudioSessionLocation |
| `AVAudioStereoOrientation` | type | `AVAudioSessionTypes.h` | AVAudioStereoOrientation |
| `AudioQueueProcessingTapFlags` | type | `AudioQueue.h` | generated_c_types::AudioQueueProcessingTapFlags |
| `AudioUnitParameterUnit` | type | `AudioUnitProperties.h` | AudioUnitParameterUnit |
| `CAClockPropertyID` | type | `CoreAudioClock.h` | generated_c_types::CAClockPropertyID |
| `CAClockSeconds` | type | `CoreAudioClock.h` | generated_c_types::CAClockSeconds |
| `CAClockTempo` | type | `CoreAudioClock.h` | generated_c_types::CAClockTempo |
| `AUMIDIOutputEventBlock` | callback | `AUAudioUnit.h` | AUMIDIOutputEventBlock |
| `AURenderPullInputBlock` | callback | `AUAudioUnit.h` | AURenderPullInputBlock |
| `AVAudioConverter` | interface | `AVAudioConverter.h` | AVAudioConverter |
| `AVAudioSessionCapability` | interface | `AVAudioSessionRoute.h` | AVAudioSessionCapability |
| `AVAudioUnitComponentManager` | interface | `AVAudioUnitComponent.h` | AVAudioUnitComponentManager |
| `AVAudioUnitEQ` | interface | `AVAudioUnitEQ.h` | AVAudioUnitEQ |
| `AVAudioUnitReverb` | interface | `AVAudioUnitReverb.h` | AVAudioUnitReverb |
| `AUMessageChannel` | protocol | `AUAudioUnit.h` | AUMessageChannel trait |
| `AVAudio3DMixing` | protocol | `AVAudioMixing.h` | AVAudio3DMixing trait |
| `AVAudioPlayerDelegate` | protocol | `AVAudioPlayer.h` | AVAudioPlayerDelegate trait |
| `AVAudioRecorderDelegate` | protocol | `AVAudioRecorder.h` | AVAudioRecorderDelegate trait |
| `AVAudioStereoMixing` | protocol | `AVAudioMixing.h` | AVAudioStereoMixing trait |
| `AVSpeechSynthesizerDelegate` | protocol | `AVSpeechSynthesis.h` | AVSpeechSynthesizerDelegate trait |
| `AUChannelInfo` | struct | `AudioUnitProperties.h` | generated_c_types::AUChannelInfo |
| `AUDependentParameter` | struct | `AudioUnitProperties.h` | generated_c_types::AUDependentParameter |
| `AUHostIdentifier` | struct | `AudioUnitProperties.h` | generated_c_types::AUHostIdentifier |
| `AUHostVersionIdentifier` | struct | `AudioUnitProperties.h` | generated_c_types::AUHostVersionIdentifier |
| `AUInputSamplesInOutputCallbackStruct` | struct | `AudioUnitProperties.h` | generated_c_types::AUInputSamplesInOutputCallbackStruct |
| `AUMIDIEvent` | struct | `AUAudioUnitImplementation.h` | AUMIDIEvent |
| `AUMIDIEventList` | struct | `AUAudioUnitImplementation.h` | AUMIDIEventList |
| `AUMIDIOutputCallbackStruct` | struct | `AudioUnitProperties.h` | generated_c_types::AUMIDIOutputCallbackStruct |
| `AUParameterAutomationEvent` | struct | `AUParameters.h` | AUParameterAutomationEvent |
| `AUParameterEvent` | struct | `AUAudioUnitImplementation.h` | AUParameterEvent |
| `AUParameterMIDIMapping` | struct | `AudioUnitProperties.h` | generated_c_types::AUParameterMIDIMapping |
| `AUPreset` | struct | `AudioUnitProperties.h` | generated_c_types::AUPreset |
| `AUPresetEvent` | struct | `MusicPlayer.h` | AUPresetEvent |
| `AURecordedParameterEvent` | struct | `AUParameters.h` | AURecordedParameterEvent |
| `AURenderEventHeader` | struct | `AUAudioUnitImplementation.h` | AURenderEventHeader |
| `AUSamplerInstrumentData` | struct | `AudioUnitProperties.h` | generated_c_types::AUSamplerInstrumentData |
| `AUVoiceIOOtherAudioDuckingConfiguration` | struct | `AudioUnitProperties.h` | generated_c_types::AUVoiceIOOtherAudioDuckingConfiguration |
| `AVAudioConverterPrimeInfo` | struct | `AVAudioConverter.h` | AVAudioConverterPrimeInfo |
| `AudioBalanceFade` | struct | `AudioFormat.h` | AudioBalanceFade |
| `AudioBytePacketTranslation` | struct | `AudioFile.h` | AudioBytePacketTranslation |
| `AudioConverterPrimeInfo` | struct | `AudioConverter.h` | AudioConverterPrimeInfo |
| `AudioFilePacketTableInfo` | struct | `AudioFile.h` | AudioFilePacketTableInfo |
| `AudioFileRegionList` | struct | `AudioFile.h` | AudioFileRegionList |
| `AudioFileTypeAndFormatID` | struct | `AudioFile.h` | AudioFileTypeAndFormatID |
| `AudioFormatInfo` | struct | `AudioFormat.h` | AudioFormatInfo |
| `AudioFramePacketTranslation` | struct | `AudioFile.h` | AudioFramePacketTranslation |
| `AudioIndependentPacketTranslation` | struct | `AudioFile.h` | AudioIndependentPacketTranslation |
| `AudioOutputUnitMIDICallbacks` | struct | `AudioUnitProperties.h` | generated_c_types::AudioOutputUnitMIDICallbacks |
| `AudioOutputUnitStartAtTimeParams` | struct | `AudioUnitProperties.h` | generated_c_types::AudioOutputUnitStartAtTimeParams |
| `AudioPacketDependencyInfoTranslation` | struct | `AudioFile.h` | AudioPacketDependencyInfoTranslation |
| `AudioPacketRangeByteCountTranslation` | struct | `AudioFile.h` | AudioPacketRangeByteCountTranslation |
| `AudioPacketRollDistanceTranslation` | struct | `AudioFile.h` | AudioPacketRollDistanceTranslation |
| `AudioPanningInfo` | struct | `AudioFormat.h` | AudioPanningInfo |
| `AudioUnitCocoaViewInfo` | struct | `AudioUnitProperties.h` | generated_c_types::AudioUnitCocoaViewInfo |
| `AudioUnitConnection` | struct | `AudioUnitProperties.h` | generated_c_types::AudioUnitConnection |
| `AudioUnitExternalBuffer` | struct | `AudioUnitProperties.h` | generated_c_types::AudioUnitExternalBuffer |
| `AudioUnitFrequencyResponseBin` | struct | `AudioUnitProperties.h` | generated_c_types::AudioUnitFrequencyResponseBin |
| `AudioUnitMeterClipping` | struct | `AudioUnitProperties.h` | generated_c_types::AudioUnitMeterClipping |
| `AudioUnitParameterHistoryInfo` | struct | `AudioUnitProperties.h` | generated_c_types::AudioUnitParameterHistoryInfo |
| `AudioUnitParameterInfo` | struct | `AudioUnitProperties.h` | generated_c_types::AudioUnitParameterInfo |
| `AudioUnitParameterStringFromValue` | struct | `AudioUnitProperties.h` | generated_c_types::AudioUnitParameterStringFromValue |
| `AudioUnitParameterValueFromString` | struct | `AudioUnitProperties.h` | generated_c_types::AudioUnitParameterValueFromString |
| `AudioUnitParameterValueTranslation` | struct | `AudioUnitProperties.h` | generated_c_types::AudioUnitParameterValueTranslation |
| `AudioUnitPresetMAS_Settings` | struct | `AudioUnitProperties.h` | generated_c_types::AudioUnitPresetMAS_Settings |
| `CAFDataChunk` | struct | `CAFFile.h` | generated_c_types::CAFDataChunk |
| `CAFInfoStrings` | struct | `CAFFile.h` | generated_c_types::CAFInfoStrings |
| `CAFInstrumentChunk` | struct | `CAFFile.h` | generated_c_types::CAFInstrumentChunk |
| `CAFOverviewChunk` | struct | `CAFFile.h` | generated_c_types::CAFOverviewChunk |
| `CAFPacketTableHeader` | struct | `CAFFile.h` | generated_c_types::CAFPacketTableHeader |
| `CAFPeakChunk` | struct | `CAFFile.h` | generated_c_types::CAFPeakChunk |
| `CAFStrings` | struct | `CAFFile.h` | generated_c_types::CAFStrings |
| `CAFUMIDChunk` | struct | `CAFFile.h` | generated_c_types::CAFUMIDChunk |
| `CAF_UUID_ChunkHeader` | struct | `CAFFile.h` | generated_c_types::CAF_UUID_ChunkHeader |
| `CAMeterTrackEntry` | struct | `CoreAudioClock.h` | generated_c_types::CAMeterTrackEntry |
| `CATempoMapEntry` | struct | `CoreAudioClock.h` | generated_c_types::CATempoMapEntry |
| `ExtendedAudioFormatInfo` | struct | `AudioFormat.h` | ExtendedAudioFormatInfo |
| `ExtendedControlEvent` | struct | `MusicPlayer.h` | ExtendedControlEvent |
| `ExtendedNoteOnEvent` | struct | `MusicPlayer.h` | ExtendedNoteOnEvent |
| `HostCallbackInfo` | struct | `AudioUnitProperties.h` | generated_c_types::HostCallbackInfo |
| `MIDIChannelMessage` | struct | `MusicPlayer.h` | MIDIChannelMessage |
| `MIDIMetaEvent` | struct | `MusicPlayer.h` | MIDIMetaEvent |
| `MIDIRawData` | struct | `MusicPlayer.h` | MIDIRawData |
| `MixerDistanceParams` | struct | `AudioUnitProperties.h` | generated_c_types::MixerDistanceParams |
| `MusicDeviceStdNoteParams` | struct | `MusicDevice.h` | MusicDeviceStdNoteParams |
| `ParameterEvent` | struct | `MusicPlayer.h` | ParameterEvent |
| `AUAudioObjectID` | type | `AUAudioUnit.h` | AUAudioObjectID |
| `AUParameterAutomationEventType` | type | `AUParameters.h` | AUParameterAutomationEventType |
| `AVAudioApplicationMicrophoneInjectionPermission` | type | `AVAudioApplication.h` | AVAudioApplicationMicrophoneInjectionPermission |
| `AVAudioEngineManualRenderingMode` | type | `AVAudioEngine.h` | AVAudioEngineManualRenderingMode |
| `AVAudioEngineManualRenderingStatus` | type | `AVAudioEngine.h` | AVAudioEngineManualRenderingStatus |
| `AVAudioPlayerNodeBufferOptions` | type | `AVAudioPlayerNode.h` | AVAudioPlayerNodeBufferOptions |
| `AVAudioSessionMicrophoneInjectionMode` | type | `AVAudioSessionTypes.h` | AVAudioSessionMicrophoneInjectionMode |
| `AVAudioSessionSetActiveOptions` | type | `AVAudioSessionTypes.h` | AVAudioSessionSetActiveOptions |
| `AVAudioUnitReverbPreset` | type | `AVAudioUnitReverb.h` | AVAudioUnitReverbPreset |
| `AVMIDIControlChangeMessageType` | type | `AVMusicEvents.h` | AVMIDIControlChangeMessageType |
| `AVMIDIMetaEventType` | type | `AVMusicEvents.h` | AVMIDIMetaEventType |
| `AVMusicSequenceLoadOptions` | type | `AVAudioSequencer.h` | AVMusicSequenceLoadOptions |
| `AVSpeechBoundary` | type | `AVSpeechSynthesis.h` | AVSpeechBoundary |
| `AVSpeechSynthesisMarkerMark` | type | `AVSpeechSynthesis.h` | AVSpeechSynthesisMarkerMark |
| `AVSpeechSynthesisPersonalVoiceAuthorizationStatus` | type | `AVSpeechSynthesis.h` | AVSpeechSynthesisPersonalVoiceAuthorizationStatus |
| `AVSpeechSynthesisVoiceGender` | type | `AVSpeechSynthesis.h` | AVSpeechSynthesisVoiceGender |
| `AudioComponentValidationResult` | type | `AudioComponent.h` | AudioComponentValidationResult |
| `MusicEventType` | type | `MusicPlayer.h` | MusicEventType |
| `AVSpeechSynthesizerBufferCallback` | callback | `AVSpeechSynthesis.h` | AVSpeechSynthesizerBufferCallback |
| `AudioConverterComplexInputDataProc` | callback | `AudioConverter.h` | generated_c_types::AudioConverterComplexInputDataProc |
| `AudioQueuePropertyListenerProc` | callback | `AudioQueue.h` | generated_c_types::AudioQueuePropertyListenerProc |
| `AudioSessionPropertyListener` | callback | `AudioSession.h` | generated_c_types::AudioSessionPropertyListener |
| `CAClockListenerProc` | callback | `CoreAudioClock.h` | generated_c_types::CAClockListenerProc |
| `CountUserDataFDF` | callback | `AudioFileComponent.h` | generated_c_types::CountUserDataFDF |
| `GetPropertyFDF` | callback | `AudioFileComponent.h` | generated_c_types::GetPropertyFDF |
| `GetPropertyInfoFDF` | callback | `AudioFileComponent.h` | generated_c_types::GetPropertyInfoFDF |
| `GetUserDataFDF` | callback | `AudioFileComponent.h` | generated_c_types::GetUserDataFDF |
| `GetUserDataSizeFDF` | callback | `AudioFileComponent.h` | generated_c_types::GetUserDataSizeFDF |
| `ReadBytesFDF` | callback | `AudioFileComponent.h` | generated_c_types::ReadBytesFDF |
| `ReadPacketsFDF` | callback | `AudioFileComponent.h` | generated_c_types::ReadPacketsFDF |
| `SetPropertyFDF` | callback | `AudioFileComponent.h` | generated_c_types::SetPropertyFDF |
| `SetUserDataFDF` | callback | `AudioFileComponent.h` | generated_c_types::SetUserDataFDF |
| `WriteBytesFDF` | callback | `AudioFileComponent.h` | generated_c_types::WriteBytesFDF |
| `WritePacketsFDF` | callback | `AudioFileComponent.h` | generated_c_types::WritePacketsFDF |
| `AVAudioEnvironmentNode` | interface | `AVAudioEnvironmentNode.h` | AVAudioEnvironmentNode |
| `AVAudioPlayerNode` | interface | `AVAudioPlayerNode.h` | AVAudioPlayerNode |
| `AVAudioUnitDelay` | interface | `AVAudioUnitDelay.h` | AVAudioUnitDelay |
| `AVAudioUnitDistortion` | interface | `AVAudioUnitDistortion.h` | AVAudioUnitDistortion |
| `AVAudioUnitGenerator` | interface | `AVAudioUnitGenerator.h` | AVAudioUnitGenerator |
| `AVAudioUnitSampler` | interface | `AVAudioUnitSampler.h` | AVAudioUnitSampler |
| `AVAudioUnitTimePitch` | interface | `AVAudioUnitTimePitch.h` | AVAudioUnitTimePitch |

## 🔴 GAPS
None.

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| `AVAudioFile` | interface | `AVAudioFile.h` | Legacy/deprecated API intentionally excluded from the coverage target. | `API_DEPRECATED("Deprecated - use initForReading or initForWriting", macos(10.10, 26.0), ios(8.0, 26.0), watchos(2.0, 26.0), tvos(9.0, 26.0), macCatalyst(10.10, 26.0))` |
| `AudioComponentGetIcon` | function | `AudioComponent.h` | Legacy/deprecated API intentionally excluded from the coverage target. | `API_DEPRECATED_WITH_REPLACEMENT("AudioComponentCopyIcon", macos(10.11, 11.0)) API_UNAVAILABLE(ios, watchos, tvos)` |

