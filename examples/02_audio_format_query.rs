use audiotoolbox::{AudioFormat, AudioStreamBasicDescription};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pcm = AudioStreamBasicDescription::linear_pcm_i16(44_100.0, 2, true);
    let info = AudioFormat::format_info(pcm)?;
    let encode_ids = AudioFormat::encode_format_ids()?;

    println!(
        "format_id={} channels={} encode_formats={}",
        info.mFormatID,
        info.mChannelsPerFrame,
        encode_ids.len()
    );
    Ok(())
}
