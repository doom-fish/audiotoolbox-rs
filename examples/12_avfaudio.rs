use audiotoolbox::{AVAudioEngine, AVAudioFormat, AV_AUDIO_PCM_FORMAT_INT16};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let format = AVAudioFormat::with_common_format(AV_AUDIO_PCM_FORMAT_INT16, 48_000.0, 2, true)?;
    let engine = AVAudioEngine::new()?;
    let output = engine.output_node()?;
    let hardware = output.input_format(0)?;

    println!(
        "format_channels={} hardware_sample_rate={:.1} engine_running={}",
        format.channel_count(),
        hardware.sample_rate(),
        engine.is_running(),
    );
    Ok(())
}
