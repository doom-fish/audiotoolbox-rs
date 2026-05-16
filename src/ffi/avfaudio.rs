use crate::{
    AudioStreamBasicDescription, AVAudioChannelCount, AVAudioCommonFormat, AVAudioNodeBus,
};
use std::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn at_av_audio_engine_new(out_handle: *mut *mut c_void) -> bool;
    pub fn at_av_audio_engine_release(handle: *mut c_void);
    pub fn at_av_audio_engine_prepare(handle: *mut c_void);
    pub fn at_av_audio_engine_start(handle: *mut c_void, out_error: *mut *mut c_char) -> bool;
    pub fn at_av_audio_engine_stop(handle: *mut c_void);
    pub fn at_av_audio_engine_reset(handle: *mut c_void);
    pub fn at_av_audio_engine_is_running(handle: *mut c_void) -> bool;
    pub fn at_av_audio_engine_output_node(handle: *mut c_void, out_handle: *mut *mut c_void)
        -> bool;
    pub fn at_av_audio_engine_main_mixer_node(
        handle: *mut c_void,
        out_handle: *mut *mut c_void,
    ) -> bool;

    pub fn at_av_audio_node_release(handle: *mut c_void);
    pub fn at_av_audio_node_number_of_inputs(handle: *mut c_void) -> u64;
    pub fn at_av_audio_node_number_of_outputs(handle: *mut c_void) -> u64;
    pub fn at_av_audio_node_input_format(
        handle: *mut c_void,
        bus: AVAudioNodeBus,
        out_handle: *mut *mut c_void,
    ) -> bool;
    pub fn at_av_audio_node_output_format(
        handle: *mut c_void,
        bus: AVAudioNodeBus,
        out_handle: *mut *mut c_void,
    ) -> bool;
    pub fn at_av_audio_node_reset(handle: *mut c_void);
    pub fn at_av_audio_node_latency(handle: *mut c_void) -> f64;
    pub fn at_av_audio_node_output_presentation_latency(handle: *mut c_void) -> f64;

    pub fn at_av_audio_format_new_standard(
        sample_rate: f64,
        channels: AVAudioChannelCount,
        out_handle: *mut *mut c_void,
    ) -> bool;
    pub fn at_av_audio_format_new_common(
        common_format: AVAudioCommonFormat,
        sample_rate: f64,
        channels: AVAudioChannelCount,
        interleaved: bool,
        out_handle: *mut *mut c_void,
    ) -> bool;
    pub fn at_av_audio_format_new_stream_description(
        description: *const AudioStreamBasicDescription,
        out_handle: *mut *mut c_void,
    ) -> bool;
    pub fn at_av_audio_format_release(handle: *mut c_void);
    pub fn at_av_audio_format_sample_rate(handle: *mut c_void) -> f64;
    pub fn at_av_audio_format_channel_count(handle: *mut c_void) -> AVAudioChannelCount;
    pub fn at_av_audio_format_common_format(handle: *mut c_void) -> AVAudioCommonFormat;
    pub fn at_av_audio_format_is_standard(handle: *mut c_void) -> bool;
    pub fn at_av_audio_format_is_interleaved(handle: *mut c_void) -> bool;
    pub fn at_av_audio_format_copy_stream_description(
        handle: *mut c_void,
        out_description: *mut AudioStreamBasicDescription,
    ) -> bool;
}
