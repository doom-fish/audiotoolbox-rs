#![allow(non_snake_case)]

use std::ffi::c_void;

pub use apple_cf::raw::{
    Boolean, CFDataRef, CFDictionaryRef, CFStringRef, CFURLRef, OSStatus, OSType,
};
/// Wraps `AudioFileTypeID`.
pub type AudioFileTypeId = u32;
/// Wraps `AudioFilePropertyID`.
pub type AudioFilePropertyId = u32;
/// Wraps `AudioFileFlags`.
pub type AudioFileFlags = u32;
/// Wraps `AudioFilePermissions`.
pub type AudioFilePermissions = i8;
/// Wraps `AudioFormatID`.
pub type AudioFormatId = u32;
/// Wraps `AudioFormatFlags`.
pub type AudioFormatFlags = u32;
/// Wraps `ExtAudioFilePropertyID`.
pub type ExtAudioFilePropertyId = u32;
/// Wraps `AudioConverterPropertyID`.
pub type AudioConverterPropertyId = u32;
/// Wraps `SystemSoundID`.
pub type SystemSoundId = u32;
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
    /// Wraps `AudioBuffer`.
    pub mSampleRate: f64,
    /// Wraps `AudioBuffer`.
    pub mFormatID: AudioFormatId,
    /// Wraps `AudioBuffer`.
    pub mFormatFlags: AudioFormatFlags,
    /// Wraps `AudioBuffer`.
    pub mBytesPerPacket: u32,
    /// Wraps `AudioBuffer`.
    pub mFramesPerPacket: u32,
    /// Wraps `AudioBuffer`.
    pub mBytesPerFrame: u32,
    /// Wraps `AudioBuffer`.
    pub mChannelsPerFrame: u32,
    /// Wraps `AudioBuffer`.
    pub mBitsPerChannel: u32,
    /// Wraps `AudioBuffer`.
    pub mReserved: u32,
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
    pub mStartOffset: i64,
    pub mVariableFramesInPacket: u32,
    pub mDataByteSize: u32,
}

unsafe extern "C" {
    /// Raw binding for `AudioToolboxAudiofilecreatewithurl`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudiofilecreatewithurl`.
    pub fn AudioFileCreateWithURL(
        in_file_ref: CFURLRef,
        in_file_type: AudioFileTypeId,
        in_format: *const AudioStreamBasicDescription,
        in_flags: AudioFileFlags,
        out_audio_file: *mut AudioFileId,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxAudiofileopenurl`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudiofileopenurl`.
    pub fn AudioFileOpenURL(
        in_file_ref: CFURLRef,
        in_permissions: AudioFilePermissions,
        in_file_type_hint: AudioFileTypeId,
        out_audio_file: *mut AudioFileId,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxAudiofileclose`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudiofileclose`.
    pub fn AudioFileClose(in_audio_file: AudioFileId) -> OSStatus;
    /// Raw binding for `AudioToolboxAudiofilereadpacketdata`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudiofilereadpacketdata`.
    pub fn AudioFileReadPacketData(
        in_audio_file: AudioFileId,
        in_use_cache: Boolean,
        io_num_bytes: *mut u32,
        out_packet_descriptions: *mut AudioStreamPacketDescription,
        in_starting_packet: i64,
        io_num_packets: *mut u32,
        out_buffer: *mut c_void,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxAudiofilereadpackets`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudiofilereadpackets`.
    pub fn AudioFileReadPackets(
        in_audio_file: AudioFileId,
        in_use_cache: Boolean,
        out_num_bytes: *mut u32,
        out_packet_descriptions: *mut AudioStreamPacketDescription,
        in_starting_packet: i64,
        io_num_packets: *mut u32,
        out_buffer: *mut c_void,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxAudiofilewritepackets`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudiofilewritepackets`.
    pub fn AudioFileWritePackets(
        in_audio_file: AudioFileId,
        in_use_cache: Boolean,
        in_num_bytes: u32,
        in_packet_descriptions: *const AudioStreamPacketDescription,
        in_starting_packet: i64,
        io_num_packets: *mut u32,
        in_buffer: *const c_void,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxAudiofilegetpropertyinfo`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudiofilegetpropertyinfo`.
    pub fn AudioFileGetPropertyInfo(
        in_audio_file: AudioFileId,
        in_property_id: AudioFilePropertyId,
        out_data_size: *mut u32,
        is_writable: *mut u32,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxAudiofilegetproperty`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudiofilegetproperty`.
    pub fn AudioFileGetProperty(
        in_audio_file: AudioFileId,
        in_property_id: AudioFilePropertyId,
        io_data_size: *mut u32,
        out_property_data: *mut c_void,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxAudiofilesetproperty`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudiofilesetproperty`.
    pub fn AudioFileSetProperty(
        in_audio_file: AudioFileId,
        in_property_id: AudioFilePropertyId,
        in_data_size: u32,
        in_property_data: *const c_void,
    ) -> OSStatus;

    /// Raw binding for `AudioToolboxExtaudiofileopenurl`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxExtaudiofileopenurl`.
    pub fn ExtAudioFileOpenURL(
        in_url: CFURLRef,
        out_ext_audio_file: *mut ExtAudioFileRef,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxExtaudiofilecreatewithurl`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxExtaudiofilecreatewithurl`.
    pub fn ExtAudioFileCreateWithURL(
        in_url: CFURLRef,
        in_file_type: AudioFileTypeId,
        in_stream_desc: *const AudioStreamBasicDescription,
        in_channel_layout: *const c_void,
        in_flags: u32,
        out_ext_audio_file: *mut ExtAudioFileRef,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxExtaudiofiledispose`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxExtaudiofiledispose`.
    pub fn ExtAudioFileDispose(in_ext_audio_file: ExtAudioFileRef) -> OSStatus;
    /// Raw binding for `AudioToolboxExtaudiofileread`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxExtaudiofileread`.
    pub fn ExtAudioFileRead(
        in_ext_audio_file: ExtAudioFileRef,
        io_number_frames: *mut u32,
        io_data: *mut AudioBufferList1,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxExtaudiofilewrite`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxExtaudiofilewrite`.
    pub fn ExtAudioFileWrite(
        in_ext_audio_file: ExtAudioFileRef,
        in_number_frames: u32,
        io_data: *const AudioBufferList1,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxExtaudiofilegetproperty`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxExtaudiofilegetproperty`.
    pub fn ExtAudioFileGetProperty(
        in_ext_audio_file: ExtAudioFileRef,
        in_property_id: ExtAudioFilePropertyId,
        io_property_data_size: *mut u32,
        out_property_data: *mut c_void,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxExtaudiofilesetproperty`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxExtaudiofilesetproperty`.
    pub fn ExtAudioFileSetProperty(
        in_ext_audio_file: ExtAudioFileRef,
        in_property_id: ExtAudioFilePropertyId,
        in_property_data_size: u32,
        in_property_data: *const c_void,
    ) -> OSStatus;

    /// Raw binding for `AudioToolboxAudioconverternew`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudioconverternew`.
    pub fn AudioConverterNew(
        in_source_format: *const AudioStreamBasicDescription,
        in_destination_format: *const AudioStreamBasicDescription,
        out_audio_converter: *mut AudioConverterRef,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxAudioconverternewspecific`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudioconverternewspecific`.
    pub fn AudioConverterNewSpecific(
        in_source_format: *const AudioStreamBasicDescription,
        in_destination_format: *const AudioStreamBasicDescription,
        in_number_class_descriptions: u32,
        in_class_descriptions: *const AudioClassDescription,
        out_audio_converter: *mut AudioConverterRef,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxAudioconverterdispose`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudioconverterdispose`.
    pub fn AudioConverterDispose(in_audio_converter: AudioConverterRef) -> OSStatus;
    /// Raw binding for `AudioToolboxAudioconverterreset`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudioconverterreset`.
    pub fn AudioConverterReset(in_audio_converter: AudioConverterRef) -> OSStatus;
    /// Raw binding for `AudioToolboxAudioconvertergetpropertyinfo`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudioconvertergetpropertyinfo`.
    pub fn AudioConverterGetPropertyInfo(
        in_audio_converter: AudioConverterRef,
        in_property_id: AudioConverterPropertyId,
        out_size: *mut u32,
        out_writable: *mut Boolean,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxAudioconvertergetproperty`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudioconvertergetproperty`.
    pub fn AudioConverterGetProperty(
        in_audio_converter: AudioConverterRef,
        in_property_id: AudioConverterPropertyId,
        io_property_data_size: *mut u32,
        out_property_data: *mut c_void,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxAudioconvertersetproperty`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudioconvertersetproperty`.
    pub fn AudioConverterSetProperty(
        in_audio_converter: AudioConverterRef,
        in_property_id: AudioConverterPropertyId,
        in_property_data_size: u32,
        in_property_data: *const c_void,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxAudioconverterfillcomplexbuffer`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudioconverterfillcomplexbuffer`.
    pub fn AudioConverterFillComplexBuffer(
        in_audio_converter: AudioConverterRef,
        in_input_data_proc: extern "C" fn(
            AudioConverterRef,
            *mut u32,
            *mut AudioBufferList1,
            *mut *mut AudioStreamPacketDescription,
            *mut c_void,
        ) -> OSStatus,
        in_input_data_proc_user_data: *mut c_void,
        io_output_data_packet_size: *mut u32,
        out_output_data: *mut AudioBufferList1,
        out_packet_description: *mut AudioStreamPacketDescription,
    ) -> OSStatus;

    /// Raw binding for `AudioToolboxAudiocomponentfindnext`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudiocomponentfindnext`.
    pub fn AudioComponentFindNext(
        in_component: AudioComponentRef,
        in_desc: *const AudioComponentDescription,
    ) -> AudioComponentRef;
    /// Raw binding for `AudioToolboxAudiocomponentcount`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudiocomponentcount`.
    pub fn AudioComponentCount(in_desc: *const AudioComponentDescription) -> u32;
    /// Raw binding for `AudioToolboxAudiocomponentcopyname`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudiocomponentcopyname`.
    pub fn AudioComponentCopyName(
        in_component: AudioComponentRef,
        out_name: *mut CFStringRef,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxAudiocomponentgetdescription`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudiocomponentgetdescription`.
    pub fn AudioComponentGetDescription(
        in_component: AudioComponentRef,
        out_desc: *mut AudioComponentDescription,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxAudiocomponentgetversion`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudiocomponentgetversion`.
    pub fn AudioComponentGetVersion(
        in_component: AudioComponentRef,
        out_version: *mut u32,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxAudiocomponentinstancenew`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudiocomponentinstancenew`.
    pub fn AudioComponentInstanceNew(
        in_component: AudioComponentRef,
        out_instance: *mut AudioComponentInstanceRef,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxAudiocomponentinstancedispose`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudiocomponentinstancedispose`.
    pub fn AudioComponentInstanceDispose(in_instance: AudioComponentInstanceRef) -> OSStatus;
    /// Raw binding for `AudioToolboxAudiocomponentinstancegetcomponent`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudiocomponentinstancegetcomponent`.
    pub fn AudioComponentInstanceGetComponent(
        in_instance: AudioComponentInstanceRef,
    ) -> AudioComponentRef;

    /// Raw binding for `AudioToolboxAudioservicescreatesystemsoundid`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudioservicescreatesystemsoundid`.
    pub fn AudioServicesCreateSystemSoundID(
        in_file_url: CFURLRef,
        out_system_sound_id: *mut SystemSoundId,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxAudioservicesdisposesystemsoundid`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudioservicesdisposesystemsoundid`.
    pub fn AudioServicesDisposeSystemSoundID(in_system_sound_id: SystemSoundId) -> OSStatus;
    /// Raw binding for `AudioToolboxAudioservicesplaysystemsound`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudioservicesplaysystemsound`.
    pub fn AudioServicesPlaySystemSound(in_system_sound_id: SystemSoundId);

    /// Raw binding for `AudioToolboxCashow`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxCashow`.
    pub fn CAShow(in_object: *mut c_void);
    /// Raw binding for `AudioToolboxCashowfile`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxCashowfile`.
    pub fn CAShowFile(in_object: *mut c_void, in_file: *mut libc::FILE);
}

/// Wraps `AUGraphRef`.
pub type AUGraphRef = *mut c_void;
/// Wraps `AudioFormatPropertyID`.
pub type AudioFormatPropertyId = u32;
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
/// Wraps `AudioUnitRef`.
pub type AudioUnitRef = *mut c_void;
/// Wraps `MIDIEndpointRef`.
pub type MIDIEndpointRef = u32;
/// Wraps `MusicSequenceType`.
pub type MusicSequenceType = u32;
/// Wraps `MusicSequenceLoadFlags`.
pub type MusicSequenceLoadFlags = u32;
/// Wraps `MusicSequenceFileTypeID`.
pub type MusicSequenceFileTypeId = u32;
/// Wraps `MusicSequenceFileFlags`.
pub type MusicSequenceFileFlags = u32;
/// Wraps `MusicTimeStamp`.
pub type MusicTimeStamp = f64;
/// Wraps `MusicSequenceRef`.
pub type MusicSequenceRef = *mut c_void;
/// Wraps `MusicTrackRef`.
pub type MusicTrackRef = *mut c_void;
/// Wraps `MusicPlayerRef`.
pub type MusicPlayerRef = *mut c_void;
/// Wraps `MusicEventIteratorRef`.
pub type MusicEventIteratorRef = *mut c_void;
/// Wraps `MusicEventType`.
pub type MusicEventType = u32;
/// Wraps `MusicTrackPropertyID`.
pub type MusicTrackPropertyId = u32;
/// Wraps `AUNode`.
pub type AUNode = i32;

pub use crate::types::{
    AUNodeInteraction, AUPresetEvent, AURenderCallback, AURenderCallbackStruct, AudioBalanceFade,
    AudioBufferList1 as AudioUnitBufferList1, AudioConverterPrimeInfo, AudioFormatInfo,
    AudioFormatListItem, AudioPanningInfo, AudioTimeStamp, AudioUnitParameterEvent,
    AudioUnitPropertyListenerProc, CABarBeatTime, ExtendedNoteOnEvent, MIDIChannelMessage,
    MIDIMetaEvent, MIDINoteMessage, MIDIRawData, MusicEventUserData, ParameterEvent,
    AUDIO_FORMAT_PROPERTY_BALANCE_FADE, AUDIO_FORMAT_PROPERTY_FIRST_PLAYABLE_FORMAT_FROM_LIST,
    AUDIO_FORMAT_PROPERTY_FORMAT_LIST, AUDIO_FORMAT_PROPERTY_OUTPUT_FORMAT_LIST,
    AUDIO_FORMAT_PROPERTY_PANNING_MATRIX, AUDIO_OUTPUT_UNIT_PROPERTY_ENABLE_IO,
    AUDIO_OUTPUT_UNIT_PROPERTY_MIDI_CALLBACKS, AUDIO_UNIT_PROPERTY_AUDIO_CHANNEL_LAYOUT,
    AUDIO_UNIT_PROPERTY_CLASS_INFO, AUDIO_UNIT_PROPERTY_CLASS_INFO_FROM_DOCUMENT,
    AUDIO_UNIT_PROPERTY_CPULOAD, AUDIO_UNIT_PROPERTY_DEPENDENT_PARAMETERS,
    AUDIO_UNIT_PROPERTY_ELEMENT_COUNT, AUDIO_UNIT_PROPERTY_HOST_CALLBACKS,
    AUDIO_UNIT_PROPERTY_INPUT_SAMPLES_IN_OUTPUT, AUDIO_UNIT_PROPERTY_LAST_RENDER_ERROR,
    AUDIO_UNIT_PROPERTY_LATENCY, AUDIO_UNIT_PROPERTY_MAKE_CONNECTION,
    AUDIO_UNIT_PROPERTY_OFFLINE_RENDER, AUDIO_UNIT_PROPERTY_PARAMETER_CLUMP_NAME,
    AUDIO_UNIT_PROPERTY_PARAMETER_INFO, AUDIO_UNIT_PROPERTY_PARAMETER_LIST,
    AUDIO_UNIT_PROPERTY_PARAMETER_STRING_FROM_VALUE,
    AUDIO_UNIT_PROPERTY_PARAMETER_VALUE_FROM_STRING, AUDIO_UNIT_PROPERTY_PARAMETER_VALUE_STRINGS,
    AUDIO_UNIT_PROPERTY_PRESENTATION_LATENCY, AUDIO_UNIT_PROPERTY_PRESENT_PRESET,
    AUDIO_UNIT_PROPERTY_SAMPLE_RATE, AUDIO_UNIT_PROPERTY_SET_RENDER_CALLBACK,
    AUDIO_UNIT_PROPERTY_STREAM_FORMAT, AUDIO_UNIT_PROPERTY_SUPPORTED_NUM_CHANNELS,
    AUDIO_UNIT_SCOPE_GLOBAL, AUDIO_UNIT_SCOPE_GROUP, AUDIO_UNIT_SCOPE_INPUT,
    AUDIO_UNIT_SCOPE_OUTPUT, MUSIC_EVENT_TYPE_AU_PRESET, MUSIC_EVENT_TYPE_EXTENDED_NOTE,
    MUSIC_EVENT_TYPE_EXTENDED_TEMPO, MUSIC_EVENT_TYPE_META, MUSIC_EVENT_TYPE_MIDI_CHANNEL_MESSAGE,
    MUSIC_EVENT_TYPE_MIDI_NOTE_MESSAGE, MUSIC_EVENT_TYPE_MIDI_RAW_DATA, MUSIC_EVENT_TYPE_PARAMETER,
    MUSIC_EVENT_TYPE_USER,
};

unsafe extern "C" {
    /// Raw binding for `AudioToolboxAudioformatgetpropertyinfo`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudioformatgetpropertyinfo`.
    pub fn AudioFormatGetPropertyInfo(
        in_property_id: AudioFormatPropertyId,
        in_specifier_size: u32,
        in_specifier: *const c_void,
        out_property_data_size: *mut u32,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxAudioformatgetproperty`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudioformatgetproperty`.
    pub fn AudioFormatGetProperty(
        in_property_id: AudioFormatPropertyId,
        in_specifier_size: u32,
        in_specifier: *const c_void,
        io_property_data_size: *mut u32,
        out_property_data: *mut c_void,
    ) -> OSStatus;

    /// Raw binding for `AudioToolboxAudiofileoptimize`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudiofileoptimize`.
    pub fn AudioFileOptimize(in_audio_file: AudioFileId) -> OSStatus;
    /// Raw binding for `AudioToolboxAudiofilereadbytes`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudiofilereadbytes`.
    pub fn AudioFileReadBytes(
        in_audio_file: AudioFileId,
        in_use_cache: Boolean,
        in_starting_byte: i64,
        io_num_bytes: *mut u32,
        out_buffer: *mut c_void,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxAudiofilewritebytes`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudiofilewritebytes`.
    pub fn AudioFileWriteBytes(
        in_audio_file: AudioFileId,
        in_use_cache: Boolean,
        in_starting_byte: i64,
        io_num_bytes: *mut u32,
        in_buffer: *const c_void,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxAudiofilecountuserdata`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudiofilecountuserdata`.
    pub fn AudioFileCountUserData(
        in_audio_file: AudioFileId,
        in_user_data_id: u32,
        out_number_items: *mut u32,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxAudiofilegetuserdatasize`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudiofilegetuserdatasize`.
    pub fn AudioFileGetUserDataSize(
        in_audio_file: AudioFileId,
        in_user_data_id: u32,
        in_index: u32,
        out_user_data_size: *mut u32,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxAudiofilegetuserdatasize64`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudiofilegetuserdatasize64`.
    pub fn AudioFileGetUserDataSize64(
        in_audio_file: AudioFileId,
        in_user_data_id: u32,
        in_index: u32,
        out_user_data_size: *mut u64,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxAudiofilegetuserdata`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudiofilegetuserdata`.
    pub fn AudioFileGetUserData(
        in_audio_file: AudioFileId,
        in_user_data_id: u32,
        in_index: u32,
        io_user_data_size: *mut u32,
        out_user_data: *mut c_void,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxAudiofilegetuserdataatoffset`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudiofilegetuserdataatoffset`.
    pub fn AudioFileGetUserDataAtOffset(
        in_audio_file: AudioFileId,
        in_user_data_id: u32,
        in_index: u32,
        in_offset: i64,
        io_user_data_size: *mut u32,
        out_user_data: *mut c_void,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxAudiofilesetuserdata`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudiofilesetuserdata`.
    pub fn AudioFileSetUserData(
        in_audio_file: AudioFileId,
        in_user_data_id: u32,
        in_index: u32,
        in_user_data_size: u32,
        in_user_data: *const c_void,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxAudiofileremoveuserdata`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudiofileremoveuserdata`.
    pub fn AudioFileRemoveUserData(
        in_audio_file: AudioFileId,
        in_user_data_id: u32,
        in_index: u32,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxAudiofilegetglobalinfosize`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudiofilegetglobalinfosize`.
    pub fn AudioFileGetGlobalInfoSize(
        in_property_id: AudioFilePropertyId,
        in_specifier_size: u32,
        in_specifier: *const c_void,
        out_property_data_size: *mut u32,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxAudiofilegetglobalinfo`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudiofilegetglobalinfo`.
    pub fn AudioFileGetGlobalInfo(
        in_property_id: AudioFilePropertyId,
        in_specifier_size: u32,
        in_specifier: *const c_void,
        io_property_data_size: *mut u32,
        out_property_data: *mut c_void,
    ) -> OSStatus;

    /// Raw binding for `AudioToolboxAudioconverterconvertbuffer`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudioconverterconvertbuffer`.
    pub fn AudioConverterConvertBuffer(
        in_audio_converter: AudioConverterRef,
        in_input_data_size: u32,
        in_input_data: *const c_void,
        io_output_data_size: *mut u32,
        out_output_data: *mut c_void,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxAudioconverterconvertcomplexbuffer`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudioconverterconvertcomplexbuffer`.
    pub fn AudioConverterConvertComplexBuffer(
        in_audio_converter: AudioConverterRef,
        in_number_pcm_frames: u32,
        in_input_data: *const AudioBufferList1,
        out_output_data: *mut AudioBufferList1,
    ) -> OSStatus;

    /// Raw binding for `AudioToolboxAudiocomponentinstancecando`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudiocomponentinstancecando`.
    pub fn AudioComponentInstanceCanDo(
        in_instance: AudioComponentInstanceRef,
        in_selector_id: i16,
    ) -> Boolean;
    /// Raw binding for `AudioToolboxAudiocomponentcopyconfigurationinfo`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudiocomponentcopyconfigurationinfo`.
    pub fn AudioComponentCopyConfigurationInfo(
        in_component: AudioComponentRef,
        out_configuration_info: *mut CFDictionaryRef,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxAudiocomponentvalidate`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudiocomponentvalidate`.
    pub fn AudioComponentValidate(
        in_component: AudioComponentRef,
        in_validation_parameters: CFDictionaryRef,
        out_validation_result: *mut u32,
    ) -> OSStatus;

    /// Raw binding for `AudioToolboxAudiounitgetpropertyinfo`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudiounitgetpropertyinfo`.
    pub fn AudioUnitGetPropertyInfo(
        in_unit: AudioUnitRef,
        in_id: AudioUnitPropertyId,
        in_scope: AudioUnitScope,
        in_element: AudioUnitElement,
        out_data_size: *mut u32,
        out_writable: *mut Boolean,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxAudiounitgetproperty`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudiounitgetproperty`.
    pub fn AudioUnitGetProperty(
        in_unit: AudioUnitRef,
        in_id: AudioUnitPropertyId,
        in_scope: AudioUnitScope,
        in_element: AudioUnitElement,
        io_data_size: *mut u32,
        out_data: *mut c_void,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxAudiounitsetproperty`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudiounitsetproperty`.
    pub fn AudioUnitSetProperty(
        in_unit: AudioUnitRef,
        in_id: AudioUnitPropertyId,
        in_scope: AudioUnitScope,
        in_element: AudioUnitElement,
        in_data_size: u32,
        in_data: *const c_void,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxAudiounitaddpropertylistener`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudiounitaddpropertylistener`.
    pub fn AudioUnitAddPropertyListener(
        in_unit: AudioUnitRef,
        in_id: AudioUnitPropertyId,
        in_proc: AudioUnitPropertyListenerProc,
        in_proc_user_data: *mut c_void,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxAudiounitremovepropertylistenerwithuserdata`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudiounitremovepropertylistenerwithuserdata`.
    pub fn AudioUnitRemovePropertyListenerWithUserData(
        in_unit: AudioUnitRef,
        in_id: AudioUnitPropertyId,
        in_proc: AudioUnitPropertyListenerProc,
        in_proc_user_data: *mut c_void,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxAudiounitaddrendernotify`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudiounitaddrendernotify`.
    pub fn AudioUnitAddRenderNotify(
        in_unit: AudioUnitRef,
        in_proc: AURenderCallback,
        in_proc_user_data: *mut c_void,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxAudiounitremoverendernotify`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudiounitremoverendernotify`.
    pub fn AudioUnitRemoveRenderNotify(
        in_unit: AudioUnitRef,
        in_proc: AURenderCallback,
        in_proc_user_data: *mut c_void,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxAudiounitgetparameter`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudiounitgetparameter`.
    pub fn AudioUnitGetParameter(
        in_unit: AudioUnitRef,
        in_id: AudioUnitParameterId,
        in_scope: AudioUnitScope,
        in_element: AudioUnitElement,
        out_value: *mut AudioUnitParameterValue,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxAudiounitsetparameter`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudiounitsetparameter`.
    pub fn AudioUnitSetParameter(
        in_unit: AudioUnitRef,
        in_id: AudioUnitParameterId,
        in_scope: AudioUnitScope,
        in_element: AudioUnitElement,
        in_value: AudioUnitParameterValue,
        in_buffer_offset_in_frames: u32,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxAudiounitscheduleparameters`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudiounitscheduleparameters`.
    pub fn AudioUnitScheduleParameters(
        in_unit: AudioUnitRef,
        in_num_param_events: u32,
        in_param_events: *const AudioUnitParameterEvent,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxAudiounitrender`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudiounitrender`.
    pub fn AudioUnitRender(
        in_unit: AudioUnitRef,
        io_action_flags: *mut AudioUnitRenderActionFlags,
        in_time_stamp: *const AudioTimeStamp,
        in_output_bus_number: u32,
        in_number_frames: u32,
        io_data: *mut AudioBufferList1,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxAudiounitinitialize`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudiounitinitialize`.
    pub fn AudioUnitInitialize(in_unit: AudioUnitRef) -> OSStatus;
    /// Raw binding for `AudioToolboxAudiounituninitialize`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudiounituninitialize`.
    pub fn AudioUnitUninitialize(in_unit: AudioUnitRef) -> OSStatus;
    /// Raw binding for `AudioToolboxAudiooutputunitstart`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudiooutputunitstart`.
    pub fn AudioOutputUnitStart(ci: AudioUnitRef) -> OSStatus;
    /// Raw binding for `AudioToolboxAudiooutputunitstop`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAudiooutputunitstop`.
    pub fn AudioOutputUnitStop(ci: AudioUnitRef) -> OSStatus;

    /// Raw binding for `AudioToolboxNewaugraph`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxNewaugraph`.
    pub fn NewAUGraph(out_graph: *mut AUGraphRef) -> OSStatus;
    /// Raw binding for `AudioToolboxDisposeaugraph`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxDisposeaugraph`.
    pub fn DisposeAUGraph(in_graph: AUGraphRef) -> OSStatus;
    /// Raw binding for `AudioToolboxAugraphopen`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAugraphopen`.
    pub fn AUGraphOpen(in_graph: AUGraphRef) -> OSStatus;
    /// Raw binding for `AudioToolboxAugraphclose`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAugraphclose`.
    pub fn AUGraphClose(in_graph: AUGraphRef) -> OSStatus;
    /// Raw binding for `AudioToolboxAugraphinitialize`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAugraphinitialize`.
    pub fn AUGraphInitialize(in_graph: AUGraphRef) -> OSStatus;
    /// Raw binding for `AudioToolboxAugraphuninitialize`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAugraphuninitialize`.
    pub fn AUGraphUninitialize(in_graph: AUGraphRef) -> OSStatus;
    /// Raw binding for `AudioToolboxAugraphstart`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAugraphstart`.
    pub fn AUGraphStart(in_graph: AUGraphRef) -> OSStatus;
    /// Raw binding for `AudioToolboxAugraphstop`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAugraphstop`.
    pub fn AUGraphStop(in_graph: AUGraphRef) -> OSStatus;
    /// Raw binding for `AudioToolboxAugraphisopen`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAugraphisopen`.
    pub fn AUGraphIsOpen(in_graph: AUGraphRef, out_is_open: *mut Boolean) -> OSStatus;
    /// Raw binding for `AudioToolboxAugraphisinitialized`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAugraphisinitialized`.
    pub fn AUGraphIsInitialized(in_graph: AUGraphRef, out_is_initialized: *mut Boolean)
        -> OSStatus;
    /// Raw binding for `AudioToolboxAugraphisrunning`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAugraphisrunning`.
    pub fn AUGraphIsRunning(in_graph: AUGraphRef, out_is_running: *mut Boolean) -> OSStatus;
    /// Raw binding for `AudioToolboxAugraphgetnodecount`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAugraphgetnodecount`.
    pub fn AUGraphGetNodeCount(in_graph: AUGraphRef, out_number_of_nodes: *mut u32) -> OSStatus;
    /// Raw binding for `AudioToolboxAugraphaddnode`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAugraphaddnode`.
    pub fn AUGraphAddNode(
        in_graph: AUGraphRef,
        in_description: *const AudioComponentDescription,
        out_node: *mut AUNode,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxAugraphconnectnodeinput`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAugraphconnectnodeinput`.
    pub fn AUGraphConnectNodeInput(
        in_graph: AUGraphRef,
        in_source_node: AUNode,
        in_source_output_number: u32,
        in_dest_node: AUNode,
        in_dest_input_number: u32,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxAugraphsetnodeinputcallback`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAugraphsetnodeinputcallback`.
    pub fn AUGraphSetNodeInputCallback(
        in_graph: AUGraphRef,
        in_dest_node: AUNode,
        in_dest_input_number: u32,
        in_input_callback: *const AURenderCallbackStruct,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxAugraphdisconnectnodeinput`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAugraphdisconnectnodeinput`.
    pub fn AUGraphDisconnectNodeInput(
        in_graph: AUGraphRef,
        in_dest_node: AUNode,
        in_dest_input_number: u32,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxAugraphclearconnections`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAugraphclearconnections`.
    pub fn AUGraphClearConnections(in_graph: AUGraphRef) -> OSStatus;
    /// Raw binding for `AudioToolboxAugraphgetnumberofinteractions`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAugraphgetnumberofinteractions`.
    pub fn AUGraphGetNumberOfInteractions(
        in_graph: AUGraphRef,
        out_num_interactions: *mut u32,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxAugraphgetinteractioninfo`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAugraphgetinteractioninfo`.
    pub fn AUGraphGetInteractionInfo(
        in_graph: AUGraphRef,
        in_interaction_index: u32,
        out_interaction: *mut AUNodeInteraction,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxAugraphcountnodeinteractions`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAugraphcountnodeinteractions`.
    pub fn AUGraphCountNodeInteractions(
        in_graph: AUGraphRef,
        in_node: AUNode,
        out_num_interactions: *mut u32,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxAugraphgetnodeinteractions`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAugraphgetnodeinteractions`.
    pub fn AUGraphGetNodeInteractions(
        in_graph: AUGraphRef,
        in_node: AUNode,
        io_num_interactions: *mut u32,
        out_interactions: *mut AUNodeInteraction,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxAugraphupdate`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAugraphupdate`.
    pub fn AUGraphUpdate(in_graph: AUGraphRef, out_is_updated: *mut Boolean) -> OSStatus;
    /// Raw binding for `AudioToolboxAugraphgetcpuload`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAugraphgetcpuload`.
    pub fn AUGraphGetCPULoad(in_graph: AUGraphRef, out_average_cpu_load: *mut f32) -> OSStatus;
    /// Raw binding for `AudioToolboxAugraphgetmaxcpuload`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAugraphgetmaxcpuload`.
    pub fn AUGraphGetMaxCPULoad(in_graph: AUGraphRef, out_max_cpu_load: *mut f32) -> OSStatus;
    /// Raw binding for `AudioToolboxAugraphaddrendernotify`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAugraphaddrendernotify`.
    pub fn AUGraphAddRenderNotify(
        in_graph: AUGraphRef,
        in_callback: AURenderCallback,
        in_ref_con: *mut c_void,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxAugraphremoverendernotify`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAugraphremoverendernotify`.
    pub fn AUGraphRemoveRenderNotify(
        in_graph: AUGraphRef,
        in_callback: AURenderCallback,
        in_ref_con: *mut c_void,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxAugraphnodeinfo`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxAugraphnodeinfo`.
    pub fn AUGraphNodeInfo(
        in_graph: AUGraphRef,
        in_node: AUNode,
        out_description: *mut AudioComponentDescription,
        out_audio_unit: *mut AudioUnitRef,
    ) -> OSStatus;

    /// Raw binding for `AudioToolboxNewmusicsequence`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxNewmusicsequence`.
    pub fn NewMusicSequence(out_sequence: *mut MusicSequenceRef) -> OSStatus;
    /// Raw binding for `AudioToolboxDisposemusicsequence`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxDisposemusicsequence`.
    pub fn DisposeMusicSequence(in_sequence: MusicSequenceRef) -> OSStatus;
    /// Raw binding for `AudioToolboxMusicsequencenewtrack`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusicsequencenewtrack`.
    pub fn MusicSequenceNewTrack(
        in_sequence: MusicSequenceRef,
        out_track: *mut MusicTrackRef,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxMusicsequencedisposetrack`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusicsequencedisposetrack`.
    pub fn MusicSequenceDisposeTrack(
        in_sequence: MusicSequenceRef,
        in_track: MusicTrackRef,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxMusicsequencegettrackcount`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusicsequencegettrackcount`.
    pub fn MusicSequenceGetTrackCount(
        in_sequence: MusicSequenceRef,
        out_number_of_tracks: *mut u32,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxMusicsequencegetindtrack`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusicsequencegetindtrack`.
    pub fn MusicSequenceGetIndTrack(
        in_sequence: MusicSequenceRef,
        in_track_index: u32,
        out_track: *mut MusicTrackRef,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxMusicsequencegettrackindex`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusicsequencegettrackindex`.
    pub fn MusicSequenceGetTrackIndex(
        in_sequence: MusicSequenceRef,
        in_track: MusicTrackRef,
        out_track_index: *mut u32,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxMusicsequencegettempotrack`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusicsequencegettempotrack`.
    pub fn MusicSequenceGetTempoTrack(
        in_sequence: MusicSequenceRef,
        out_tempo_track: *mut MusicTrackRef,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxMusicsequencesetaugraph`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusicsequencesetaugraph`.
    pub fn MusicSequenceSetAUGraph(in_sequence: MusicSequenceRef, in_graph: AUGraphRef)
        -> OSStatus;
    /// Raw binding for `AudioToolboxMusicsequencegetaugraph`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusicsequencegetaugraph`.
    pub fn MusicSequenceGetAUGraph(
        in_sequence: MusicSequenceRef,
        out_graph: *mut AUGraphRef,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxMusicsequencesetmidiendpoint`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusicsequencesetmidiendpoint`.
    pub fn MusicSequenceSetMIDIEndpoint(
        in_sequence: MusicSequenceRef,
        in_endpoint: MIDIEndpointRef,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxMusicsequencesetsequencetype`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusicsequencesetsequencetype`.
    pub fn MusicSequenceSetSequenceType(
        in_sequence: MusicSequenceRef,
        in_sequence_type: MusicSequenceType,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxMusicsequencegetsequencetype`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusicsequencegetsequencetype`.
    pub fn MusicSequenceGetSequenceType(
        in_sequence: MusicSequenceRef,
        out_sequence_type: *mut MusicSequenceType,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxMusicsequencefileload`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusicsequencefileload`.
    pub fn MusicSequenceFileLoad(
        in_sequence: MusicSequenceRef,
        in_file_ref: CFURLRef,
        in_file_type_hint: MusicSequenceFileTypeId,
        in_flags: MusicSequenceLoadFlags,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxMusicsequencefileloaddata`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusicsequencefileloaddata`.
    pub fn MusicSequenceFileLoadData(
        in_sequence: MusicSequenceRef,
        in_data: CFDataRef,
        in_file_type_hint: MusicSequenceFileTypeId,
        in_flags: MusicSequenceLoadFlags,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxMusicsequencefilecreate`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusicsequencefilecreate`.
    pub fn MusicSequenceFileCreate(
        in_sequence: MusicSequenceRef,
        in_file_ref: CFURLRef,
        in_file_type: MusicSequenceFileTypeId,
        in_flags: MusicSequenceFileFlags,
        in_resolution: i16,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxMusicsequencefilecreatedata`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusicsequencefilecreatedata`.
    pub fn MusicSequenceFileCreateData(
        in_sequence: MusicSequenceRef,
        in_file_type: MusicSequenceFileTypeId,
        in_flags: MusicSequenceFileFlags,
        in_resolution: i16,
        out_data: *mut CFDataRef,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxMusicsequencegetsecondsforbeats`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusicsequencegetsecondsforbeats`.
    pub fn MusicSequenceGetSecondsForBeats(
        in_sequence: MusicSequenceRef,
        in_beats: MusicTimeStamp,
        out_seconds: *mut f64,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxMusicsequencegetbeatsforseconds`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusicsequencegetbeatsforseconds`.
    pub fn MusicSequenceGetBeatsForSeconds(
        in_sequence: MusicSequenceRef,
        in_seconds: f64,
        out_beats: *mut MusicTimeStamp,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxMusicsequencebeatstobarbeattime`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusicsequencebeatstobarbeattime`.
    pub fn MusicSequenceBeatsToBarBeatTime(
        in_sequence: MusicSequenceRef,
        in_beats: MusicTimeStamp,
        in_subbeat_divisor: u32,
        out_bar_beat_time: *mut CABarBeatTime,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxMusicsequencebarbeattimetobeats`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusicsequencebarbeattimetobeats`.
    pub fn MusicSequenceBarBeatTimeToBeats(
        in_sequence: MusicSequenceRef,
        in_bar_beat_time: *const CABarBeatTime,
        out_beats: *mut MusicTimeStamp,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxMusicsequencegetinfodictionary`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusicsequencegetinfodictionary`.
    pub fn MusicSequenceGetInfoDictionary(in_sequence: MusicSequenceRef) -> CFDictionaryRef;
    /// Raw binding for `AudioToolboxMusictrackgetsequence`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusictrackgetsequence`.
    pub fn MusicTrackGetSequence(
        in_track: MusicTrackRef,
        out_sequence: *mut MusicSequenceRef,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxMusictracksetdestnode`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusictracksetdestnode`.
    pub fn MusicTrackSetDestNode(in_track: MusicTrackRef, in_node: AUNode) -> OSStatus;
    /// Raw binding for `AudioToolboxMusictracksetdestmidiendpoint`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusictracksetdestmidiendpoint`.
    pub fn MusicTrackSetDestMIDIEndpoint(
        in_track: MusicTrackRef,
        in_endpoint: MIDIEndpointRef,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxMusictrackgetdestnode`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusictrackgetdestnode`.
    pub fn MusicTrackGetDestNode(in_track: MusicTrackRef, out_node: *mut AUNode) -> OSStatus;
    /// Raw binding for `AudioToolboxMusictrackgetdestmidiendpoint`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusictrackgetdestmidiendpoint`.
    pub fn MusicTrackGetDestMIDIEndpoint(
        in_track: MusicTrackRef,
        out_endpoint: *mut MIDIEndpointRef,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxMusictracksetproperty`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusictracksetproperty`.
    pub fn MusicTrackSetProperty(
        in_track: MusicTrackRef,
        in_property_id: MusicTrackPropertyId,
        in_data: *const c_void,
        in_length: u32,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxMusictrackgetproperty`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusictrackgetproperty`.
    pub fn MusicTrackGetProperty(
        in_track: MusicTrackRef,
        in_property_id: MusicTrackPropertyId,
        out_data: *mut c_void,
        io_length: *mut u32,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxMusictrackmoveevents`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusictrackmoveevents`.
    pub fn MusicTrackMoveEvents(
        in_track: MusicTrackRef,
        in_start_time: MusicTimeStamp,
        in_end_time: MusicTimeStamp,
        in_move_time: MusicTimeStamp,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxMusictrackclear`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusictrackclear`.
    pub fn MusicTrackClear(
        in_track: MusicTrackRef,
        in_start_time: MusicTimeStamp,
        in_end_time: MusicTimeStamp,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxMusictrackcut`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusictrackcut`.
    pub fn MusicTrackCut(
        in_track: MusicTrackRef,
        in_start_time: MusicTimeStamp,
        in_end_time: MusicTimeStamp,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxMusictrackcopyinsert`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusictrackcopyinsert`.
    pub fn MusicTrackCopyInsert(
        in_source_track: MusicTrackRef,
        in_source_start_time: MusicTimeStamp,
        in_source_end_time: MusicTimeStamp,
        in_dest_track: MusicTrackRef,
        in_dest_insert_time: MusicTimeStamp,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxMusictrackmerge`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusictrackmerge`.
    pub fn MusicTrackMerge(
        in_source_track: MusicTrackRef,
        in_source_start_time: MusicTimeStamp,
        in_source_end_time: MusicTimeStamp,
        in_dest_track: MusicTrackRef,
        in_dest_insert_time: MusicTimeStamp,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxMusictracknewmidinoteevent`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusictracknewmidinoteevent`.
    pub fn MusicTrackNewMIDINoteEvent(
        in_track: MusicTrackRef,
        in_time_stamp: MusicTimeStamp,
        in_note_message: *const MIDINoteMessage,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxMusictracknewmidichannelevent`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusictracknewmidichannelevent`.
    pub fn MusicTrackNewMIDIChannelEvent(
        in_track: MusicTrackRef,
        in_time_stamp: MusicTimeStamp,
        in_message: *const MIDIChannelMessage,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxMusictracknewmidirawdataevent`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusictracknewmidirawdataevent`.
    pub fn MusicTrackNewMIDIRawDataEvent(
        in_track: MusicTrackRef,
        in_time_stamp: MusicTimeStamp,
        in_raw_data: *const MIDIRawData,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxMusictracknewextendednoteevent`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusictracknewextendednoteevent`.
    pub fn MusicTrackNewExtendedNoteEvent(
        in_track: MusicTrackRef,
        in_time_stamp: MusicTimeStamp,
        in_note_event: *const ExtendedNoteOnEvent,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxMusictracknewparameterevent`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusictracknewparameterevent`.
    pub fn MusicTrackNewParameterEvent(
        in_track: MusicTrackRef,
        in_time_stamp: MusicTimeStamp,
        in_event: *const ParameterEvent,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxMusictracknewextendedtempoevent`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusictracknewextendedtempoevent`.
    pub fn MusicTrackNewExtendedTempoEvent(
        in_track: MusicTrackRef,
        in_time_stamp: MusicTimeStamp,
        in_bpm: f64,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxMusictracknewmetaevent`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusictracknewmetaevent`.
    pub fn MusicTrackNewMetaEvent(
        in_track: MusicTrackRef,
        in_time_stamp: MusicTimeStamp,
        in_meta_event: *const MIDIMetaEvent,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxMusictracknewuserevent`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusictracknewuserevent`.
    pub fn MusicTrackNewUserEvent(
        in_track: MusicTrackRef,
        in_time_stamp: MusicTimeStamp,
        in_user_data: *const MusicEventUserData,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxMusictracknewaupresetevent`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusictracknewaupresetevent`.
    pub fn MusicTrackNewAUPresetEvent(
        in_track: MusicTrackRef,
        in_time_stamp: MusicTimeStamp,
        in_preset_event: *const AUPresetEvent,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxNewmusiceventiterator`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxNewmusiceventiterator`.
    pub fn NewMusicEventIterator(
        in_track: MusicTrackRef,
        out_iterator: *mut MusicEventIteratorRef,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxDisposemusiceventiterator`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxDisposemusiceventiterator`.
    pub fn DisposeMusicEventIterator(in_iterator: MusicEventIteratorRef) -> OSStatus;
    /// Raw binding for `AudioToolboxMusiceventiteratorseek`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusiceventiteratorseek`.
    pub fn MusicEventIteratorSeek(
        in_iterator: MusicEventIteratorRef,
        in_time_stamp: MusicTimeStamp,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxMusiceventiteratornextevent`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusiceventiteratornextevent`.
    pub fn MusicEventIteratorNextEvent(in_iterator: MusicEventIteratorRef) -> OSStatus;
    /// Raw binding for `AudioToolboxMusiceventiteratorpreviousevent`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusiceventiteratorpreviousevent`.
    pub fn MusicEventIteratorPreviousEvent(in_iterator: MusicEventIteratorRef) -> OSStatus;
    /// Raw binding for `AudioToolboxMusiceventiteratorgeteventinfo`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusiceventiteratorgeteventinfo`.
    pub fn MusicEventIteratorGetEventInfo(
        in_iterator: MusicEventIteratorRef,
        out_time_stamp: *mut MusicTimeStamp,
        out_event_type: *mut MusicEventType,
        out_event_data: *mut *const c_void,
        out_event_data_size: *mut u32,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxMusiceventiteratorseteventinfo`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusiceventiteratorseteventinfo`.
    pub fn MusicEventIteratorSetEventInfo(
        in_iterator: MusicEventIteratorRef,
        in_event_type: MusicEventType,
        in_event_data: *const c_void,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxMusiceventiteratorseteventtime`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusiceventiteratorseteventtime`.
    pub fn MusicEventIteratorSetEventTime(
        in_iterator: MusicEventIteratorRef,
        in_time_stamp: MusicTimeStamp,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxMusiceventiteratordeleteevent`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusiceventiteratordeleteevent`.
    pub fn MusicEventIteratorDeleteEvent(in_iterator: MusicEventIteratorRef) -> OSStatus;
    /// Raw binding for `AudioToolboxMusiceventiteratorhaspreviousevent`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusiceventiteratorhaspreviousevent`.
    pub fn MusicEventIteratorHasPreviousEvent(
        in_iterator: MusicEventIteratorRef,
        out_has_previous_event: *mut Boolean,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxMusiceventiteratorhasnextevent`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusiceventiteratorhasnextevent`.
    pub fn MusicEventIteratorHasNextEvent(
        in_iterator: MusicEventIteratorRef,
        out_has_next_event: *mut Boolean,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxMusiceventiteratorhascurrentevent`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusiceventiteratorhascurrentevent`.
    pub fn MusicEventIteratorHasCurrentEvent(
        in_iterator: MusicEventIteratorRef,
        out_has_current_event: *mut Boolean,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxNewmusicplayer`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxNewmusicplayer`.
    pub fn NewMusicPlayer(out_player: *mut MusicPlayerRef) -> OSStatus;
    /// Raw binding for `AudioToolboxDisposemusicplayer`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxDisposemusicplayer`.
    pub fn DisposeMusicPlayer(in_player: MusicPlayerRef) -> OSStatus;
    /// Raw binding for `AudioToolboxMusicplayersetsequence`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusicplayersetsequence`.
    pub fn MusicPlayerSetSequence(
        in_player: MusicPlayerRef,
        in_sequence: MusicSequenceRef,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxMusicplayergetsequence`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusicplayergetsequence`.
    pub fn MusicPlayerGetSequence(
        in_player: MusicPlayerRef,
        out_sequence: *mut MusicSequenceRef,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxMusicplayersettime`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusicplayersettime`.
    pub fn MusicPlayerSetTime(in_player: MusicPlayerRef, in_time: MusicTimeStamp) -> OSStatus;
    /// Raw binding for `AudioToolboxMusicplayergettime`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusicplayergettime`.
    pub fn MusicPlayerGetTime(in_player: MusicPlayerRef, out_time: *mut MusicTimeStamp)
        -> OSStatus;
    /// Raw binding for `AudioToolboxMusicplayergethosttimeforbeats`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusicplayergethosttimeforbeats`.
    pub fn MusicPlayerGetHostTimeForBeats(
        in_player: MusicPlayerRef,
        in_beats: MusicTimeStamp,
        out_host_time: *mut u64,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxMusicplayergetbeatsforhosttime`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusicplayergetbeatsforhosttime`.
    pub fn MusicPlayerGetBeatsForHostTime(
        in_player: MusicPlayerRef,
        in_host_time: u64,
        out_beats: *mut MusicTimeStamp,
    ) -> OSStatus;
    /// Raw binding for `AudioToolboxMusicplayerpreroll`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusicplayerpreroll`.
    pub fn MusicPlayerPreroll(in_player: MusicPlayerRef) -> OSStatus;
    /// Raw binding for `AudioToolboxMusicplayerstart`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusicplayerstart`.
    pub fn MusicPlayerStart(in_player: MusicPlayerRef) -> OSStatus;
    /// Raw binding for `AudioToolboxMusicplayerstop`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusicplayerstop`.
    pub fn MusicPlayerStop(in_player: MusicPlayerRef) -> OSStatus;
    /// Raw binding for `AudioToolboxMusicplayerisplaying`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusicplayerisplaying`.
    pub fn MusicPlayerIsPlaying(in_player: MusicPlayerRef, out_is_playing: *mut u32) -> OSStatus;
    /// Raw binding for `AudioToolboxMusicplayersetplayratescalar`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusicplayersetplayratescalar`.
    pub fn MusicPlayerSetPlayRateScalar(in_player: MusicPlayerRef, in_scale_rate: f64) -> OSStatus;
    /// Raw binding for `AudioToolboxMusicplayergetplayratescalar`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioToolboxMusicplayergetplayratescalar`.
    pub fn MusicPlayerGetPlayRateScalar(
        in_player: MusicPlayerRef,
        out_scale_rate: *mut f64,
    ) -> OSStatus;
}
