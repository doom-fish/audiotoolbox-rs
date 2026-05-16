use audiotoolbox::{AudioConversionInput, AudioConverter, AudioStreamBasicDescription};

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    println!("converted_packets={} output_bytes={}", output.packet_count, output.data.len());
    Ok(())
}
