use audiotoolbox::{AudioFileStream, Result, AUDIO_FILE_AIFF_TYPE, AUDIO_FORMAT_LINEAR_PCM};

const fn glass_path() -> &'static str {
    "/System/Library/Sounds/Glass.aiff"
}

#[test]
fn audio_file_stream_parses_glass_aiff() -> Result<()> {
    let bytes = std::fs::read(glass_path()).expect("Glass.aiff should be readable");
    let stream = AudioFileStream::open(AUDIO_FILE_AIFF_TYPE)?;
    stream.parse_bytes(&bytes, 0)?;

    assert_eq!(stream.file_format()?, AUDIO_FILE_AIFF_TYPE);
    assert_eq!(stream.data_format()?.mFormatID, AUDIO_FORMAT_LINEAR_PCM);
    assert!(stream.maximum_packet_size()? > 0);
    assert!(stream.audio_data_packet_count()? > 0);
    assert!(stream.ready_to_produce_packets() || stream.packet_count_seen() > 0);
    Ok(())
}
