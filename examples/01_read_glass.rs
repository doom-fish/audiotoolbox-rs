use audiotoolbox::AudioFile;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sound_path = "/System/Library/Sounds/Glass.aiff";
    let audio_file = AudioFile::open(sound_path)?;
    let format = audio_file.data_format()?;
    let duration = audio_file.estimated_duration()?;
    let packet_count = audio_file.packet_count()?;
    let packet_data = audio_file.read_packet_data(0, 1, false)?;

    println!(
        "opened={} sample_rate={:.1}Hz channels={} packets={} duration={:.3}s first_read={}B",
        sound_path,
        format.mSampleRate,
        format.mChannelsPerFrame,
        packet_count,
        duration,
        packet_data.data.len()
    );
    println!("✅ audiotoolbox read OK");

    Ok(())
}
