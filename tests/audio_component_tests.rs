use audiotoolbox::{
    AudioComponent, AudioComponentDescription, Result, AUDIO_COMPONENT_TYPE_FORMAT_CONVERTER,
    AUDIO_UNIT_SUBTYPE_AU_CONVERTER,
};

#[test]
fn audio_component_finds_converter_unit() -> Result<()> {
    let description = AudioComponentDescription::apple(
        AUDIO_COMPONENT_TYPE_FORMAT_CONVERTER,
        AUDIO_UNIT_SUBTYPE_AU_CONVERTER,
    );
    assert!(AudioComponent::count(description) > 0);

    let component = AudioComponent::iter(description)
        .next()
        .expect("converter component should exist");
    let name = component.copy_name()?;
    let component_description = component.description()?;
    let configuration = component.copy_configuration_info_raw()?;
    let validation_result = component.validate_raw(None)?;
    let instance = component.new_instance()?;
    let parent = instance.component()?;

    assert!(!name.is_empty());
    assert_eq!(
        component_description.component_type,
        AUDIO_COMPONENT_TYPE_FORMAT_CONVERTER
    );
    assert_eq!(
        parent.description()?.component_sub_type,
        AUDIO_UNIT_SUBTYPE_AU_CONVERTER
    );
    assert!(!configuration.is_null());
    assert!(validation_result <= 5);
    let _ = instance.can_do(0);
    Ok(())
}
