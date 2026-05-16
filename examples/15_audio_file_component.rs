use audiotoolbox::{AudioFileComponent, AUDIO_FILE_AIFF_TYPE};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let component = AudioFileComponent::open("/System/Library/Sounds/Glass.aiff")?;

    println!(
        "can_read_aiff={} sample_rate={:.1}",
        component.can_read(AUDIO_FILE_AIFF_TYPE)?,
        component.data_format()?.mSampleRate,
    );
    Ok(())
}
