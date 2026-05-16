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

    assert_eq!(format.mFormatID, AUDIO_FORMAT_LINEAR_PCM);
    assert!(duration > 0.0);
    assert!(packet_count > 0);
    assert!(!packet_data.data.is_empty());
    Ok(())
}
