use std::thread;

use audiotoolbox::{
    AUGraph, AudioComponentDescription, AudioUnit, Result, AUDIO_COMPONENT_TYPE_MUSIC_DEVICE,
    AUDIO_COMPONENT_TYPE_OUTPUT, AUDIO_UNIT_SUBTYPE_GENERIC_OUTPUT,
};

const DLS_SYNTH: u32 = u32::from_be_bytes(*b"dls ");

fn on_parallel_threads(work: fn() -> Result<()>) -> Result<()> {
    let threads: Vec<_> = (0..4).map(|_| thread::spawn(work)).collect();
    for thread in threads {
        thread.join().expect("worker thread")?;
    }
    Ok(())
}

#[test]
fn synth_units_initialize_on_parallel_threads() -> Result<()> {
    on_parallel_threads(|| {
        for _ in 0..6 {
            let unit = AudioUnit::new_apple(AUDIO_COMPONENT_TYPE_MUSIC_DEVICE, DLS_SYNTH)?;
            unit.initialize()?;
            unit.uninitialize()?;
        }
        Ok(())
    })
}

#[test]
fn synth_graphs_initialize_on_parallel_threads() -> Result<()> {
    on_parallel_threads(|| {
        for _ in 0..4 {
            let graph = AUGraph::new()?;
            let synth = graph.add_node(AudioComponentDescription::apple(
                AUDIO_COMPONENT_TYPE_MUSIC_DEVICE,
                DLS_SYNTH,
            ))?;
            let output = graph.add_node(AudioComponentDescription::apple(
                AUDIO_COMPONENT_TYPE_OUTPUT,
                AUDIO_UNIT_SUBTYPE_GENERIC_OUTPUT,
            ))?;
            graph.open()?;
            graph.connect_node_input(synth, 0, output, 0)?;
            graph.initialize()?;
            graph.uninitialize()?;
        }
        Ok(())
    })
}
