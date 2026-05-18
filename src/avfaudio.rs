use crate::{
    ffi, internal::error_from_owned_ptr, AVAudioChannelCount, AVAudioCommonFormat,
    AVAudioFrameCount, AVAudioNodeBus, AudioStreamBasicDescription, AudioToolboxError, Result,
};
use std::{ffi::c_void, mem::MaybeUninit};

#[derive(Debug)]
/// Wraps `AVAudioEngine`.
pub struct AVAudioEngine {
    handle: *mut c_void,
}

#[derive(Debug)]
/// Wraps `AVAudioNode`.
pub struct AVAudioNode {
    handle: *mut c_void,
}

#[derive(Debug)]
/// Wraps `AVAudioFormat`.
pub struct AVAudioFormat {
    handle: *mut c_void,
}

#[derive(Debug)]
/// Wraps `AVAudioPCMBuffer`.
pub struct AVAudioPCMBuffer {
    handle: *mut c_void,
}

#[derive(Debug)]
/// Wraps `AVAudioSequencer`.
pub struct AVAudioSequencer {
    handle: *mut c_void,
}

impl AVAudioEngine {
    /// Wraps `AVAudioEngineInit`.
    ///
    /// The returned wrapper owns the underlying AudioToolbox.framework handle and releases it on drop.
    pub fn new() -> Result<Self> {
        let mut handle = std::ptr::null_mut();
        if unsafe { ffi::avfaudio::at_av_audio_engine_new(&mut handle) } {
            Self::from_handle(handle, "AVAudioEngineInit")
        } else {
            Err(AudioToolboxError::message(
                "AVAudioEngineInit",
                "framework returned a null AVAudioEngine",
            ))
        }
    }

    /// Wraps `AVAudioEnginePrepare`.
    pub fn prepare(&self) {
        unsafe { ffi::avfaudio::at_av_audio_engine_prepare(self.handle) };
    }

    /// Wraps `AVAudioEngineStart`.
    pub fn start(&self) -> Result<()> {
        let mut error = std::ptr::null_mut();
        if unsafe { ffi::avfaudio::at_av_audio_engine_start(self.handle, &mut error) } {
            Ok(())
        } else {
            Err(error_from_owned_ptr("AVAudioEngineStart", error))
        }
    }

    /// Wraps `AVAudioEngineStop`.
    pub fn stop(&self) {
        unsafe { ffi::avfaudio::at_av_audio_engine_stop(self.handle) };
    }

    /// Wraps `AVAudioEngineReset`.
    pub fn reset(&self) {
        unsafe { ffi::avfaudio::at_av_audio_engine_reset(self.handle) };
    }

    /// Wraps `AVAudioEngineIsRunning`.
    pub fn is_running(&self) -> bool {
        unsafe { ffi::avfaudio::at_av_audio_engine_is_running(self.handle) }
    }

    /// Wraps `AVAudioEngineOutputNode`.
    pub fn output_node(&self) -> Result<AVAudioNode> {
        let mut handle = std::ptr::null_mut();
        let ok = unsafe { ffi::avfaudio::at_av_audio_engine_output_node(self.handle, &mut handle) };
        if ok {
            AVAudioNode::from_handle(handle, "AVAudioEngineOutputNode")
        } else {
            Err(AudioToolboxError::message(
                "AVAudioEngineOutputNode",
                "framework returned a null AVAudioNode",
            ))
        }
    }

    /// Wraps `AVAudioEngineMainMixerNode`.
    pub fn main_mixer_node(&self) -> Result<AVAudioNode> {
        let mut handle = std::ptr::null_mut();
        let ok =
            unsafe { ffi::avfaudio::at_av_audio_engine_main_mixer_node(self.handle, &mut handle) };
        if ok {
            AVAudioNode::from_handle(handle, "AVAudioEngineMainMixerNode")
        } else {
            Err(AudioToolboxError::message(
                "AVAudioEngineMainMixerNode",
                "framework returned a null AVAudioNode",
            ))
        }
    }

    fn from_handle(handle: *mut c_void, operation: &'static str) -> Result<Self> {
        if handle.is_null() {
            Err(AudioToolboxError::message(
                operation,
                "framework returned a null AVAudioEngine",
            ))
        } else {
            Ok(Self { handle })
        }
    }

    fn release(&mut self) {
        if !self.handle.is_null() {
            unsafe { ffi::avfaudio::at_av_audio_engine_release(self.handle) };
            self.handle = std::ptr::null_mut();
        }
    }
}

impl AVAudioNode {
    /// Wraps `AVAudioNodeNumberOfInputs`.
    pub fn number_of_inputs(&self) -> u64 {
        unsafe { ffi::avfaudio::at_av_audio_node_number_of_inputs(self.handle) }
    }

    /// Wraps `AVAudioNodeNumberOfOutputs`.
    pub fn number_of_outputs(&self) -> u64 {
        unsafe { ffi::avfaudio::at_av_audio_node_number_of_outputs(self.handle) }
    }

    /// Wraps `AVAudioNodeInputFormatForBus`.
    pub fn input_format(&self, bus: AVAudioNodeBus) -> Result<AVAudioFormat> {
        let mut handle = std::ptr::null_mut();
        let ok =
            unsafe { ffi::avfaudio::at_av_audio_node_input_format(self.handle, bus, &mut handle) };
        if ok {
            AVAudioFormat::from_handle(handle, "AVAudioNodeInputFormatForBus")
        } else {
            Err(AudioToolboxError::message(
                "AVAudioNodeInputFormatForBus",
                "framework returned a null AVAudioFormat",
            ))
        }
    }

    /// Wraps `AVAudioNodeOutputFormatForBus`.
    pub fn output_format(&self, bus: AVAudioNodeBus) -> Result<AVAudioFormat> {
        let mut handle = std::ptr::null_mut();
        let ok =
            unsafe { ffi::avfaudio::at_av_audio_node_output_format(self.handle, bus, &mut handle) };
        if ok {
            AVAudioFormat::from_handle(handle, "AVAudioNodeOutputFormatForBus")
        } else {
            Err(AudioToolboxError::message(
                "AVAudioNodeOutputFormatForBus",
                "framework returned a null AVAudioFormat",
            ))
        }
    }

    /// Wraps `AVAudioNodeReset`.
    pub fn reset(&self) {
        unsafe { ffi::avfaudio::at_av_audio_node_reset(self.handle) };
    }

    /// Wraps `AVAudioNodeLatency`.
    pub fn latency(&self) -> f64 {
        unsafe { ffi::avfaudio::at_av_audio_node_latency(self.handle) }
    }

    /// Wraps `AVAudioNodeOutputPresentationLatency`.
    pub fn output_presentation_latency(&self) -> f64 {
        unsafe { ffi::avfaudio::at_av_audio_node_output_presentation_latency(self.handle) }
    }

    fn from_handle(handle: *mut c_void, operation: &'static str) -> Result<Self> {
        if handle.is_null() {
            Err(AudioToolboxError::message(
                operation,
                "framework returned a null AVAudioNode",
            ))
        } else {
            Ok(Self { handle })
        }
    }

    fn release(&mut self) {
        if !self.handle.is_null() {
            unsafe { ffi::avfaudio::at_av_audio_node_release(self.handle) };
            self.handle = std::ptr::null_mut();
        }
    }
}

impl AVAudioFormat {
    /// Wraps `AVAudioFormatInitStandard`.
    pub fn standard(sample_rate: f64, channels: AVAudioChannelCount) -> Result<Self> {
        let mut handle = std::ptr::null_mut();
        let ok = unsafe {
            ffi::avfaudio::at_av_audio_format_new_standard(sample_rate, channels, &mut handle)
        };
        if ok {
            Self::from_handle(handle, "AVAudioFormatInitStandard")
        } else {
            Err(AudioToolboxError::message(
                "AVAudioFormatInitStandard",
                "framework failed to create an AVAudioFormat",
            ))
        }
    }

    /// Wraps `AVAudioFormatInitCommonFormat`.
    pub fn with_common_format(
        common_format: AVAudioCommonFormat,
        sample_rate: f64,
        channels: AVAudioChannelCount,
        interleaved: bool,
    ) -> Result<Self> {
        let mut handle = std::ptr::null_mut();
        let ok = unsafe {
            ffi::avfaudio::at_av_audio_format_new_common(
                common_format,
                sample_rate,
                channels,
                interleaved,
                &mut handle,
            )
        };
        if ok {
            Self::from_handle(handle, "AVAudioFormatInitCommonFormat")
        } else {
            Err(AudioToolboxError::message(
                "AVAudioFormatInitCommonFormat",
                "framework failed to create an AVAudioFormat",
            ))
        }
    }

    /// Wraps `AVAudioFormatInitStreamDescription`.
    pub fn from_stream_description(description: &AudioStreamBasicDescription) -> Result<Self> {
        let mut handle = std::ptr::null_mut();
        let ok = unsafe {
            ffi::avfaudio::at_av_audio_format_new_stream_description(
                std::ptr::from_ref(description),
                &mut handle,
            )
        };
        if ok {
            Self::from_handle(handle, "AVAudioFormatInitStreamDescription")
        } else {
            Err(AudioToolboxError::message(
                "AVAudioFormatInitStreamDescription",
                "framework failed to create an AVAudioFormat",
            ))
        }
    }

    /// Wraps `AVAudioFormatSampleRate`.
    pub fn sample_rate(&self) -> f64 {
        unsafe { ffi::avfaudio::at_av_audio_format_sample_rate(self.handle) }
    }

    /// Wraps `AVAudioFormatChannelCount`.
    pub fn channel_count(&self) -> AVAudioChannelCount {
        unsafe { ffi::avfaudio::at_av_audio_format_channel_count(self.handle) }
    }

    /// Wraps `AVAudioFormatCommonFormat`.
    pub fn common_format(&self) -> AVAudioCommonFormat {
        unsafe { ffi::avfaudio::at_av_audio_format_common_format(self.handle) }
    }

    /// Wraps `AVAudioFormatIsStandard`.
    pub fn is_standard(&self) -> bool {
        unsafe { ffi::avfaudio::at_av_audio_format_is_standard(self.handle) }
    }

    /// Wraps `AVAudioFormatIsInterleaved`.
    pub fn is_interleaved(&self) -> bool {
        unsafe { ffi::avfaudio::at_av_audio_format_is_interleaved(self.handle) }
    }

    /// Wraps `AVAudioFormatStreamDescription`.
    pub fn stream_description(&self) -> Result<AudioStreamBasicDescription> {
        let mut description = MaybeUninit::<AudioStreamBasicDescription>::uninit();
        let ok = unsafe {
            ffi::avfaudio::at_av_audio_format_copy_stream_description(
                self.handle,
                description.as_mut_ptr(),
            )
        };
        if ok {
            Ok(unsafe { description.assume_init() })
        } else {
            Err(AudioToolboxError::message(
                "AVAudioFormatStreamDescription",
                "framework returned a null AudioStreamBasicDescription",
            ))
        }
    }

    fn from_handle(handle: *mut c_void, operation: &'static str) -> Result<Self> {
        if handle.is_null() {
            Err(AudioToolboxError::message(
                operation,
                "framework returned a null AVAudioFormat",
            ))
        } else {
            Ok(Self { handle })
        }
    }

    fn release(&mut self) {
        if !self.handle.is_null() {
            unsafe { ffi::avfaudio::at_av_audio_format_release(self.handle) };
            self.handle = std::ptr::null_mut();
        }
    }
}

impl AVAudioPCMBuffer {
    /// Wraps `AVAudioPCMBufferInit`.
    ///
    /// The returned wrapper owns the underlying AudioToolbox.framework handle and releases it on drop.
    pub fn new(format: &AVAudioFormat, frame_capacity: AVAudioFrameCount) -> Result<Self> {
        let mut handle = std::ptr::null_mut();
        let ok = unsafe {
            ffi::avfaudio::at_av_audio_pcm_buffer_new(format.handle, frame_capacity, &mut handle)
        };
        if ok {
            Self::from_handle(handle, "AVAudioPCMBufferInit")
        } else {
            Err(AudioToolboxError::message(
                "AVAudioPCMBufferInit",
                "framework failed to create an AVAudioPCMBuffer",
            ))
        }
    }

    /// Wraps `AVAudioPCMBufferFormat`.
    pub fn format(&self) -> Result<AVAudioFormat> {
        let mut handle = std::ptr::null_mut();
        let ok = unsafe { ffi::avfaudio::at_av_audio_pcm_buffer_format(self.handle, &mut handle) };
        if ok {
            AVAudioFormat::from_handle(handle, "AVAudioPCMBufferFormat")
        } else {
            Err(AudioToolboxError::message(
                "AVAudioPCMBufferFormat",
                "framework returned a null AVAudioFormat",
            ))
        }
    }

    /// Wraps `AVAudioPCMBufferFrameCapacity`.
    pub fn frame_capacity(&self) -> AVAudioFrameCount {
        unsafe { ffi::avfaudio::at_av_audio_pcm_buffer_frame_capacity(self.handle) }
    }

    /// Wraps `AVAudioPCMBufferFrameLength`.
    pub fn frame_length(&self) -> AVAudioFrameCount {
        unsafe { ffi::avfaudio::at_av_audio_pcm_buffer_frame_length(self.handle) }
    }

    /// Wraps `AVAudioPCMBufferSetFrameLength`.
    pub fn set_frame_length(&self, frame_length: AVAudioFrameCount) {
        unsafe {
            ffi::avfaudio::at_av_audio_pcm_buffer_set_frame_length(self.handle, frame_length);
        }
    }

    /// Wraps `AVAudioPCMBufferStride`.
    pub fn stride(&self) -> AVAudioChannelCount {
        unsafe { ffi::avfaudio::at_av_audio_pcm_buffer_stride(self.handle) }
    }

    fn from_handle(handle: *mut c_void, operation: &'static str) -> Result<Self> {
        if handle.is_null() {
            Err(AudioToolboxError::message(
                operation,
                "framework returned a null AVAudioPCMBuffer",
            ))
        } else {
            Ok(Self { handle })
        }
    }

    fn release(&mut self) {
        if !self.handle.is_null() {
            unsafe { ffi::avfaudio::at_av_audio_pcm_buffer_release(self.handle) };
            self.handle = std::ptr::null_mut();
        }
    }
}

impl AVAudioSequencer {
    /// Wraps `AVAudioSequencerInit`.
    ///
    /// The returned wrapper owns the underlying AudioToolbox.framework handle and releases it on drop.
    pub fn new() -> Result<Self> {
        let mut handle = std::ptr::null_mut();
        if unsafe { ffi::avfaudio::at_av_audio_sequencer_new(&mut handle) } {
            Self::from_handle(handle, "AVAudioSequencerInit")
        } else {
            Err(AudioToolboxError::message(
                "AVAudioSequencerInit",
                "framework failed to create an AVAudioSequencer",
            ))
        }
    }

    /// Wraps `AVAudioSequencerInitWithAudioEngine`.
    pub fn with_engine(engine: &AVAudioEngine) -> Result<Self> {
        let mut handle = std::ptr::null_mut();
        let ok = unsafe {
            ffi::avfaudio::at_av_audio_sequencer_new_with_engine(engine.handle, &mut handle)
        };
        if ok {
            Self::from_handle(handle, "AVAudioSequencerInitWithAudioEngine")
        } else {
            Err(AudioToolboxError::message(
                "AVAudioSequencerInitWithAudioEngine",
                "framework failed to create an AVAudioSequencer",
            ))
        }
    }

    /// Wraps `AVAudioSequencerTrackCount`.
    pub fn track_count(&self) -> u64 {
        unsafe { ffi::avfaudio::at_av_audio_sequencer_track_count(self.handle) }
    }

    /// Wraps `AVAudioSequencerPrepareToPlay`.
    pub fn prepare_to_play(&self) {
        unsafe { ffi::avfaudio::at_av_audio_sequencer_prepare_to_play(self.handle) };
    }

    /// Wraps `AVAudioSequencerStart`.
    pub fn start(&self) -> Result<()> {
        let mut error = std::ptr::null_mut();
        if unsafe { ffi::avfaudio::at_av_audio_sequencer_start(self.handle, &mut error) } {
            Ok(())
        } else {
            Err(error_from_owned_ptr("AVAudioSequencerStart", error))
        }
    }

    /// Wraps `AVAudioSequencerStop`.
    pub fn stop(&self) {
        unsafe { ffi::avfaudio::at_av_audio_sequencer_stop(self.handle) };
    }

    /// Wraps `AVAudioSequencerIsPlaying`.
    pub fn is_playing(&self) -> bool {
        unsafe { ffi::avfaudio::at_av_audio_sequencer_is_playing(self.handle) }
    }

    /// Wraps `AVAudioSequencerCurrentPositionSeconds`.
    pub fn current_position_seconds(&self) -> f64 {
        unsafe { ffi::avfaudio::at_av_audio_sequencer_current_position_seconds(self.handle) }
    }

    /// Wraps `AVAudioSequencerSetCurrentPositionSeconds`.
    pub fn set_current_position_seconds(&self, position: f64) {
        unsafe {
            ffi::avfaudio::at_av_audio_sequencer_set_current_position_seconds(
                self.handle,
                position,
            );
        }
    }

    /// Wraps `AVAudioSequencerRate`.
    pub fn rate(&self) -> f32 {
        unsafe { ffi::avfaudio::at_av_audio_sequencer_rate(self.handle) }
    }

    /// Wraps `AVAudioSequencerSetRate`.
    pub fn set_rate(&self, rate: f32) {
        unsafe { ffi::avfaudio::at_av_audio_sequencer_set_rate(self.handle, rate) };
    }

    fn from_handle(handle: *mut c_void, operation: &'static str) -> Result<Self> {
        if handle.is_null() {
            Err(AudioToolboxError::message(
                operation,
                "framework returned a null AVAudioSequencer",
            ))
        } else {
            Ok(Self { handle })
        }
    }

    fn release(&mut self) {
        if !self.handle.is_null() {
            unsafe { ffi::avfaudio::at_av_audio_sequencer_release(self.handle) };
            self.handle = std::ptr::null_mut();
        }
    }
}

impl Drop for AVAudioEngine {
    fn drop(&mut self) {
        self.release();
    }
}

impl Drop for AVAudioNode {
    fn drop(&mut self) {
        self.release();
    }
}

impl Drop for AVAudioFormat {
    fn drop(&mut self) {
        self.release();
    }
}

impl Drop for AVAudioPCMBuffer {
    fn drop(&mut self) {
        self.release();
    }
}

impl Drop for AVAudioSequencer {
    fn drop(&mut self) {
        self.release();
    }
}
