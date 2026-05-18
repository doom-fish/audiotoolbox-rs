#![allow(non_snake_case)]

use std::{ffi::c_void, fmt};

pub use apple_cf::raw::{
    CFArrayRef, CFDataRef, CFDictionaryRef, CFPropertyListRef, CFStringRef, CFURLRef,
};

/// Wraps `Boolean`.
pub type Boolean = u8;
/// Wraps `OSStatus`.
pub type OSStatus = i32;
/// Wraps `OSType`.
pub type OSType = u32;

/// Wraps `AudioFormatID`.
pub type AudioFormatId = u32;
/// Wraps `AudioFormatFlags`.
pub type AudioFormatFlags = u32;
/// Wraps `AudioFormatPropertyID`.
pub type AudioFormatPropertyId = u32;
/// Wraps `AudioChannelLayoutTag`.
pub type AudioChannelLayoutTag = u32;
/// Wraps `AudioPanningMode`.
pub type AudioPanningMode = u32;
/// Wraps `AudioBalanceFadeType`.
pub type AudioBalanceFadeType = u32;
/// Wraps `AudioFileTypeID`.
pub type AudioFileTypeId = u32;
/// Wraps `AudioFilePropertyID`.
pub type AudioFilePropertyId = u32;
/// Wraps `AudioFileComponentPropertyID`.
pub type AudioFileComponentPropertyId = u32;
/// Wraps `AudioFileFlags`.
pub type AudioFileFlags = u32;
/// Wraps `AudioFilePermissions`.
pub type AudioFilePermissions = i8;
/// Wraps `AudioFileRegionFlags`.
pub type AudioFileRegionFlags = u32;
/// Wraps `AudioBytePacketTranslationFlags`.
pub type AudioBytePacketTranslationFlags = u32;
/// Wraps `ExtAudioFilePropertyID`.
pub type ExtAudioFilePropertyId = u32;
/// Wraps `AudioConverterPropertyID`.
pub type AudioConverterPropertyId = u32;
/// Wraps `AudioConverterOptions`.
pub type AudioConverterOptions = u32;
/// Wraps `AudioComponentFlags`.
pub type AudioComponentFlags = u32;
/// Wraps `AudioComponentInstantiationOptions`.
pub type AudioComponentInstantiationOptions = u32;
/// Wraps `AudioComponentValidationResult`.
pub type AudioComponentValidationResult = u32;
/// Wraps `AudioServicesPropertyID`.
pub type AudioServicesPropertyId = u32;
/// Wraps `SystemSoundID`.
pub type SystemSoundId = u32;
/// Wraps `AudioUnitPropertyID`.
pub type AudioUnitPropertyId = u32;
/// Wraps `AudioUnitScope`.
pub type AudioUnitScope = u32;
/// Wraps `AudioUnitElement`.
pub type AudioUnitElement = u32;
/// Wraps `AudioUnitParameterID`.
pub type AudioUnitParameterId = u32;
/// Wraps `AudioUnitParameterValue`.
pub type AudioUnitParameterValue = f32;
/// Wraps `AudioUnitRenderActionFlags`.
pub type AudioUnitRenderActionFlags = u32;
/// Wraps `AUParameterEventType`.
pub type AUParameterEventType = u32;
/// Wraps `AUAudioFrameCount`.
pub type AUAudioFrameCount = u32;
/// Wraps `AUNode`.
pub type AUNode = i32;
/// Wraps `AudioQueuePropertyID`.
pub type AudioQueuePropertyId = u32;
/// Wraps `AudioQueueParameterID`.
pub type AudioQueueParameterId = u32;
/// Wraps `AudioQueueParameterValue`.
pub type AudioQueueParameterValue = f32;
/// Wraps `AudioFileStreamPropertyID`.
pub type AudioFileStreamPropertyId = u32;
/// Wraps `AudioFileStreamParseFlags`.
pub type AudioFileStreamParseFlags = u32;
/// Wraps `AudioFileStreamSeekFlags`.
pub type AudioFileStreamSeekFlags = u32;
/// Wraps `AVAudioCommonFormat`.
pub type AVAudioCommonFormat = u64;
/// Wraps `AVAudioChannelCount`.
pub type AVAudioChannelCount = u32;
/// Wraps `AVAudioNodeBus`.
pub type AVAudioNodeBus = u64;
/// Wraps `AVAudioFramePosition`.
pub type AVAudioFramePosition = i64;
/// Wraps `AVAudioFrameCount`.
pub type AVAudioFrameCount = u32;
/// Wraps `AVAudioPacketCount`.
pub type AVAudioPacketCount = u32;
/// Wraps `AVMusicTimeStamp`.
pub type AVMusicTimeStamp = f64;
/// Wraps `AVAudioPlayerNodeBufferOptions`.
pub type AVAudioPlayerNodeBufferOptions = u64;
/// Wraps `AVMusicSequenceLoadOptions`.
pub type AVMusicSequenceLoadOptions = u64;
/// Wraps `AVAudioEngineManualRenderingMode`.
pub type AVAudioEngineManualRenderingMode = i64;
/// Wraps `AVAudioEngineManualRenderingStatus`.
pub type AVAudioEngineManualRenderingStatus = i64;
/// Wraps `AVSpeechBoundary`.
pub type AVSpeechBoundary = i64;
/// Wraps `AVSpeechSynthesisVoiceGender`.
pub type AVSpeechSynthesisVoiceGender = i64;
/// Wraps `AVSpeechSynthesisMarkerMark`.
pub type AVSpeechSynthesisMarkerMark = i64;
/// Wraps `AVSpeechSynthesisPersonalVoiceAuthorizationStatus`.
pub type AVSpeechSynthesisPersonalVoiceAuthorizationStatus = u64;
/// Wraps `MusicEventType`.
pub type MusicEventType = u32;
/// Wraps `MIDIEndpointRef`.
pub type MIDIEndpointRef = u32;
/// Wraps `MusicDeviceInstrumentID`.
pub type MusicDeviceInstrumentId = u32;
/// Wraps `MusicDeviceGroupID`.
pub type MusicDeviceGroupId = u32;
/// Wraps `NoteInstanceID`.
pub type NoteInstanceId = u32;
/// Wraps `MusicSequenceType`.
pub type MusicSequenceType = u32;
/// Wraps `MusicSequenceFileTypeID`.
pub type MusicSequenceFileTypeId = u32;
/// Wraps `MusicSequenceFileFlags`.
pub type MusicSequenceFileFlags = u32;
/// Wraps `MusicSequenceLoadFlags`.
pub type MusicSequenceLoadFlags = u32;
/// Wraps `MusicTimeStamp`.
pub type MusicTimeStamp = f64;

/// Wraps `AudioFileID`.
pub type AudioFileId = *mut c_void;
/// Wraps `ExtAudioFileRef`.
pub type ExtAudioFileRef = *mut c_void;
/// Wraps `AudioConverterRef`.
pub type AudioConverterRef = *mut c_void;
/// Wraps `AudioComponentRef`.
pub type AudioComponentRef = *mut c_void;
/// Wraps `AudioComponentInstanceRef`.
pub type AudioComponentInstanceRef = *mut c_void;
/// Wraps `AudioUnitRef`.
pub type AudioUnitRef = *mut c_void;
/// Wraps `AudioQueueRef`.
pub type AudioQueueRef = *mut c_void;
/// Wraps `AudioQueueTimelineRef`.
pub type AudioQueueTimelineRef = *mut c_void;
/// Wraps `MusicSequenceRef`.
pub type MusicSequenceRef = *mut c_void;
/// Wraps `MusicTrackRef`.
pub type MusicTrackRef = *mut c_void;
/// Wraps `MusicPlayerRef`.
pub type MusicPlayerRef = *mut c_void;
/// Wraps `MusicEventIteratorRef`.
pub type MusicEventIteratorRef = *mut c_void;
/// Wraps `AudioFileStreamID`.
pub type AudioFileStreamId = *mut c_void;

/// Packs a `FourCharCode` value used by AudioToolbox.framework.
pub const fn fourcc(bytes: [u8; 4]) -> u32 {
    u32::from_be_bytes(bytes)
}

/// Wraps `noErr`.
pub const NO_ERR: OSStatus = 0;
/// Wraps `kAudioFileEndOfFileError`.
pub const AUDIO_FILE_END_OF_FILE_ERROR: OSStatus = -39;

/// Wraps `kAudioFileReadPermission`.
pub const AUDIO_FILE_READ_PERMISSION: AudioFilePermissions = 0x01;
/// Wraps `kAudioFileWritePermission`.
pub const AUDIO_FILE_WRITE_PERMISSION: AudioFilePermissions = 0x02;
/// Wraps `kAudioFileReadWritePermission`.
pub const AUDIO_FILE_READ_WRITE_PERMISSION: AudioFilePermissions = 0x03;

/// Wraps `kAudioFileFlagsEraseFile`.
pub const AUDIO_FILE_FLAGS_ERASE_FILE: AudioFileFlags = 1;

/// Wraps `kAudioFileAIFFType`.
pub const AUDIO_FILE_AIFF_TYPE: AudioFileTypeId = fourcc(*b"AIFF");
/// Wraps `kAudioFileWaveType`.
pub const AUDIO_FILE_WAVE_TYPE: AudioFileTypeId = fourcc(*b"WAVE");
/// Wraps `kAudioFileM4AType`.
pub const AUDIO_FILE_M4A_TYPE: AudioFileTypeId = fourcc(*b"m4af");
/// Wraps `kAudioFileCAFType`.
pub const AUDIO_FILE_CAF_TYPE: AudioFileTypeId = fourcc(*b"caff");
/// Wraps `kAudioFileMIDIType`.
pub const AUDIO_FILE_MIDI_TYPE: AudioFileTypeId = fourcc(*b"Midi");
/// Wraps `kCAFFileMagic`.
pub const CAF_FILE_MAGIC: u32 = fourcc(*b"caff");

/// Wraps `kAudioFilePropertyDataFormat`.
pub const AUDIO_FILE_PROPERTY_DATA_FORMAT: AudioFilePropertyId = fourcc(*b"dfmt");
/// Wraps `kAudioFilePropertyMagicCookieData`.
pub const AUDIO_FILE_PROPERTY_MAGIC_COOKIE_DATA: AudioFilePropertyId = fourcc(*b"mgic");
/// Wraps `kAudioFilePropertyAudioDataPacketCount`.
pub const AUDIO_FILE_PROPERTY_AUDIO_DATA_PACKET_COUNT: AudioFilePropertyId = fourcc(*b"pcnt");
/// Wraps `kAudioFilePropertyMaximumPacketSize`.
pub const AUDIO_FILE_PROPERTY_MAXIMUM_PACKET_SIZE: AudioFilePropertyId = fourcc(*b"psze");
/// Wraps `kAudioFilePropertyEstimatedDuration`.
pub const AUDIO_FILE_PROPERTY_ESTIMATED_DURATION: AudioFilePropertyId = fourcc(*b"edur");
/// Wraps `kAudioFilePropertyAudioDataByteCount`.
pub const AUDIO_FILE_PROPERTY_AUDIO_DATA_BYTE_COUNT: AudioFilePropertyId = fourcc(*b"bcnt");
/// Wraps `kAudioFilePropertyDataOffset`.
pub const AUDIO_FILE_PROPERTY_DATA_OFFSET: AudioFilePropertyId = fourcc(*b"doff");

/// Wraps `kExtAudioFilePropertyFileDataFormat`.
pub const EXT_AUDIO_FILE_PROPERTY_FILE_DATA_FORMAT: ExtAudioFilePropertyId = fourcc(*b"ffmt");
/// Wraps `kExtAudioFilePropertyClientDataFormat`.
pub const EXT_AUDIO_FILE_PROPERTY_CLIENT_DATA_FORMAT: ExtAudioFilePropertyId = fourcc(*b"cfmt");
/// Wraps `kExtAudioFilePropertyAudioConverter`.
pub const EXT_AUDIO_FILE_PROPERTY_AUDIO_CONVERTER: ExtAudioFilePropertyId = fourcc(*b"acnv");
/// Wraps `kExtAudioFilePropertyAudioFile`.
pub const EXT_AUDIO_FILE_PROPERTY_AUDIO_FILE: ExtAudioFilePropertyId = fourcc(*b"afil");
/// Wraps `kExtAudioFilePropertyFileLengthFrames`.
pub const EXT_AUDIO_FILE_PROPERTY_FILE_LENGTH_FRAMES: ExtAudioFilePropertyId = fourcc(*b"#frm");

/// Wraps `kAudioConverterPropertyEncodeBitRate`.
pub const AUDIO_CONVERTER_PROPERTY_ENCODE_BIT_RATE: AudioConverterPropertyId = fourcc(*b"brat");
/// Wraps `kAudioConverterPropertyCurrentOutputStreamDescription`.
pub const AUDIO_CONVERTER_PROPERTY_CURRENT_OUTPUT_STREAM_DESCRIPTION: AudioConverterPropertyId =
    fourcc(*b"acod");
/// Wraps `kAudioConverterPropertyCurrentInputStreamDescription`.
pub const AUDIO_CONVERTER_PROPERTY_CURRENT_INPUT_STREAM_DESCRIPTION: AudioConverterPropertyId =
    fourcc(*b"acid");
/// Wraps `kAudioConverterPropertyMaximumOutputPacketSize`.
pub const AUDIO_CONVERTER_PROPERTY_MAXIMUM_OUTPUT_PACKET_SIZE: AudioConverterPropertyId =
    fourcc(*b"xops");
/// Wraps `kAudioConverterPropertyCalculateInputBufferSize`.
pub const AUDIO_CONVERTER_PROPERTY_CALCULATE_INPUT_BUFFER_SIZE: AudioConverterPropertyId =
    fourcc(*b"cibs");
/// Wraps `kAudioConverterPropertyCalculateOutputBufferSize`.
pub const AUDIO_CONVERTER_PROPERTY_CALCULATE_OUTPUT_BUFFER_SIZE: AudioConverterPropertyId =
    fourcc(*b"cobs");
/// Wraps `kAudioConverterPropertyPrimeMethod`.
pub const AUDIO_CONVERTER_PROPERTY_PRIME_METHOD: AudioConverterPropertyId = fourcc(*b"prmm");
/// Wraps `kAudioConverterPropertyPrimeInfo`.
pub const AUDIO_CONVERTER_PROPERTY_PRIME_INFO: AudioConverterPropertyId = fourcc(*b"prim");
/// Wraps `kAudioConverterPropertyApplicableEncodeBitRates`.
pub const AUDIO_CONVERTER_PROPERTY_APPLICABLE_ENCODE_BIT_RATES: AudioConverterPropertyId =
    fourcc(*b"aebr");
/// Wraps `kAudioConverterPropertyApplicableEncodeSampleRates`.
pub const AUDIO_CONVERTER_PROPERTY_APPLICABLE_ENCODE_SAMPLE_RATES: AudioConverterPropertyId =
    fourcc(*b"aesr");

/// Wraps `kAudioFormatLinearPCM`.
pub const AUDIO_FORMAT_LINEAR_PCM: AudioFormatId = fourcc(*b"lpcm");
/// Wraps `kAudioFormatAppleIMA4`.
pub const AUDIO_FORMAT_APPLE_IMA4: AudioFormatId = fourcc(*b"ima4");
/// Wraps `kAudioFormatMpeg4AAC`.
pub const AUDIO_FORMAT_MPEG4_AAC: AudioFormatId = fourcc(*b"aac ");

/// Wraps `kAudioFormatFlagIsFloat`.
pub const AUDIO_FORMAT_FLAG_IS_FLOAT: AudioFormatFlags = 1 << 0;
/// Wraps `kAudioFormatFlagIsBigEndian`.
pub const AUDIO_FORMAT_FLAG_IS_BIG_ENDIAN: AudioFormatFlags = 1 << 1;
/// Wraps `kAudioFormatFlagIsSignedInteger`.
pub const AUDIO_FORMAT_FLAG_IS_SIGNED_INTEGER: AudioFormatFlags = 1 << 2;
/// Wraps `kAudioFormatFlagIsPacked`.
pub const AUDIO_FORMAT_FLAG_IS_PACKED: AudioFormatFlags = 1 << 3;
/// Wraps `kAudioFormatFlagIsNonInterleaved`.
pub const AUDIO_FORMAT_FLAG_IS_NON_INTERLEAVED: AudioFormatFlags = 1 << 5;
/// Wraps `kAudioFormatFlagsNativeEndian`.
pub const AUDIO_FORMAT_FLAGS_NATIVE_ENDIAN: AudioFormatFlags = if cfg!(target_endian = "big") {
    AUDIO_FORMAT_FLAG_IS_BIG_ENDIAN
} else {
    0
};

/// Wraps `kLinearPCMFormatFlagIsFloat`.
pub const LINEAR_PCM_FORMAT_FLAG_IS_FLOAT: AudioFormatFlags = AUDIO_FORMAT_FLAG_IS_FLOAT;
/// Wraps `kLinearPCMFormatFlagIsBigEndian`.
pub const LINEAR_PCM_FORMAT_FLAG_IS_BIG_ENDIAN: AudioFormatFlags = AUDIO_FORMAT_FLAG_IS_BIG_ENDIAN;
/// Wraps `kLinearPCMFormatFlagIsSignedInteger`.
pub const LINEAR_PCM_FORMAT_FLAG_IS_SIGNED_INTEGER: AudioFormatFlags =
    AUDIO_FORMAT_FLAG_IS_SIGNED_INTEGER;
/// Wraps `kLinearPCMFormatFlagIsPacked`.
pub const LINEAR_PCM_FORMAT_FLAG_IS_PACKED: AudioFormatFlags = AUDIO_FORMAT_FLAG_IS_PACKED;
/// Wraps `kLinearPCMFormatFlagIsNonInterleaved`.
pub const LINEAR_PCM_FORMAT_FLAG_IS_NON_INTERLEAVED: AudioFormatFlags =
    AUDIO_FORMAT_FLAG_IS_NON_INTERLEAVED;

/// Wraps `kAudioFormatPropertyFormatInfo`.
pub const AUDIO_FORMAT_PROPERTY_FORMAT_INFO: AudioFormatPropertyId = fourcc(*b"fmti");
/// Wraps `kAudioFormatPropertyEncodeFormatIds`.
pub const AUDIO_FORMAT_PROPERTY_ENCODE_FORMAT_IDS: AudioFormatPropertyId = fourcc(*b"acof");
/// Wraps `kAudioFormatPropertyDecodeFormatIds`.
pub const AUDIO_FORMAT_PROPERTY_DECODE_FORMAT_IDS: AudioFormatPropertyId = fourcc(*b"acif");
/// Wraps `kAudioFormatPropertyFormatIsVBR`.
pub const AUDIO_FORMAT_PROPERTY_FORMAT_IS_VBR: AudioFormatPropertyId = fourcc(*b"fvbr");
/// Wraps `kAudioFormatPropertyFormatIsExternallyFramed`.
pub const AUDIO_FORMAT_PROPERTY_FORMAT_IS_EXTERNALLY_FRAMED: AudioFormatPropertyId =
    fourcc(*b"fexf");
/// Wraps `kAudioFormatPropertyFormatEmploysDependentPackets`.
pub const AUDIO_FORMAT_PROPERTY_FORMAT_EMPLOYS_DEPENDENT_PACKETS: AudioFormatPropertyId =
    fourcc(*b"fdep");
/// Wraps `kAudioFormatPropertyFormatName`.
pub const AUDIO_FORMAT_PROPERTY_FORMAT_NAME: AudioFormatPropertyId = fourcc(*b"fnam");
/// Wraps `kAudioFormatPropertyFormatList`.
pub const AUDIO_FORMAT_PROPERTY_FORMAT_LIST: AudioFormatPropertyId = fourcc(*b"flst");
/// Wraps `kAudioFormatPropertyOutputFormatList`.
pub const AUDIO_FORMAT_PROPERTY_OUTPUT_FORMAT_LIST: AudioFormatPropertyId = fourcc(*b"ofls");
/// Wraps `kAudioFormatPropertyFirstPlayableFormatFromList`.
pub const AUDIO_FORMAT_PROPERTY_FIRST_PLAYABLE_FORMAT_FROM_LIST: AudioFormatPropertyId =
    fourcc(*b"fpfl");
/// Wraps `kAudioFormatPropertyEncoders`.
pub const AUDIO_FORMAT_PROPERTY_ENCODERS: AudioFormatPropertyId = fourcc(*b"aven");
/// Wraps `kAudioFormatPropertyDecoders`.
pub const AUDIO_FORMAT_PROPERTY_DECODERS: AudioFormatPropertyId = fourcc(*b"avde");
/// Wraps `kAudioFormatPropertyAvailableEncodeBitRates`.
pub const AUDIO_FORMAT_PROPERTY_AVAILABLE_ENCODE_BIT_RATES: AudioFormatPropertyId =
    fourcc(*b"aebr");
/// Wraps `kAudioFormatPropertyAvailableEncodeSampleRates`.
pub const AUDIO_FORMAT_PROPERTY_AVAILABLE_ENCODE_SAMPLE_RATES: AudioFormatPropertyId =
    fourcc(*b"aesr");
/// Wraps `kAudioFormatPropertyPanningMatrix`.
pub const AUDIO_FORMAT_PROPERTY_PANNING_MATRIX: AudioFormatPropertyId = fourcc(*b"panm");
/// Wraps `kAudioFormatPropertyBalanceFade`.
pub const AUDIO_FORMAT_PROPERTY_BALANCE_FADE: AudioFormatPropertyId = fourcc(*b"balf");

/// Wraps `kAudioComponentTypeOutput`.
pub const AUDIO_COMPONENT_TYPE_OUTPUT: u32 = fourcc(*b"auou");
/// Wraps `kAudioComponentTypeMusicDevice`.
pub const AUDIO_COMPONENT_TYPE_MUSIC_DEVICE: u32 = fourcc(*b"aumu");
/// Wraps `kAudioComponentTypeMusicEffect`.
pub const AUDIO_COMPONENT_TYPE_MUSIC_EFFECT: u32 = fourcc(*b"aumf");
/// Wraps `kAudioComponentTypeFormatConverter`.
pub const AUDIO_COMPONENT_TYPE_FORMAT_CONVERTER: u32 = fourcc(*b"aufc");
/// Wraps `kAudioComponentTypeEffect`.
pub const AUDIO_COMPONENT_TYPE_EFFECT: u32 = fourcc(*b"aufx");
/// Wraps `kAudioComponentTypeMixer`.
pub const AUDIO_COMPONENT_TYPE_MIXER: u32 = fourcc(*b"aumx");
/// Wraps `kAudioComponentTypePanner`.
pub const AUDIO_COMPONENT_TYPE_PANNER: u32 = fourcc(*b"aupn");
/// Wraps `kAudioComponentTypeGenerator`.
pub const AUDIO_COMPONENT_TYPE_GENERATOR: u32 = fourcc(*b"augn");
/// Wraps `kAudioComponentTypeOfflineEffect`.
pub const AUDIO_COMPONENT_TYPE_OFFLINE_EFFECT: u32 = fourcc(*b"auol");
/// Wraps `kAudioComponentTypeMIDIProcessor`.
pub const AUDIO_COMPONENT_TYPE_MIDI_PROCESSOR: u32 = fourcc(*b"aumi");
/// Wraps `kAudioComponentManufacturerApple`.
pub const AUDIO_COMPONENT_MANUFACTURER_APPLE: u32 = fourcc(*b"appl");
/// Wraps `kAudioComponentInstantiationLoadOutOfProcess`.
pub const AUDIO_COMPONENT_INSTANTIATION_LOAD_OUT_OF_PROCESS: AudioComponentInstantiationOptions = 1;
/// Wraps `kAudioComponentInstantiationLoadInProcess`.
pub const AUDIO_COMPONENT_INSTANTIATION_LOAD_IN_PROCESS: AudioComponentInstantiationOptions = 2;
/// Wraps `kAudioComponentValidationResultUnknown`.
pub const AUDIO_COMPONENT_VALIDATION_RESULT_UNKNOWN: AudioComponentValidationResult = 0;
/// Wraps `kAudioComponentValidationResultPassed`.
pub const AUDIO_COMPONENT_VALIDATION_RESULT_PASSED: AudioComponentValidationResult = 1;
/// Wraps `kAudioComponentValidationResultFailed`.
pub const AUDIO_COMPONENT_VALIDATION_RESULT_FAILED: AudioComponentValidationResult = 2;
/// Wraps `kAudioComponentValidationResultTimedOut`.
pub const AUDIO_COMPONENT_VALIDATION_RESULT_TIMED_OUT: AudioComponentValidationResult = 3;
/// Wraps `kAudioComponentValidationResultUnauthorizedErrorOpen`.
pub const AUDIO_COMPONENT_VALIDATION_RESULT_UNAUTHORIZED_ERROR_OPEN:
    AudioComponentValidationResult = 4;
/// Wraps `kAudioComponentValidationResultUnauthorizedErrorInit`.
pub const AUDIO_COMPONENT_VALIDATION_RESULT_UNAUTHORIZED_ERROR_INIT:
    AudioComponentValidationResult = 5;
/// Wraps `kAudioUnitSubtypeAUConverter`.
pub const AUDIO_UNIT_SUBTYPE_AU_CONVERTER: u32 = fourcc(*b"conv");
/// Wraps `kAudioUnitSubtypeGenericOutput`.
pub const AUDIO_UNIT_SUBTYPE_GENERIC_OUTPUT: u32 = fourcc(*b"genr");
/// Wraps `kAudioUnitSubtypeDefaultOutput`.
pub const AUDIO_UNIT_SUBTYPE_DEFAULT_OUTPUT: u32 = fourcc(*b"def ");

/// Wraps `kAudioUnitScopeGlobal`.
pub const AUDIO_UNIT_SCOPE_GLOBAL: AudioUnitScope = 0;
/// Wraps `kAudioUnitScopeInput`.
pub const AUDIO_UNIT_SCOPE_INPUT: AudioUnitScope = 1;
/// Wraps `kAudioUnitScopeOutput`.
pub const AUDIO_UNIT_SCOPE_OUTPUT: AudioUnitScope = 2;
/// Wraps `kAudioUnitScopeGroup`.
pub const AUDIO_UNIT_SCOPE_GROUP: AudioUnitScope = 3;

/// Wraps `kAudioUnitPropertyClassInfo`.
pub const AUDIO_UNIT_PROPERTY_CLASS_INFO: AudioUnitPropertyId = 0;
/// Wraps `kAudioUnitPropertyMakeConnection`.
pub const AUDIO_UNIT_PROPERTY_MAKE_CONNECTION: AudioUnitPropertyId = 1;
/// Wraps `kAudioUnitPropertySampleRate`.
pub const AUDIO_UNIT_PROPERTY_SAMPLE_RATE: AudioUnitPropertyId = 2;
/// Wraps `kAudioUnitPropertyParameterList`.
pub const AUDIO_UNIT_PROPERTY_PARAMETER_LIST: AudioUnitPropertyId = 3;
/// Wraps `kAudioUnitPropertyParameterInfo`.
pub const AUDIO_UNIT_PROPERTY_PARAMETER_INFO: AudioUnitPropertyId = 4;
/// Wraps `kAudioUnitPropertyCpuload`.
pub const AUDIO_UNIT_PROPERTY_CPULOAD: AudioUnitPropertyId = 6;
/// Wraps `kAudioUnitPropertyStreamFormat`.
pub const AUDIO_UNIT_PROPERTY_STREAM_FORMAT: AudioUnitPropertyId = 8;
/// Wraps `kAudioUnitPropertyElementCount`.
pub const AUDIO_UNIT_PROPERTY_ELEMENT_COUNT: AudioUnitPropertyId = 11;
/// Wraps `kAudioUnitPropertyLatency`.
pub const AUDIO_UNIT_PROPERTY_LATENCY: AudioUnitPropertyId = 12;
/// Wraps `kAudioUnitPropertySupportedNumChannels`.
pub const AUDIO_UNIT_PROPERTY_SUPPORTED_NUM_CHANNELS: AudioUnitPropertyId = 13;
/// Wraps `kAudioUnitPropertyParameterValueStrings`.
pub const AUDIO_UNIT_PROPERTY_PARAMETER_VALUE_STRINGS: AudioUnitPropertyId = 16;
/// Wraps `kAudioUnitPropertyAudioChannelLayout`.
pub const AUDIO_UNIT_PROPERTY_AUDIO_CHANNEL_LAYOUT: AudioUnitPropertyId = 19;
/// Wraps `kAudioUnitPropertyLastRenderError`.
pub const AUDIO_UNIT_PROPERTY_LAST_RENDER_ERROR: AudioUnitPropertyId = 22;
/// Wraps `kAudioUnitPropertySetRenderCallback`.
pub const AUDIO_UNIT_PROPERTY_SET_RENDER_CALLBACK: AudioUnitPropertyId = 23;
/// Wraps `kAudioUnitPropertyHostCallbacks`.
pub const AUDIO_UNIT_PROPERTY_HOST_CALLBACKS: AudioUnitPropertyId = 27;
/// Wraps `kAudioUnitPropertyParameterStringFromValue`.
pub const AUDIO_UNIT_PROPERTY_PARAMETER_STRING_FROM_VALUE: AudioUnitPropertyId = 33;
/// Wraps `kAudioUnitPropertyParameterClumpName`.
pub const AUDIO_UNIT_PROPERTY_PARAMETER_CLUMP_NAME: AudioUnitPropertyId = 35;
/// Wraps `kAudioUnitPropertyPresentPreset`.
pub const AUDIO_UNIT_PROPERTY_PRESENT_PRESET: AudioUnitPropertyId = 36;
/// Wraps `kAudioUnitPropertyOfflineRender`.
pub const AUDIO_UNIT_PROPERTY_OFFLINE_RENDER: AudioUnitPropertyId = 37;
/// Wraps `kAudioUnitPropertyParameterValueFromString`.
pub const AUDIO_UNIT_PROPERTY_PARAMETER_VALUE_FROM_STRING: AudioUnitPropertyId = 38;
/// Wraps `kAudioUnitPropertyPresentationLatency`.
pub const AUDIO_UNIT_PROPERTY_PRESENTATION_LATENCY: AudioUnitPropertyId = 40;
/// Wraps `kAudioUnitPropertyDependentParameters`.
pub const AUDIO_UNIT_PROPERTY_DEPENDENT_PARAMETERS: AudioUnitPropertyId = 45;
/// Wraps `kAudioUnitPropertyInputSamplesInOutput`.
pub const AUDIO_UNIT_PROPERTY_INPUT_SAMPLES_IN_OUTPUT: AudioUnitPropertyId = 49;
/// Wraps `kAudioUnitPropertyClassInfoFromDocument`.
pub const AUDIO_UNIT_PROPERTY_CLASS_INFO_FROM_DOCUMENT: AudioUnitPropertyId = 50;
/// Wraps `kAudioOutputUnitPropertyEnableIO`.
pub const AUDIO_OUTPUT_UNIT_PROPERTY_ENABLE_IO: AudioUnitPropertyId = 2003;
/// Wraps `kAudioOutputUnitPropertyMIDICallbacks`.
pub const AUDIO_OUTPUT_UNIT_PROPERTY_MIDI_CALLBACKS: AudioUnitPropertyId = 2010;

/// Wraps `kAudioQueuePropertyIsRunning`.
pub const AUDIO_QUEUE_PROPERTY_IS_RUNNING: AudioQueuePropertyId = fourcc(*b"aqrn");
/// Wraps `kAudioQueueDevicePropertySampleRate`.
pub const AUDIO_QUEUE_DEVICE_PROPERTY_SAMPLE_RATE: AudioQueuePropertyId = fourcc(*b"aqsr");
/// Wraps `kAudioQueueDevicePropertyNumberChannels`.
pub const AUDIO_QUEUE_DEVICE_PROPERTY_NUMBER_CHANNELS: AudioQueuePropertyId = fourcc(*b"aqdc");
/// Wraps `kAudioQueuePropertyCurrentDevice`.
pub const AUDIO_QUEUE_PROPERTY_CURRENT_DEVICE: AudioQueuePropertyId = fourcc(*b"aqcd");
/// Wraps `kAudioQueuePropertyMagicCookie`.
pub const AUDIO_QUEUE_PROPERTY_MAGIC_COOKIE: AudioQueuePropertyId = fourcc(*b"aqmc");
/// Wraps `kAudioQueuePropertyMaximumOutputPacketSize`.
pub const AUDIO_QUEUE_PROPERTY_MAXIMUM_OUTPUT_PACKET_SIZE: AudioQueuePropertyId = fourcc(*b"xops");
/// Wraps `kAudioQueuePropertyStreamDescription`.
pub const AUDIO_QUEUE_PROPERTY_STREAM_DESCRIPTION: AudioQueuePropertyId = fourcc(*b"aqft");
/// Wraps `kAudioQueueParamVolume`.
pub const AUDIO_QUEUE_PARAM_VOLUME: AudioQueueParameterId = 1;
/// Wraps `kAudioQueueParamVolumeRampTime`.
pub const AUDIO_QUEUE_PARAM_VOLUME_RAMP_TIME: AudioQueueParameterId = 4;
/// Wraps `kAudioQueueErrCannotStart`.
pub const AUDIO_QUEUE_ERR_CANNOT_START: OSStatus = -66681;
/// Wraps `kAudioQueueErrCannotStartYet`.
pub const AUDIO_QUEUE_ERR_CANNOT_START_YET: OSStatus = -66665;

/// Wraps `kAudioServicesPropertyIsUISound`.
pub const AUDIO_SERVICES_PROPERTY_IS_UI_SOUND: AudioServicesPropertyId = fourcc(*b"isui");
/// Wraps `kAudioServicesPropertyCompletePlaybackIfAppDies`.
pub const AUDIO_SERVICES_PROPERTY_COMPLETE_PLAYBACK_IF_APP_DIES: AudioServicesPropertyId =
    fourcc(*b"ifdi");
/// Wraps `kSystemSoundUserPreferredAlert`.
pub const SYSTEM_SOUND_USER_PREFERRED_ALERT: SystemSoundId = 0x0000_1000;
/// Wraps `kSystemSoundFlashScreen`.
pub const SYSTEM_SOUND_FLASH_SCREEN: SystemSoundId = 0x0000_0FFE;

/// Wraps `kMusicEventTypeNull`.
pub const MUSIC_EVENT_TYPE_NULL: MusicEventType = 0;
/// Wraps `kMusicEventTypeExtendedNote`.
pub const MUSIC_EVENT_TYPE_EXTENDED_NOTE: MusicEventType = 1;
/// Wraps `kMusicEventTypeExtendedControl`.
pub const MUSIC_EVENT_TYPE_EXTENDED_CONTROL: MusicEventType = 2;
/// Wraps `kMusicEventTypeExtendedTempo`.
pub const MUSIC_EVENT_TYPE_EXTENDED_TEMPO: MusicEventType = 3;
/// Wraps `kMusicEventTypeUser`.
pub const MUSIC_EVENT_TYPE_USER: MusicEventType = 4;
/// Wraps `kMusicEventTypeMeta`.
pub const MUSIC_EVENT_TYPE_META: MusicEventType = 5;
/// Wraps `kMusicEventTypeMIDINoteMessage`.
pub const MUSIC_EVENT_TYPE_MIDI_NOTE_MESSAGE: MusicEventType = 6;
/// Wraps `kMusicEventTypeMIDIChannelMessage`.
pub const MUSIC_EVENT_TYPE_MIDI_CHANNEL_MESSAGE: MusicEventType = 7;
/// Wraps `kMusicEventTypeMIDIRawData`.
pub const MUSIC_EVENT_TYPE_MIDI_RAW_DATA: MusicEventType = 8;
/// Wraps `kMusicEventTypeParameter`.
pub const MUSIC_EVENT_TYPE_PARAMETER: MusicEventType = 9;
/// Wraps `kMusicEventTypeAUPreset`.
pub const MUSIC_EVENT_TYPE_AU_PRESET: MusicEventType = 10;
/// Wraps `kMusicSequenceTypeBeats`.
pub const MUSIC_SEQUENCE_TYPE_BEATS: MusicSequenceType = fourcc(*b"beat");
/// Wraps `kMusicSequenceTypeSeconds`.
pub const MUSIC_SEQUENCE_TYPE_SECONDS: MusicSequenceType = fourcc(*b"secs");
/// Wraps `kMusicSequenceTypeSamples`.
pub const MUSIC_SEQUENCE_TYPE_SAMPLES: MusicSequenceType = fourcc(*b"samp");
/// Wraps `kMusicSequenceFileAnyType`.
pub const MUSIC_SEQUENCE_FILE_ANY_TYPE: MusicSequenceFileTypeId = 0;
/// Wraps `kMusicSequenceFileMIDIType`.
pub const MUSIC_SEQUENCE_FILE_MIDI_TYPE: MusicSequenceFileTypeId = fourcc(*b"midi");
/// Wraps `kMusicSequenceFileImelodyType`.
pub const MUSIC_SEQUENCE_FILE_IMELODY_TYPE: MusicSequenceFileTypeId = fourcc(*b"imel");
/// Wraps `kMusicSequenceFileFlagsDefault`.
pub const MUSIC_SEQUENCE_FILE_FLAGS_DEFAULT: MusicSequenceFileFlags = 0;
/// Wraps `kMusicSequenceFileFlagsEraseFile`.
pub const MUSIC_SEQUENCE_FILE_FLAGS_ERASE_FILE: MusicSequenceFileFlags = 1;
/// Wraps `kMusicSequenceLoadSmfPreserveTracks`.
pub const MUSIC_SEQUENCE_LOAD_SMF_PRESERVE_TRACKS: MusicSequenceLoadFlags = 0;
/// Wraps `kMusicSequenceLoadSmfChannelsToTracks`.
pub const MUSIC_SEQUENCE_LOAD_SMF_CHANNELS_TO_TRACKS: MusicSequenceLoadFlags = 1 << 0;
/// Wraps `kSequenceTrackPropertyLoopInfo`.
pub const SEQUENCE_TRACK_PROPERTY_LOOP_INFO: u32 = 0;
/// Wraps `kSequenceTrackPropertyOffsetTime`.
pub const SEQUENCE_TRACK_PROPERTY_OFFSET_TIME: u32 = 1;
/// Wraps `kSequenceTrackPropertyMuteStatus`.
pub const SEQUENCE_TRACK_PROPERTY_MUTE_STATUS: u32 = 2;
/// Wraps `kSequenceTrackPropertySoloStatus`.
pub const SEQUENCE_TRACK_PROPERTY_SOLO_STATUS: u32 = 3;
/// Wraps `kSequenceTrackPropertyAutomatedParameters`.
pub const SEQUENCE_TRACK_PROPERTY_AUTOMATED_PARAMETERS: u32 = 4;
/// Wraps `kSequenceTrackPropertyTrackLength`.
pub const SEQUENCE_TRACK_PROPERTY_TRACK_LENGTH: u32 = 5;
/// Wraps `kSequenceTrackPropertyTimeResolution`.
pub const SEQUENCE_TRACK_PROPERTY_TIME_RESOLUTION: u32 = 6;
/// Wraps `kAudioToolboxErrNoSequence`.
pub const AUDIO_TOOLBOX_ERR_NO_SEQUENCE: OSStatus = -10854;

/// Wraps `kAudioFileStreamPropertyFlagPropertyIsCached`.
pub const AUDIO_FILE_STREAM_PROPERTY_FLAG_PROPERTY_IS_CACHED: u32 = 1;
/// Wraps `kAudioFileStreamPropertyFlagCacheProperty`.
pub const AUDIO_FILE_STREAM_PROPERTY_FLAG_CACHE_PROPERTY: u32 = 2;
/// Wraps `kAudioFileStreamParseFlagDiscontinuity`.
pub const AUDIO_FILE_STREAM_PARSE_FLAG_DISCONTINUITY: AudioFileStreamParseFlags = 1;
/// Wraps `kAudioFileStreamSeekFlagOffsetIsEstimated`.
pub const AUDIO_FILE_STREAM_SEEK_FLAG_OFFSET_IS_ESTIMATED: AudioFileStreamSeekFlags = 1;
/// Wraps `kAudioFileStreamPropertyReadyToProducePackets`.
pub const AUDIO_FILE_STREAM_PROPERTY_READY_TO_PRODUCE_PACKETS: AudioFileStreamPropertyId =
    fourcc(*b"redy");
/// Wraps `kAudioFileStreamPropertyFileFormat`.
pub const AUDIO_FILE_STREAM_PROPERTY_FILE_FORMAT: AudioFileStreamPropertyId = fourcc(*b"ffmt");
/// Wraps `kAudioFileStreamPropertyDataFormat`.
pub const AUDIO_FILE_STREAM_PROPERTY_DATA_FORMAT: AudioFileStreamPropertyId = fourcc(*b"dfmt");
/// Wraps `kAudioFileStreamPropertyMagicCookieData`.
pub const AUDIO_FILE_STREAM_PROPERTY_MAGIC_COOKIE_DATA: AudioFileStreamPropertyId =
    fourcc(*b"mgic");
/// Wraps `kAudioFileStreamPropertyAudioDataByteCount`.
pub const AUDIO_FILE_STREAM_PROPERTY_AUDIO_DATA_BYTE_COUNT: AudioFileStreamPropertyId =
    fourcc(*b"bcnt");
/// Wraps `kAudioFileStreamPropertyAudioDataPacketCount`.
pub const AUDIO_FILE_STREAM_PROPERTY_AUDIO_DATA_PACKET_COUNT: AudioFileStreamPropertyId =
    fourcc(*b"pcnt");
/// Wraps `kAudioFileStreamPropertyMaximumPacketSize`.
pub const AUDIO_FILE_STREAM_PROPERTY_MAXIMUM_PACKET_SIZE: AudioFileStreamPropertyId =
    fourcc(*b"psze");
/// Wraps `kAudioFileStreamPropertyDataOffset`.
pub const AUDIO_FILE_STREAM_PROPERTY_DATA_OFFSET: AudioFileStreamPropertyId = fourcc(*b"doff");
/// Wraps `kAudioFileStreamPropertyBitRate`.
pub const AUDIO_FILE_STREAM_PROPERTY_BIT_RATE: AudioFileStreamPropertyId = fourcc(*b"brat");

/// Wraps `kAudioFileComponentCanRead`.
pub const AUDIO_FILE_COMPONENT_CAN_READ: AudioFileComponentPropertyId = fourcc(*b"cnrd");
/// Wraps `kAudioFileComponentCanWrite`.
pub const AUDIO_FILE_COMPONENT_CAN_WRITE: AudioFileComponentPropertyId = fourcc(*b"cnwr");
/// Wraps `kAudioFileComponentFileTypeName`.
pub const AUDIO_FILE_COMPONENT_FILE_TYPE_NAME: AudioFileComponentPropertyId = fourcc(*b"ftnm");
/// Wraps `kAudioFileComponentUtisForType`.
pub const AUDIO_FILE_COMPONENT_UTIS_FOR_TYPE: AudioFileComponentPropertyId = fourcc(*b"futi");
/// Wraps `kAudioFileComponentMimeTypesForType`.
pub const AUDIO_FILE_COMPONENT_MIME_TYPES_FOR_TYPE: AudioFileComponentPropertyId = fourcc(*b"fmim");
/// Wraps `kAudioFileComponentExtensionsForType`.
pub const AUDIO_FILE_COMPONENT_EXTENSIONS_FOR_TYPE: AudioFileComponentPropertyId = fourcc(*b"fext");
/// Wraps `kAudioFileComponentAvailableFormatIds`.
pub const AUDIO_FILE_COMPONENT_AVAILABLE_FORMAT_IDS: AudioFileComponentPropertyId =
    fourcc(*b"fmid");
/// Wraps `kAudioFileComponentAvailableStreamDescriptionsForFormat`.
pub const AUDIO_FILE_COMPONENT_AVAILABLE_STREAM_DESCRIPTIONS_FOR_FORMAT:
    AudioFileComponentPropertyId = fourcc(*b"sdid");
/// Wraps `kAudioFileComponentFastDispatchTable`.
pub const AUDIO_FILE_COMPONENT_FAST_DISPATCH_TABLE: AudioFileComponentPropertyId = fourcc(*b"fdft");
/// Wraps `kAudioFileComponentHfsTypeCodesForType`.
pub const AUDIO_FILE_COMPONENT_HFS_TYPE_CODES_FOR_TYPE: AudioFileComponentPropertyId =
    fourcc(*b"fhfs");

/// Wraps `kAVAudioOtherFormat`.
pub const AV_AUDIO_OTHER_FORMAT: AVAudioCommonFormat = 0;
/// Wraps `kAVAudioPCMFormatFloat32`.
pub const AV_AUDIO_PCM_FORMAT_FLOAT32: AVAudioCommonFormat = 1;
/// Wraps `kAVAudioPCMFormatFloat64`.
pub const AV_AUDIO_PCM_FORMAT_FLOAT64: AVAudioCommonFormat = 2;
/// Wraps `kAVAudioPCMFormatInt16`.
pub const AV_AUDIO_PCM_FORMAT_INT16: AVAudioCommonFormat = 3;
/// Wraps `kAVAudioPCMFormatInt32`.
pub const AV_AUDIO_PCM_FORMAT_INT32: AVAudioCommonFormat = 4;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Wraps `AudioComponentDescription`.
pub struct AudioComponentDescription {
    /// Wraps `AudioClassDescription`.
    pub component_type: OSType,
    /// Wraps `AudioClassDescription`.
    pub component_sub_type: OSType,
    /// Wraps `AudioClassDescription`.
    pub component_manufacturer: OSType,
    /// Wraps `AudioClassDescription`.
    pub component_flags: u32,
    /// Wraps `AudioClassDescription`.
    pub component_flags_mask: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Wraps `AudioClassDescription`.
pub struct AudioClassDescription {
    /// Wraps `AudioStreamBasicDescription`.
    pub mType: OSType,
    /// Wraps `AudioStreamBasicDescription`.
    pub mSubType: OSType,
    /// Wraps `AudioStreamBasicDescription`.
    pub mManufacturer: OSType,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
/// Wraps `AudioStreamBasicDescription`.
pub struct AudioStreamBasicDescription {
    /// Wraps `AudioPanningInfo`.
    pub mSampleRate: f64,
    /// Wraps `AudioPanningInfo`.
    pub mFormatID: AudioFormatId,
    /// Wraps `AudioPanningInfo`.
    pub mFormatFlags: AudioFormatFlags,
    /// Wraps `AudioPanningInfo`.
    pub mBytesPerPacket: u32,
    /// Wraps `AudioPanningInfo`.
    pub mFramesPerPacket: u32,
    /// Wraps `AudioPanningInfo`.
    pub mBytesPerFrame: u32,
    /// Wraps `AudioPanningInfo`.
    pub mChannelsPerFrame: u32,
    /// Wraps `AudioPanningInfo`.
    pub mBitsPerChannel: u32,
    /// Wraps `AudioPanningInfo`.
    pub mReserved: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
/// Wraps `AudioPanningInfo`.
pub struct AudioPanningInfo {
    pub mPanningMode: AudioPanningMode,
    pub mCoordinateFlags: u32,
    pub mCoordinates: [f32; 3],
    /// Wraps `AudioBalanceFade`.
    pub mGainScale: f32,
    /// Wraps `AudioBalanceFade`.
    pub mOutputChannelMap: *const c_void,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
/// Wraps `AudioBalanceFade`.
pub struct AudioBalanceFade {
    /// Wraps `AudioFormatInfo`.
    pub mLeftRightBalance: f32,
    /// Wraps `AudioFormatInfo`.
    pub mBackFrontFade: f32,
    /// Wraps `AudioFormatInfo`.
    pub mType: AudioBalanceFadeType,
    /// Wraps `AudioFormatInfo`.
    pub mChannelLayout: *const c_void,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
/// Wraps `AudioFormatInfo`.
pub struct AudioFormatInfo {
    /// Wraps `AudioFormatListItem`.
    pub mASBD: AudioStreamBasicDescription,
    /// Wraps `AudioFormatListItem`.
    pub mMagicCookie: *const c_void,
    /// Wraps `AudioFormatListItem`.
    pub mMagicCookieSize: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
/// Wraps `AudioFormatListItem`.
pub struct AudioFormatListItem {
    /// Wraps `ExtendedAudioFormatInfo`.
    pub mASBD: AudioStreamBasicDescription,
    /// Wraps `ExtendedAudioFormatInfo`.
    pub mChannelLayoutTag: AudioChannelLayoutTag,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
/// Wraps `ExtendedAudioFormatInfo`.
pub struct ExtendedAudioFormatInfo {
    /// Wraps `AudioValueRange`.
    pub mASBD: AudioStreamBasicDescription,
    /// Wraps `AudioValueRange`.
    pub mMagicCookie: *const c_void,
    /// Wraps `AudioValueRange`.
    pub mMagicCookieSize: u32,
    /// Wraps `AudioValueRange`.
    pub mClassDescription: AudioClassDescription,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
/// Wraps `AudioValueRange`.
pub struct AudioValueRange {
    /// Wraps `AudioBuffer`.
    pub mMinimum: f64,
    /// Wraps `AudioBuffer`.
    pub mMaximum: f64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Wraps `AudioBuffer`.
pub struct AudioBuffer {
    /// Wraps `AudioBufferList`.
    pub mNumberChannels: u32,
    /// Wraps `AudioBufferList`.
    pub mDataByteSize: u32,
    /// Wraps `AudioBufferList`.
    pub mData: *mut c_void,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
/// Wraps `AudioBufferList`.
pub struct AudioBufferList<const N: usize> {
    pub mNumberBuffers: u32,
    pub mBuffers: [AudioBuffer; N],
}

/// Wraps `AudioBufferList1`.
pub type AudioBufferList1 = AudioBufferList<1>;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
/// Wraps `AudioStreamPacketDescription`.
pub struct AudioStreamPacketDescription {
    /// Wraps `SMPTETime`.
    pub mStartOffset: i64,
    /// Wraps `SMPTETime`.
    pub mVariableFramesInPacket: u32,
    /// Wraps `SMPTETime`.
    pub mDataByteSize: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
/// Wraps `SMPTETime`.
pub struct SMPTETime {
    /// Wraps `AudioTimeStamp`.
    pub mSubframes: i16,
    /// Wraps `AudioTimeStamp`.
    pub mSubframeDivisor: i16,
    /// Wraps `AudioTimeStamp`.
    pub mCounter: u32,
    /// Wraps `AudioTimeStamp`.
    pub mType: u32,
    /// Wraps `AudioTimeStamp`.
    pub mFlags: u32,
    /// Wraps `AudioTimeStamp`.
    pub mHours: i16,
    /// Wraps `AudioTimeStamp`.
    pub mMinutes: i16,
    /// Wraps `AudioTimeStamp`.
    pub mSeconds: i16,
    /// Wraps `AudioTimeStamp`.
    pub mFrames: i16,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
/// Wraps `AudioTimeStamp`.
pub struct AudioTimeStamp {
    /// Wraps `AudioFileSMPTETime`.
    pub mSampleTime: f64,
    /// Wraps `AudioFileSMPTETime`.
    pub mHostTime: u64,
    /// Wraps `AudioFileSMPTETime`.
    pub mRateScalar: f64,
    /// Wraps `AudioFileSMPTETime`.
    pub mWordClockTime: u64,
    /// Wraps `AudioFileSMPTETime`.
    pub mSMPTETime: SMPTETime,
    /// Wraps `AudioFileSMPTETime`.
    pub mFlags: u32,
    /// Wraps `AudioFileSMPTETime`.
    pub mReserved: u32,
}

/// Wraps `AudioFileSMPTETime`.
pub type AudioFileSmpteTime = SMPTETime;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
/// Wraps `AudioFileMarker`.
pub struct AudioFileMarker {
    /// Wraps `AudioFileMarkerList`.
    pub mFramePosition: f64,
    /// Wraps `AudioFileMarkerList`.
    pub mName: CFStringRef,
    /// Wraps `AudioFileMarkerList`.
    pub mMarkerID: i32,
    /// Wraps `AudioFileMarkerList`.
    pub mSMPTETime: AudioFileSmpteTime,
    /// Wraps `AudioFileMarkerList`.
    pub mType: u32,
    /// Wraps `AudioFileMarkerList`.
    pub mReserved: u16,
    /// Wraps `AudioFileMarkerList`.
    pub mChannel: u16,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
/// Wraps `AudioFileMarkerList`.
pub struct AudioFileMarkerList {
    pub mSMPTE_TimeType: u32,
    pub mNumberMarkers: u32,
    pub mMarkers: [AudioFileMarker; 1],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
/// Wraps `AudioFileRegion`.
pub struct AudioFileRegion {
    pub mRegionID: u32,
    pub mName: CFStringRef,
    pub mFlags: AudioFileRegionFlags,
    pub mNumberMarkers: u32,
    pub mMarkers: [AudioFileMarker; 1],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
/// Wraps `AudioFileRegionList`.
pub struct AudioFileRegionList {
    pub mSMPTE_TimeType: u32,
    pub mNumberRegions: u32,
    pub mRegions: [AudioFileRegion; 1],
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Wraps `AudioFramePacketTranslation`.
pub struct AudioFramePacketTranslation {
    /// Wraps `AudioBytePacketTranslation`.
    pub mFrame: i64,
    /// Wraps `AudioBytePacketTranslation`.
    pub mPacket: i64,
    /// Wraps `AudioBytePacketTranslation`.
    pub mFrameOffsetInPacket: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Wraps `AudioBytePacketTranslation`.
pub struct AudioBytePacketTranslation {
    /// Wraps `AudioFilePacketTableInfo`.
    pub mByte: i64,
    /// Wraps `AudioFilePacketTableInfo`.
    pub mPacket: i64,
    /// Wraps `AudioFilePacketTableInfo`.
    pub mByteOffsetInPacket: u32,
    /// Wraps `AudioFilePacketTableInfo`.
    pub mFlags: AudioBytePacketTranslationFlags,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Wraps `AudioFilePacketTableInfo`.
pub struct AudioFilePacketTableInfo {
    /// Wraps `AudioPacketRangeByteCountTranslation`.
    pub mNumberValidFrames: i64,
    /// Wraps `AudioPacketRangeByteCountTranslation`.
    pub mPrimingFrames: i32,
    /// Wraps `AudioPacketRangeByteCountTranslation`.
    pub mRemainderFrames: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Wraps `AudioPacketRangeByteCountTranslation`.
pub struct AudioPacketRangeByteCountTranslation {
    /// Wraps `AudioPacketRollDistanceTranslation`.
    pub mPacket: i64,
    /// Wraps `AudioPacketRollDistanceTranslation`.
    pub mPacketCount: i64,
    /// Wraps `AudioPacketRollDistanceTranslation`.
    pub mByteCountUpperBound: i64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Wraps `AudioPacketRollDistanceTranslation`.
pub struct AudioPacketRollDistanceTranslation {
    /// Wraps `AudioIndependentPacketTranslation`.
    pub mPacket: i64,
    /// Wraps `AudioIndependentPacketTranslation`.
    pub mRollDistance: i64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Wraps `AudioIndependentPacketTranslation`.
pub struct AudioIndependentPacketTranslation {
    /// Wraps `AudioPacketDependencyInfoTranslation`.
    pub mPacket: i64,
    /// Wraps `AudioPacketDependencyInfoTranslation`.
    pub mIndependentlyDecodablePacket: i64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Wraps `AudioPacketDependencyInfoTranslation`.
pub struct AudioPacketDependencyInfoTranslation {
    /// Wraps `AudioFileTypeAndFormatID`.
    pub mPacket: i64,
    /// Wraps `AudioFileTypeAndFormatID`.
    pub mIsIndependentlyDecodable: u32,
    /// Wraps `AudioFileTypeAndFormatID`.
    pub mNumberPrerollPackets: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Wraps `AudioFileTypeAndFormatID`.
pub struct AudioFileTypeAndFormatId {
    /// Wraps `AudioConverterPrimeInfo`.
    pub mFileType: AudioFileTypeId,
    /// Wraps `AudioConverterPrimeInfo`.
    pub mFormatID: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Wraps `AudioConverterPrimeInfo`.
pub struct AudioConverterPrimeInfo {
    /// Wraps `AudioUnitParameterEventRamp`.
    pub leadingFrames: u32,
    /// Wraps `AudioUnitParameterEventRamp`.
    pub trailingFrames: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
/// Wraps `AudioUnitParameterEventRamp`.
pub struct AudioUnitParameterEventRamp {
    /// Wraps `AudioUnitParameterEventImmediate`.
    pub startBufferOffset: i32,
    /// Wraps `AudioUnitParameterEventImmediate`.
    pub durationInFrames: u32,
    /// Wraps `AudioUnitParameterEventImmediate`.
    pub startValue: AudioUnitParameterValue,
    /// Wraps `AudioUnitParameterEventImmediate`.
    pub endValue: AudioUnitParameterValue,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
/// Wraps `AudioUnitParameterEventImmediate`.
pub struct AudioUnitParameterEventImmediate {
    /// Wraps `AudioUnitParameterEventValues`.
    pub bufferOffset: u32,
    /// Wraps `AudioUnitParameterEventValues`.
    pub value: AudioUnitParameterValue,
}

#[repr(C)]
#[derive(Clone, Copy)]
/// Wraps `AudioUnitParameterEventValues`.
pub union AudioUnitParameterEventValues {
    /// Wraps `AudioUnitParameterEvent`.
    pub ramp: AudioUnitParameterEventRamp,
    /// Wraps `AudioUnitParameterEvent`.
    pub immediate: AudioUnitParameterEventImmediate,
}

#[repr(C)]
#[derive(Clone, Copy)]
/// Wraps `AudioUnitParameterEvent`.
pub struct AudioUnitParameterEvent {
    /// Wraps `AudioUnitParameter`.
    pub scope: AudioUnitScope,
    /// Wraps `AudioUnitParameter`.
    pub element: AudioUnitElement,
    /// Wraps `AudioUnitParameter`.
    pub parameter: AudioUnitParameterId,
    /// Wraps `AudioUnitParameter`.
    pub eventType: AUParameterEventType,
    /// Wraps `AudioUnitParameter`.
    pub eventValues: AudioUnitParameterEventValues,
}

impl fmt::Debug for AudioUnitParameterEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AudioUnitParameterEvent")
            .field("scope", &self.scope)
            .field("element", &self.element)
            .field("parameter", &self.parameter)
            .field("eventType", &self.eventType)
            .field("eventValues", &"<union>")
            .finish()
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
/// Wraps `AudioUnitParameter`.
pub struct AudioUnitParameter {
    /// Wraps `AudioUnitProperty`.
    pub mAudioUnit: AudioUnitRef,
    /// Wraps `AudioUnitProperty`.
    pub mParameterID: AudioUnitParameterId,
    /// Wraps `AudioUnitProperty`.
    pub mScope: AudioUnitScope,
    /// Wraps `AudioUnitProperty`.
    pub mElement: AudioUnitElement,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
/// Wraps `AudioUnitProperty`.
pub struct AudioUnitProperty {
    /// Wraps `AURenderCallback`.
    pub mAudioUnit: AudioUnitRef,
    /// Wraps `AURenderCallback`.
    pub mPropertyID: AudioUnitPropertyId,
    /// Wraps `AURenderCallback`.
    pub mScope: AudioUnitScope,
    /// Wraps `AURenderCallback`.
    pub mElement: AudioUnitElement,
}

/// Wraps `AURenderCallback`.
pub type AURenderCallback = unsafe extern "C" fn(
    *mut c_void,
    *mut AudioUnitRenderActionFlags,
    *const AudioTimeStamp,
    u32,
    u32,
    *mut AudioBufferList1,
) -> OSStatus;

/// Wraps `AudioUnitPropertyListenerProc`.
pub type AudioUnitPropertyListenerProc = unsafe extern "C" fn(
    *mut c_void,
    AudioUnitRef,
    AudioUnitPropertyId,
    AudioUnitScope,
    AudioUnitElement,
);

#[repr(C)]
#[derive(Clone, Copy)]
/// Wraps `AURenderCallbackStruct`.
pub struct AURenderCallbackStruct {
    /// Wraps `AudioUnitNodeConnection`.
    pub inputProc: AURenderCallback,
    /// Wraps `AudioUnitNodeConnection`.
    pub inputProcRefCon: *mut c_void,
}

impl fmt::Debug for AURenderCallbackStruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AURenderCallbackStruct")
            .field("inputProc", &(self.inputProc as *const c_void))
            .field("inputProcRefCon", &self.inputProcRefCon)
            .finish()
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Wraps `AudioUnitNodeConnection`.
pub struct AudioUnitNodeConnection {
    /// Wraps `AUNodeRenderCallback`.
    pub sourceNode: AUNode,
    /// Wraps `AUNodeRenderCallback`.
    pub sourceOutputNumber: u32,
    /// Wraps `AUNodeRenderCallback`.
    pub destNode: AUNode,
    /// Wraps `AUNodeRenderCallback`.
    pub destInputNumber: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
/// Wraps `AUNodeRenderCallback`.
pub struct AUNodeRenderCallback {
    /// Wraps `AUNodeInteractionDetails`.
    pub destNode: AUNode,
    /// Wraps `AUNodeInteractionDetails`.
    pub destInputNumber: AudioUnitElement,
    /// Wraps `AUNodeInteractionDetails`.
    pub cback: AURenderCallbackStruct,
}

#[repr(C)]
#[derive(Clone, Copy)]
/// Wraps `AUNodeInteractionDetails`.
pub union AUNodeInteractionDetails {
    /// Wraps `AUNodeInteraction`.
    pub connection: AudioUnitNodeConnection,
    /// Wraps `AUNodeInteraction`.
    pub inputCallback: AUNodeRenderCallback,
}

#[repr(C)]
#[derive(Clone, Copy)]
/// Wraps `AUNodeInteraction`.
pub struct AUNodeInteraction {
    /// Wraps `AudioQueueBuffer`.
    pub nodeInteractionType: u32,
    /// Wraps `AudioQueueBuffer`.
    pub nodeInteraction: AUNodeInteractionDetails,
}

impl fmt::Debug for AUNodeInteraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AUNodeInteraction")
            .field("nodeInteractionType", &self.nodeInteractionType)
            .field("nodeInteraction", &"<union>")
            .finish()
    }
}

#[repr(C)]
#[derive(Debug)]
/// Wraps `AudioQueueBuffer`.
pub struct AudioQueueBuffer {
    /// Wraps `AudioQueueBufferRef`.
    pub mAudioDataBytesCapacity: u32,
    /// Wraps `AudioQueueBufferRef`.
    pub mAudioData: *mut c_void,
    /// Wraps `AudioQueueBufferRef`.
    pub mAudioDataByteSize: u32,
    /// Wraps `AudioQueueBufferRef`.
    pub mUserData: *mut c_void,
    /// Wraps `AudioQueueBufferRef`.
    pub mPacketDescriptionCapacity: u32,
    /// Wraps `AudioQueueBufferRef`.
    pub mPacketDescriptions: *mut AudioStreamPacketDescription,
    /// Wraps `AudioQueueBufferRef`.
    pub mPacketDescriptionCount: u32,
}

/// Wraps `AudioQueueBufferRef`.
pub type AudioQueueBufferRef = *mut AudioQueueBuffer;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Wraps `CAFFileHeader`.
pub struct CAFFileHeader {
    /// Wraps `CAFChunkHeader`.
    pub mFileType: u32,
    /// Wraps `CAFChunkHeader`.
    pub mFileVersion: u16,
    /// Wraps `CAFChunkHeader`.
    pub mFileFlags: u16,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Wraps `CAFChunkHeader`.
pub struct CAFChunkHeader {
    /// Wraps `MIDINoteMessage`.
    pub mChunkType: u32,
    /// Wraps `MIDINoteMessage`.
    pub mChunkSize: i64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
/// Wraps `MIDINoteMessage`.
pub struct MIDINoteMessage {
    /// Wraps `MIDIChannelMessage`.
    pub channel: u8,
    /// Wraps `MIDIChannelMessage`.
    pub note: u8,
    /// Wraps `MIDIChannelMessage`.
    pub velocity: u8,
    /// Wraps `MIDIChannelMessage`.
    pub releaseVelocity: u8,
    /// Wraps `MIDIChannelMessage`.
    pub duration: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Wraps `MIDIChannelMessage`.
pub struct MIDIChannelMessage {
    /// Wraps `MIDIRawData`.
    pub status: u8,
    /// Wraps `MIDIRawData`.
    pub data1: u8,
    /// Wraps `MIDIRawData`.
    pub data2: u8,
    /// Wraps `MIDIRawData`.
    pub reserved: u8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
/// Wraps `MIDIRawData`.
pub struct MIDIRawData {
    pub length: u32,
    pub data: [u8; 1],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
/// Wraps `MIDIMetaEvent`.
pub struct MIDIMetaEvent {
    pub metaEventType: u8,
    pub unused1: u8,
    pub unused2: u8,
    pub unused3: u8,
    pub dataLength: u32,
    pub data: [u8; 1],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
/// Wraps `MusicEventUserData`.
pub struct MusicEventUserData {
    pub length: u32,
    pub data: [u8; 1],
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
/// Wraps `MusicDeviceStdNoteParams`.
pub struct MusicDeviceStdNoteParams {
    /// Wraps `NoteParamsControlValue`.
    pub argCount: u32,
    /// Wraps `NoteParamsControlValue`.
    pub mPitch: f32,
    /// Wraps `NoteParamsControlValue`.
    pub mVelocity: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
/// Wraps `NoteParamsControlValue`.
pub struct NoteParamsControlValue {
    /// Wraps `MusicDeviceNoteParams`.
    pub mID: AudioUnitParameterId,
    /// Wraps `MusicDeviceNoteParams`.
    pub mValue: AudioUnitParameterValue,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
/// Wraps `MusicDeviceNoteParams`.
pub struct MusicDeviceNoteParams {
    pub argCount: u32,
    pub mPitch: f32,
    pub mVelocity: f32,
    pub mControls: [NoteParamsControlValue; 1],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
/// Wraps `ExtendedNoteOnEvent`.
pub struct ExtendedNoteOnEvent {
    /// Wraps `ParameterEvent`.
    pub instrumentID: MusicDeviceInstrumentId,
    /// Wraps `ParameterEvent`.
    pub groupID: MusicDeviceGroupId,
    /// Wraps `ParameterEvent`.
    pub duration: f32,
    /// Wraps `ParameterEvent`.
    pub extendedParams: MusicDeviceNoteParams,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
/// Wraps `ParameterEvent`.
pub struct ParameterEvent {
    /// Wraps `ExtendedTempoEvent`.
    pub parameterID: AudioUnitParameterId,
    /// Wraps `ExtendedTempoEvent`.
    pub scope: AudioUnitScope,
    /// Wraps `ExtendedTempoEvent`.
    pub element: AudioUnitElement,
    /// Wraps `ExtendedTempoEvent`.
    pub value: AudioUnitParameterValue,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
/// Wraps `ExtendedTempoEvent`.
pub struct ExtendedTempoEvent {
    /// Wraps `AUPresetEvent`.
    pub bpm: f64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
/// Wraps `AUPresetEvent`.
pub struct AUPresetEvent {
    /// Wraps `ExtendedControlEvent`.
    pub scope: AudioUnitScope,
    /// Wraps `ExtendedControlEvent`.
    pub element: AudioUnitElement,
    /// Wraps `ExtendedControlEvent`.
    pub preset: CFPropertyListRef,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
/// Wraps `ExtendedControlEvent`.
pub struct ExtendedControlEvent {
    /// Wraps `MusicTrackLoopInfo`.
    pub groupID: MusicDeviceGroupId,
    /// Wraps `MusicTrackLoopInfo`.
    pub controlID: AudioUnitParameterId,
    /// Wraps `MusicTrackLoopInfo`.
    pub value: AudioUnitParameterValue,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
/// Wraps `MusicTrackLoopInfo`.
pub struct MusicTrackLoopInfo {
    /// Wraps `CABarBeatTime`.
    pub loopDuration: MusicTimeStamp,
    /// Wraps `CABarBeatTime`.
    pub numberOfLoops: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Wraps `CABarBeatTime`.
pub struct CABarBeatTime {
    /// Wraps `AVAudio3DPoint`.
    pub bar: i32,
    /// Wraps `AVAudio3DPoint`.
    pub beat: u16,
    /// Wraps `AVAudio3DPoint`.
    pub subbeat: u16,
    /// Wraps `AVAudio3DPoint`.
    pub subbeatDivisor: u16,
    /// Wraps `AVAudio3DPoint`.
    pub reserved: u16,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
/// Wraps `AVAudio3DPoint`.
pub struct AVAudio3DPoint {
    /// Wraps `AVAudio3DVectorOrientation`.
    pub x: f32,
    /// Wraps `AVAudio3DVectorOrientation`.
    pub y: f32,
    /// Wraps `AVAudio3DVectorOrientation`.
    pub z: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
/// Wraps `AVAudio3DVectorOrientation`.
pub struct AVAudio3DVectorOrientation {
    /// Wraps `AVAudio3DAngularOrientation`.
    pub forward: AVAudio3DPoint,
    /// Wraps `AVAudio3DAngularOrientation`.
    pub up: AVAudio3DPoint,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
/// Wraps `AVAudio3DAngularOrientation`.
pub struct AVAudio3DAngularOrientation {
    /// Wraps `AVAudioConverterPrimeInfo`.
    pub yaw: f32,
    /// Wraps `AVAudioConverterPrimeInfo`.
    pub pitch: f32,
    /// Wraps `AVAudioConverterPrimeInfo`.
    pub roll: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Wraps `AVAudioConverterPrimeInfo`.
pub struct AVAudioConverterPrimeInfo {
    /// Wraps `AVBeatRange`.
    pub leadingFrames: AVAudioFrameCount,
    /// Wraps `AVBeatRange`.
    pub trailingFrames: AVAudioFrameCount,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
/// Wraps `AVBeatRange`.
pub struct AVBeatRange {
    pub start: AVMusicTimeStamp,
    pub length: AVMusicTimeStamp,
}
