use audiotoolbox::{
    AudioComponent, AudioComponentDescription, AUDIO_COMPONENT_TYPE_FORMAT_CONVERTER,
    AUDIO_UNIT_SUBTYPE_AU_CONVERTER,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let description = AudioComponentDescription::apple(
        AUDIO_COMPONENT_TYPE_FORMAT_CONVERTER,
        AUDIO_UNIT_SUBTYPE_AU_CONVERTER,
    );
    let component = AudioComponent::iter(description)
        .next()
        .expect("converter component should exist");

    println!(
        "component={} version={}",
        component.copy_name()?,
        component.version()?
    );
    Ok(())
}
