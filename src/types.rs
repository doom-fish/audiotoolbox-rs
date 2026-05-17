#![allow(non_snake_case)]

use std::ffi::c_void;

pub type Boolean = u8;
pub type OSStatus = i32;
pub type OSType = u32;

pub type AudioFormatId = u32;
pub type AudioFormatFlags = u32;
pub type AudioFormatPropertyId = u32;
pub type AudioChannelLayoutTag = u32;
pub type AudioPanningMode = u32;
pub type AudioBalanceFadeType = u32;
pub type AudioFileTypeId = u32;
pub type AudioFilePropertyId = u32;
pub type AudioFileComponentPropertyId = u32;
pub type AudioFileFlags = u32;
pub type AudioFilePermissions = i8;
pub type AudioFileRegionFlags = u32;
pub type AudioBytePacketTranslationFlags = u32;
pub type ExtAudioFilePropertyId = u32;
pub type AudioConverterPropertyId = u32;
pub type AudioConverterOptions = u32;
pub type AudioComponentFlags = u32;
pub type AudioComponentInstantiationOptions = u32;
pub type AudioComponentValidationResult = u32;
pub type AudioServicesPropertyId = u32;
pub type SystemSoundId = u32;
pub type AudioUnitPropertyId = u32;
pub type AudioUnitScope = u32;
pub type AudioUnitElement = u32;
pub type AudioUnitParameterId = u32;
pub type AudioUnitParameterValue = f32;
pub type AudioUnitRenderActionFlags = u32;
pub type AUParameterEventType = u32;
pub type AUAudioFrameCount = u32;
pub type AUNode = i32;
pub type AudioQueuePropertyId = u32;
pub type AudioQueueParameterId = u32;
pub type AudioQueueParameterValue = f32;
pub type AudioFileStreamPropertyId = u32;
pub type AudioFileStreamParseFlags = u32;
pub type AudioFileStreamSeekFlags = u32;
pub type AVAudioCommonFormat = u64;
pub type AVAudioChannelCount = u32;
pub type AVAudioNodeBus = u64;
pub type AVAudioFramePosition = i64;
pub type AVAudioFrameCount = u32;
pub type AVAudioPacketCount = u32;
pub type AVMusicTimeStamp = f64;
pub type AVAudioPlayerNodeBufferOptions = u64;
pub type AVMusicSequenceLoadOptions = u64;
pub type AVAudioEngineManualRenderingMode = i64;
pub type AVAudioEngineManualRenderingStatus = i64;
pub type AVSpeechBoundary = i64;
pub type AVSpeechSynthesisVoiceGender = i64;
pub type AVSpeechSynthesisMarkerMark = i64;
pub type AVSpeechSynthesisPersonalVoiceAuthorizationStatus = u64;
pub type MusicEventType = u32;
pub type MIDIEndpointRef = u32;
pub type MusicDeviceInstrumentId = u32;
pub type MusicDeviceGroupId = u32;
pub type NoteInstanceId = u32;
pub type MusicSequenceType = u32;
pub type MusicSequenceFileTypeId = u32;
pub type MusicSequenceFileFlags = u32;
pub type MusicSequenceLoadFlags = u32;
pub type MusicTimeStamp = f64;

pub type AudioFileId = *mut c_void;
pub type ExtAudioFileRef = *mut c_void;
pub type AudioConverterRef = *mut c_void;
pub type AudioComponentRef = *mut c_void;
pub type AudioComponentInstanceRef = *mut c_void;
pub type AudioUnitRef = *mut c_void;
pub type AudioQueueRef = *mut c_void;
pub type AudioQueueTimelineRef = *mut c_void;
pub type MusicSequenceRef = *mut c_void;
pub type MusicTrackRef = *mut c_void;
pub type MusicPlayerRef = *mut c_void;
pub type MusicEventIteratorRef = *mut c_void;
pub type AudioFileStreamId = *mut c_void;
pub type CFURLRef = *const c_void;
pub type CFStringRef = *const c_void;
pub type CFDataRef = *const c_void;
pub type CFArrayRef = *const c_void;
pub type CFDictionaryRef = *const c_void;
pub type CFPropertyListRef = *const c_void;

pub const fn fourcc(bytes: [u8; 4]) -> u32 {
    u32::from_be_bytes(bytes)
}

pub const NO_ERR: OSStatus = 0;
pub const AUDIO_FILE_END_OF_FILE_ERROR: OSStatus = -39;

pub const AUDIO_FILE_READ_PERMISSION: AudioFilePermissions = 0x01;
pub const AUDIO_FILE_WRITE_PERMISSION: AudioFilePermissions = 0x02;
pub const AUDIO_FILE_READ_WRITE_PERMISSION: AudioFilePermissions = 0x03;

pub const AUDIO_FILE_FLAGS_ERASE_FILE: AudioFileFlags = 1;

pub const AUDIO_FILE_AIFF_TYPE: AudioFileTypeId = fourcc(*b"AIFF");
pub const AUDIO_FILE_WAVE_TYPE: AudioFileTypeId = fourcc(*b"WAVE");
pub const AUDIO_FILE_M4A_TYPE: AudioFileTypeId = fourcc(*b"m4af");
pub const AUDIO_FILE_CAF_TYPE: AudioFileTypeId = fourcc(*b"caff");
pub const AUDIO_FILE_MIDI_TYPE: AudioFileTypeId = fourcc(*b"Midi");
pub const CAF_FILE_MAGIC: u32 = fourcc(*b"caff");

pub const AUDIO_FILE_PROPERTY_DATA_FORMAT: AudioFilePropertyId = fourcc(*b"dfmt");
pub const AUDIO_FILE_PROPERTY_MAGIC_COOKIE_DATA: AudioFilePropertyId = fourcc(*b"mgic");
pub const AUDIO_FILE_PROPERTY_AUDIO_DATA_PACKET_COUNT: AudioFilePropertyId = fourcc(*b"pcnt");
pub const AUDIO_FILE_PROPERTY_MAXIMUM_PACKET_SIZE: AudioFilePropertyId = fourcc(*b"psze");
pub const AUDIO_FILE_PROPERTY_ESTIMATED_DURATION: AudioFilePropertyId = fourcc(*b"edur");
pub const AUDIO_FILE_PROPERTY_AUDIO_DATA_BYTE_COUNT: AudioFilePropertyId = fourcc(*b"bcnt");
pub const AUDIO_FILE_PROPERTY_DATA_OFFSET: AudioFilePropertyId = fourcc(*b"doff");

pub const EXT_AUDIO_FILE_PROPERTY_FILE_DATA_FORMAT: ExtAudioFilePropertyId = fourcc(*b"ffmt");
pub const EXT_AUDIO_FILE_PROPERTY_CLIENT_DATA_FORMAT: ExtAudioFilePropertyId = fourcc(*b"cfmt");
pub const EXT_AUDIO_FILE_PROPERTY_AUDIO_CONVERTER: ExtAudioFilePropertyId = fourcc(*b"acnv");
pub const EXT_AUDIO_FILE_PROPERTY_AUDIO_FILE: ExtAudioFilePropertyId = fourcc(*b"afil");
pub const EXT_AUDIO_FILE_PROPERTY_FILE_LENGTH_FRAMES: ExtAudioFilePropertyId = fourcc(*b"#frm");

pub const AUDIO_CONVERTER_PROPERTY_ENCODE_BIT_RATE: AudioConverterPropertyId = fourcc(*b"brat");
pub const AUDIO_CONVERTER_PROPERTY_CURRENT_OUTPUT_STREAM_DESCRIPTION: AudioConverterPropertyId =
    fourcc(*b"acod");
pub const AUDIO_CONVERTER_PROPERTY_CURRENT_INPUT_STREAM_DESCRIPTION: AudioConverterPropertyId =
    fourcc(*b"acid");
pub const AUDIO_CONVERTER_PROPERTY_MAXIMUM_OUTPUT_PACKET_SIZE: AudioConverterPropertyId =
    fourcc(*b"xops");
pub const AUDIO_CONVERTER_PROPERTY_CALCULATE_INPUT_BUFFER_SIZE: AudioConverterPropertyId =
    fourcc(*b"cibs");
pub const AUDIO_CONVERTER_PROPERTY_CALCULATE_OUTPUT_BUFFER_SIZE: AudioConverterPropertyId =
    fourcc(*b"cobs");
pub const AUDIO_CONVERTER_PROPERTY_PRIME_METHOD: AudioConverterPropertyId = fourcc(*b"prmm");
pub const AUDIO_CONVERTER_PROPERTY_PRIME_INFO: AudioConverterPropertyId = fourcc(*b"prim");
pub const AUDIO_CONVERTER_PROPERTY_APPLICABLE_ENCODE_BIT_RATES: AudioConverterPropertyId =
    fourcc(*b"aebr");
pub const AUDIO_CONVERTER_PROPERTY_APPLICABLE_ENCODE_SAMPLE_RATES: AudioConverterPropertyId =
    fourcc(*b"aesr");

pub const AUDIO_FORMAT_LINEAR_PCM: AudioFormatId = fourcc(*b"lpcm");
pub const AUDIO_FORMAT_APPLE_IMA4: AudioFormatId = fourcc(*b"ima4");
pub const AUDIO_FORMAT_MPEG4_AAC: AudioFormatId = fourcc(*b"aac ");

pub const AUDIO_FORMAT_FLAG_IS_FLOAT: AudioFormatFlags = 1 << 0;
pub const AUDIO_FORMAT_FLAG_IS_BIG_ENDIAN: AudioFormatFlags = 1 << 1;
pub const AUDIO_FORMAT_FLAG_IS_SIGNED_INTEGER: AudioFormatFlags = 1 << 2;
pub const AUDIO_FORMAT_FLAG_IS_PACKED: AudioFormatFlags = 1 << 3;
pub const AUDIO_FORMAT_FLAG_IS_NON_INTERLEAVED: AudioFormatFlags = 1 << 5;
pub const AUDIO_FORMAT_FLAGS_NATIVE_ENDIAN: AudioFormatFlags = if cfg!(target_endian = "big") {
    AUDIO_FORMAT_FLAG_IS_BIG_ENDIAN
} else {
    0
};

pub const LINEAR_PCM_FORMAT_FLAG_IS_FLOAT: AudioFormatFlags = AUDIO_FORMAT_FLAG_IS_FLOAT;
pub const LINEAR_PCM_FORMAT_FLAG_IS_BIG_ENDIAN: AudioFormatFlags = AUDIO_FORMAT_FLAG_IS_BIG_ENDIAN;
pub const LINEAR_PCM_FORMAT_FLAG_IS_SIGNED_INTEGER: AudioFormatFlags =
    AUDIO_FORMAT_FLAG_IS_SIGNED_INTEGER;
pub const LINEAR_PCM_FORMAT_FLAG_IS_PACKED: AudioFormatFlags = AUDIO_FORMAT_FLAG_IS_PACKED;
pub const LINEAR_PCM_FORMAT_FLAG_IS_NON_INTERLEAVED: AudioFormatFlags =
    AUDIO_FORMAT_FLAG_IS_NON_INTERLEAVED;

pub const AUDIO_FORMAT_PROPERTY_FORMAT_INFO: AudioFormatPropertyId = fourcc(*b"fmti");
pub const AUDIO_FORMAT_PROPERTY_ENCODE_FORMAT_IDS: AudioFormatPropertyId = fourcc(*b"acof");
pub const AUDIO_FORMAT_PROPERTY_DECODE_FORMAT_IDS: AudioFormatPropertyId = fourcc(*b"acif");
pub const AUDIO_FORMAT_PROPERTY_FORMAT_IS_VBR: AudioFormatPropertyId = fourcc(*b"fvbr");
pub const AUDIO_FORMAT_PROPERTY_FORMAT_IS_EXTERNALLY_FRAMED: AudioFormatPropertyId =
    fourcc(*b"fexf");
pub const AUDIO_FORMAT_PROPERTY_FORMAT_EMPLOYS_DEPENDENT_PACKETS: AudioFormatPropertyId =
    fourcc(*b"fdep");
pub const AUDIO_FORMAT_PROPERTY_FORMAT_NAME: AudioFormatPropertyId = fourcc(*b"fnam");
pub const AUDIO_FORMAT_PROPERTY_FORMAT_LIST: AudioFormatPropertyId = fourcc(*b"flst");
pub const AUDIO_FORMAT_PROPERTY_OUTPUT_FORMAT_LIST: AudioFormatPropertyId = fourcc(*b"ofls");
pub const AUDIO_FORMAT_PROPERTY_FIRST_PLAYABLE_FORMAT_FROM_LIST: AudioFormatPropertyId =
    fourcc(*b"fpfl");
pub const AUDIO_FORMAT_PROPERTY_ENCODERS: AudioFormatPropertyId = fourcc(*b"aven");
pub const AUDIO_FORMAT_PROPERTY_DECODERS: AudioFormatPropertyId = fourcc(*b"avde");
pub const AUDIO_FORMAT_PROPERTY_AVAILABLE_ENCODE_BIT_RATES: AudioFormatPropertyId =
    fourcc(*b"aebr");
pub const AUDIO_FORMAT_PROPERTY_AVAILABLE_ENCODE_SAMPLE_RATES: AudioFormatPropertyId =
    fourcc(*b"aesr");
pub const AUDIO_FORMAT_PROPERTY_PANNING_MATRIX: AudioFormatPropertyId = fourcc(*b"panm");
pub const AUDIO_FORMAT_PROPERTY_BALANCE_FADE: AudioFormatPropertyId = fourcc(*b"balf");

pub const AUDIO_COMPONENT_TYPE_OUTPUT: u32 = fourcc(*b"auou");
pub const AUDIO_COMPONENT_TYPE_MUSIC_DEVICE: u32 = fourcc(*b"aumu");
pub const AUDIO_COMPONENT_TYPE_MUSIC_EFFECT: u32 = fourcc(*b"aumf");
pub const AUDIO_COMPONENT_TYPE_FORMAT_CONVERTER: u32 = fourcc(*b"aufc");
pub const AUDIO_COMPONENT_TYPE_EFFECT: u32 = fourcc(*b"aufx");
pub const AUDIO_COMPONENT_TYPE_MIXER: u32 = fourcc(*b"aumx");
pub const AUDIO_COMPONENT_TYPE_PANNER: u32 = fourcc(*b"aupn");
pub const AUDIO_COMPONENT_TYPE_GENERATOR: u32 = fourcc(*b"augn");
pub const AUDIO_COMPONENT_TYPE_OFFLINE_EFFECT: u32 = fourcc(*b"auol");
pub const AUDIO_COMPONENT_TYPE_MIDI_PROCESSOR: u32 = fourcc(*b"aumi");
pub const AUDIO_COMPONENT_MANUFACTURER_APPLE: u32 = fourcc(*b"appl");
pub const AUDIO_COMPONENT_INSTANTIATION_LOAD_OUT_OF_PROCESS: AudioComponentInstantiationOptions = 1;
pub const AUDIO_COMPONENT_INSTANTIATION_LOAD_IN_PROCESS: AudioComponentInstantiationOptions = 2;
pub const AUDIO_COMPONENT_VALIDATION_RESULT_UNKNOWN: AudioComponentValidationResult = 0;
pub const AUDIO_COMPONENT_VALIDATION_RESULT_PASSED: AudioComponentValidationResult = 1;
pub const AUDIO_COMPONENT_VALIDATION_RESULT_FAILED: AudioComponentValidationResult = 2;
pub const AUDIO_COMPONENT_VALIDATION_RESULT_TIMED_OUT: AudioComponentValidationResult = 3;
pub const AUDIO_COMPONENT_VALIDATION_RESULT_UNAUTHORIZED_ERROR_OPEN:
    AudioComponentValidationResult = 4;
pub const AUDIO_COMPONENT_VALIDATION_RESULT_UNAUTHORIZED_ERROR_INIT:
    AudioComponentValidationResult = 5;
pub const AUDIO_UNIT_SUBTYPE_AU_CONVERTER: u32 = fourcc(*b"conv");
pub const AUDIO_UNIT_SUBTYPE_GENERIC_OUTPUT: u32 = fourcc(*b"genr");
pub const AUDIO_UNIT_SUBTYPE_DEFAULT_OUTPUT: u32 = fourcc(*b"def ");

pub const AUDIO_UNIT_SCOPE_GLOBAL: AudioUnitScope = 0;
pub const AUDIO_UNIT_SCOPE_INPUT: AudioUnitScope = 1;
pub const AUDIO_UNIT_SCOPE_OUTPUT: AudioUnitScope = 2;
pub const AUDIO_UNIT_SCOPE_GROUP: AudioUnitScope = 3;

pub const AUDIO_UNIT_PROPERTY_CLASS_INFO: AudioUnitPropertyId = 0;
pub const AUDIO_UNIT_PROPERTY_MAKE_CONNECTION: AudioUnitPropertyId = 1;
pub const AUDIO_UNIT_PROPERTY_SAMPLE_RATE: AudioUnitPropertyId = 2;
pub const AUDIO_UNIT_PROPERTY_PARAMETER_LIST: AudioUnitPropertyId = 3;
pub const AUDIO_UNIT_PROPERTY_PARAMETER_INFO: AudioUnitPropertyId = 4;
pub const AUDIO_UNIT_PROPERTY_CPULOAD: AudioUnitPropertyId = 6;
pub const AUDIO_UNIT_PROPERTY_STREAM_FORMAT: AudioUnitPropertyId = 8;
pub const AUDIO_UNIT_PROPERTY_ELEMENT_COUNT: AudioUnitPropertyId = 11;
pub const AUDIO_UNIT_PROPERTY_LATENCY: AudioUnitPropertyId = 12;
pub const AUDIO_UNIT_PROPERTY_SUPPORTED_NUM_CHANNELS: AudioUnitPropertyId = 13;
pub const AUDIO_UNIT_PROPERTY_PARAMETER_VALUE_STRINGS: AudioUnitPropertyId = 16;
pub const AUDIO_UNIT_PROPERTY_AUDIO_CHANNEL_LAYOUT: AudioUnitPropertyId = 19;
pub const AUDIO_UNIT_PROPERTY_LAST_RENDER_ERROR: AudioUnitPropertyId = 22;
pub const AUDIO_UNIT_PROPERTY_SET_RENDER_CALLBACK: AudioUnitPropertyId = 23;
pub const AUDIO_UNIT_PROPERTY_HOST_CALLBACKS: AudioUnitPropertyId = 27;
pub const AUDIO_UNIT_PROPERTY_PARAMETER_STRING_FROM_VALUE: AudioUnitPropertyId = 33;
pub const AUDIO_UNIT_PROPERTY_PARAMETER_CLUMP_NAME: AudioUnitPropertyId = 35;
pub const AUDIO_UNIT_PROPERTY_PRESENT_PRESET: AudioUnitPropertyId = 36;
pub const AUDIO_UNIT_PROPERTY_OFFLINE_RENDER: AudioUnitPropertyId = 37;
pub const AUDIO_UNIT_PROPERTY_PARAMETER_VALUE_FROM_STRING: AudioUnitPropertyId = 38;
pub const AUDIO_UNIT_PROPERTY_PRESENTATION_LATENCY: AudioUnitPropertyId = 40;
pub const AUDIO_UNIT_PROPERTY_DEPENDENT_PARAMETERS: AudioUnitPropertyId = 45;
pub const AUDIO_UNIT_PROPERTY_INPUT_SAMPLES_IN_OUTPUT: AudioUnitPropertyId = 49;
pub const AUDIO_UNIT_PROPERTY_CLASS_INFO_FROM_DOCUMENT: AudioUnitPropertyId = 50;
pub const AUDIO_OUTPUT_UNIT_PROPERTY_ENABLE_IO: AudioUnitPropertyId = 2003;
pub const AUDIO_OUTPUT_UNIT_PROPERTY_MIDI_CALLBACKS: AudioUnitPropertyId = 2010;

pub const AUDIO_QUEUE_PROPERTY_IS_RUNNING: AudioQueuePropertyId = fourcc(*b"aqrn");
pub const AUDIO_QUEUE_DEVICE_PROPERTY_SAMPLE_RATE: AudioQueuePropertyId = fourcc(*b"aqsr");
pub const AUDIO_QUEUE_DEVICE_PROPERTY_NUMBER_CHANNELS: AudioQueuePropertyId = fourcc(*b"aqdc");
pub const AUDIO_QUEUE_PROPERTY_CURRENT_DEVICE: AudioQueuePropertyId = fourcc(*b"aqcd");
pub const AUDIO_QUEUE_PROPERTY_MAGIC_COOKIE: AudioQueuePropertyId = fourcc(*b"aqmc");
pub const AUDIO_QUEUE_PROPERTY_MAXIMUM_OUTPUT_PACKET_SIZE: AudioQueuePropertyId = fourcc(*b"xops");
pub const AUDIO_QUEUE_PROPERTY_STREAM_DESCRIPTION: AudioQueuePropertyId = fourcc(*b"aqft");
pub const AUDIO_QUEUE_PARAM_VOLUME: AudioQueueParameterId = 1;
pub const AUDIO_QUEUE_PARAM_VOLUME_RAMP_TIME: AudioQueueParameterId = 4;
pub const AUDIO_QUEUE_ERR_CANNOT_START: OSStatus = -66681;
pub const AUDIO_QUEUE_ERR_CANNOT_START_YET: OSStatus = -66665;

pub const AUDIO_SERVICES_PROPERTY_IS_UI_SOUND: AudioServicesPropertyId = fourcc(*b"isui");
pub const AUDIO_SERVICES_PROPERTY_COMPLETE_PLAYBACK_IF_APP_DIES: AudioServicesPropertyId =
    fourcc(*b"ifdi");
pub const SYSTEM_SOUND_USER_PREFERRED_ALERT: SystemSoundId = 0x0000_1000;
pub const SYSTEM_SOUND_FLASH_SCREEN: SystemSoundId = 0x0000_0FFE;

pub const MUSIC_EVENT_TYPE_NULL: MusicEventType = 0;
pub const MUSIC_EVENT_TYPE_EXTENDED_NOTE: MusicEventType = 1;
pub const MUSIC_EVENT_TYPE_EXTENDED_CONTROL: MusicEventType = 2;
pub const MUSIC_EVENT_TYPE_EXTENDED_TEMPO: MusicEventType = 3;
pub const MUSIC_EVENT_TYPE_USER: MusicEventType = 4;
pub const MUSIC_EVENT_TYPE_META: MusicEventType = 5;
pub const MUSIC_EVENT_TYPE_MIDI_NOTE_MESSAGE: MusicEventType = 6;
pub const MUSIC_EVENT_TYPE_MIDI_CHANNEL_MESSAGE: MusicEventType = 7;
pub const MUSIC_EVENT_TYPE_MIDI_RAW_DATA: MusicEventType = 8;
pub const MUSIC_EVENT_TYPE_PARAMETER: MusicEventType = 9;
pub const MUSIC_EVENT_TYPE_AU_PRESET: MusicEventType = 10;
pub const MUSIC_SEQUENCE_TYPE_BEATS: MusicSequenceType = fourcc(*b"beat");
pub const MUSIC_SEQUENCE_TYPE_SECONDS: MusicSequenceType = fourcc(*b"secs");
pub const MUSIC_SEQUENCE_TYPE_SAMPLES: MusicSequenceType = fourcc(*b"samp");
pub const MUSIC_SEQUENCE_FILE_ANY_TYPE: MusicSequenceFileTypeId = 0;
pub const MUSIC_SEQUENCE_FILE_MIDI_TYPE: MusicSequenceFileTypeId = fourcc(*b"midi");
pub const MUSIC_SEQUENCE_FILE_IMELODY_TYPE: MusicSequenceFileTypeId = fourcc(*b"imel");
pub const MUSIC_SEQUENCE_FILE_FLAGS_DEFAULT: MusicSequenceFileFlags = 0;
pub const MUSIC_SEQUENCE_FILE_FLAGS_ERASE_FILE: MusicSequenceFileFlags = 1;
pub const MUSIC_SEQUENCE_LOAD_SMF_PRESERVE_TRACKS: MusicSequenceLoadFlags = 0;
pub const MUSIC_SEQUENCE_LOAD_SMF_CHANNELS_TO_TRACKS: MusicSequenceLoadFlags = 1 << 0;
pub const SEQUENCE_TRACK_PROPERTY_LOOP_INFO: u32 = 0;
pub const SEQUENCE_TRACK_PROPERTY_OFFSET_TIME: u32 = 1;
pub const SEQUENCE_TRACK_PROPERTY_MUTE_STATUS: u32 = 2;
pub const SEQUENCE_TRACK_PROPERTY_SOLO_STATUS: u32 = 3;
pub const SEQUENCE_TRACK_PROPERTY_AUTOMATED_PARAMETERS: u32 = 4;
pub const SEQUENCE_TRACK_PROPERTY_TRACK_LENGTH: u32 = 5;
pub const SEQUENCE_TRACK_PROPERTY_TIME_RESOLUTION: u32 = 6;
pub const AUDIO_TOOLBOX_ERR_NO_SEQUENCE: OSStatus = -10854;

pub const AUDIO_FILE_STREAM_PROPERTY_FLAG_PROPERTY_IS_CACHED: u32 = 1;
pub const AUDIO_FILE_STREAM_PROPERTY_FLAG_CACHE_PROPERTY: u32 = 2;
pub const AUDIO_FILE_STREAM_PARSE_FLAG_DISCONTINUITY: AudioFileStreamParseFlags = 1;
pub const AUDIO_FILE_STREAM_SEEK_FLAG_OFFSET_IS_ESTIMATED: AudioFileStreamSeekFlags = 1;
pub const AUDIO_FILE_STREAM_PROPERTY_READY_TO_PRODUCE_PACKETS: AudioFileStreamPropertyId =
    fourcc(*b"redy");
pub const AUDIO_FILE_STREAM_PROPERTY_FILE_FORMAT: AudioFileStreamPropertyId = fourcc(*b"ffmt");
pub const AUDIO_FILE_STREAM_PROPERTY_DATA_FORMAT: AudioFileStreamPropertyId = fourcc(*b"dfmt");
pub const AUDIO_FILE_STREAM_PROPERTY_MAGIC_COOKIE_DATA: AudioFileStreamPropertyId =
    fourcc(*b"mgic");
pub const AUDIO_FILE_STREAM_PROPERTY_AUDIO_DATA_BYTE_COUNT: AudioFileStreamPropertyId =
    fourcc(*b"bcnt");
pub const AUDIO_FILE_STREAM_PROPERTY_AUDIO_DATA_PACKET_COUNT: AudioFileStreamPropertyId =
    fourcc(*b"pcnt");
pub const AUDIO_FILE_STREAM_PROPERTY_MAXIMUM_PACKET_SIZE: AudioFileStreamPropertyId =
    fourcc(*b"psze");
pub const AUDIO_FILE_STREAM_PROPERTY_DATA_OFFSET: AudioFileStreamPropertyId = fourcc(*b"doff");
pub const AUDIO_FILE_STREAM_PROPERTY_BIT_RATE: AudioFileStreamPropertyId = fourcc(*b"brat");

pub const AUDIO_FILE_COMPONENT_CAN_READ: AudioFileComponentPropertyId = fourcc(*b"cnrd");
pub const AUDIO_FILE_COMPONENT_CAN_WRITE: AudioFileComponentPropertyId = fourcc(*b"cnwr");
pub const AUDIO_FILE_COMPONENT_FILE_TYPE_NAME: AudioFileComponentPropertyId = fourcc(*b"ftnm");
pub const AUDIO_FILE_COMPONENT_UTIS_FOR_TYPE: AudioFileComponentPropertyId = fourcc(*b"futi");
pub const AUDIO_FILE_COMPONENT_MIME_TYPES_FOR_TYPE: AudioFileComponentPropertyId = fourcc(*b"fmim");
pub const AUDIO_FILE_COMPONENT_EXTENSIONS_FOR_TYPE: AudioFileComponentPropertyId = fourcc(*b"fext");
pub const AUDIO_FILE_COMPONENT_AVAILABLE_FORMAT_IDS: AudioFileComponentPropertyId =
    fourcc(*b"fmid");
pub const AUDIO_FILE_COMPONENT_AVAILABLE_STREAM_DESCRIPTIONS_FOR_FORMAT:
    AudioFileComponentPropertyId = fourcc(*b"sdid");
pub const AUDIO_FILE_COMPONENT_FAST_DISPATCH_TABLE: AudioFileComponentPropertyId = fourcc(*b"fdft");
pub const AUDIO_FILE_COMPONENT_HFS_TYPE_CODES_FOR_TYPE: AudioFileComponentPropertyId =
    fourcc(*b"fhfs");

pub const AV_AUDIO_OTHER_FORMAT: AVAudioCommonFormat = 0;
pub const AV_AUDIO_PCM_FORMAT_FLOAT32: AVAudioCommonFormat = 1;
pub const AV_AUDIO_PCM_FORMAT_FLOAT64: AVAudioCommonFormat = 2;
pub const AV_AUDIO_PCM_FORMAT_INT16: AVAudioCommonFormat = 3;
pub const AV_AUDIO_PCM_FORMAT_INT32: AVAudioCommonFormat = 4;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioComponentDescription {
    pub component_type: OSType,
    pub component_sub_type: OSType,
    pub component_manufacturer: OSType,
    pub component_flags: u32,
    pub component_flags_mask: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioClassDescription {
    pub mType: OSType,
    pub mSubType: OSType,
    pub mManufacturer: OSType,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AudioStreamBasicDescription {
    pub mSampleRate: f64,
    pub mFormatID: AudioFormatId,
    pub mFormatFlags: AudioFormatFlags,
    pub mBytesPerPacket: u32,
    pub mFramesPerPacket: u32,
    pub mBytesPerFrame: u32,
    pub mChannelsPerFrame: u32,
    pub mBitsPerChannel: u32,
    pub mReserved: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AudioPanningInfo {
    pub mPanningMode: AudioPanningMode,
    pub mCoordinateFlags: u32,
    pub mCoordinates: [f32; 3],
    pub mGainScale: f32,
    pub mOutputChannelMap: *const c_void,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AudioBalanceFade {
    pub mLeftRightBalance: f32,
    pub mBackFrontFade: f32,
    pub mType: AudioBalanceFadeType,
    pub mChannelLayout: *const c_void,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AudioFormatInfo {
    pub mASBD: AudioStreamBasicDescription,
    pub mMagicCookie: *const c_void,
    pub mMagicCookieSize: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AudioFormatListItem {
    pub mASBD: AudioStreamBasicDescription,
    pub mChannelLayoutTag: AudioChannelLayoutTag,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ExtendedAudioFormatInfo {
    pub mASBD: AudioStreamBasicDescription,
    pub mMagicCookie: *const c_void,
    pub mMagicCookieSize: u32,
    pub mClassDescription: AudioClassDescription,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AudioValueRange {
    pub mMinimum: f64,
    pub mMaximum: f64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioBuffer {
    pub mNumberChannels: u32,
    pub mDataByteSize: u32,
    pub mData: *mut c_void,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AudioBufferList<const N: usize> {
    pub mNumberBuffers: u32,
    pub mBuffers: [AudioBuffer; N],
}

pub type AudioBufferList1 = AudioBufferList<1>;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct AudioStreamPacketDescription {
    pub mStartOffset: i64,
    pub mVariableFramesInPacket: u32,
    pub mDataByteSize: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SMPTETime {
    pub mSubframes: i16,
    pub mSubframeDivisor: i16,
    pub mCounter: u32,
    pub mType: u32,
    pub mFlags: u32,
    pub mHours: i16,
    pub mMinutes: i16,
    pub mSeconds: i16,
    pub mFrames: i16,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AudioTimeStamp {
    pub mSampleTime: f64,
    pub mHostTime: u64,
    pub mRateScalar: f64,
    pub mWordClockTime: u64,
    pub mSMPTETime: SMPTETime,
    pub mFlags: u32,
    pub mReserved: u32,
}

pub type AudioFileSmpteTime = SMPTETime;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AudioFileMarker {
    pub mFramePosition: f64,
    pub mName: CFStringRef,
    pub mMarkerID: i32,
    pub mSMPTETime: AudioFileSmpteTime,
    pub mType: u32,
    pub mReserved: u16,
    pub mChannel: u16,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AudioFileMarkerList {
    pub mSMPTE_TimeType: u32,
    pub mNumberMarkers: u32,
    pub mMarkers: [AudioFileMarker; 1],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AudioFileRegion {
    pub mRegionID: u32,
    pub mName: CFStringRef,
    pub mFlags: AudioFileRegionFlags,
    pub mNumberMarkers: u32,
    pub mMarkers: [AudioFileMarker; 1],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AudioFileRegionList {
    pub mSMPTE_TimeType: u32,
    pub mNumberRegions: u32,
    pub mRegions: [AudioFileRegion; 1],
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioFramePacketTranslation {
    pub mFrame: i64,
    pub mPacket: i64,
    pub mFrameOffsetInPacket: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioBytePacketTranslation {
    pub mByte: i64,
    pub mPacket: i64,
    pub mByteOffsetInPacket: u32,
    pub mFlags: AudioBytePacketTranslationFlags,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioFilePacketTableInfo {
    pub mNumberValidFrames: i64,
    pub mPrimingFrames: i32,
    pub mRemainderFrames: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioPacketRangeByteCountTranslation {
    pub mPacket: i64,
    pub mPacketCount: i64,
    pub mByteCountUpperBound: i64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioPacketRollDistanceTranslation {
    pub mPacket: i64,
    pub mRollDistance: i64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioIndependentPacketTranslation {
    pub mPacket: i64,
    pub mIndependentlyDecodablePacket: i64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioPacketDependencyInfoTranslation {
    pub mPacket: i64,
    pub mIsIndependentlyDecodable: u32,
    pub mNumberPrerollPackets: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioFileTypeAndFormatId {
    pub mFileType: AudioFileTypeId,
    pub mFormatID: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioConverterPrimeInfo {
    pub leadingFrames: u32,
    pub trailingFrames: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AudioUnitParameterEventRamp {
    pub startBufferOffset: i32,
    pub durationInFrames: u32,
    pub startValue: AudioUnitParameterValue,
    pub endValue: AudioUnitParameterValue,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AudioUnitParameterEventImmediate {
    pub bufferOffset: u32,
    pub value: AudioUnitParameterValue,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union AudioUnitParameterEventValues {
    pub ramp: AudioUnitParameterEventRamp,
    pub immediate: AudioUnitParameterEventImmediate,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct AudioUnitParameterEvent {
    pub scope: AudioUnitScope,
    pub element: AudioUnitElement,
    pub parameter: AudioUnitParameterId,
    pub eventType: AUParameterEventType,
    pub eventValues: AudioUnitParameterEventValues,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AudioUnitParameter {
    pub mAudioUnit: AudioUnitRef,
    pub mParameterID: AudioUnitParameterId,
    pub mScope: AudioUnitScope,
    pub mElement: AudioUnitElement,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AudioUnitProperty {
    pub mAudioUnit: AudioUnitRef,
    pub mPropertyID: AudioUnitPropertyId,
    pub mScope: AudioUnitScope,
    pub mElement: AudioUnitElement,
}

pub type AURenderCallback = unsafe extern "C" fn(
    *mut c_void,
    *mut AudioUnitRenderActionFlags,
    *const AudioTimeStamp,
    u32,
    u32,
    *mut AudioBufferList1,
) -> OSStatus;

pub type AudioUnitPropertyListenerProc = unsafe extern "C" fn(
    *mut c_void,
    AudioUnitRef,
    AudioUnitPropertyId,
    AudioUnitScope,
    AudioUnitElement,
);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct AURenderCallbackStruct {
    pub inputProc: AURenderCallback,
    pub inputProcRefCon: *mut c_void,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioUnitNodeConnection {
    pub sourceNode: AUNode,
    pub sourceOutputNumber: u32,
    pub destNode: AUNode,
    pub destInputNumber: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct AUNodeRenderCallback {
    pub destNode: AUNode,
    pub destInputNumber: AudioUnitElement,
    pub cback: AURenderCallbackStruct,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union AUNodeInteractionDetails {
    pub connection: AudioUnitNodeConnection,
    pub inputCallback: AUNodeRenderCallback,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct AUNodeInteraction {
    pub nodeInteractionType: u32,
    pub nodeInteraction: AUNodeInteractionDetails,
}

#[repr(C)]
#[derive(Debug)]
pub struct AudioQueueBuffer {
    pub mAudioDataBytesCapacity: u32,
    pub mAudioData: *mut c_void,
    pub mAudioDataByteSize: u32,
    pub mUserData: *mut c_void,
    pub mPacketDescriptionCapacity: u32,
    pub mPacketDescriptions: *mut AudioStreamPacketDescription,
    pub mPacketDescriptionCount: u32,
}

pub type AudioQueueBufferRef = *mut AudioQueueBuffer;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CAFFileHeader {
    pub mFileType: u32,
    pub mFileVersion: u16,
    pub mFileFlags: u16,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CAFChunkHeader {
    pub mChunkType: u32,
    pub mChunkSize: i64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MIDINoteMessage {
    pub channel: u8,
    pub note: u8,
    pub velocity: u8,
    pub releaseVelocity: u8,
    pub duration: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MIDIChannelMessage {
    pub status: u8,
    pub data1: u8,
    pub data2: u8,
    pub reserved: u8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MIDIRawData {
    pub length: u32,
    pub data: [u8; 1],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
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
pub struct MusicEventUserData {
    pub length: u32,
    pub data: [u8; 1],
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MusicDeviceStdNoteParams {
    pub argCount: u32,
    pub mPitch: f32,
    pub mVelocity: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NoteParamsControlValue {
    pub mID: AudioUnitParameterId,
    pub mValue: AudioUnitParameterValue,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MusicDeviceNoteParams {
    pub argCount: u32,
    pub mPitch: f32,
    pub mVelocity: f32,
    pub mControls: [NoteParamsControlValue; 1],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ExtendedNoteOnEvent {
    pub instrumentID: MusicDeviceInstrumentId,
    pub groupID: MusicDeviceGroupId,
    pub duration: f32,
    pub extendedParams: MusicDeviceNoteParams,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ParameterEvent {
    pub parameterID: AudioUnitParameterId,
    pub scope: AudioUnitScope,
    pub element: AudioUnitElement,
    pub value: AudioUnitParameterValue,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ExtendedTempoEvent {
    pub bpm: f64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AUPresetEvent {
    pub scope: AudioUnitScope,
    pub element: AudioUnitElement,
    pub preset: CFPropertyListRef,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ExtendedControlEvent {
    pub groupID: MusicDeviceGroupId,
    pub controlID: AudioUnitParameterId,
    pub value: AudioUnitParameterValue,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MusicTrackLoopInfo {
    pub loopDuration: MusicTimeStamp,
    pub numberOfLoops: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CABarBeatTime {
    pub bar: i32,
    pub beat: u16,
    pub subbeat: u16,
    pub subbeatDivisor: u16,
    pub reserved: u16,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AVAudio3DPoint {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AVAudio3DVectorOrientation {
    pub forward: AVAudio3DPoint,
    pub up: AVAudio3DPoint,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AVAudio3DAngularOrientation {
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AVAudioConverterPrimeInfo {
    pub leadingFrames: AVAudioFrameCount,
    pub trailingFrames: AVAudioFrameCount,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AVBeatRange {
    pub start: AVMusicTimeStamp,
    pub length: AVMusicTimeStamp,
}
