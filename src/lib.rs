#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::copy_iterator,
    clippy::doc_markdown,
    clippy::elidable_lifetime_names,
    clippy::missing_const_for_fn,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::needless_pass_by_value,
    clippy::redundant_pub_crate,
    clippy::similar_names,
    clippy::struct_field_names,
    clippy::use_self
)]
#![doc = include_str!("../README.md")]

#[cfg(not(target_os = "macos"))]
compile_error!("audiotoolbox only supports macOS");

mod audio_component;
mod audio_converter;
mod audio_file;
mod audio_services;
mod cf;
mod debug;
mod error;
mod ext_audio_file;
mod ffi;
mod format;

pub use apple_cf;
pub use audio_component::{
    AudioComponent, AudioComponentInstance, AudioComponentIter, AUDIO_COMPONENT_MANUFACTURER_APPLE,
    AUDIO_COMPONENT_TYPE_EFFECT, AUDIO_COMPONENT_TYPE_FORMAT_CONVERTER,
    AUDIO_COMPONENT_TYPE_GENERATOR, AUDIO_COMPONENT_TYPE_MIDI_PROCESSOR,
    AUDIO_COMPONENT_TYPE_MIXER, AUDIO_COMPONENT_TYPE_MUSIC_DEVICE,
    AUDIO_COMPONENT_TYPE_MUSIC_EFFECT, AUDIO_COMPONENT_TYPE_OFFLINE_EFFECT,
    AUDIO_COMPONENT_TYPE_OUTPUT, AUDIO_COMPONENT_TYPE_PANNER,
};
pub use audio_converter::{
    AudioConversionInput, AudioConversionOutput, AudioConverter, BorrowedAudioConverter,
};
pub use audio_file::{AudioFile, PacketData, PropertyInfo};
pub use audio_services::{
    SystemSound, SYSTEM_SOUND_FLASH_SCREEN, SYSTEM_SOUND_USER_PREFERRED_ALERT,
};
pub use debug::{
    ca_show, ca_show_to_stderr, ca_show_to_stdout, flush_debug_output, AudioToolboxDebugObject,
};
pub use error::{AudioToolboxError, Result};
pub use ext_audio_file::{ExtAudioFile, InterleavedAudioBuffer};
pub use ffi::{
    fourcc, AudioBuffer, AudioBufferList, AudioBufferList1, AudioClassDescription,
    AudioComponentDescription, AudioConverterPropertyId, AudioFileFlags, AudioFilePermissions,
    AudioFilePropertyId, AudioFileTypeId, AudioFormatFlags, AudioFormatId,
    AudioStreamBasicDescription, AudioStreamPacketDescription, ExtAudioFilePropertyId, OSStatus,
    OSType, AUDIO_CONVERTER_PROPERTY_CURRENT_INPUT_STREAM_DESCRIPTION,
    AUDIO_CONVERTER_PROPERTY_CURRENT_OUTPUT_STREAM_DESCRIPTION,
    AUDIO_CONVERTER_PROPERTY_ENCODE_BIT_RATE, AUDIO_CONVERTER_PROPERTY_MAXIMUM_OUTPUT_PACKET_SIZE,
    AUDIO_FILE_AIFF_TYPE, AUDIO_FILE_CAF_TYPE, AUDIO_FILE_END_OF_FILE_ERROR,
    AUDIO_FILE_FLAGS_ERASE_FILE, AUDIO_FILE_M4A_TYPE, AUDIO_FILE_PROPERTY_AUDIO_DATA_PACKET_COUNT,
    AUDIO_FILE_PROPERTY_DATA_FORMAT, AUDIO_FILE_PROPERTY_ESTIMATED_DURATION,
    AUDIO_FILE_PROPERTY_MAGIC_COOKIE_DATA, AUDIO_FILE_PROPERTY_MAXIMUM_PACKET_SIZE,
    AUDIO_FILE_READ_PERMISSION, AUDIO_FILE_READ_WRITE_PERMISSION, AUDIO_FILE_WAVE_TYPE,
    AUDIO_FILE_WRITE_PERMISSION, AUDIO_FORMAT_APPLE_IMA4, AUDIO_FORMAT_FLAGS_NATIVE_ENDIAN,
    AUDIO_FORMAT_FLAG_IS_BIG_ENDIAN, AUDIO_FORMAT_FLAG_IS_FLOAT,
    AUDIO_FORMAT_FLAG_IS_NON_INTERLEAVED, AUDIO_FORMAT_FLAG_IS_PACKED,
    AUDIO_FORMAT_FLAG_IS_SIGNED_INTEGER, AUDIO_FORMAT_LINEAR_PCM, AUDIO_FORMAT_MPEG4_AAC,
    EXT_AUDIO_FILE_PROPERTY_AUDIO_CONVERTER, EXT_AUDIO_FILE_PROPERTY_AUDIO_FILE,
    EXT_AUDIO_FILE_PROPERTY_CLIENT_DATA_FORMAT, EXT_AUDIO_FILE_PROPERTY_FILE_DATA_FORMAT,
    EXT_AUDIO_FILE_PROPERTY_FILE_LENGTH_FRAMES, LINEAR_PCM_FORMAT_FLAG_IS_BIG_ENDIAN,
    LINEAR_PCM_FORMAT_FLAG_IS_FLOAT, LINEAR_PCM_FORMAT_FLAG_IS_NON_INTERLEAVED,
    LINEAR_PCM_FORMAT_FLAG_IS_PACKED, LINEAR_PCM_FORMAT_FLAG_IS_SIGNED_INTEGER, NO_ERR,
};
pub use format::{fourcc_to_string, is_printable_fourcc};
