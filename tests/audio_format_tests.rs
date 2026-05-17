use audiotoolbox::{
    fourcc_to_string, is_printable_fourcc, AudioFormat, AudioStreamBasicDescription, Result,
    AUDIO_FILE_AIFF_TYPE, AUDIO_FORMAT_LINEAR_PCM, AUDIO_FORMAT_PROPERTY_ENCODE_FORMAT_IDS,
};

#[test]
fn audio_format_queries_work() -> Result<()> {
    let input = AudioStreamBasicDescription::linear_pcm_i16(44_100.0, 2, true);
    let format_info = AudioFormat::format_info(input)?;

    assert_eq!(format_info.mFormatID, AUDIO_FORMAT_LINEAR_PCM);
    assert!(
        AudioFormat::property_info(
            AUDIO_FORMAT_PROPERTY_ENCODE_FORMAT_IDS,
            Option::<&u32>::None,
        )? > 0
    );
    assert!(!AudioFormat::encode_format_ids()?.is_empty());
    assert!(AudioFormat::decode_format_ids()?.contains(&AUDIO_FORMAT_LINEAR_PCM));
    assert!(!AudioFormat::format_is_vbr(&format_info)?);
    assert_eq!(fourcc_to_string(AUDIO_FILE_AIFF_TYPE), "AIFF");
    assert!(is_printable_fourcc(AUDIO_FILE_AIFF_TYPE));
    Ok(())
}
