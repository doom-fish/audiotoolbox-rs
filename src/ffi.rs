#![allow(non_snake_case)]

use std::ffi::c_void;

pub type OSStatus = i32;
pub type OSType = u32;
pub type AudioFileTypeId = u32;
pub type AudioFilePropertyId = u32;
pub type AudioFileFlags = u32;
pub type AudioFilePermissions = i8;
pub type AudioFormatId = u32;
pub type AudioFormatFlags = u32;
pub type ExtAudioFilePropertyId = u32;
pub type AudioConverterPropertyId = u32;
pub type SystemSoundId = u32;
pub type Boolean = u8;
pub type CFURLRef = *const c_void;
pub type CFStringRef = *const c_void;
pub type AudioFileId = *mut c_void;
pub type ExtAudioFileRef = *mut c_void;
pub type AudioConverterRef = *mut c_void;
pub type AudioComponentRef = *mut c_void;
pub type AudioComponentInstanceRef = *mut c_void;

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

pub const AUDIO_FILE_PROPERTY_DATA_FORMAT: AudioFilePropertyId = fourcc(*b"dfmt");
pub const AUDIO_FILE_PROPERTY_MAGIC_COOKIE_DATA: AudioFilePropertyId = fourcc(*b"mgic");
pub const AUDIO_FILE_PROPERTY_AUDIO_DATA_PACKET_COUNT: AudioFilePropertyId = fourcc(*b"pcnt");
pub const AUDIO_FILE_PROPERTY_MAXIMUM_PACKET_SIZE: AudioFilePropertyId = fourcc(*b"psze");
pub const AUDIO_FILE_PROPERTY_ESTIMATED_DURATION: AudioFilePropertyId = fourcc(*b"edur");

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

unsafe extern "C" {
    pub fn AudioFileCreateWithURL(
        in_file_ref: CFURLRef,
        in_file_type: AudioFileTypeId,
        in_format: *const AudioStreamBasicDescription,
        in_flags: AudioFileFlags,
        out_audio_file: *mut AudioFileId,
    ) -> OSStatus;
    pub fn AudioFileOpenURL(
        in_file_ref: CFURLRef,
        in_permissions: AudioFilePermissions,
        in_file_type_hint: AudioFileTypeId,
        out_audio_file: *mut AudioFileId,
    ) -> OSStatus;
    pub fn AudioFileClose(in_audio_file: AudioFileId) -> OSStatus;
    pub fn AudioFileReadPacketData(
        in_audio_file: AudioFileId,
        in_use_cache: Boolean,
        io_num_bytes: *mut u32,
        out_packet_descriptions: *mut AudioStreamPacketDescription,
        in_starting_packet: i64,
        io_num_packets: *mut u32,
        out_buffer: *mut c_void,
    ) -> OSStatus;
    pub fn AudioFileReadPackets(
        in_audio_file: AudioFileId,
        in_use_cache: Boolean,
        out_num_bytes: *mut u32,
        out_packet_descriptions: *mut AudioStreamPacketDescription,
        in_starting_packet: i64,
        io_num_packets: *mut u32,
        out_buffer: *mut c_void,
    ) -> OSStatus;
    pub fn AudioFileWritePackets(
        in_audio_file: AudioFileId,
        in_use_cache: Boolean,
        in_num_bytes: u32,
        in_packet_descriptions: *const AudioStreamPacketDescription,
        in_starting_packet: i64,
        io_num_packets: *mut u32,
        in_buffer: *const c_void,
    ) -> OSStatus;
    pub fn AudioFileGetPropertyInfo(
        in_audio_file: AudioFileId,
        in_property_id: AudioFilePropertyId,
        out_data_size: *mut u32,
        is_writable: *mut u32,
    ) -> OSStatus;
    pub fn AudioFileGetProperty(
        in_audio_file: AudioFileId,
        in_property_id: AudioFilePropertyId,
        io_data_size: *mut u32,
        out_property_data: *mut c_void,
    ) -> OSStatus;
    pub fn AudioFileSetProperty(
        in_audio_file: AudioFileId,
        in_property_id: AudioFilePropertyId,
        in_data_size: u32,
        in_property_data: *const c_void,
    ) -> OSStatus;

    pub fn ExtAudioFileOpenURL(
        in_url: CFURLRef,
        out_ext_audio_file: *mut ExtAudioFileRef,
    ) -> OSStatus;
    pub fn ExtAudioFileCreateWithURL(
        in_url: CFURLRef,
        in_file_type: AudioFileTypeId,
        in_stream_desc: *const AudioStreamBasicDescription,
        in_channel_layout: *const c_void,
        in_flags: u32,
        out_ext_audio_file: *mut ExtAudioFileRef,
    ) -> OSStatus;
    pub fn ExtAudioFileDispose(in_ext_audio_file: ExtAudioFileRef) -> OSStatus;
    pub fn ExtAudioFileRead(
        in_ext_audio_file: ExtAudioFileRef,
        io_number_frames: *mut u32,
        io_data: *mut AudioBufferList1,
    ) -> OSStatus;
    pub fn ExtAudioFileWrite(
        in_ext_audio_file: ExtAudioFileRef,
        in_number_frames: u32,
        io_data: *const AudioBufferList1,
    ) -> OSStatus;
    pub fn ExtAudioFileGetProperty(
        in_ext_audio_file: ExtAudioFileRef,
        in_property_id: ExtAudioFilePropertyId,
        io_property_data_size: *mut u32,
        out_property_data: *mut c_void,
    ) -> OSStatus;
    pub fn ExtAudioFileSetProperty(
        in_ext_audio_file: ExtAudioFileRef,
        in_property_id: ExtAudioFilePropertyId,
        in_property_data_size: u32,
        in_property_data: *const c_void,
    ) -> OSStatus;

    pub fn AudioConverterNew(
        in_source_format: *const AudioStreamBasicDescription,
        in_destination_format: *const AudioStreamBasicDescription,
        out_audio_converter: *mut AudioConverterRef,
    ) -> OSStatus;
    pub fn AudioConverterNewSpecific(
        in_source_format: *const AudioStreamBasicDescription,
        in_destination_format: *const AudioStreamBasicDescription,
        in_number_class_descriptions: u32,
        in_class_descriptions: *const AudioClassDescription,
        out_audio_converter: *mut AudioConverterRef,
    ) -> OSStatus;
    pub fn AudioConverterDispose(in_audio_converter: AudioConverterRef) -> OSStatus;
    pub fn AudioConverterReset(in_audio_converter: AudioConverterRef) -> OSStatus;
    pub fn AudioConverterGetPropertyInfo(
        in_audio_converter: AudioConverterRef,
        in_property_id: AudioConverterPropertyId,
        out_size: *mut u32,
        out_writable: *mut Boolean,
    ) -> OSStatus;
    pub fn AudioConverterGetProperty(
        in_audio_converter: AudioConverterRef,
        in_property_id: AudioConverterPropertyId,
        io_property_data_size: *mut u32,
        out_property_data: *mut c_void,
    ) -> OSStatus;
    pub fn AudioConverterSetProperty(
        in_audio_converter: AudioConverterRef,
        in_property_id: AudioConverterPropertyId,
        in_property_data_size: u32,
        in_property_data: *const c_void,
    ) -> OSStatus;
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

    pub fn AudioComponentFindNext(
        in_component: AudioComponentRef,
        in_desc: *const AudioComponentDescription,
    ) -> AudioComponentRef;
    pub fn AudioComponentCount(in_desc: *const AudioComponentDescription) -> u32;
    pub fn AudioComponentCopyName(
        in_component: AudioComponentRef,
        out_name: *mut CFStringRef,
    ) -> OSStatus;
    pub fn AudioComponentGetDescription(
        in_component: AudioComponentRef,
        out_desc: *mut AudioComponentDescription,
    ) -> OSStatus;
    pub fn AudioComponentGetVersion(
        in_component: AudioComponentRef,
        out_version: *mut u32,
    ) -> OSStatus;
    pub fn AudioComponentInstanceNew(
        in_component: AudioComponentRef,
        out_instance: *mut AudioComponentInstanceRef,
    ) -> OSStatus;
    pub fn AudioComponentInstanceDispose(in_instance: AudioComponentInstanceRef) -> OSStatus;
    pub fn AudioComponentInstanceGetComponent(
        in_instance: AudioComponentInstanceRef,
    ) -> AudioComponentRef;

    pub fn AudioServicesCreateSystemSoundID(
        in_file_url: CFURLRef,
        out_system_sound_id: *mut SystemSoundId,
    ) -> OSStatus;
    pub fn AudioServicesDisposeSystemSoundID(in_system_sound_id: SystemSoundId) -> OSStatus;
    pub fn AudioServicesPlaySystemSound(in_system_sound_id: SystemSoundId);

    pub fn CAShow(in_object: *mut c_void);
    pub fn CAShowFile(in_object: *mut c_void, in_file: *mut libc::FILE);
}
