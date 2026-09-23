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

mod support;

#[test]
fn audio_unit_renders_into_an_owned_buffer_list() -> Result<()> {
    let (unit, output) = support::ramp_converter_unit()?;
    let time_stamp = support::sample_time_stamp(0.0);
    let mut flags = 0;

    let mut list = audiotoolbox::OwnedAudioBufferList::for_format(&output, 256)?;
    unit.render(&mut flags, &time_stamp, 0, 256, &mut list)?;
    let data = list.data(0).expect("rendered buffer");
    assert_eq!(data.len(), 1024);
    for (index, chunk) in data.chunks_exact(4).enumerate() {
        let sample = f32::from_ne_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        let expected = f32::from(i16::try_from(index).unwrap() * support::RAMP_STEP) / 32_768.0;
        assert!(
            (sample - expected).abs() < 1e-6,
            "frame {index} was {sample}"
        );
    }

    let mut two_buffers = audiotoolbox::OwnedAudioBufferList::new(2, 1, 4096)?;
    assert!(unit
        .render(&mut flags, &time_stamp, 0, 256, &mut two_buffers)
        .is_err());
    let mut too_small = audiotoolbox::OwnedAudioBufferList::for_format(&output, 128)?;
    assert!(unit
        .render(&mut flags, &time_stamp, 0, 256, &mut too_small)
        .is_err());
    unit.uninitialize()?;
    Ok(())
}

#[test]
fn typed_property_reads_check_the_returned_size() -> Result<()> {
    let unit = AudioUnit::new_apple(
        AUDIO_COMPONENT_TYPE_FORMAT_CONVERTER,
        AUDIO_UNIT_SUBTYPE_AU_CONVERTER,
    )?;
    assert!(unit
        .get_property_typed::<u32>(
            AUDIO_UNIT_PROPERTY_STREAM_FORMAT,
            AUDIO_UNIT_SCOPE_INPUT,
            0,
            "AudioUnitGetProperty(stream format as u32)",
        )
        .is_err());
    assert!(unit
        .get_property_typed::<[AudioStreamBasicDescription; 2]>(
            AUDIO_UNIT_PROPERTY_STREAM_FORMAT,
            AUDIO_UNIT_SCOPE_INPUT,
            0,
            "AudioUnitGetProperty(stream format as two)",
        )
        .is_err());
    let format = unit.get_property_typed::<AudioStreamBasicDescription>(
        AUDIO_UNIT_PROPERTY_STREAM_FORMAT,
        AUDIO_UNIT_SCOPE_INPUT,
        0,
        "AudioUnitGetProperty(stream format)",
    )?;
    assert!(format.mSampleRate > 0.0);
    Ok(())
}
