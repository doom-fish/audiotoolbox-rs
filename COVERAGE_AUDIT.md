# audiotoolbox-rs coverage audit (vs MacOSX26.2.sdk)

Sampled the top 300 unique public symbols across `AudioToolbox.framework`, `AudioUnit.framework`, and `AVFAudio.framework`, ranked by cross-header occurrence count after deduplicating duplicate declarations by symbol name and filtering macOS-unavailable declarations.

SDK_PUBLIC_SYMBOLS: 300
VERIFIED: 55
GAPS: 243
EXEMPT: 2
COVERAGE_PCT: 18.3%

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

## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |
| `CAClockRef` | type | `CoreAudioClock.h` | CoreAudio clock APIs are not wrapped. |
| `AVAudioSequencerInfoDictionaryKey` | type | `AVAudioSequencer.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AVAudioTime` | interface | `AVAudioTime.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AVMusicTimeStamp` | type | `AVAudioTypes.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AUValue` | type | `AUAudioUnit.h` | No AUAudioUnit / parameter-tree surface is currently wrapped. |
| `AVAudioMixing` | protocol | `AVAudioMixing.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AudioCodec` | type | `AudioCodec.h` | Codec plug-in APIs are not wrapped. |
| `AudioUnitParameter` | struct | `AUComponent.h` | Advanced AudioUnit property/callback/render surfaces are not wrapped. |
| `AVAudioPCMBuffer` | interface | `AVAudioBuffer.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AudioUnitRenderActionFlags` | type | `AUAudioUnit.h` | No AUAudioUnit / parameter-tree surface is currently wrapped. |
| `AVAudioUnit` | interface | `AVAudioUnit.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AVMusicEvent` | interface | `AVMusicEvents.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `MusicEventIterator` | type | `MusicPlayer.h` | Sequence editing, event iteration, or music-device surfaces remain unwrapped. |
| `AVSpeechSynthesizer` | interface | `AVSpeechSynthesis.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AudioCodecPropertyID` | type | `AudioCodec.h` | Codec plug-in APIs are not wrapped. |
| `AVSpeechUtterance` | interface | `AVSpeechSynthesis.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AudioFileMarker` | struct | `AudioFile.h` | Marker/region metadata and callback-style file APIs are not wrapped. |
| `AUEventSampleTime` | type | `AUAudioUnit.h` | No AUAudioUnit / parameter-tree surface is currently wrapped. |
| `AVAudioFramePosition` | type | `AVAudioTypes.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AUParameter` | interface | `AUParameters.h` | No AUAudioUnit / parameter-tree surface is currently wrapped. |
| `AVAudioBuffer` | interface | `AVAudioBuffer.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AVAudioUnitEffect` | interface | `AVAudioUnitEffect.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AVMusicTrack` | interface | `AVAudioSequencer.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AudioFileRegion` | struct | `AudioFile.h` | Marker/region metadata and callback-style file APIs are not wrapped. |
| `CAClockTime` | struct | `CoreAudioClock.h` | CoreAudio clock APIs are not wrapped. |
| `AUParameterAddress` | type | `AUAudioUnit.h` | No AUAudioUnit / parameter-tree surface is currently wrapped. |
| `AUParameterNode` | interface | `AUParameters.h` | No AUAudioUnit / parameter-tree surface is currently wrapped. |
| `AudioFileMarkerList` | struct | `AudioFile.h` | Marker/region metadata and callback-style file APIs are not wrapped. |
| `AUParameterListenerRef` | type | `AudioUnitUtilities.h` | Advanced AudioUnit property/callback/render surfaces are not wrapped. |
| `AVAudioSessionPolarPattern` | type | `AVAudioSessionRoute.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AudioUnitParameterOptions` | type | `AudioUnitProperties.h` | Advanced AudioUnit property/callback/render surfaces are not wrapped. |
| `AUAudioUnitPreset` | interface | `AUAudioUnit.h` | No AUAudioUnit / parameter-tree surface is currently wrapped. |
| `AVAudioConnectionPoint` | interface | `AVAudioConnectionPoint.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AVAudioPlayer` | interface | `AVAudioPlayer.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AVAudioRecorder` | interface | `AVAudioRecorder.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AVSpeechSynthesisMarker` | interface | `AVSpeechSynthesis.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `GetAudioUnitParameterDisplayType` | function | `AudioUnitProperties.h` | Advanced AudioUnit property/callback/render surfaces are not wrapped. |
| `AVAudio3DPoint` | struct | `AVAudioTypes.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AudioUnitEvent` | struct | `AudioUnitUtilities.h` | Advanced AudioUnit property/callback/render surfaces are not wrapped. |
| `AUParameterObserverToken` | type | `AUParameters.h` | No AUAudioUnit / parameter-tree surface is currently wrapped. |
| `AURenderEvent` | type | `AUAudioUnitImplementation.h` | No AUAudioUnit / parameter-tree surface is currently wrapped. |
| `AVAudioSessionOrientation` | type | `AVAudioSessionRoute.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AudioSessionPropertyID` | type | `AudioSession.h` | Not currently reachable from the crate's public API. |
| `MusicDeviceComponent` | type | `MusicDevice.h` | Sequence editing, event iteration, or music-device surfaces remain unwrapped. |
| `AURenderCallback` | callback | `AUComponent.h` | Advanced AudioUnit property/callback/render surfaces are not wrapped. |
| `AudioFile_GetSizeProc` | callback | `AudioFile.h` | Marker/region metadata and callback-style file APIs are not wrapped. |
| `AudioFile_ReadProc` | callback | `AudioFile.h` | Marker/region metadata and callback-style file APIs are not wrapped. |
| `AudioFile_SetSizeProc` | callback | `AudioFile.h` | Marker/region metadata and callback-style file APIs are not wrapped. |
| `AudioFile_WriteProc` | callback | `AudioFile.h` | Marker/region metadata and callback-style file APIs are not wrapped. |
| `AUAudioUnitBus` | interface | `AUAudioUnitImplementation.h` | No AUAudioUnit / parameter-tree surface is currently wrapped. |
| `AVAudioUnitComponent` | interface | `AVAudioUnitComponent.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `MIDIEventList` | struct | `MusicDevice.h` | Sequence editing, event iteration, or music-device surfaces remain unwrapped. |
| `AVAudio3DVector` | type | `AVAudioTypes.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `MIDIEndpointRef` | type | `MusicPlayer.h` | Sequence editing, event iteration, or music-device surfaces remain unwrapped. |
| `MusicDeviceGroupID` | type | `MusicDevice.h` | Sequence editing, event iteration, or music-device surfaces remain unwrapped. |
| `AudioUnitPropertyListenerProc` | callback | `AUComponent.h` | Advanced AudioUnit property/callback/render surfaces are not wrapped. |
| `AUParameterGroup` | interface | `AUParameters.h` | No AUAudioUnit / parameter-tree surface is currently wrapped. |
| `AVAudioUnitTimeEffect` | interface | `AVAudioUnitTimeEffect.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AVMIDIChannelEvent` | interface | `AVMusicEvents.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AVSpeechSynthesisVoice` | interface | `AVSpeechSynthesis.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AVAudio3DAngularOrientation` | struct | `AVAudioTypes.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AVAudio3DVectorOrientation` | struct | `AVAudioTypes.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `CAFMarker` | struct | `CAFFile.h` | Not currently reachable from the crate's public API. |
| `CAFRegion` | struct | `CAFFile.h` | Not currently reachable from the crate's public API. |
| `MusicDeviceNoteParams` | struct | `MusicDevice.h` | Sequence editing, event iteration, or music-device surfaces remain unwrapped. |
| `AUEventListenerRef` | type | `AudioUnitUtilities.h` | Advanced AudioUnit property/callback/render surfaces are not wrapped. |
| `AVAudioPacketCount` | type | `AVAudioTypes.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AVAudioPlayerNodeCompletionCallbackType` | type | `AVAudioPlayerNode.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AudioQueueProcessingTapRef` | type | `AudioQueue.h` | Advanced queue metadata/callback surfaces are not wrapped. |
| `CAClockBeats` | type | `CoreAudioClock.h` | CoreAudio clock APIs are not wrapped. |
| `MusicDeviceInstrumentID` | type | `MusicDevice.h` | Sequence editing, event iteration, or music-device surfaces remain unwrapped. |
| `AUAudioUnitBusArray` | interface | `AUAudioUnitImplementation.h` | No AUAudioUnit / parameter-tree surface is currently wrapped. |
| `AVAudioIONode` | interface | `AVAudioIONode.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AVAudioMixerNode` | interface | `AVAudioMixerNode.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AVSpeechSynthesisProviderRequest` | interface | `AVSpeechSynthesisProvider.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AVSpeechSynthesisProviderVoice` | interface | `AVSpeechSynthesisProvider.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AUNodeInteraction` | struct | `AUGraph.h` | Legacy AUGraph graph-management APIs are not wrapped. |
| `AUNumVersion` | struct | `AudioUnitProperties.h` | Advanced AudioUnit property/callback/render surfaces are not wrapped. |
| `AURenderCallbackStruct` | struct | `AudioUnitProperties.h` | Advanced AudioUnit property/callback/render surfaces are not wrapped. |
| `AudioUnitNodeConnection` | struct | `AUGraph.h` | Legacy AUGraph graph-management APIs are not wrapped. |
| `AudioUnitParameterEvent` | struct | `AUComponent.h` | Advanced AudioUnit property/callback/render surfaces are not wrapped. |
| `AUAudioUnitBusType` | type | `AUAudioUnit.h` | No AUAudioUnit / parameter-tree surface is currently wrapped. |
| `AURenderEventType` | type | `AUAudioUnitImplementation.h` | No AUAudioUnit / parameter-tree surface is currently wrapped. |
| `AVAudioSessionCategoryOptions` | type | `AVAudioSessionTypes.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `CAClockTimeFormat` | type | `CoreAudioClock.h` | CoreAudio clock APIs are not wrapped. |
| `MIDIChannelNumber` | type | `AUAudioUnit.h` | No AUAudioUnit / parameter-tree surface is currently wrapped. |
| `MIDIPacketList` | type | `AudioUnitProperties.h` | Advanced AudioUnit property/callback/render surfaces are not wrapped. |
| `NoteInstanceID` | type | `MusicDevice.h` | Sequence editing, event iteration, or music-device surfaces remain unwrapped. |
| `AUMIDIEventListBlock` | callback | `AudioUnitProperties.h` | Advanced AudioUnit property/callback/render surfaces are not wrapped. |
| `AVAudioNodeCompletionHandler` | callback | `AVAudioTypes.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AVAudioPlayerNodeCompletionHandler` | callback | `AVAudioPlayerNode.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AUParameterTree` | interface | `AUParameters.h` | No AUAudioUnit / parameter-tree surface is currently wrapped. |
| `AVAudioSequencer` | interface | `AVAudioSequencer.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AVAudioUnitMIDIInstrument` | interface | `AVAudioUnitMIDIInstrument.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AUNodeRenderCallback` | struct | `AUGraph.h` | Legacy AUGraph graph-management APIs are not wrapped. |
| `AVAudioVoiceProcessingOtherAudioDuckingConfiguration` | struct | `AVAudioIONode.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AudioCodecMagicCookieInfo` | struct | `AudioCodec.h` | Codec plug-in APIs are not wrapped. |
| `AudioComponentPlugInInterface` | struct | `AudioComponent.h` | Not currently reachable from the crate's public API. |
| `AudioFile_SMPTE_Time` | struct | `AudioFile.h` | Marker/region metadata and callback-style file APIs are not wrapped. |
| `AudioQueueParameterEvent` | struct | `AudioQueue.h` | Advanced queue metadata/callback surfaces are not wrapped. |
| `AudioUnitOtherPluginDesc` | struct | `AudioUnitProperties.h` | Advanced AudioUnit property/callback/render surfaces are not wrapped. |
| `AudioUnitParameterNameInfo` | struct | `AudioUnitProperties.h` | Advanced AudioUnit property/callback/render surfaces are not wrapped. |
| `AudioUnitPresetMAS_SettingData` | struct | `AudioUnitProperties.h` | Advanced AudioUnit property/callback/render surfaces are not wrapped. |
| `AudioUnitProperty` | struct | `AUComponent.h` | Advanced AudioUnit property/callback/render surfaces are not wrapped. |
| `AudioUnitRenderContext` | struct | `AudioUnitProperties.h` | Advanced AudioUnit property/callback/render surfaces are not wrapped. |
| `CAFAudioDescription` | struct | `CAFFile.h` | Not currently reachable from the crate's public API. |
| `CAFMarkerChunk` | struct | `CAFFile.h` | Not currently reachable from the crate's public API. |
| `CAFOverviewSample` | struct | `CAFFile.h` | Not currently reachable from the crate's public API. |
| `CAFPositionPeak` | struct | `CAFFile.h` | Not currently reachable from the crate's public API. |
| `CAFRegionChunk` | struct | `CAFFile.h` | Not currently reachable from the crate's public API. |
| `CAFStringID` | struct | `CAFFile.h` | Not currently reachable from the crate's public API. |
| `CAF_SMPTE_Time` | struct | `CAFFile.h` | Not currently reachable from the crate's public API. |
| `MusicEventUserData` | struct | `MusicPlayer.h` | Sequence editing, event iteration, or music-device surfaces remain unwrapped. |
| `NoteParamsControlValue` | struct | `MusicDevice.h` | Sequence editing, event iteration, or music-device surfaces remain unwrapped. |
| `ScheduledAudioFileRegion` | struct | `AudioUnitProperties.h` | Advanced AudioUnit property/callback/render surfaces are not wrapped. |
| `ScheduledAudioSlice` | struct | `AudioUnitProperties.h` | Advanced AudioUnit property/callback/render surfaces are not wrapped. |
| `AUAudioUnitStatus` | type | `AUAudioUnit.h` | No AUAudioUnit / parameter-tree surface is currently wrapped. |
| `AVAudioSessionLocation` | type | `AVAudioSessionRoute.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AVAudioStereoOrientation` | type | `AVAudioSessionTypes.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AudioQueueProcessingTapFlags` | type | `AudioQueue.h` | Advanced queue metadata/callback surfaces are not wrapped. |
| `AudioUnitParameterUnit` | type | `AudioUnitProperties.h` | Advanced AudioUnit property/callback/render surfaces are not wrapped. |
| `CAClockPropertyID` | type | `CoreAudioClock.h` | CoreAudio clock APIs are not wrapped. |
| `CAClockSeconds` | type | `CoreAudioClock.h` | CoreAudio clock APIs are not wrapped. |
| `CAClockTempo` | type | `CoreAudioClock.h` | CoreAudio clock APIs are not wrapped. |
| `AUMIDIOutputEventBlock` | callback | `AUAudioUnit.h` | No AUAudioUnit / parameter-tree surface is currently wrapped. |
| `AURenderPullInputBlock` | callback | `AUAudioUnit.h` | No AUAudioUnit / parameter-tree surface is currently wrapped. |
| `AVAudioConverter` | interface | `AVAudioConverter.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AVAudioSessionCapability` | interface | `AVAudioSessionRoute.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AVAudioUnitComponentManager` | interface | `AVAudioUnitComponent.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AVAudioUnitEQ` | interface | `AVAudioUnitEQ.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AVAudioUnitReverb` | interface | `AVAudioUnitReverb.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AUMessageChannel` | protocol | `AUAudioUnit.h` | No AUAudioUnit / parameter-tree surface is currently wrapped. |
| `AVAudio3DMixing` | protocol | `AVAudioMixing.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AVAudioPlayerDelegate` | protocol | `AVAudioPlayer.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AVAudioRecorderDelegate` | protocol | `AVAudioRecorder.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AVAudioStereoMixing` | protocol | `AVAudioMixing.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AVSpeechSynthesizerDelegate` | protocol | `AVSpeechSynthesis.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AUChannelInfo` | struct | `AudioUnitProperties.h` | Advanced AudioUnit property/callback/render surfaces are not wrapped. |
| `AUDependentParameter` | struct | `AudioUnitProperties.h` | Advanced AudioUnit property/callback/render surfaces are not wrapped. |
| `AUHostIdentifier` | struct | `AudioUnitProperties.h` | Advanced AudioUnit property/callback/render surfaces are not wrapped. |
| `AUHostVersionIdentifier` | struct | `AudioUnitProperties.h` | Advanced AudioUnit property/callback/render surfaces are not wrapped. |
| `AUInputSamplesInOutputCallbackStruct` | struct | `AudioUnitProperties.h` | Advanced AudioUnit property/callback/render surfaces are not wrapped. |
| `AUMIDIEvent` | struct | `AUAudioUnitImplementation.h` | No AUAudioUnit / parameter-tree surface is currently wrapped. |
| `AUMIDIEventList` | struct | `AUAudioUnitImplementation.h` | No AUAudioUnit / parameter-tree surface is currently wrapped. |
| `AUMIDIOutputCallbackStruct` | struct | `AudioUnitProperties.h` | Advanced AudioUnit property/callback/render surfaces are not wrapped. |
| `AUParameterAutomationEvent` | struct | `AUParameters.h` | No AUAudioUnit / parameter-tree surface is currently wrapped. |
| `AUParameterEvent` | struct | `AUAudioUnitImplementation.h` | No AUAudioUnit / parameter-tree surface is currently wrapped. |
| `AUParameterMIDIMapping` | struct | `AudioUnitProperties.h` | Advanced AudioUnit property/callback/render surfaces are not wrapped. |
| `AUPreset` | struct | `AudioUnitProperties.h` | Advanced AudioUnit property/callback/render surfaces are not wrapped. |
| `AUPresetEvent` | struct | `MusicPlayer.h` | Sequence editing, event iteration, or music-device surfaces remain unwrapped. |
| `AURecordedParameterEvent` | struct | `AUParameters.h` | No AUAudioUnit / parameter-tree surface is currently wrapped. |
| `AURenderEventHeader` | struct | `AUAudioUnitImplementation.h` | No AUAudioUnit / parameter-tree surface is currently wrapped. |
| `AUSamplerInstrumentData` | struct | `AudioUnitProperties.h` | Advanced AudioUnit property/callback/render surfaces are not wrapped. |
| `AUVoiceIOOtherAudioDuckingConfiguration` | struct | `AudioUnitProperties.h` | Advanced AudioUnit property/callback/render surfaces are not wrapped. |
| `AVAudioConverterPrimeInfo` | struct | `AVAudioConverter.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AudioBalanceFade` | struct | `AudioFormat.h` | Not currently reachable from the crate's public API. |
| `AudioBytePacketTranslation` | struct | `AudioFile.h` | Marker/region metadata and callback-style file APIs are not wrapped. |
| `AudioConverterPrimeInfo` | struct | `AudioConverter.h` | Not currently reachable from the crate's public API. |
| `AudioFilePacketTableInfo` | struct | `AudioFile.h` | Marker/region metadata and callback-style file APIs are not wrapped. |
| `AudioFileRegionList` | struct | `AudioFile.h` | Marker/region metadata and callback-style file APIs are not wrapped. |
| `AudioFileTypeAndFormatID` | struct | `AudioFile.h` | Marker/region metadata and callback-style file APIs are not wrapped. |
| `AudioFormatInfo` | struct | `AudioFormat.h` | Not currently reachable from the crate's public API. |
| `AudioFramePacketTranslation` | struct | `AudioFile.h` | Marker/region metadata and callback-style file APIs are not wrapped. |
| `AudioIndependentPacketTranslation` | struct | `AudioFile.h` | Marker/region metadata and callback-style file APIs are not wrapped. |
| `AudioOutputUnitMIDICallbacks` | struct | `AudioUnitProperties.h` | Advanced AudioUnit property/callback/render surfaces are not wrapped. |
| `AudioOutputUnitStartAtTimeParams` | struct | `AudioUnitProperties.h` | Advanced AudioUnit property/callback/render surfaces are not wrapped. |
| `AudioPacketDependencyInfoTranslation` | struct | `AudioFile.h` | Marker/region metadata and callback-style file APIs are not wrapped. |
| `AudioPacketRangeByteCountTranslation` | struct | `AudioFile.h` | Marker/region metadata and callback-style file APIs are not wrapped. |
| `AudioPacketRollDistanceTranslation` | struct | `AudioFile.h` | Marker/region metadata and callback-style file APIs are not wrapped. |
| `AudioPanningInfo` | struct | `AudioFormat.h` | Not currently reachable from the crate's public API. |
| `AudioUnitCocoaViewInfo` | struct | `AudioUnitProperties.h` | Advanced AudioUnit property/callback/render surfaces are not wrapped. |
| `AudioUnitConnection` | struct | `AudioUnitProperties.h` | Advanced AudioUnit property/callback/render surfaces are not wrapped. |
| `AudioUnitExternalBuffer` | struct | `AudioUnitProperties.h` | Advanced AudioUnit property/callback/render surfaces are not wrapped. |
| `AudioUnitFrequencyResponseBin` | struct | `AudioUnitProperties.h` | Advanced AudioUnit property/callback/render surfaces are not wrapped. |
| `AudioUnitMeterClipping` | struct | `AudioUnitProperties.h` | Advanced AudioUnit property/callback/render surfaces are not wrapped. |
| `AudioUnitParameterHistoryInfo` | struct | `AudioUnitProperties.h` | Advanced AudioUnit property/callback/render surfaces are not wrapped. |
| `AudioUnitParameterInfo` | struct | `AudioUnitProperties.h` | Advanced AudioUnit property/callback/render surfaces are not wrapped. |
| `AudioUnitParameterStringFromValue` | struct | `AudioUnitProperties.h` | Advanced AudioUnit property/callback/render surfaces are not wrapped. |
| `AudioUnitParameterValueFromString` | struct | `AudioUnitProperties.h` | Advanced AudioUnit property/callback/render surfaces are not wrapped. |
| `AudioUnitParameterValueTranslation` | struct | `AudioUnitProperties.h` | Advanced AudioUnit property/callback/render surfaces are not wrapped. |
| `AudioUnitPresetMAS_Settings` | struct | `AudioUnitProperties.h` | Advanced AudioUnit property/callback/render surfaces are not wrapped. |
| `CAFDataChunk` | struct | `CAFFile.h` | Not currently reachable from the crate's public API. |
| `CAFInfoStrings` | struct | `CAFFile.h` | Not currently reachable from the crate's public API. |
| `CAFInstrumentChunk` | struct | `CAFFile.h` | Not currently reachable from the crate's public API. |
| `CAFOverviewChunk` | struct | `CAFFile.h` | Not currently reachable from the crate's public API. |
| `CAFPacketTableHeader` | struct | `CAFFile.h` | Not currently reachable from the crate's public API. |
| `CAFPeakChunk` | struct | `CAFFile.h` | Not currently reachable from the crate's public API. |
| `CAFStrings` | struct | `CAFFile.h` | Not currently reachable from the crate's public API. |
| `CAFUMIDChunk` | struct | `CAFFile.h` | Not currently reachable from the crate's public API. |
| `CAF_UUID_ChunkHeader` | struct | `CAFFile.h` | Not currently reachable from the crate's public API. |
| `CAMeterTrackEntry` | struct | `CoreAudioClock.h` | CoreAudio clock APIs are not wrapped. |
| `CATempoMapEntry` | struct | `CoreAudioClock.h` | CoreAudio clock APIs are not wrapped. |
| `ExtendedAudioFormatInfo` | struct | `AudioFormat.h` | Not currently reachable from the crate's public API. |
| `ExtendedControlEvent` | struct | `MusicPlayer.h` | Sequence editing, event iteration, or music-device surfaces remain unwrapped. |
| `ExtendedNoteOnEvent` | struct | `MusicPlayer.h` | Sequence editing, event iteration, or music-device surfaces remain unwrapped. |
| `HostCallbackInfo` | struct | `AudioUnitProperties.h` | Advanced AudioUnit property/callback/render surfaces are not wrapped. |
| `MIDIChannelMessage` | struct | `MusicPlayer.h` | Sequence editing, event iteration, or music-device surfaces remain unwrapped. |
| `MIDIMetaEvent` | struct | `MusicPlayer.h` | Sequence editing, event iteration, or music-device surfaces remain unwrapped. |
| `MIDIRawData` | struct | `MusicPlayer.h` | Sequence editing, event iteration, or music-device surfaces remain unwrapped. |
| `MixerDistanceParams` | struct | `AudioUnitProperties.h` | Advanced AudioUnit property/callback/render surfaces are not wrapped. |
| `MusicDeviceStdNoteParams` | struct | `MusicDevice.h` | Sequence editing, event iteration, or music-device surfaces remain unwrapped. |
| `ParameterEvent` | struct | `MusicPlayer.h` | Sequence editing, event iteration, or music-device surfaces remain unwrapped. |
| `AUAudioObjectID` | type | `AUAudioUnit.h` | No AUAudioUnit / parameter-tree surface is currently wrapped. |
| `AUParameterAutomationEventType` | type | `AUParameters.h` | No AUAudioUnit / parameter-tree surface is currently wrapped. |
| `AVAudioApplicationMicrophoneInjectionPermission` | type | `AVAudioApplication.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AVAudioEngineManualRenderingMode` | type | `AVAudioEngine.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AVAudioEngineManualRenderingStatus` | type | `AVAudioEngine.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AVAudioPlayerNodeBufferOptions` | type | `AVAudioPlayerNode.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AVAudioSessionMicrophoneInjectionMode` | type | `AVAudioSessionTypes.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AVAudioSessionSetActiveOptions` | type | `AVAudioSessionTypes.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AVAudioUnitReverbPreset` | type | `AVAudioUnitReverb.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AVMIDIControlChangeMessageType` | type | `AVMusicEvents.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AVMIDIMetaEventType` | type | `AVMusicEvents.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AVMusicSequenceLoadOptions` | type | `AVAudioSequencer.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AVSpeechBoundary` | type | `AVSpeechSynthesis.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AVSpeechSynthesisMarkerMark` | type | `AVSpeechSynthesis.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AVSpeechSynthesisPersonalVoiceAuthorizationStatus` | type | `AVSpeechSynthesis.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AVSpeechSynthesisVoiceGender` | type | `AVSpeechSynthesis.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AudioComponentValidationResult` | type | `AudioComponent.h` | Not currently reachable from the crate's public API. |
| `MusicEventType` | type | `MusicPlayer.h` | Sequence editing, event iteration, or music-device surfaces remain unwrapped. |
| `AVSpeechSynthesizerBufferCallback` | callback | `AVSpeechSynthesis.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AudioConverterComplexInputDataProc` | callback | `AudioConverter.h` | Not currently reachable from the crate's public API. |
| `AudioQueuePropertyListenerProc` | callback | `AudioQueue.h` | Advanced queue metadata/callback surfaces are not wrapped. |
| `AudioSessionPropertyListener` | callback | `AudioSession.h` | Not currently reachable from the crate's public API. |
| `CAClockListenerProc` | callback | `CoreAudioClock.h` | CoreAudio clock APIs are not wrapped. |
| `CountUserDataFDF` | callback | `AudioFileComponent.h` | AudioFile component plug-in interfaces are not wrapped. |
| `GetPropertyFDF` | callback | `AudioFileComponent.h` | AudioFile component plug-in interfaces are not wrapped. |
| `GetPropertyInfoFDF` | callback | `AudioFileComponent.h` | AudioFile component plug-in interfaces are not wrapped. |
| `GetUserDataFDF` | callback | `AudioFileComponent.h` | AudioFile component plug-in interfaces are not wrapped. |
| `GetUserDataSizeFDF` | callback | `AudioFileComponent.h` | AudioFile component plug-in interfaces are not wrapped. |
| `ReadBytesFDF` | callback | `AudioFileComponent.h` | AudioFile component plug-in interfaces are not wrapped. |
| `ReadPacketsFDF` | callback | `AudioFileComponent.h` | AudioFile component plug-in interfaces are not wrapped. |
| `SetPropertyFDF` | callback | `AudioFileComponent.h` | AudioFile component plug-in interfaces are not wrapped. |
| `SetUserDataFDF` | callback | `AudioFileComponent.h` | AudioFile component plug-in interfaces are not wrapped. |
| `WriteBytesFDF` | callback | `AudioFileComponent.h` | AudioFile component plug-in interfaces are not wrapped. |
| `WritePacketsFDF` | callback | `AudioFileComponent.h` | AudioFile component plug-in interfaces are not wrapped. |
| `AVAudioEnvironmentNode` | interface | `AVAudioEnvironmentNode.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AVAudioPlayerNode` | interface | `AVAudioPlayerNode.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AVAudioUnitDelay` | interface | `AVAudioUnitDelay.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AVAudioUnitDistortion` | interface | `AVAudioUnitDistortion.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AVAudioUnitGenerator` | interface | `AVAudioUnitGenerator.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AVAudioUnitSampler` | interface | `AVAudioUnitSampler.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |
| `AVAudioUnitTimePitch` | interface | `AVAudioUnitTimePitch.h` | No AVFAudio Objective-C wrapper surface is currently exposed. |

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| `AVAudioFile` | interface | `AVAudioFile.h` | Legacy/deprecated API intentionally excluded from the coverage target. | `API_DEPRECATED("Deprecated - use initForReading or initForWriting", macos(10.10, 26.0), ios(8.0, 26.0), watchos(2.0, 26.0), tvos(9.0, 26.0), macCatalyst(10.10, 26.0))` |
| `AudioComponentGetIcon` | function | `AudioComponent.h` | Legacy/deprecated API intentionally excluded from the coverage target. | `API_DEPRECATED_WITH_REPLACEMENT("AudioComponentCopyIcon", macos(10.11, 11.0)) API_UNAVAILABLE(ios, watchos, tvos)` |

