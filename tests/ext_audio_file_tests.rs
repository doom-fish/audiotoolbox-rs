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

#[test]
fn ext_audio_file_write_is_bounded_by_the_buffer() -> Result<()> {
    let path = std::path::Path::new(env!("CARGO_TARGET_TMPDIR")).join("ext-audio-file-write.caf");
    let format = AudioStreamBasicDescription::linear_pcm_i16(44_100.0, 1, true);
    let file = ExtAudioFile::create_erasing(&path, audiotoolbox::AUDIO_FILE_CAF_TYPE, &format)?;
    file.set_client_data_format(&format)?;

    let mut buffer = InterleavedAudioBuffer::new(1, 2, 64)?;
    buffer.set_valid_byte_size(32);
    assert!(file.write_interleaved(17, &buffer).is_err());
    file.write_interleaved(16, &buffer)?;

    let wide = InterleavedAudioBuffer::new(1, 4, 64)?;
    assert!(file.write_interleaved(1, &wide).is_err());
    let mut wrong_frame_size = InterleavedAudioBuffer::new(1, 4, 64)?;
    assert!(file.read_interleaved(&mut wrong_frame_size, 1).is_err());
    file.close()?;
    std::fs::remove_file(&path).expect("remove the test file");
    Ok(())
}
