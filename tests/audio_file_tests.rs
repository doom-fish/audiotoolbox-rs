use audiotoolbox::{AudioFile, Result, AUDIO_FORMAT_LINEAR_PCM};

const fn glass_path() -> &'static str {
    "/System/Library/Sounds/Glass.aiff"
}

#[test]
fn audio_file_reads_glass_properties() -> Result<()> {
    let audio_file = AudioFile::open(glass_path())?;
    let format = audio_file.data_format()?;
    let duration = audio_file.estimated_duration()?;
    let packet_count = audio_file.packet_count()?;
    let packet_data = audio_file.read_packet_data(0, 1, false)?;
    let byte_count = audio_file.audio_data_byte_count()?;
    let data_offset = audio_file.data_offset()?;
    let header_bytes = audio_file.read_bytes(0, 32, false)?;

    assert_eq!(format.mFormatID, AUDIO_FORMAT_LINEAR_PCM);
    assert!(duration > 0.0);
    assert!(packet_count > 0);
    assert!(!packet_data.data.is_empty());
    assert!(byte_count > 0);
    assert!(data_offset >= 0);
    assert_eq!(header_bytes.len(), 32);
    Ok(())
}

#[test]
fn paths_are_passed_as_exact_bytes() {
    use std::os::unix::ffi::OsStrExt;

    let dir = std::path::Path::new(env!("CARGO_TARGET_TMPDIR"));
    let lossy = dir.join("exact-path-\u{FFFD}.caf");
    let _ = std::fs::remove_file(&lossy);
    let raw = dir.join(std::ffi::OsStr::from_bytes(b"exact-path-\xff.caf"));
    let format = audiotoolbox::AudioStreamBasicDescription::linear_pcm_i16(44_100.0, 1, true);

    let created = AudioFile::create(
        &raw,
        audiotoolbox::AUDIO_FILE_CAF_TYPE,
        &format,
        audiotoolbox::AUDIO_FILE_FLAGS_ERASE_FILE,
    );
    assert!(created.is_err(), "the file system rejects non-UTF-8 names");
    assert!(
        !lossy.exists(),
        "a lossy conversion created a different file"
    );
}

#[test]
fn relative_paths_resolve_against_the_working_directory() -> Result<()> {
    let relative = std::path::Path::new("target/tmp/relative-path.caf");
    std::fs::create_dir_all("target/tmp").expect("create target/tmp");
    let format = audiotoolbox::AudioStreamBasicDescription::linear_pcm_i16(44_100.0, 1, true);
    AudioFile::create(
        relative,
        audiotoolbox::AUDIO_FILE_CAF_TYPE,
        &format,
        audiotoolbox::AUDIO_FILE_FLAGS_ERASE_FILE,
    )?
    .close()?;
    assert!(relative.exists());
    assert_eq!(
        AudioFile::open(relative)?.data_format()?.mBitsPerChannel,
        16
    );
    std::fs::remove_file(relative).expect("remove the test file");
    Ok(())
}
