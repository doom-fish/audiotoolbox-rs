use audiotoolbox::{
    AudioStreamBasicDescription, AudioUnit, AUDIO_COMPONENT_TYPE_FORMAT_CONVERTER,
    AUDIO_UNIT_SCOPE_INPUT, AUDIO_UNIT_SCOPE_OUTPUT, AUDIO_UNIT_SUBTYPE_AU_CONVERTER,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let unit = AudioUnit::new_apple(
        AUDIO_COMPONENT_TYPE_FORMAT_CONVERTER,
        AUDIO_UNIT_SUBTYPE_AU_CONVERTER,
    )?;
    let input = AudioStreamBasicDescription::linear_pcm_i16(44_100.0, 1, true);
    let output = AudioStreamBasicDescription::linear_pcm_f32(44_100.0, 1, true);

    unit.set_stream_format(AUDIO_UNIT_SCOPE_INPUT, 0, &input)?;
    unit.set_stream_format(AUDIO_UNIT_SCOPE_OUTPUT, 0, &output)?;
    unit.initialize()?;
    println!("input_bits={} output_bits={}",
        unit.stream_format(AUDIO_UNIT_SCOPE_INPUT, 0)?.mBitsPerChannel,
        unit.stream_format(AUDIO_UNIT_SCOPE_OUTPUT, 0)?.mBitsPerChannel,
    );
    unit.uninitialize()?;
    Ok(())
}
