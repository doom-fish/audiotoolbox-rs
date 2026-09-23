use audiotoolbox::{
    AudioFile, AudioFileStream, Result, AUDIO_FILE_AIFF_TYPE, AUDIO_FORMAT_LINEAR_PCM,
};

const fn glass_path() -> &'static str {
    "/System/Library/Sounds/Glass.aiff"
}

#[test]
fn audio_file_stream_parses_glass_aiff() -> Result<()> {
    let bytes = std::fs::read(glass_path()).expect("Glass.aiff should be readable");
    let stream = AudioFileStream::open(AUDIO_FILE_AIFF_TYPE)?;
    let packets = stream.parse_bytes(&bytes, 0)?;

    assert_eq!(stream.file_format()?, AUDIO_FILE_AIFF_TYPE);
    assert_eq!(stream.data_format()?.mFormatID, AUDIO_FORMAT_LINEAR_PCM);
    assert!(stream.maximum_packet_size()? > 0);
    assert!(stream.audio_data_packet_count()? > 0);
    assert!(stream.ready_to_produce_packets());
    assert!(packets.packet_count > 0);
    assert_eq!(u64::from(packets.packet_count), stream.packet_count_seen());
    Ok(())
}

#[test]
fn chunked_parse_returns_every_audio_byte_in_order() -> Result<()> {
    let bytes = std::fs::read(glass_path()).expect("Glass.aiff should be readable");
    let file = AudioFile::open(glass_path())?;
    let data_offset = usize::try_from(file.data_offset()?).unwrap();
    let byte_count = usize::try_from(file.audio_data_byte_count()?).unwrap();

    let stream = AudioFileStream::open(AUDIO_FILE_AIFF_TYPE)?;
    let mut audio = Vec::new();
    let mut packet_count = 0_u64;
    for chunk in bytes.chunks(4093) {
        let packets = stream.parse_bytes(chunk, 0)?;
        assert!(packets.packet_descriptions.is_empty());
        packet_count += u64::from(packets.packet_count);
        audio.extend_from_slice(&packets.data);
    }

    assert_eq!(audio.len(), byte_count);
    assert_eq!(audio, bytes[data_offset..data_offset + byte_count]);
    assert_eq!(
        packet_count,
        u64::try_from(stream.audio_data_packet_count()?).unwrap()
    );
    assert_eq!(packet_count, stream.packet_count_seen());
    Ok(())
}

#[test]
fn seek_maps_packets_to_byte_offsets() -> Result<()> {
    let bytes = std::fs::read(glass_path()).expect("Glass.aiff should be readable");
    let stream = AudioFileStream::open(AUDIO_FILE_AIFF_TYPE)?;
    stream.parse_bytes(&bytes, 0)?;
    let bytes_per_packet = i64::from(stream.data_format()?.mBytesPerPacket);

    let (start, estimated) = stream.seek(0)?;
    assert!(!estimated);
    let (tenth, _) = stream.seek(10)?;
    assert_eq!(tenth - start, 10 * bytes_per_packet);
    Ok(())
}
