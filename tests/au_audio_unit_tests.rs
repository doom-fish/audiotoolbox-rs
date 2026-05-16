use audiotoolbox::{
    AUAudioUnit, AudioComponentDescription, Result, AUDIO_COMPONENT_TYPE_FORMAT_CONVERTER,
    AUDIO_UNIT_SUBTYPE_AU_CONVERTER,
};

#[test]
fn au_audio_unit_reports_component_metadata() -> Result<()> {
    let description = AudioComponentDescription::apple(
        AUDIO_COMPONENT_TYPE_FORMAT_CONVERTER,
        AUDIO_UNIT_SUBTYPE_AU_CONVERTER,
    );
    let unit = AUAudioUnit::new_in_process(description)?;
    let updated_maximum_frames = unit.maximum_frames_to_render().max(256);

    assert_eq!(unit.component_description()?.component_type, description.component_type);
    assert_eq!(unit.component_description()?.component_sub_type, description.component_sub_type);
    assert!(unit.component_name()?.is_some());
    assert!(unit.audio_unit_name()?.is_some());
    assert!(unit.manufacturer_name()?.is_some());
    assert!(unit.input_bus_count() >= 1);
    assert!(unit.output_bus_count() >= 1);

    unit.set_maximum_frames_to_render(updated_maximum_frames);
    assert_eq!(unit.maximum_frames_to_render(), updated_maximum_frames);
    assert!(!unit.render_resources_allocated());
    Ok(())
}
