use audiotoolbox::{
    AudioFileComponent, AudioStreamBasicDescription, Result, AUDIO_FILE_AIFF_TYPE,
    AUDIO_FILE_PROPERTY_DATA_FORMAT, AUDIO_FORMAT_LINEAR_PCM,
};

const fn glass_path() -> &'static str {
    "/System/Library/Sounds/Glass.aiff"
}

#[test]
fn audio_file_component_opens_and_queries_glass() -> Result<()> {
    let component = AudioFileComponent::open(glass_path())?;
    let info = component.property_info(AUDIO_FILE_PROPERTY_DATA_FORMAT)?;
    let format = component.data_format()?;

    assert!(component.can_read(AUDIO_FILE_AIFF_TYPE)?);
    assert_eq!(
        info.data_size as usize,
        std::mem::size_of::<AudioStreamBasicDescription>()
    );
    assert_eq!(format.mFormatID, AUDIO_FORMAT_LINEAR_PCM);
    component.close_file()?;
    Ok(())
}
