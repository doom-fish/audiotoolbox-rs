use audiotoolbox::{
    AudioStreamBasicDescription, AudioUnit, Result, AUDIO_COMPONENT_TYPE_FORMAT_CONVERTER,
    AUDIO_UNIT_PROPERTY_STREAM_FORMAT, AUDIO_UNIT_SCOPE_INPUT, AUDIO_UNIT_SCOPE_OUTPUT,
    AUDIO_UNIT_SUBTYPE_AU_CONVERTER,
};

#[test]
fn audio_unit_sets_stream_formats() -> Result<()> {
    let unit = AudioUnit::new_apple(
        AUDIO_COMPONENT_TYPE_FORMAT_CONVERTER,
        AUDIO_UNIT_SUBTYPE_AU_CONVERTER,
    )?;
    let input = AudioStreamBasicDescription::linear_pcm_i16(44_100.0, 1, true);
    let output = AudioStreamBasicDescription::linear_pcm_f32(44_100.0, 1, true);

    unit.set_stream_format(AUDIO_UNIT_SCOPE_INPUT, 0, &input)?;
    unit.set_stream_format(AUDIO_UNIT_SCOPE_OUTPUT, 0, &output)?;
    let (input_stream_size, input_stream_writable) =
        unit.property_info(AUDIO_UNIT_PROPERTY_STREAM_FORMAT, AUDIO_UNIT_SCOPE_INPUT, 0)?;

    assert_eq!(
        unit.stream_format(AUDIO_UNIT_SCOPE_INPUT, 0)?
            .mBitsPerChannel,
        16
    );
    assert_eq!(
        unit.stream_format(AUDIO_UNIT_SCOPE_OUTPUT, 0)?
            .mBitsPerChannel,
        32
    );
    assert!((unit.sample_rate(AUDIO_UNIT_SCOPE_INPUT, 0)? - 44_100.0).abs() < f64::EPSILON);
    assert_eq!(
        input_stream_size as usize,
        std::mem::size_of::<AudioStreamBasicDescription>()
    );
    assert!(input_stream_writable);
    assert!(unit.element_count(AUDIO_UNIT_SCOPE_INPUT)? >= 1);
    assert!(unit.latency()? >= 0.0);
    assert_eq!(unit.last_render_error()?, 0);
    unit.initialize()?;
    unit.uninitialize()?;
    Ok(())
}
