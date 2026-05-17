use audiotoolbox::{AudioStreamBasicDescription, ExtAudioFile, InterleavedAudioBuffer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = ExtAudioFile::open("/System/Library/Sounds/Glass.aiff")?;
    let file_format = file.file_data_format()?;
    let client_format = AudioStreamBasicDescription::linear_pcm_f32(
        file_format.mSampleRate,
        file_format.mChannelsPerFrame,
        true,
    );
    file.set_client_data_format(&client_format)?;
    let mut buffer = InterleavedAudioBuffer::new(
        client_format.mChannelsPerFrame,
        client_format.mBytesPerFrame,
        256,
    )?;
    let frames = file.read_interleaved(&mut buffer, 256)?;

    println!(
        "decoded_frames={} decoded_bytes={}",
        frames,
        buffer.as_bytes().len()
    );
    Ok(())
}
