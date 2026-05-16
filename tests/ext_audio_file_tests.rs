use audiotoolbox::{AudioStreamBasicDescription, ExtAudioFile, InterleavedAudioBuffer, Result};

const fn glass_path() -> &'static str {
    "/System/Library/Sounds/Glass.aiff"
}

#[test]
fn ext_audio_file_reads_interleaved_pcm() -> Result<()> {
    let file = ExtAudioFile::open(glass_path())?;
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
        512,
    )?;
    let frames = file.read_interleaved(&mut buffer, 512)?;

    assert!(frames > 0);
    assert!(!buffer.as_bytes().is_empty());
    Ok(())
}
