# AudioToolbox coverage audit

This audit tracks every public type/function discovered in the targeted AudioToolbox headers. Constants/macros are intentionally omitted except where they need an explicit skip note.

## Summary

| Area | APIs tracked | ✅ | 🟡 | ⏭️ |
| --- | ---: | ---: | ---: | ---: |
| AudioFile | 53 | 10 | 43 | 0 |
| ExtAudioFile | 16 | 8 | 8 | 0 |
| AudioConverter | 28 | 9 | 19 | 0 |
| AudioFormat | 12 | 2 | 10 | 0 |
| AudioComponent | 24 | 11 | 13 | 0 |
| AudioUnit | 55 | 8 | 47 | 0 |
| AudioOutputUnit | 4 | 2 | 2 | 0 |
| AudioQueue | 58 | 13 | 45 | 0 |
| Music | 95 | 18 | 77 | 0 |
| AudioServices | 18 | 6 | 11 | 1 |
| CAFFile | 21 | 2 | 19 | 0 |
| AudioFileStream | 13 | 6 | 7 | 0 |

## AudioFile

Source header: `AudioFile.h`

| Kind | Symbol | Status | Notes |
| --- | --- | --- | --- |
| function | `NumBytesToNumAudioFileMarkers` | 🟡 partial | File-optimization helpers and the wider property matrix remain partial. |
| function | `NumAudioFileMarkersToNumBytes` | 🟡 partial | File-optimization helpers and the wider property matrix remain partial. |
| function | `NextAudioFileRegion` | 🟡 partial | File-optimization helpers and the wider property matrix remain partial. |
| function | `AudioFileCreateWithURL` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `AudioFileOpenURL` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `AudioFileInitializeWithCallbacks` | 🟡 partial | File-optimization helpers and the wider property matrix remain partial. |
| function | `AudioFileOpenWithCallbacks` | 🟡 partial | File-optimization helpers and the wider property matrix remain partial. |
| function | `AudioFileClose` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `AudioFileOptimize` | 🟡 partial | File-optimization helpers and the wider property matrix remain partial. |
| function | `AudioFileReadBytes` | 🟡 partial | File-optimization helpers and the wider property matrix remain partial. |
| function | `AudioFileWriteBytes` | 🟡 partial | File-optimization helpers and the wider property matrix remain partial. |
| function | `AudioFileReadPacketData` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `AudioFileReadPackets` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `AudioFileWritePackets` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `AudioFileWritePacketsWithDependencies` | 🟡 partial | File-optimization helpers and the wider property matrix remain partial. |
| function | `AudioFileCountUserData` | 🟡 partial | File-optimization helpers and the wider property matrix remain partial. |
| function | `AudioFileGetUserDataSize` | 🟡 partial | File-optimization helpers and the wider property matrix remain partial. |
| function | `AudioFileGetUserDataSize64` | 🟡 partial | File-optimization helpers and the wider property matrix remain partial. |
| function | `AudioFileGetUserData` | 🟡 partial | File-optimization helpers and the wider property matrix remain partial. |
| function | `AudioFileGetUserDataAtOffset` | 🟡 partial | File-optimization helpers and the wider property matrix remain partial. |
| function | `AudioFileSetUserData` | 🟡 partial | File-optimization helpers and the wider property matrix remain partial. |
| function | `AudioFileRemoveUserData` | 🟡 partial | File-optimization helpers and the wider property matrix remain partial. |
| function | `AudioFileGetPropertyInfo` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `AudioFileGetProperty` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `AudioFileSetProperty` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `AudioFileGetGlobalInfoSize` | 🟡 partial | File-optimization helpers and the wider property matrix remain partial. |
| function | `AudioFileGetGlobalInfo` | 🟡 partial | File-optimization helpers and the wider property matrix remain partial. |
| function | `AudioFileCreate` | 🟡 partial | File-optimization helpers and the wider property matrix remain partial. |
| function | `AudioFileInitialize` | 🟡 partial | File-optimization helpers and the wider property matrix remain partial. |
| function | `AudioFileOpen` | 🟡 partial | File-optimization helpers and the wider property matrix remain partial. |
| type | `AudioFileTypeID` | 🟡 partial | File-optimization helpers and the wider property matrix remain partial. |
| type | `AudioFileID` | ✅ implemented | Available through the safe Rust surface or raw-ffi feature. |
| type | `AudioFilePropertyID` | 🟡 partial | File-optimization helpers and the wider property matrix remain partial. |
| type | `AudioFile_ReadProc` | 🟡 partial | File-optimization helpers and the wider property matrix remain partial. |
| type | `AudioFile_WriteProc` | 🟡 partial | File-optimization helpers and the wider property matrix remain partial. |
| type | `AudioFile_GetSizeProc` | 🟡 partial | File-optimization helpers and the wider property matrix remain partial. |
| type | `AudioFile_SetSizeProc` | 🟡 partial | File-optimization helpers and the wider property matrix remain partial. |
| type | `AudioFile_SMPTE_Time` | 🟡 partial | File-optimization helpers and the wider property matrix remain partial. |
| type | `AudioFileMarker` | 🟡 partial | File-optimization helpers and the wider property matrix remain partial. |
| type | `AudioFileMarkerList` | 🟡 partial | File-optimization helpers and the wider property matrix remain partial. |
| type | `AudioFileRegion` | 🟡 partial | File-optimization helpers and the wider property matrix remain partial. |
| type | `AudioFileRegionList` | 🟡 partial | File-optimization helpers and the wider property matrix remain partial. |
| type | `AudioFramePacketTranslation` | 🟡 partial | File-optimization helpers and the wider property matrix remain partial. |
| type | `AudioBytePacketTranslation` | 🟡 partial | File-optimization helpers and the wider property matrix remain partial. |
| type | `AudioFilePacketTableInfo` | 🟡 partial | File-optimization helpers and the wider property matrix remain partial. |
| type | `AudioPacketRangeByteCountTranslation` | 🟡 partial | File-optimization helpers and the wider property matrix remain partial. |
| type | `AudioPacketRollDistanceTranslation` | 🟡 partial | File-optimization helpers and the wider property matrix remain partial. |
| type | `AudioIndependentPacketTranslation` | 🟡 partial | File-optimization helpers and the wider property matrix remain partial. |
| type | `AudioPacketDependencyInfoTranslation` | 🟡 partial | File-optimization helpers and the wider property matrix remain partial. |
| type | `AudioFileTypeAndFormatID` | 🟡 partial | File-optimization helpers and the wider property matrix remain partial. |
| type | `AudioFileFlags` | 🟡 partial | File-optimization helpers and the wider property matrix remain partial. |
| type | `AudioFileRegionFlags` | 🟡 partial | File-optimization helpers and the wider property matrix remain partial. |

## ExtAudioFile

Source header: `ExtendedAudioFile.h`

| Kind | Symbol | Status | Notes |
| --- | --- | --- | --- |
| function | `ExtAudioFileOpenURL` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `ExtAudioFileWrapAudioFileID` | 🟡 partial | Seek/tell, channel-layout, and file-format mutation helpers remain partial. |
| function | `ExtAudioFileCreateWithURL` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `ExtAudioFileOpen` | 🟡 partial | Seek/tell, channel-layout, and file-format mutation helpers remain partial. |
| function | `ExtAudioFileCreateNew` | 🟡 partial | Seek/tell, channel-layout, and file-format mutation helpers remain partial. |
| function | `ExtAudioFileDispose` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `ExtAudioFileRead` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `ExtAudioFileWrite` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `ExtAudioFileWriteAsync` | 🟡 partial | Seek/tell, channel-layout, and file-format mutation helpers remain partial. |
| function | `ExtAudioFileSeek` | 🟡 partial | Seek/tell, channel-layout, and file-format mutation helpers remain partial. |
| function | `ExtAudioFileTell` | 🟡 partial | Seek/tell, channel-layout, and file-format mutation helpers remain partial. |
| function | `ExtAudioFileGetPropertyInfo` | 🟡 partial | Seek/tell, channel-layout, and file-format mutation helpers remain partial. |
| function | `ExtAudioFileGetProperty` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `ExtAudioFileSetProperty` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| type | `ExtAudioFileRef` | ✅ implemented | Available through the safe Rust surface or raw-ffi feature. |
| type | `ExtAudioFile` | 🟡 partial | Seek/tell, channel-layout, and file-format mutation helpers remain partial. |

## AudioConverter

Source header: `AudioConverter.h`

| Kind | Symbol | Status | Notes |
| --- | --- | --- | --- |
| function | `AudioConverterPrepare` | 🟡 partial | Streaming callback plumbing beyond the one-shot helper remains partial. |
| function | `AudioConverterNew` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `AudioConverterNewSpecific` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `AudioConverterNewWithOptions` | 🟡 partial | Streaming callback plumbing beyond the one-shot helper remains partial. |
| function | `AudioConverterDispose` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `AudioConverterReset` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `AudioConverterGetPropertyInfo` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `AudioConverterGetProperty` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `AudioConverterSetProperty` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `AudioConverterConvertBuffer` | 🟡 partial | Streaming callback plumbing beyond the one-shot helper remains partial. |
| function | `AudioConverterFillComplexBuffer` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `AudioConverterFillComplexBufferRealtimeSafe` | 🟡 partial | Streaming callback plumbing beyond the one-shot helper remains partial. |
| function | `AudioConverterFillComplexBufferWithPacketDependencies` | 🟡 partial | Streaming callback plumbing beyond the one-shot helper remains partial. |
| function | `AudioConverterConvertComplexBuffer` | 🟡 partial | Streaming callback plumbing beyond the one-shot helper remains partial. |
| function | `AudioConverterFillBuffer` | 🟡 partial | Streaming callback plumbing beyond the one-shot helper remains partial. |
| type | `AudioConverterRef` | ✅ implemented | Available through the safe Rust surface or raw-ffi feature. |
| type | `AudioConverterComplexInputDataProc` | 🟡 partial | Streaming callback plumbing beyond the one-shot helper remains partial. |
| type | `AudioConverterComplexInputDataProcRealtimeSafe` | 🟡 partial | Streaming callback plumbing beyond the one-shot helper remains partial. |
| type | `AudioConverterInputDataProc` | 🟡 partial | Streaming callback plumbing beyond the one-shot helper remains partial. |
| type | `AudioConverterPrimeInfo` | 🟡 partial | Streaming callback plumbing beyond the one-shot helper remains partial. |
| type | `AudioConverter` | 🟡 partial | Streaming callback plumbing beyond the one-shot helper remains partial. |
| type | `macOS` | 🟡 partial | Streaming callback plumbing beyond the one-shot helper remains partial. |
| type | `Dithering` | 🟡 partial | Streaming callback plumbing beyond the one-shot helper remains partial. |
| type | `Quality` | 🟡 partial | Streaming callback plumbing beyond the one-shot helper remains partial. |
| type | `Sample` | 🟡 partial | Streaming callback plumbing beyond the one-shot helper remains partial. |
| type | `Prime` | 🟡 partial | Streaming callback plumbing beyond the one-shot helper remains partial. |
| type | `AudioConverterOptions` | 🟡 partial | Streaming callback plumbing beyond the one-shot helper remains partial. |
| type | `AudioConverterPropertyID` | 🟡 partial | Streaming callback plumbing beyond the one-shot helper remains partial. |

## AudioFormat

Source header: `AudioFormat.h`

| Kind | Symbol | Status | Notes |
| --- | --- | --- | --- |
| function | `AudioFormatGetPropertyInfo` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `AudioFormatGetProperty` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| type | `AudioFormatPropertyID` | 🟡 partial | Most property families are exposed through generic query helpers rather than dedicated wrappers. |
| type | `AudioPanningInfo` | 🟡 partial | Most property families are exposed through generic query helpers rather than dedicated wrappers. |
| type | `AudioBalanceFade` | 🟡 partial | Most property families are exposed through generic query helpers rather than dedicated wrappers. |
| type | `AudioFormatInfo` | 🟡 partial | Most property families are exposed through generic query helpers rather than dedicated wrappers. |
| type | `ExtendedAudioFormatInfo` | 🟡 partial | Most property families are exposed through generic query helpers rather than dedicated wrappers. |
| type | `AudioPanningMode` | 🟡 partial | Most property families are exposed through generic query helpers rather than dedicated wrappers. |
| type | `AudioBalanceFadeType` | 🟡 partial | Most property families are exposed through generic query helpers rather than dedicated wrappers. |
| type | `AudioFormatProperty` | 🟡 partial | Most property families are exposed through generic query helpers rather than dedicated wrappers. |
| type | `AudioCodecComponentType` | 🟡 partial | Most property families are exposed through generic query helpers rather than dedicated wrappers. |
| type | `AudioCodecComponentManufacturer` | 🟡 partial | Most property families are exposed through generic query helpers rather than dedicated wrappers. |

## AudioComponent

Source header: `AudioComponent.h`

| Kind | Symbol | Status | Notes |
| --- | --- | --- | --- |
| function | `AudioComponentFindNext` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `AudioComponentCount` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `AudioComponentCopyName` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `AudioComponentGetDescription` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `AudioComponentGetVersion` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `AudioComponentGetIcon` | 🟡 partial | Registration and listener APIs remain partial. |
| function | `AudioComponentInstanceNew` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `AudioComponentInstantiate` | 🟡 partial | Registration and listener APIs remain partial. |
| function | `AudioComponentInstanceDispose` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `AudioComponentInstanceGetComponent` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `AudioComponentInstanceCanDo` | 🟡 partial | Registration and listener APIs remain partial. |
| function | `AudioComponentRegister` | 🟡 partial | Registration and listener APIs remain partial. |
| function | `AudioComponentCopyConfigurationInfo` | 🟡 partial | Registration and listener APIs remain partial. |
| function | `AudioComponentValidate` | 🟡 partial | Registration and listener APIs remain partial. |
| function | `AudioComponentValidateWithResults` | 🟡 partial | Registration and listener APIs remain partial. |
| type | `AudioComponent` | ✅ implemented | Available through the safe Rust surface or raw-ffi feature. |
| type | `AudioComponentInstance` | ✅ implemented | Available through the safe Rust surface or raw-ffi feature. |
| type | `AudioComponentMethod` | 🟡 partial | Registration and listener APIs remain partial. |
| type | `AudioComponentFactoryFunction` | 🟡 partial | Registration and listener APIs remain partial. |
| type | `AudioComponentDescription` | ✅ implemented | Available through the safe Rust surface or raw-ffi feature. |
| type | `AudioComponentPlugInInterface` | 🟡 partial | Registration and listener APIs remain partial. |
| type | `AudioComponentFlags` | 🟡 partial | Registration and listener APIs remain partial. |
| type | `AudioComponentInstantiationOptions` | 🟡 partial | Registration and listener APIs remain partial. |
| type | `AudioComponentValidationResult` | 🟡 partial | Registration and listener APIs remain partial. |

## AudioUnit

Source header: `AUComponent.h`

| Kind | Symbol | Status | Notes |
| --- | --- | --- | --- |
| function | `AudioUnitInitialize` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `AudioUnitUninitialize` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `AudioUnitGetPropertyInfo` | 🟡 partial | Render callbacks, listeners, and most property/parameter families remain partial. |
| function | `AudioUnitGetProperty` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `AudioUnitSetProperty` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `AudioUnitAddPropertyListener` | 🟡 partial | Render callbacks, listeners, and most property/parameter families remain partial. |
| function | `AudioUnitRemovePropertyListenerWithUserData` | 🟡 partial | Render callbacks, listeners, and most property/parameter families remain partial. |
| function | `AudioUnitAddRenderNotify` | 🟡 partial | Render callbacks, listeners, and most property/parameter families remain partial. |
| function | `AudioUnitRemoveRenderNotify` | 🟡 partial | Render callbacks, listeners, and most property/parameter families remain partial. |
| function | `AudioUnitGetParameter` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `AudioUnitSetParameter` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `AudioUnitScheduleParameters` | 🟡 partial | Render callbacks, listeners, and most property/parameter families remain partial. |
| function | `AudioUnitRender` | 🟡 partial | Render callbacks, listeners, and most property/parameter families remain partial. |
| function | `AudioUnitReset` | 🟡 partial | Render callbacks, listeners, and most property/parameter families remain partial. |
| function | `AudioOutputUnitPublish` | 🟡 partial | Render callbacks, listeners, and most property/parameter families remain partial. |
| function | `AudioComponentGetLastActiveTime` | 🟡 partial | Render callbacks, listeners, and most property/parameter families remain partial. |
| function | `AudioComponentCopyIcon` | 🟡 partial | Render callbacks, listeners, and most property/parameter families remain partial. |
| function | `AudioUnitExtensionSetComponentList` | 🟡 partial | Render callbacks, listeners, and most property/parameter families remain partial. |
| function | `AudioUnitExtensionCopyComponentList` | 🟡 partial | Render callbacks, listeners, and most property/parameter families remain partial. |
| type | `AudioUnit` | ✅ implemented | Available through the safe Rust surface or raw-ffi feature. |
| type | `AudioUnitPropertyID` | 🟡 partial | Render callbacks, listeners, and most property/parameter families remain partial. |
| type | `AudioUnitScope` | 🟡 partial | Render callbacks, listeners, and most property/parameter families remain partial. |
| type | `AudioUnitElement` | 🟡 partial | Render callbacks, listeners, and most property/parameter families remain partial. |
| type | `AudioUnitParameterID` | 🟡 partial | Render callbacks, listeners, and most property/parameter families remain partial. |
| type | `AudioUnitParameterValue` | 🟡 partial | Render callbacks, listeners, and most property/parameter families remain partial. |
| type | `AURenderCallback` | 🟡 partial | Render callbacks, listeners, and most property/parameter families remain partial. |
| type | `AudioUnitPropertyListenerProc` | 🟡 partial | Render callbacks, listeners, and most property/parameter families remain partial. |
| type | `AUInputSamplesInOutputCallback` | 🟡 partial | Render callbacks, listeners, and most property/parameter families remain partial. |
| type | `AudioUnitGetParameterProc` | 🟡 partial | Render callbacks, listeners, and most property/parameter families remain partial. |
| type | `AudioUnitSetParameterProc` | 🟡 partial | Render callbacks, listeners, and most property/parameter families remain partial. |
| type | `AudioUnitRenderProc` | 🟡 partial | Render callbacks, listeners, and most property/parameter families remain partial. |
| type | `AudioUnitParameterEvent` | 🟡 partial | Render callbacks, listeners, and most property/parameter families remain partial. |
| type | `AudioUnitParameter` | 🟡 partial | Render callbacks, listeners, and most property/parameter families remain partial. |
| type | `AudioUnitProperty` | 🟡 partial | Render callbacks, listeners, and most property/parameter families remain partial. |
| type | `Apple` | 🟡 partial | Render callbacks, listeners, and most property/parameter families remain partial. |
| type | `AudioUnitRenderActionFlags` | 🟡 partial | Render callbacks, listeners, and most property/parameter families remain partial. |
| type | `AudioComponent` | ✅ implemented | Available through the safe Rust surface or raw-ffi feature. |
| type | `AUParameterEventType` | 🟡 partial | Render callbacks, listeners, and most property/parameter families remain partial. |
| type | `deprecated` | 🟡 partial | Render callbacks, listeners, and most property/parameter families remain partial. |
| type | `AudioUnitInitializeProc` | 🟡 partial | Render callbacks, listeners, and most property/parameter families remain partial. |
| type | `AudioUnitUninitializeProc` | 🟡 partial | Render callbacks, listeners, and most property/parameter families remain partial. |
| type | `AudioUnitGetPropertyInfoProc` | 🟡 partial | Render callbacks, listeners, and most property/parameter families remain partial. |
| type | `AudioUnitGetPropertyProc` | 🟡 partial | Render callbacks, listeners, and most property/parameter families remain partial. |
| type | `AudioUnitSetPropertyProc` | 🟡 partial | Render callbacks, listeners, and most property/parameter families remain partial. |
| type | `AudioUnitAddPropertyListenerProc` | 🟡 partial | Render callbacks, listeners, and most property/parameter families remain partial. |
| type | `AudioUnitRemovePropertyListenerProc` | 🟡 partial | Render callbacks, listeners, and most property/parameter families remain partial. |
| type | `AudioUnitRemovePropertyListenerWithUserDataProc` | 🟡 partial | Render callbacks, listeners, and most property/parameter families remain partial. |
| type | `AudioUnitAddRenderNotifyProc` | 🟡 partial | Render callbacks, listeners, and most property/parameter families remain partial. |
| type | `AudioUnitRemoveRenderNotifyProc` | 🟡 partial | Render callbacks, listeners, and most property/parameter families remain partial. |
| type | `AudioUnitScheduleParametersProc` | 🟡 partial | Render callbacks, listeners, and most property/parameter families remain partial. |
| type | `AudioUnitResetProc` | 🟡 partial | Render callbacks, listeners, and most property/parameter families remain partial. |
| type | `AudioUnitComplexRenderProc` | 🟡 partial | Render callbacks, listeners, and most property/parameter families remain partial. |
| type | `AudioUnitProcessProc` | 🟡 partial | Render callbacks, listeners, and most property/parameter families remain partial. |
| type | `AudioUnitProcessMultipleProc` | 🟡 partial | Render callbacks, listeners, and most property/parameter families remain partial. |

## AudioOutputUnit

Source header: `AudioOutputUnit.h`

| Kind | Symbol | Status | Notes |
| --- | --- | --- | --- |
| function | `AudioOutputUnitStart` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `AudioOutputUnitStop` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| type | `AudioOutputUnitStartProc` | 🟡 partial | Only start/stop are wrapped; output timing and specialised properties remain partial. |
| type | `AudioOutputUnitStopProc` | 🟡 partial | Only start/stop are wrapped; output timing and specialised properties remain partial. |

## AudioQueue

Source header: `AudioQueue.h`

| Kind | Symbol | Status | Notes |
| --- | --- | --- | --- |
| function | `AudioQueueNewOutput` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `AudioQueueNewInput` | 🟡 partial | Input queues, buffer enqueue/dequeue, listeners, and timelines remain partial. |
| function | `AudioQueueNewOutputWithDispatchQueue` | 🟡 partial | Input queues, buffer enqueue/dequeue, listeners, and timelines remain partial. |
| function | `AudioQueueNewInputWithDispatchQueue` | 🟡 partial | Input queues, buffer enqueue/dequeue, listeners, and timelines remain partial. |
| function | `AudioQueueDispose` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `AudioQueueAllocateBuffer` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `AudioQueueAllocateBufferWithPacketDescriptions` | 🟡 partial | Input queues, buffer enqueue/dequeue, listeners, and timelines remain partial. |
| function | `AudioQueueFreeBuffer` | 🟡 partial | Input queues, buffer enqueue/dequeue, listeners, and timelines remain partial. |
| function | `AudioQueueEnqueueBuffer` | 🟡 partial | Input queues, buffer enqueue/dequeue, listeners, and timelines remain partial. |
| function | `AudioQueueEnqueueBufferWithParameters` | 🟡 partial | Input queues, buffer enqueue/dequeue, listeners, and timelines remain partial. |
| function | `AudioQueueStart` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `AudioQueuePrime` | 🟡 partial | Input queues, buffer enqueue/dequeue, listeners, and timelines remain partial. |
| function | `AudioQueueStop` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `AudioQueuePause` | 🟡 partial | Input queues, buffer enqueue/dequeue, listeners, and timelines remain partial. |
| function | `AudioQueueFlush` | 🟡 partial | Input queues, buffer enqueue/dequeue, listeners, and timelines remain partial. |
| function | `AudioQueueReset` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `AudioQueueGetParameter` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `AudioQueueSetParameter` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `AudioQueueGetProperty` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `AudioQueueSetProperty` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `AudioQueueGetPropertySize` | 🟡 partial | Input queues, buffer enqueue/dequeue, listeners, and timelines remain partial. |
| function | `AudioQueueAddPropertyListener` | 🟡 partial | Input queues, buffer enqueue/dequeue, listeners, and timelines remain partial. |
| function | `AudioQueueRemovePropertyListener` | 🟡 partial | Input queues, buffer enqueue/dequeue, listeners, and timelines remain partial. |
| function | `AudioQueueCreateTimeline` | 🟡 partial | Input queues, buffer enqueue/dequeue, listeners, and timelines remain partial. |
| function | `AudioQueueDisposeTimeline` | 🟡 partial | Input queues, buffer enqueue/dequeue, listeners, and timelines remain partial. |
| function | `AudioQueueGetCurrentTime` | 🟡 partial | Input queues, buffer enqueue/dequeue, listeners, and timelines remain partial. |
| function | `AudioQueueDeviceGetCurrentTime` | 🟡 partial | Input queues, buffer enqueue/dequeue, listeners, and timelines remain partial. |
| function | `AudioQueueDeviceTranslateTime` | 🟡 partial | Input queues, buffer enqueue/dequeue, listeners, and timelines remain partial. |
| function | `AudioQueueDeviceGetNearestStartTime` | 🟡 partial | Input queues, buffer enqueue/dequeue, listeners, and timelines remain partial. |
| function | `AudioQueueSetOfflineRenderFormat` | 🟡 partial | Input queues, buffer enqueue/dequeue, listeners, and timelines remain partial. |
| function | `AudioQueueOfflineRender` | 🟡 partial | Input queues, buffer enqueue/dequeue, listeners, and timelines remain partial. |
| function | `AudioQueueProcessingTapNew` | 🟡 partial | Input queues, buffer enqueue/dequeue, listeners, and timelines remain partial. |
| function | `AudioQueueProcessingTapDispose` | 🟡 partial | Input queues, buffer enqueue/dequeue, listeners, and timelines remain partial. |
| function | `AudioQueueProcessingTapGetSourceAudio` | 🟡 partial | Input queues, buffer enqueue/dequeue, listeners, and timelines remain partial. |
| function | `AudioQueueProcessingTapGetQueueTime` | 🟡 partial | Input queues, buffer enqueue/dequeue, listeners, and timelines remain partial. |
| type | `AudioQueuePropertyID` | 🟡 partial | Input queues, buffer enqueue/dequeue, listeners, and timelines remain partial. |
| type | `AudioQueueParameterID` | 🟡 partial | Input queues, buffer enqueue/dequeue, listeners, and timelines remain partial. |
| type | `AudioQueueParameterValue` | 🟡 partial | Input queues, buffer enqueue/dequeue, listeners, and timelines remain partial. |
| type | `AudioQueueRef` | ✅ implemented | Available through the safe Rust surface or raw-ffi feature. |
| type | `AudioQueueTimelineRef` | 🟡 partial | Input queues, buffer enqueue/dequeue, listeners, and timelines remain partial. |
| type | `AudioQueueBufferRef` | ✅ implemented | Available through the safe Rust surface or raw-ffi feature. |
| type | `AudioQueueProcessingTapRef` | 🟡 partial | Input queues, buffer enqueue/dequeue, listeners, and timelines remain partial. |
| type | `AudioQueueOutputCallbackBlock` | 🟡 partial | Input queues, buffer enqueue/dequeue, listeners, and timelines remain partial. |
| type | `AudioQueueInputCallbackBlock` | 🟡 partial | Input queues, buffer enqueue/dequeue, listeners, and timelines remain partial. |
| type | `AudioQueueOutputCallback` | 🟡 partial | Input queues, buffer enqueue/dequeue, listeners, and timelines remain partial. |
| type | `AudioQueueInputCallback` | 🟡 partial | Input queues, buffer enqueue/dequeue, listeners, and timelines remain partial. |
| type | `AudioQueuePropertyListenerProc` | 🟡 partial | Input queues, buffer enqueue/dequeue, listeners, and timelines remain partial. |
| type | `AudioQueueProcessingTapCallback` | 🟡 partial | Input queues, buffer enqueue/dequeue, listeners, and timelines remain partial. |
| type | `AudioQueueBuffer` | ✅ implemented | Available through the safe Rust surface or raw-ffi feature. |
| type | `AudioQueueParameterEvent` | 🟡 partial | Input queues, buffer enqueue/dequeue, listeners, and timelines remain partial. |
| type | `AudioQueueLevelMeterState` | 🟡 partial | Input queues, buffer enqueue/dequeue, listeners, and timelines remain partial. |
| type | `AudioQueueChannelAssignment` | 🟡 partial | Input queues, buffer enqueue/dequeue, listeners, and timelines remain partial. |
| type | `Result` | 🟡 partial | Input queues, buffer enqueue/dequeue, listeners, and timelines remain partial. |
| type | `Time` | 🟡 partial | Input queues, buffer enqueue/dequeue, listeners, and timelines remain partial. |
| type | `AudioQueueHardwareCodecPolicy` | 🟡 partial | Input queues, buffer enqueue/dequeue, listeners, and timelines remain partial. |
| type | `AudioQueue` | 🟡 partial | Input queues, buffer enqueue/dequeue, listeners, and timelines remain partial. |
| type | `AudioQueueProcessingTapFlags` | 🟡 partial | Input queues, buffer enqueue/dequeue, listeners, and timelines remain partial. |

## Music

Source header: `MusicPlayer.h`

| Kind | Symbol | Status | Notes |
| --- | --- | --- | --- |
| function | `NewMusicPlayer` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `DisposeMusicPlayer` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `MusicPlayerSetSequence` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `MusicPlayerGetSequence` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `MusicPlayerSetTime` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `MusicPlayerGetTime` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `MusicPlayerGetHostTimeForBeats` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `MusicPlayerGetBeatsForHostTime` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `MusicPlayerPreroll` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `MusicPlayerStart` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `MusicPlayerStop` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `MusicPlayerIsPlaying` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `MusicPlayerSetPlayRateScalar` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `MusicPlayerGetPlayRateScalar` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `NewMusicSequence` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `DisposeMusicSequence` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `MusicSequenceNewTrack` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `MusicSequenceDisposeTrack` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `MusicSequenceGetTrackCount` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `MusicSequenceGetIndTrack` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `MusicSequenceGetTrackIndex` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `MusicSequenceGetTempoTrack` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `MusicSequenceSetAUGraph` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `MusicSequenceGetAUGraph` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `MusicSequenceSetMIDIEndpoint` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `MusicSequenceSetSequenceType` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `MusicSequenceGetSequenceType` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `MusicSequenceFileLoad` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `MusicSequenceFileLoadData` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `MusicSequenceSetSMPTEResolution` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `MusicSequenceGetSMPTEResolution` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `MusicSequenceFileCreate` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `MusicSequenceFileCreateData` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `MusicSequenceReverse` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `MusicSequenceGetSecondsForBeats` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `MusicSequenceGetBeatsForSeconds` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `MusicSequenceSetUserCallback` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `MusicSequenceBeatsToBarBeatTime` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `MusicSequenceBarBeatTimeToBeats` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `MusicSequenceGetInfoDictionary` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `MusicTrackGetSequence` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `MusicTrackSetDestNode` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `MusicTrackSetDestMIDIEndpoint` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `MusicTrackGetDestNode` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `MusicTrackGetDestMIDIEndpoint` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `MusicTrackSetProperty` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `MusicTrackGetProperty` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `MusicTrackMoveEvents` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `MusicTrackClear` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `MusicTrackCut` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `MusicTrackCopyInsert` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `MusicTrackMerge` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `MusicTrackNewMIDINoteEvent` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `MusicTrackNewMIDIChannelEvent` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `MusicTrackNewMIDIRawDataEvent` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `MusicTrackNewExtendedNoteEvent` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `MusicTrackNewParameterEvent` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `MusicTrackNewExtendedTempoEvent` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `MusicTrackNewMetaEvent` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `MusicEventUserData` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `MusicTrackNewAUPresetEvent` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `NewMusicEventIterator` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `DisposeMusicEventIterator` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `MusicEventIteratorSeek` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `MusicEventIteratorNextEvent` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `MusicEventIteratorPreviousEvent` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `MusicEventIteratorGetEventInfo` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `MusicEventIteratorSetEventInfo` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `MusicEventIteratorSetEventTime` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `MusicEventIteratorDeleteEvent` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `MusicEventIteratorHasPreviousEvent` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `MusicEventIteratorHasNextEvent` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| function | `MusicEventIteratorHasCurrentEvent` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| type | `MusicTimeStamp` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| type | `MusicSequenceUserCallback` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| type | `kMusicTimeStamp_EndOfTrack` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| type | `MIDINoteMessage` | ✅ implemented | Available through the safe Rust surface or raw-ffi feature. |
| type | `MIDIChannelMessage` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| type | `MIDIRawData` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| type | `MIDIMetaEvent` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| type | `ExtendedNoteOnEvent` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| type | `ParameterEvent` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| type | `ExtendedTempoEvent` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| type | `AUPresetEvent` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| type | `CABarBeatTime` | ✅ implemented | Available through the safe Rust surface or raw-ffi feature. |
| type | `MusicTrackLoopInfo` | ✅ implemented | Available through the safe Rust surface or raw-ffi feature. |
| type | `MusicSequenceLoadFlags` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| type | `MusicSequenceType` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| type | `MusicSequenceFileTypeID` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| type | `MusicSequenceFileFlags` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| type | `ExtendedControlEvent` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |
| type | `MusicPlayer` | ✅ implemented | Available through the safe Rust surface or raw-ffi feature. |
| type | `MusicSequence` | ✅ implemented | Available through the safe Rust surface or raw-ffi feature. |
| type | `MusicTrack` | ✅ implemented | Available through the safe Rust surface or raw-ffi feature. |
| type | `MusicEventIterator` | 🟡 partial | Sequence file I/O, tempo, routing, and most event families remain partial. |

## AudioServices

Source header: `AudioServices.h`

| Kind | Symbol | Status | Notes |
| --- | --- | --- | --- |
| function | `AudioServicesCreateSystemSoundID` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `AudioServicesDisposeSystemSoundID` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `AudioServicesPlayAlertSoundWithCompletion` | 🟡 partial | Completion-block helpers and mobile-only behavior remain partial/skipped. |
| function | `AudioServicesPlaySystemSoundWithCompletion` | 🟡 partial | Completion-block helpers and mobile-only behavior remain partial/skipped. |
| function | `AudioServicesGetPropertyInfo` | 🟡 partial | Completion-block helpers and mobile-only behavior remain partial/skipped. |
| function | `AudioServicesGetProperty` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `AudioServicesSetProperty` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `AudioServicesPlayAlertSound` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `AudioServicesPlaySystemSound` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `AudioServicesAddSystemSoundCompletion` | 🟡 partial | Completion-block helpers and mobile-only behavior remain partial/skipped. |
| function | `AudioServicesRemoveSystemSoundCompletion` | 🟡 partial | Completion-block helpers and mobile-only behavior remain partial/skipped. |
| function | `AudioServicesPlaySystemSoundWithDetails` | 🟡 partial | Completion-block helpers and mobile-only behavior remain partial/skipped. |
| function | `AudioServicesPlayAlertSoundWithDetails` | 🟡 partial | Completion-block helpers and mobile-only behavior remain partial/skipped. |
| type | `SystemSoundID` | 🟡 partial | Completion-block helpers and mobile-only behavior remain partial/skipped. |
| type | `AudioServicesPropertyID` | 🟡 partial | Completion-block helpers and mobile-only behavior remain partial/skipped. |
| type | `AudioServicesSystemSoundCompletionProc` | 🟡 partial | Completion-block helpers and mobile-only behavior remain partial/skipped. |
| type | `AudioServices` | 🟡 partial | Completion-block helpers and mobile-only behavior remain partial/skipped. |
| constant | `kSystemSoundID_Vibrate` | ⏭️ skipped | iOS-only vibration constant; unavailable on macOS. |

## CAFFile

Source header: `CAFFile.h`

| Kind | Symbol | Status | Notes |
| --- | --- | --- | --- |
| type | `CAFFileHeader` | ✅ implemented | Available through the safe Rust surface or raw-ffi feature. |
| type | `CAFChunkHeader` | ✅ implemented | Available through the safe Rust surface or raw-ffi feature. |
| type | `CAF_UUID_ChunkHeader` | 🟡 partial | Byte-level CAF header parsing is wrapped; chunk-specific structs/helpers remain partial. |
| type | `CAFAudioDescription` | 🟡 partial | Byte-level CAF header parsing is wrapped; chunk-specific structs/helpers remain partial. |
| type | `CAFPacketTableHeader` | 🟡 partial | Byte-level CAF header parsing is wrapped; chunk-specific structs/helpers remain partial. |
| type | `CAFDataChunk` | 🟡 partial | Byte-level CAF header parsing is wrapped; chunk-specific structs/helpers remain partial. |
| type | `CAF_SMPTE_Time` | 🟡 partial | Byte-level CAF header parsing is wrapped; chunk-specific structs/helpers remain partial. |
| type | `CAFMarker` | 🟡 partial | Byte-level CAF header parsing is wrapped; chunk-specific structs/helpers remain partial. |
| type | `CAFMarkerChunk` | 🟡 partial | Byte-level CAF header parsing is wrapped; chunk-specific structs/helpers remain partial. |
| type | `CAFRegion` | 🟡 partial | Byte-level CAF header parsing is wrapped; chunk-specific structs/helpers remain partial. |
| type | `CAFRegionChunk` | 🟡 partial | Byte-level CAF header parsing is wrapped; chunk-specific structs/helpers remain partial. |
| type | `CAFInstrumentChunk` | 🟡 partial | Byte-level CAF header parsing is wrapped; chunk-specific structs/helpers remain partial. |
| type | `CAFStringID` | 🟡 partial | Byte-level CAF header parsing is wrapped; chunk-specific structs/helpers remain partial. |
| type | `CAFStrings` | 🟡 partial | Byte-level CAF header parsing is wrapped; chunk-specific structs/helpers remain partial. |
| type | `CAFInfoStrings` | 🟡 partial | Byte-level CAF header parsing is wrapped; chunk-specific structs/helpers remain partial. |
| type | `CAFPositionPeak` | 🟡 partial | Byte-level CAF header parsing is wrapped; chunk-specific structs/helpers remain partial. |
| type | `CAFPeakChunk` | 🟡 partial | Byte-level CAF header parsing is wrapped; chunk-specific structs/helpers remain partial. |
| type | `CAFOverviewSample` | 🟡 partial | Byte-level CAF header parsing is wrapped; chunk-specific structs/helpers remain partial. |
| type | `CAFOverviewChunk` | 🟡 partial | Byte-level CAF header parsing is wrapped; chunk-specific structs/helpers remain partial. |
| type | `CAFUMIDChunk` | 🟡 partial | Byte-level CAF header parsing is wrapped; chunk-specific structs/helpers remain partial. |
| type | `CAFAudioFormatListItem` | 🟡 partial | Byte-level CAF header parsing is wrapped; chunk-specific structs/helpers remain partial. |

## AudioFileStream

Source header: `AudioFileStream.h`

| Kind | Symbol | Status | Notes |
| --- | --- | --- | --- |
| function | `AudioFileStreamOpen` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `AudioFileStreamParseBytes` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `AudioFileStreamSeek` | 🟡 partial | Seek helpers, callback data surfaces, and cached-property flags remain partial. |
| function | `AudioFileStreamGetPropertyInfo` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `AudioFileStreamGetProperty` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| function | `AudioFileStreamSetProperty` | 🟡 partial | Seek helpers, callback data surfaces, and cached-property flags remain partial. |
| function | `AudioFileStreamClose` | ✅ implemented | Wrapped by the Swift bridge and surfaced through the safe Rust API. |
| type | `AudioFileStreamPropertyFlags` | 🟡 partial | Seek helpers, callback data surfaces, and cached-property flags remain partial. |
| type | `AudioFileStreamParseFlags` | 🟡 partial | Seek helpers, callback data surfaces, and cached-property flags remain partial. |
| type | `AudioFileStream` | 🟡 partial | Seek helpers, callback data surfaces, and cached-property flags remain partial. |
| type | `AudioFileStreamID` | ✅ implemented | Available through the safe Rust surface or raw-ffi feature. |
| type | `AudioFileStream_PropertyListenerProc` | 🟡 partial | Seek helpers, callback data surfaces, and cached-property flags remain partial. |
| type | `AudioFileStream_PacketsProc` | 🟡 partial | Seek helpers, callback data surfaces, and cached-property flags remain partial. |

