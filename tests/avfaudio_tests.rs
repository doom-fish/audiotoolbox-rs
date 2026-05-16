use audiotoolbox::{
    AVAudioEngine, AVAudioFormat, Result, AV_AUDIO_PCM_FORMAT_INT16,
};

#[test]
fn avfaudio_format_and_node_smoke_test() -> Result<()> {
    let standard = AVAudioFormat::standard(44_100.0, 2)?;
    let standard_stream = standard.stream_description()?;
    let interleaved = AVAudioFormat::with_common_format(
        AV_AUDIO_PCM_FORMAT_INT16,
        48_000.0,
        1,
        true,
    )?;

    assert!(standard.is_standard());
    assert_eq!(standard.channel_count(), 2);
    assert_eq!(standard_stream.mChannelsPerFrame, 2);
    assert_eq!(interleaved.common_format(), AV_AUDIO_PCM_FORMAT_INT16);
    assert!(interleaved.is_interleaved());
    assert_eq!(interleaved.stream_description()?.mBitsPerChannel, 16);

    let engine = AVAudioEngine::new()?;
    let output = engine.output_node()?;
    let output_format = output.input_format(0)?;

    assert!(output.number_of_inputs() >= 1);
    assert!(output_format.sample_rate() > 0.0);
    assert!(output_format.channel_count() >= 1);
    Ok(())
}
