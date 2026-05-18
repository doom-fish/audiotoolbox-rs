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

mod au_audio_unit;
mod au_graph;
mod audio_component;
mod audio_converter;
mod audio_file;
mod audio_file_component;
mod audio_file_stream;
mod audio_queue;
mod audio_services;
mod audio_unit;
mod avfaudio;
mod caf_file;
mod debug;
mod error;
mod ext_audio_file;
mod extended_types;
mod ffi;
mod format;
#[doc(hidden)]
pub mod generated_c_types;
mod internal;
mod music;
mod types;

#[cfg(feature = "raw-ffi")]
/// Bindings for the `AudioToolbox` portion of AudioToolbox.framework.
pub mod raw_ffi;

pub use apple_cf;
pub use au_audio_unit::AUAudioUnit;
pub use au_graph::AUGraph;
pub use audio_component::{AudioComponent, AudioComponentInstance, AudioComponentIter};
pub use audio_converter::{
    AudioConversionInput, AudioConversionOutput, AudioConverter, BorrowedAudioConverter,
};
pub use audio_file::{AudioFile, PacketData, PropertyInfo};
pub use audio_file_component::AudioFileComponent;
pub use audio_file_stream::AudioFileStream;
pub use audio_queue::{AudioQueue, AudioQueueBufferHandle};
pub use audio_services::SystemSound;
pub use audio_unit::AudioUnit;
pub use avfaudio::{AVAudioEngine, AVAudioFormat, AVAudioNode, AVAudioPCMBuffer, AVAudioSequencer};
pub use caf_file::CafFile;
pub use debug::{
    ca_show, ca_show_to_stderr, ca_show_to_stdout, flush_debug_output, AudioToolboxDebugObject,
};
pub use error::{AudioToolboxError, Result};
pub use ext_audio_file::{ExtAudioFile, InterleavedAudioBuffer};
pub use extended_types::*;
pub use format::{fourcc_to_string, is_printable_fourcc, AudioFormat};
pub use music::{MusicEventInfo, MusicEventIterator, MusicPlayer, MusicSequence, MusicTrack};
pub use types::*;
