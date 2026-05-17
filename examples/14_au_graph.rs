use audiotoolbox::{
    AUGraph, AudioComponentDescription, AUDIO_COMPONENT_TYPE_OUTPUT,
    AUDIO_UNIT_SUBTYPE_GENERIC_OUTPUT,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let graph = AUGraph::new()?;
    let output_node = graph.add_node(AudioComponentDescription::apple(
        AUDIO_COMPONENT_TYPE_OUTPUT,
        AUDIO_UNIT_SUBTYPE_GENERIC_OUTPUT,
    ))?;
    let description = graph.node_description(output_node)?;
    graph.open()?;

    println!(
        "nodes={} output_type={} output_subtype={}",
        graph.node_count()?,
        audiotoolbox::fourcc_to_string(description.component_type),
        audiotoolbox::fourcc_to_string(description.component_sub_type),
    );
    Ok(())
}
