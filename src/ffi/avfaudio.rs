use crate::{
    AVAudioChannelCount, AVAudioCommonFormat, AVAudioFrameCount, AVAudioNodeBus,
    AudioStreamBasicDescription,
};
use std::ffi::{c_char, c_void};

unsafe extern "C" {
    /// Raw binding for `AVAudioEngineNew`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AVAudioEngineNew`.
    pub fn at_av_audio_engine_new(out_handle: *mut *mut c_void) -> bool;
    /// Raw binding for `AVAudioEngineRelease`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AVAudioEngineRelease`.
    pub fn at_av_audio_engine_release(handle: *mut c_void);
    /// Raw binding for `AVAudioEnginePrepare`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AVAudioEnginePrepare`.
    pub fn at_av_audio_engine_prepare(handle: *mut c_void);
    /// Raw binding for `AVAudioEngineStart`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AVAudioEngineStart`.
    pub fn at_av_audio_engine_start(handle: *mut c_void, out_error: *mut *mut c_char) -> bool;
    /// Raw binding for `AVAudioEngineStop`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AVAudioEngineStop`.
    pub fn at_av_audio_engine_stop(handle: *mut c_void);
    /// Raw binding for `AVAudioEngineReset`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AVAudioEngineReset`.
    pub fn at_av_audio_engine_reset(handle: *mut c_void);
    /// Raw binding for `AVAudioEngineIsRunning`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AVAudioEngineIsRunning`.
    pub fn at_av_audio_engine_is_running(handle: *mut c_void) -> bool;
    /// Raw binding for `AVAudioEngineOutputNode`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AVAudioEngineOutputNode`.
    pub fn at_av_audio_engine_output_node(
        handle: *mut c_void,
        out_handle: *mut *mut c_void,
    ) -> bool;
    /// Raw binding for `AVAudioEngineMainMixerNode`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AVAudioEngineMainMixerNode`.
    pub fn at_av_audio_engine_main_mixer_node(
        handle: *mut c_void,
        out_handle: *mut *mut c_void,
    ) -> bool;

    /// Raw binding for `AVAudioNodeRelease`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AVAudioNodeRelease`.
    pub fn at_av_audio_node_release(handle: *mut c_void);
    /// Raw binding for `AVAudioNodeNumberOfInputs`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AVAudioNodeNumberOfInputs`.
    pub fn at_av_audio_node_number_of_inputs(handle: *mut c_void) -> u64;
    /// Raw binding for `AVAudioNodeNumberOfOutputs`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AVAudioNodeNumberOfOutputs`.
    pub fn at_av_audio_node_number_of_outputs(handle: *mut c_void) -> u64;
    /// Raw binding for `AVAudioNodeInputFormat`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AVAudioNodeInputFormat`.
    pub fn at_av_audio_node_input_format(
        handle: *mut c_void,
        bus: AVAudioNodeBus,
        out_handle: *mut *mut c_void,
    ) -> bool;
    /// Raw binding for `AVAudioNodeOutputFormat`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AVAudioNodeOutputFormat`.
    pub fn at_av_audio_node_output_format(
        handle: *mut c_void,
        bus: AVAudioNodeBus,
        out_handle: *mut *mut c_void,
    ) -> bool;
    /// Raw binding for `AVAudioNodeReset`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AVAudioNodeReset`.
    pub fn at_av_audio_node_reset(handle: *mut c_void);
    /// Raw binding for `AVAudioNodeLatency`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AVAudioNodeLatency`.
    pub fn at_av_audio_node_latency(handle: *mut c_void) -> f64;
    /// Raw binding for `AVAudioNodeOutputPresentationLatency`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AVAudioNodeOutputPresentationLatency`.
    pub fn at_av_audio_node_output_presentation_latency(handle: *mut c_void) -> f64;

    /// Raw binding for `AVAudioFormatNewStandard`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AVAudioFormatNewStandard`.
    pub fn at_av_audio_format_new_standard(
        sample_rate: f64,
        channels: AVAudioChannelCount,
        out_handle: *mut *mut c_void,
    ) -> bool;
    /// Raw binding for `AVAudioFormatNewCommon`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AVAudioFormatNewCommon`.
    pub fn at_av_audio_format_new_common(
        common_format: AVAudioCommonFormat,
        sample_rate: f64,
        channels: AVAudioChannelCount,
        interleaved: bool,
        out_handle: *mut *mut c_void,
    ) -> bool;
    /// Raw binding for `AVAudioFormatNewStreamDescription`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AVAudioFormatNewStreamDescription`.
    pub fn at_av_audio_format_new_stream_description(
        description: *const AudioStreamBasicDescription,
        out_handle: *mut *mut c_void,
    ) -> bool;
    /// Raw binding for `AVAudioFormatRelease`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AVAudioFormatRelease`.
    pub fn at_av_audio_format_release(handle: *mut c_void);
    /// Raw binding for `AVAudioFormatSampleRate`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AVAudioFormatSampleRate`.
    pub fn at_av_audio_format_sample_rate(handle: *mut c_void) -> f64;
    /// Raw binding for `AVAudioFormatChannelCount`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AVAudioFormatChannelCount`.
    pub fn at_av_audio_format_channel_count(handle: *mut c_void) -> AVAudioChannelCount;
    /// Raw binding for `AVAudioFormatCommonFormat`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AVAudioFormatCommonFormat`.
    pub fn at_av_audio_format_common_format(handle: *mut c_void) -> AVAudioCommonFormat;
    /// Raw binding for `AVAudioFormatIsStandard`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AVAudioFormatIsStandard`.
    pub fn at_av_audio_format_is_standard(handle: *mut c_void) -> bool;
    /// Raw binding for `AVAudioFormatIsInterleaved`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AVAudioFormatIsInterleaved`.
    pub fn at_av_audio_format_is_interleaved(handle: *mut c_void) -> bool;
    /// Raw binding for `AVAudioFormatCopyStreamDescription`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AVAudioFormatCopyStreamDescription`.
    pub fn at_av_audio_format_copy_stream_description(
        handle: *mut c_void,
        out_description: *mut AudioStreamBasicDescription,
    ) -> bool;

    /// Raw binding for `AVAudioPCMBufferNew`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AVAudioPCMBufferNew`.
    pub fn at_av_audio_pcm_buffer_new(
        format_handle: *mut c_void,
        frame_capacity: AVAudioFrameCount,
        out_handle: *mut *mut c_void,
    ) -> bool;
    /// Raw binding for `AVAudioPCMBufferRelease`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AVAudioPCMBufferRelease`.
    pub fn at_av_audio_pcm_buffer_release(handle: *mut c_void);
    /// Raw binding for `AVAudioPCMBufferFormat`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AVAudioPCMBufferFormat`.
    pub fn at_av_audio_pcm_buffer_format(handle: *mut c_void, out_handle: *mut *mut c_void)
        -> bool;
    /// Raw binding for `AVAudioPCMBufferFrameCapacity`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AVAudioPCMBufferFrameCapacity`.
    pub fn at_av_audio_pcm_buffer_frame_capacity(handle: *mut c_void) -> AVAudioFrameCount;
    /// Raw binding for `AVAudioPCMBufferFrameLength`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AVAudioPCMBufferFrameLength`.
    pub fn at_av_audio_pcm_buffer_frame_length(handle: *mut c_void) -> AVAudioFrameCount;
    /// Raw binding for `AVAudioPCMBufferSetFrameLength`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AVAudioPCMBufferSetFrameLength`.
    pub fn at_av_audio_pcm_buffer_set_frame_length(
        handle: *mut c_void,
        frame_length: AVAudioFrameCount,
    );
    /// Raw binding for `AVAudioPCMBufferStride`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AVAudioPCMBufferStride`.
    pub fn at_av_audio_pcm_buffer_stride(handle: *mut c_void) -> AVAudioChannelCount;

    /// Raw binding for `AVAudioSequencerNew`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AVAudioSequencerNew`.
    pub fn at_av_audio_sequencer_new(out_handle: *mut *mut c_void) -> bool;
    /// Raw binding for `AVAudioSequencerNewWithEngine`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AVAudioSequencerNewWithEngine`.
    pub fn at_av_audio_sequencer_new_with_engine(
        engine_handle: *mut c_void,
        out_handle: *mut *mut c_void,
    ) -> bool;
    /// Raw binding for `AVAudioSequencerRelease`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AVAudioSequencerRelease`.
    pub fn at_av_audio_sequencer_release(handle: *mut c_void);
    /// Raw binding for `AVAudioSequencerTrackCount`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AVAudioSequencerTrackCount`.
    pub fn at_av_audio_sequencer_track_count(handle: *mut c_void) -> u64;
    /// Raw binding for `AVAudioSequencerPrepareToPlay`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AVAudioSequencerPrepareToPlay`.
    pub fn at_av_audio_sequencer_prepare_to_play(handle: *mut c_void);
    /// Raw binding for `AVAudioSequencerStart`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AVAudioSequencerStart`.
    pub fn at_av_audio_sequencer_start(handle: *mut c_void, out_error: *mut *mut c_char) -> bool;
    /// Raw binding for `AVAudioSequencerStop`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AVAudioSequencerStop`.
    pub fn at_av_audio_sequencer_stop(handle: *mut c_void);
    /// Raw binding for `AVAudioSequencerIsPlaying`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AVAudioSequencerIsPlaying`.
    pub fn at_av_audio_sequencer_is_playing(handle: *mut c_void) -> bool;
    /// Raw binding for `AVAudioSequencerCurrentPositionSeconds`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AVAudioSequencerCurrentPositionSeconds`.
    pub fn at_av_audio_sequencer_current_position_seconds(handle: *mut c_void) -> f64;
    /// Raw binding for `AVAudioSequencerSetCurrentPositionSeconds`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AVAudioSequencerSetCurrentPositionSeconds`.
    pub fn at_av_audio_sequencer_set_current_position_seconds(handle: *mut c_void, position: f64);
    /// Raw binding for `AVAudioSequencerRate`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AVAudioSequencerRate`.
    pub fn at_av_audio_sequencer_rate(handle: *mut c_void) -> f32;
    /// Raw binding for `AVAudioSequencerSetRate`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AVAudioSequencerSetRate`.
    pub fn at_av_audio_sequencer_set_rate(handle: *mut c_void, rate: f32);
}
