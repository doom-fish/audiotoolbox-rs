use audiotoolbox::{
    AUAudioUnit, AudioComponentDescription, AUDIO_COMPONENT_TYPE_FORMAT_CONVERTER,
    AUDIO_UNIT_SUBTYPE_AU_CONVERTER,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let description = AudioComponentDescription::apple(
        AUDIO_COMPONENT_TYPE_FORMAT_CONVERTER,
        AUDIO_UNIT_SUBTYPE_AU_CONVERTER,
    );
    let unit = AUAudioUnit::new_in_process(description)?;

    println!(
        "component={:?} input_busses={} output_busses={} max_frames={}",
        unit.component_description()?,
        unit.input_bus_count(),
        unit.output_bus_count(),
        unit.maximum_frames_to_render(),
    );
    Ok(())
}
