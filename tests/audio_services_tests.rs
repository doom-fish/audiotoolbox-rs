use audiotoolbox::{Result, SystemSound};

const fn glass_path() -> &'static str {
    "/System/Library/Sounds/Glass.aiff"
}

#[test]
fn system_sound_creates_id() -> Result<()> {
    let sound = SystemSound::from_path(glass_path())?;
    assert!(sound.id() > 0);
    Ok(())
}
