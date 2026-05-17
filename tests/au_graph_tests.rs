use audiotoolbox::{
    AUGraph, AudioComponentDescription, Result, AUDIO_COMPONENT_TYPE_OUTPUT,
    AUDIO_UNIT_SUBTYPE_GENERIC_OUTPUT,
};

#[test]
fn au_graph_adds_and_describes_nodes() -> Result<()> {
    let graph = AUGraph::new()?;
    let output_description = AudioComponentDescription::apple(
        AUDIO_COMPONENT_TYPE_OUTPUT,
        AUDIO_UNIT_SUBTYPE_GENERIC_OUTPUT,
    );
    let output_node = graph.add_node(output_description)?;

    assert_eq!(graph.node_count()?, 1);
    assert_eq!(graph.node_description(output_node)?, output_description);
    graph.open()?;
    assert!(graph.is_open()?);
    assert!(!graph.is_initialized()?);
    assert!(!graph.is_running()?);
    let _ = graph.update()?;
    assert!(graph.cpu_load()? >= 0.0);
    assert_eq!(graph.interaction_count()?, 0);
    Ok(())
}
