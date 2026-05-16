use audiotoolbox::SystemSound;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sound = SystemSound::from_path("/System/Library/Sounds/Glass.aiff")?;
    println!("system_sound_id={}", sound.id());
    Ok(())
}
