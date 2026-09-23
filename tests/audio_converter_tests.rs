use audiotoolbox::{
    AudioConversionInput, AudioConverter, AudioStreamBasicDescription, OwnedAudioBufferList, Result,
};

const fn input(data: &[u8], packet_count: u32, channels: u32) -> AudioConversionInput<'_> {
    AudioConversionInput {
        data,
        packet_count,
        packet_descriptions: None,
        channels,
    }
}

fn f32_samples(bytes: &[u8]) -> Vec<f32> {
    bytes
        .chunks_exact(4)
        .map(|chunk| f32::from_ne_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
        .collect()
}

fn i16_bytes(samples: impl IntoIterator<Item = i16>) -> Vec<u8> {
    samples.into_iter().flat_map(i16::to_ne_bytes).collect()
}

fn f32_bytes(samples: impl IntoIterator<Item = f32>) -> Vec<u8> {
    samples.into_iter().flat_map(f32::to_ne_bytes).collect()
}

#[test]
fn audio_converter_converts_pcm() -> Result<()> {
    let source = AudioStreamBasicDescription::linear_pcm_i16(44_100.0, 1, true);
    let destination = AudioStreamBasicDescription::linear_pcm_f32(44_100.0, 1, true);
    let mut converter = AudioConverter::new(&source, &destination)?;
    let input_bytes = [0_u8, 0, 255, 127, 0, 128, 0, 0];
    let output = converter.fill_complex_buffer(input(&input_bytes, 4, 1), 4)?;
    let converted_buffer = converter.convert_buffer(&input_bytes, 16)?;

    assert_eq!(converter.calculate_input_buffer_size(16)?, 8);
    assert_eq!(converter.calculate_output_buffer_size(8)?, 16);
    assert_eq!(output.packet_count, 4);
    assert_eq!(output.data.len(), 16);
    assert_eq!(converted_buffer.len(), 16);
    assert_eq!(converter.finish(4)?.packet_count, 0);
    Ok(())
}

#[test]
fn leftover_input_outlives_the_callers_buffer() -> Result<()> {
    let source = AudioStreamBasicDescription::linear_pcm_i16(44_100.0, 1, true);
    let destination = AudioStreamBasicDescription::linear_pcm_f32(44_100.0, 1, true);
    let mut converter = AudioConverter::new(&source, &destination)?;

    let mut caller_buffer = i16_bytes((0..64).map(|value| value * 100));
    let first = converter.fill_complex_buffer(input(&caller_buffer, 64, 1), 16)?;
    assert_eq!(first.packet_count, 16);
    caller_buffer.fill(0xAA);
    drop(caller_buffer);

    let mut samples = f32_samples(&first.data);
    loop {
        let next = converter.fill_complex_buffer(input(&[], 0, 1), 16)?;
        if next.packet_count == 0 {
            break;
        }
        samples.extend(f32_samples(&next.data));
    }

    assert_eq!(samples.len(), 64);
    for (index, sample) in samples.iter().enumerate() {
        let expected = f32::from(i16::try_from(index).unwrap() * 100) / 32_768.0;
        assert!(
            (sample - expected).abs() < 1e-6,
            "sample {index} was {sample}"
        );
    }
    Ok(())
}

#[test]
fn chunked_sample_rate_conversion_keeps_the_stream_open_until_finish() -> Result<()> {
    let source = AudioStreamBasicDescription::linear_pcm_f32(44_100.0, 1, true);
    let destination = AudioStreamBasicDescription::linear_pcm_f32(48_000.0, 1, true);
    let mut converter = AudioConverter::new(&source, &destination)?;
    let chunk =
        f32_bytes((0..1024).map(|index| (f32::from(u16::try_from(index).unwrap()) * 0.05).sin()));

    let mut total = 0_u32;
    for _ in 0..8 {
        let output = converter.fill_complex_buffer(input(&chunk, 1024, 1), 4096)?;
        assert!(output.packet_count > 0);
        total += output.packet_count;
    }
    loop {
        let output = converter.finish(4096)?;
        if output.packet_count == 0 {
            break;
        }
        total += output.packet_count;
    }

    let expected = 8 * 1024 * 48_000 / 44_100;
    assert!(
        total.abs_diff(expected) <= 64,
        "converted {total} frames, expected about {expected}"
    );
    assert!(converter
        .fill_complex_buffer(input(&chunk, 1024, 1), 16)
        .is_err());
    converter.reset()?;
    assert!(
        converter
            .fill_complex_buffer(input(&chunk, 1024, 1), 4096)?
            .packet_count
            > 0
    );
    Ok(())
}

#[test]
fn fill_complex_buffer_rejects_inconsistent_input() -> Result<()> {
    let source = AudioStreamBasicDescription::linear_pcm_i16(44_100.0, 1, true);
    let destination = AudioStreamBasicDescription::linear_pcm_f32(44_100.0, 1, true);
    let mut converter = AudioConverter::new(&source, &destination)?;
    let bytes = [0_u8; 8];

    assert!(converter
        .fill_complex_buffer(input(&bytes, 5, 1), 8)
        .is_err());
    assert!(converter
        .fill_complex_buffer(input(&bytes, 0, 1), 8)
        .is_err());
    assert!(converter
        .fill_complex_buffer(input(&bytes, 4, 1), 0)
        .is_err());
    assert_eq!(
        converter
            .fill_complex_buffer(input(&bytes, 4, 1), 8)?
            .packet_count,
        4
    );
    Ok(())
}

#[test]
fn convert_complex_buffer_validates_the_buffer_lists() -> Result<()> {
    let source = AudioStreamBasicDescription::linear_pcm_f32(44_100.0, 2, true);
    let destination = AudioStreamBasicDescription::linear_pcm_i16(44_100.0, 2, true);
    let converter = AudioConverter::new(&source, &destination)?;

    let mut input_list = OwnedAudioBufferList::for_format(&source, 16)?;
    input_list
        .data_mut(0)
        .expect("input buffer")
        .copy_from_slice(&f32_bytes((0..32).map(|index| {
            if index % 2 == 0 {
                0.5
            } else {
                -0.5
            }
        })));
    let mut output_list = OwnedAudioBufferList::for_format(&destination, 16)?;
    converter.convert_complex_buffer(16, &input_list, &mut output_list)?;

    let output = output_list.data(0).expect("output buffer");
    assert_eq!(output.len(), 64);
    let first = i16::from_ne_bytes([output[0], output[1]]);
    let second = i16::from_ne_bytes([output[2], output[3]]);
    assert_eq!(first, 16_384);
    assert_eq!(second, -16_384);

    assert!(converter
        .convert_complex_buffer(17, &input_list, &mut output_list)
        .is_err());
    let mut two_buffers = OwnedAudioBufferList::new(2, 1, 1024)?;
    assert!(converter
        .convert_complex_buffer(16, &input_list, &mut two_buffers)
        .is_err());
    input_list.set_data_byte_size(0, 8)?;
    assert!(converter
        .convert_complex_buffer(16, &input_list, &mut output_list)
        .is_err());
    Ok(())
}

#[test]
fn chunked_aac_encoding_does_not_flush_the_encoder_on_every_call() -> Result<()> {
    let source = AudioStreamBasicDescription::linear_pcm_f32(44_100.0, 1, true);
    let destination = AudioStreamBasicDescription {
        mSampleRate: 44_100.0,
        mFormatID: audiotoolbox::AUDIO_FORMAT_MPEG4_AAC,
        mFormatFlags: 0,
        mBytesPerPacket: 0,
        mFramesPerPacket: 1024,
        mBytesPerFrame: 0,
        mChannelsPerFrame: 1,
        mBitsPerChannel: 0,
        mReserved: 0,
    };
    let mut converter = AudioConverter::new(&source, &destination)?;
    let chunk =
        f32_bytes((0..1000).map(|index| (f32::from(u16::try_from(index).unwrap()) * 0.05).sin()));

    let mut per_call = Vec::new();
    for _ in 0..8 {
        let output = converter.fill_complex_buffer(input(&chunk, 1000, 1), 64)?;
        assert_eq!(
            output.packet_descriptions.len(),
            output.packet_count as usize
        );
        per_call.push(output.packet_count);
    }
    let mut tail = 0;
    loop {
        let output = converter.finish(64)?;
        if output.packet_count == 0 {
            break;
        }
        tail += output.packet_count;
    }

    assert!(
        per_call[1..].iter().all(|&packets| packets > 0),
        "an early end of stream stopped the encoder: {per_call:?}"
    );
    assert!(
        tail > 0,
        "finish must flush the frames the encoder still holds"
    );
    let total: u32 = per_call.iter().sum::<u32>() + tail;
    assert!(
        total > 8000 / 1024,
        "only {total} AAC packets for 8000 frames"
    );
    Ok(())
}
