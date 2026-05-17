use audiotoolbox::{AudioConversionInput, AudioConverter, AudioStreamBasicDescription, Result};

#[test]
fn audio_converter_converts_pcm_once() -> Result<()> {
    let source = AudioStreamBasicDescription::linear_pcm_i16(44_100.0, 1, true);
    let destination = AudioStreamBasicDescription::linear_pcm_f32(44_100.0, 1, true);
    let converter = AudioConverter::new(&source, &destination)?;
    let input_bytes = [0_u8, 0, 255, 127, 0, 128, 0, 0];
    let output = converter.fill_complex_buffer_once(
        AudioConversionInput {
            data: &input_bytes,
            packet_count: 4,
            packet_descriptions: None,
            channels: 1,
        },
        4,
    )?;
    let converted_buffer = converter.convert_buffer(&input_bytes, 16)?;

    assert_eq!(converter.calculate_input_buffer_size(16)?, 8);
    assert_eq!(converter.calculate_output_buffer_size(8)?, 16);
    assert_eq!(output.packet_count, 4);
    assert_eq!(output.data.len(), 16);
    assert_eq!(converted_buffer.len(), 16);
    Ok(())
}
