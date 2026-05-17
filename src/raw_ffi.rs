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

pub type AUGraphRef = *mut c_void;
pub type AudioFormatPropertyId = u32;
pub type AudioUnitPropertyId = u32;
pub type AudioUnitScope = u32;
pub type AudioUnitElement = u32;
pub type AudioUnitParameterId = u32;
pub type AudioUnitParameterValue = f32;
pub type AudioUnitRenderActionFlags = u32;
pub type AudioUnitRef = *mut c_void;
pub type MIDIEndpointRef = u32;
pub type MusicSequenceType = u32;
pub type MusicSequenceLoadFlags = u32;
pub type MusicSequenceFileTypeId = u32;
pub type MusicSequenceFileFlags = u32;
pub type MusicTimeStamp = f64;
pub type MusicSequenceRef = *mut c_void;
pub type MusicTrackRef = *mut c_void;
pub type MusicPlayerRef = *mut c_void;
pub type MusicEventIteratorRef = *mut c_void;
pub type MusicEventType = u32;
pub type CFDataRef = *const c_void;
pub type CFDictionaryRef = *const c_void;
pub type MusicTrackPropertyId = u32;
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
    pub fn AudioFormatGetPropertyInfo(
        in_property_id: AudioFormatPropertyId,
        in_specifier_size: u32,
        in_specifier: *const c_void,
        out_property_data_size: *mut u32,
    ) -> OSStatus;
    pub fn AudioFormatGetProperty(
        in_property_id: AudioFormatPropertyId,
        in_specifier_size: u32,
        in_specifier: *const c_void,
        io_property_data_size: *mut u32,
        out_property_data: *mut c_void,
    ) -> OSStatus;

    pub fn AudioFileOptimize(in_audio_file: AudioFileId) -> OSStatus;
    pub fn AudioFileReadBytes(
        in_audio_file: AudioFileId,
        in_use_cache: Boolean,
        in_starting_byte: i64,
        io_num_bytes: *mut u32,
        out_buffer: *mut c_void,
    ) -> OSStatus;
    pub fn AudioFileWriteBytes(
        in_audio_file: AudioFileId,
        in_use_cache: Boolean,
        in_starting_byte: i64,
        io_num_bytes: *mut u32,
        in_buffer: *const c_void,
    ) -> OSStatus;
    pub fn AudioFileCountUserData(
        in_audio_file: AudioFileId,
        in_user_data_id: u32,
        out_number_items: *mut u32,
    ) -> OSStatus;
    pub fn AudioFileGetUserDataSize(
        in_audio_file: AudioFileId,
        in_user_data_id: u32,
        in_index: u32,
        out_user_data_size: *mut u32,
    ) -> OSStatus;
    pub fn AudioFileGetUserDataSize64(
        in_audio_file: AudioFileId,
        in_user_data_id: u32,
        in_index: u32,
        out_user_data_size: *mut u64,
    ) -> OSStatus;
    pub fn AudioFileGetUserData(
        in_audio_file: AudioFileId,
        in_user_data_id: u32,
        in_index: u32,
        io_user_data_size: *mut u32,
        out_user_data: *mut c_void,
    ) -> OSStatus;
    pub fn AudioFileGetUserDataAtOffset(
        in_audio_file: AudioFileId,
        in_user_data_id: u32,
        in_index: u32,
        in_offset: i64,
        io_user_data_size: *mut u32,
        out_user_data: *mut c_void,
    ) -> OSStatus;
    pub fn AudioFileSetUserData(
        in_audio_file: AudioFileId,
        in_user_data_id: u32,
        in_index: u32,
        in_user_data_size: u32,
        in_user_data: *const c_void,
    ) -> OSStatus;
    pub fn AudioFileRemoveUserData(
        in_audio_file: AudioFileId,
        in_user_data_id: u32,
        in_index: u32,
    ) -> OSStatus;
    pub fn AudioFileGetGlobalInfoSize(
        in_property_id: AudioFilePropertyId,
        in_specifier_size: u32,
        in_specifier: *const c_void,
        out_property_data_size: *mut u32,
    ) -> OSStatus;
    pub fn AudioFileGetGlobalInfo(
        in_property_id: AudioFilePropertyId,
        in_specifier_size: u32,
        in_specifier: *const c_void,
        io_property_data_size: *mut u32,
        out_property_data: *mut c_void,
    ) -> OSStatus;

    pub fn AudioConverterConvertBuffer(
        in_audio_converter: AudioConverterRef,
        in_input_data_size: u32,
        in_input_data: *const c_void,
        io_output_data_size: *mut u32,
        out_output_data: *mut c_void,
    ) -> OSStatus;
    pub fn AudioConverterConvertComplexBuffer(
        in_audio_converter: AudioConverterRef,
        in_number_pcm_frames: u32,
        in_input_data: *const AudioBufferList1,
        out_output_data: *mut AudioBufferList1,
    ) -> OSStatus;

    pub fn AudioComponentInstanceCanDo(
        in_instance: AudioComponentInstanceRef,
        in_selector_id: i16,
    ) -> Boolean;
    pub fn AudioComponentCopyConfigurationInfo(
        in_component: AudioComponentRef,
        out_configuration_info: *mut CFDictionaryRef,
    ) -> OSStatus;
    pub fn AudioComponentValidate(
        in_component: AudioComponentRef,
        in_validation_parameters: CFDictionaryRef,
        out_validation_result: *mut u32,
    ) -> OSStatus;

    pub fn AudioUnitGetPropertyInfo(
        in_unit: AudioUnitRef,
        in_id: AudioUnitPropertyId,
        in_scope: AudioUnitScope,
        in_element: AudioUnitElement,
        out_data_size: *mut u32,
        out_writable: *mut Boolean,
    ) -> OSStatus;
    pub fn AudioUnitGetProperty(
        in_unit: AudioUnitRef,
        in_id: AudioUnitPropertyId,
        in_scope: AudioUnitScope,
        in_element: AudioUnitElement,
        io_data_size: *mut u32,
        out_data: *mut c_void,
    ) -> OSStatus;
    pub fn AudioUnitSetProperty(
        in_unit: AudioUnitRef,
        in_id: AudioUnitPropertyId,
        in_scope: AudioUnitScope,
        in_element: AudioUnitElement,
        in_data_size: u32,
        in_data: *const c_void,
    ) -> OSStatus;
    pub fn AudioUnitAddPropertyListener(
        in_unit: AudioUnitRef,
        in_id: AudioUnitPropertyId,
        in_proc: AudioUnitPropertyListenerProc,
        in_proc_user_data: *mut c_void,
    ) -> OSStatus;
    pub fn AudioUnitRemovePropertyListenerWithUserData(
        in_unit: AudioUnitRef,
        in_id: AudioUnitPropertyId,
        in_proc: AudioUnitPropertyListenerProc,
        in_proc_user_data: *mut c_void,
    ) -> OSStatus;
    pub fn AudioUnitAddRenderNotify(
        in_unit: AudioUnitRef,
        in_proc: AURenderCallback,
        in_proc_user_data: *mut c_void,
    ) -> OSStatus;
    pub fn AudioUnitRemoveRenderNotify(
        in_unit: AudioUnitRef,
        in_proc: AURenderCallback,
        in_proc_user_data: *mut c_void,
    ) -> OSStatus;
    pub fn AudioUnitGetParameter(
        in_unit: AudioUnitRef,
        in_id: AudioUnitParameterId,
        in_scope: AudioUnitScope,
        in_element: AudioUnitElement,
        out_value: *mut AudioUnitParameterValue,
    ) -> OSStatus;
    pub fn AudioUnitSetParameter(
        in_unit: AudioUnitRef,
        in_id: AudioUnitParameterId,
        in_scope: AudioUnitScope,
        in_element: AudioUnitElement,
        in_value: AudioUnitParameterValue,
        in_buffer_offset_in_frames: u32,
    ) -> OSStatus;
    pub fn AudioUnitScheduleParameters(
        in_unit: AudioUnitRef,
        in_num_param_events: u32,
        in_param_events: *const AudioUnitParameterEvent,
    ) -> OSStatus;
    pub fn AudioUnitRender(
        in_unit: AudioUnitRef,
        io_action_flags: *mut AudioUnitRenderActionFlags,
        in_time_stamp: *const AudioTimeStamp,
        in_output_bus_number: u32,
        in_number_frames: u32,
        io_data: *mut AudioBufferList1,
    ) -> OSStatus;
    pub fn AudioUnitInitialize(in_unit: AudioUnitRef) -> OSStatus;
    pub fn AudioUnitUninitialize(in_unit: AudioUnitRef) -> OSStatus;
    pub fn AudioOutputUnitStart(ci: AudioUnitRef) -> OSStatus;
    pub fn AudioOutputUnitStop(ci: AudioUnitRef) -> OSStatus;

    pub fn NewAUGraph(out_graph: *mut AUGraphRef) -> OSStatus;
    pub fn DisposeAUGraph(in_graph: AUGraphRef) -> OSStatus;
    pub fn AUGraphOpen(in_graph: AUGraphRef) -> OSStatus;
    pub fn AUGraphClose(in_graph: AUGraphRef) -> OSStatus;
    pub fn AUGraphInitialize(in_graph: AUGraphRef) -> OSStatus;
    pub fn AUGraphUninitialize(in_graph: AUGraphRef) -> OSStatus;
    pub fn AUGraphStart(in_graph: AUGraphRef) -> OSStatus;
    pub fn AUGraphStop(in_graph: AUGraphRef) -> OSStatus;
    pub fn AUGraphIsOpen(in_graph: AUGraphRef, out_is_open: *mut Boolean) -> OSStatus;
    pub fn AUGraphIsInitialized(in_graph: AUGraphRef, out_is_initialized: *mut Boolean)
        -> OSStatus;
    pub fn AUGraphIsRunning(in_graph: AUGraphRef, out_is_running: *mut Boolean) -> OSStatus;
    pub fn AUGraphGetNodeCount(in_graph: AUGraphRef, out_number_of_nodes: *mut u32) -> OSStatus;
    pub fn AUGraphAddNode(
        in_graph: AUGraphRef,
        in_description: *const AudioComponentDescription,
        out_node: *mut AUNode,
    ) -> OSStatus;
    pub fn AUGraphConnectNodeInput(
        in_graph: AUGraphRef,
        in_source_node: AUNode,
        in_source_output_number: u32,
        in_dest_node: AUNode,
        in_dest_input_number: u32,
    ) -> OSStatus;
    pub fn AUGraphSetNodeInputCallback(
        in_graph: AUGraphRef,
        in_dest_node: AUNode,
        in_dest_input_number: u32,
        in_input_callback: *const AURenderCallbackStruct,
    ) -> OSStatus;
    pub fn AUGraphDisconnectNodeInput(
        in_graph: AUGraphRef,
        in_dest_node: AUNode,
        in_dest_input_number: u32,
    ) -> OSStatus;
    pub fn AUGraphClearConnections(in_graph: AUGraphRef) -> OSStatus;
    pub fn AUGraphGetNumberOfInteractions(
        in_graph: AUGraphRef,
        out_num_interactions: *mut u32,
    ) -> OSStatus;
    pub fn AUGraphGetInteractionInfo(
        in_graph: AUGraphRef,
        in_interaction_index: u32,
        out_interaction: *mut AUNodeInteraction,
    ) -> OSStatus;
    pub fn AUGraphCountNodeInteractions(
        in_graph: AUGraphRef,
        in_node: AUNode,
        out_num_interactions: *mut u32,
    ) -> OSStatus;
    pub fn AUGraphGetNodeInteractions(
        in_graph: AUGraphRef,
        in_node: AUNode,
        io_num_interactions: *mut u32,
        out_interactions: *mut AUNodeInteraction,
    ) -> OSStatus;
    pub fn AUGraphUpdate(in_graph: AUGraphRef, out_is_updated: *mut Boolean) -> OSStatus;
    pub fn AUGraphGetCPULoad(in_graph: AUGraphRef, out_average_cpu_load: *mut f32) -> OSStatus;
    pub fn AUGraphGetMaxCPULoad(in_graph: AUGraphRef, out_max_cpu_load: *mut f32) -> OSStatus;
    pub fn AUGraphAddRenderNotify(
        in_graph: AUGraphRef,
        in_callback: AURenderCallback,
        in_ref_con: *mut c_void,
    ) -> OSStatus;
    pub fn AUGraphRemoveRenderNotify(
        in_graph: AUGraphRef,
        in_callback: AURenderCallback,
        in_ref_con: *mut c_void,
    ) -> OSStatus;
    pub fn AUGraphNodeInfo(
        in_graph: AUGraphRef,
        in_node: AUNode,
        out_description: *mut AudioComponentDescription,
        out_audio_unit: *mut AudioUnitRef,
    ) -> OSStatus;

    pub fn NewMusicSequence(out_sequence: *mut MusicSequenceRef) -> OSStatus;
    pub fn DisposeMusicSequence(in_sequence: MusicSequenceRef) -> OSStatus;
    pub fn MusicSequenceNewTrack(
        in_sequence: MusicSequenceRef,
        out_track: *mut MusicTrackRef,
    ) -> OSStatus;
    pub fn MusicSequenceDisposeTrack(
        in_sequence: MusicSequenceRef,
        in_track: MusicTrackRef,
    ) -> OSStatus;
    pub fn MusicSequenceGetTrackCount(
        in_sequence: MusicSequenceRef,
        out_number_of_tracks: *mut u32,
    ) -> OSStatus;
    pub fn MusicSequenceGetIndTrack(
        in_sequence: MusicSequenceRef,
        in_track_index: u32,
        out_track: *mut MusicTrackRef,
    ) -> OSStatus;
    pub fn MusicSequenceGetTrackIndex(
        in_sequence: MusicSequenceRef,
        in_track: MusicTrackRef,
        out_track_index: *mut u32,
    ) -> OSStatus;
    pub fn MusicSequenceGetTempoTrack(
        in_sequence: MusicSequenceRef,
        out_tempo_track: *mut MusicTrackRef,
    ) -> OSStatus;
    pub fn MusicSequenceSetAUGraph(in_sequence: MusicSequenceRef, in_graph: AUGraphRef)
        -> OSStatus;
    pub fn MusicSequenceGetAUGraph(
        in_sequence: MusicSequenceRef,
        out_graph: *mut AUGraphRef,
    ) -> OSStatus;
    pub fn MusicSequenceSetMIDIEndpoint(
        in_sequence: MusicSequenceRef,
        in_endpoint: MIDIEndpointRef,
    ) -> OSStatus;
    pub fn MusicSequenceSetSequenceType(
        in_sequence: MusicSequenceRef,
        in_sequence_type: MusicSequenceType,
    ) -> OSStatus;
    pub fn MusicSequenceGetSequenceType(
        in_sequence: MusicSequenceRef,
        out_sequence_type: *mut MusicSequenceType,
    ) -> OSStatus;
    pub fn MusicSequenceFileLoad(
        in_sequence: MusicSequenceRef,
        in_file_ref: CFURLRef,
        in_file_type_hint: MusicSequenceFileTypeId,
        in_flags: MusicSequenceLoadFlags,
    ) -> OSStatus;
    pub fn MusicSequenceFileLoadData(
        in_sequence: MusicSequenceRef,
        in_data: CFDataRef,
        in_file_type_hint: MusicSequenceFileTypeId,
        in_flags: MusicSequenceLoadFlags,
    ) -> OSStatus;
    pub fn MusicSequenceFileCreate(
        in_sequence: MusicSequenceRef,
        in_file_ref: CFURLRef,
        in_file_type: MusicSequenceFileTypeId,
        in_flags: MusicSequenceFileFlags,
        in_resolution: i16,
    ) -> OSStatus;
    pub fn MusicSequenceFileCreateData(
        in_sequence: MusicSequenceRef,
        in_file_type: MusicSequenceFileTypeId,
        in_flags: MusicSequenceFileFlags,
        in_resolution: i16,
        out_data: *mut CFDataRef,
    ) -> OSStatus;
    pub fn MusicSequenceGetSecondsForBeats(
        in_sequence: MusicSequenceRef,
        in_beats: MusicTimeStamp,
        out_seconds: *mut f64,
    ) -> OSStatus;
    pub fn MusicSequenceGetBeatsForSeconds(
        in_sequence: MusicSequenceRef,
        in_seconds: f64,
        out_beats: *mut MusicTimeStamp,
    ) -> OSStatus;
    pub fn MusicSequenceBeatsToBarBeatTime(
        in_sequence: MusicSequenceRef,
        in_beats: MusicTimeStamp,
        in_subbeat_divisor: u32,
        out_bar_beat_time: *mut CABarBeatTime,
    ) -> OSStatus;
    pub fn MusicSequenceBarBeatTimeToBeats(
        in_sequence: MusicSequenceRef,
        in_bar_beat_time: *const CABarBeatTime,
        out_beats: *mut MusicTimeStamp,
    ) -> OSStatus;
    pub fn MusicSequenceGetInfoDictionary(in_sequence: MusicSequenceRef) -> CFDictionaryRef;
    pub fn MusicTrackGetSequence(
        in_track: MusicTrackRef,
        out_sequence: *mut MusicSequenceRef,
    ) -> OSStatus;
    pub fn MusicTrackSetDestNode(in_track: MusicTrackRef, in_node: AUNode) -> OSStatus;
    pub fn MusicTrackSetDestMIDIEndpoint(
        in_track: MusicTrackRef,
        in_endpoint: MIDIEndpointRef,
    ) -> OSStatus;
    pub fn MusicTrackGetDestNode(in_track: MusicTrackRef, out_node: *mut AUNode) -> OSStatus;
    pub fn MusicTrackGetDestMIDIEndpoint(
        in_track: MusicTrackRef,
        out_endpoint: *mut MIDIEndpointRef,
    ) -> OSStatus;
    pub fn MusicTrackSetProperty(
        in_track: MusicTrackRef,
        in_property_id: MusicTrackPropertyId,
        in_data: *const c_void,
        in_length: u32,
    ) -> OSStatus;
    pub fn MusicTrackGetProperty(
        in_track: MusicTrackRef,
        in_property_id: MusicTrackPropertyId,
        out_data: *mut c_void,
        io_length: *mut u32,
    ) -> OSStatus;
    pub fn MusicTrackMoveEvents(
        in_track: MusicTrackRef,
        in_start_time: MusicTimeStamp,
        in_end_time: MusicTimeStamp,
        in_move_time: MusicTimeStamp,
    ) -> OSStatus;
    pub fn MusicTrackClear(
        in_track: MusicTrackRef,
        in_start_time: MusicTimeStamp,
        in_end_time: MusicTimeStamp,
    ) -> OSStatus;
    pub fn MusicTrackCut(
        in_track: MusicTrackRef,
        in_start_time: MusicTimeStamp,
        in_end_time: MusicTimeStamp,
    ) -> OSStatus;
    pub fn MusicTrackCopyInsert(
        in_source_track: MusicTrackRef,
        in_source_start_time: MusicTimeStamp,
        in_source_end_time: MusicTimeStamp,
        in_dest_track: MusicTrackRef,
        in_dest_insert_time: MusicTimeStamp,
    ) -> OSStatus;
    pub fn MusicTrackMerge(
        in_source_track: MusicTrackRef,
        in_source_start_time: MusicTimeStamp,
        in_source_end_time: MusicTimeStamp,
        in_dest_track: MusicTrackRef,
        in_dest_insert_time: MusicTimeStamp,
    ) -> OSStatus;
    pub fn MusicTrackNewMIDINoteEvent(
        in_track: MusicTrackRef,
        in_time_stamp: MusicTimeStamp,
        in_note_message: *const MIDINoteMessage,
    ) -> OSStatus;
    pub fn MusicTrackNewMIDIChannelEvent(
        in_track: MusicTrackRef,
        in_time_stamp: MusicTimeStamp,
        in_message: *const MIDIChannelMessage,
    ) -> OSStatus;
    pub fn MusicTrackNewMIDIRawDataEvent(
        in_track: MusicTrackRef,
        in_time_stamp: MusicTimeStamp,
        in_raw_data: *const MIDIRawData,
    ) -> OSStatus;
    pub fn MusicTrackNewExtendedNoteEvent(
        in_track: MusicTrackRef,
        in_time_stamp: MusicTimeStamp,
        in_note_event: *const ExtendedNoteOnEvent,
    ) -> OSStatus;
    pub fn MusicTrackNewParameterEvent(
        in_track: MusicTrackRef,
        in_time_stamp: MusicTimeStamp,
        in_event: *const ParameterEvent,
    ) -> OSStatus;
    pub fn MusicTrackNewExtendedTempoEvent(
        in_track: MusicTrackRef,
        in_time_stamp: MusicTimeStamp,
        in_bpm: f64,
    ) -> OSStatus;
    pub fn MusicTrackNewMetaEvent(
        in_track: MusicTrackRef,
        in_time_stamp: MusicTimeStamp,
        in_meta_event: *const MIDIMetaEvent,
    ) -> OSStatus;
    pub fn MusicTrackNewUserEvent(
        in_track: MusicTrackRef,
        in_time_stamp: MusicTimeStamp,
        in_user_data: *const MusicEventUserData,
    ) -> OSStatus;
    pub fn MusicTrackNewAUPresetEvent(
        in_track: MusicTrackRef,
        in_time_stamp: MusicTimeStamp,
        in_preset_event: *const AUPresetEvent,
    ) -> OSStatus;
    pub fn NewMusicEventIterator(
        in_track: MusicTrackRef,
        out_iterator: *mut MusicEventIteratorRef,
    ) -> OSStatus;
    pub fn DisposeMusicEventIterator(in_iterator: MusicEventIteratorRef) -> OSStatus;
    pub fn MusicEventIteratorSeek(
        in_iterator: MusicEventIteratorRef,
        in_time_stamp: MusicTimeStamp,
    ) -> OSStatus;
    pub fn MusicEventIteratorNextEvent(in_iterator: MusicEventIteratorRef) -> OSStatus;
    pub fn MusicEventIteratorPreviousEvent(in_iterator: MusicEventIteratorRef) -> OSStatus;
    pub fn MusicEventIteratorGetEventInfo(
        in_iterator: MusicEventIteratorRef,
        out_time_stamp: *mut MusicTimeStamp,
        out_event_type: *mut MusicEventType,
        out_event_data: *mut *const c_void,
        out_event_data_size: *mut u32,
    ) -> OSStatus;
    pub fn MusicEventIteratorSetEventInfo(
        in_iterator: MusicEventIteratorRef,
        in_event_type: MusicEventType,
        in_event_data: *const c_void,
    ) -> OSStatus;
    pub fn MusicEventIteratorSetEventTime(
        in_iterator: MusicEventIteratorRef,
        in_time_stamp: MusicTimeStamp,
    ) -> OSStatus;
    pub fn MusicEventIteratorDeleteEvent(in_iterator: MusicEventIteratorRef) -> OSStatus;
    pub fn MusicEventIteratorHasPreviousEvent(
        in_iterator: MusicEventIteratorRef,
        out_has_previous_event: *mut Boolean,
    ) -> OSStatus;
    pub fn MusicEventIteratorHasNextEvent(
        in_iterator: MusicEventIteratorRef,
        out_has_next_event: *mut Boolean,
    ) -> OSStatus;
    pub fn MusicEventIteratorHasCurrentEvent(
        in_iterator: MusicEventIteratorRef,
        out_has_current_event: *mut Boolean,
    ) -> OSStatus;
    pub fn NewMusicPlayer(out_player: *mut MusicPlayerRef) -> OSStatus;
    pub fn DisposeMusicPlayer(in_player: MusicPlayerRef) -> OSStatus;
    pub fn MusicPlayerSetSequence(
        in_player: MusicPlayerRef,
        in_sequence: MusicSequenceRef,
    ) -> OSStatus;
    pub fn MusicPlayerGetSequence(
        in_player: MusicPlayerRef,
        out_sequence: *mut MusicSequenceRef,
    ) -> OSStatus;
    pub fn MusicPlayerSetTime(in_player: MusicPlayerRef, in_time: MusicTimeStamp) -> OSStatus;
    pub fn MusicPlayerGetTime(in_player: MusicPlayerRef, out_time: *mut MusicTimeStamp)
        -> OSStatus;
    pub fn MusicPlayerGetHostTimeForBeats(
        in_player: MusicPlayerRef,
        in_beats: MusicTimeStamp,
        out_host_time: *mut u64,
    ) -> OSStatus;
    pub fn MusicPlayerGetBeatsForHostTime(
        in_player: MusicPlayerRef,
        in_host_time: u64,
        out_beats: *mut MusicTimeStamp,
    ) -> OSStatus;
    pub fn MusicPlayerPreroll(in_player: MusicPlayerRef) -> OSStatus;
    pub fn MusicPlayerStart(in_player: MusicPlayerRef) -> OSStatus;
    pub fn MusicPlayerStop(in_player: MusicPlayerRef) -> OSStatus;
    pub fn MusicPlayerIsPlaying(in_player: MusicPlayerRef, out_is_playing: *mut u32) -> OSStatus;
    pub fn MusicPlayerSetPlayRateScalar(in_player: MusicPlayerRef, in_scale_rate: f64) -> OSStatus;
    pub fn MusicPlayerGetPlayRateScalar(
        in_player: MusicPlayerRef,
        out_scale_rate: *mut f64,
    ) -> OSStatus;
}
