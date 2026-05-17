use audiotoolbox::{MIDINoteMessage, MusicPlayer, MusicSequence};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sequence = MusicSequence::new()?;
    let track = sequence.new_track()?;
    track.new_midi_note_event(
        0.0,
        MIDINoteMessage {
            channel: 0,
            note: 60,
            velocity: 64,
            releaseVelocity: 0,
            duration: 0.25,
        },
    )?;
    let player = MusicPlayer::new()?;
    player.set_sequence(&sequence)?;
    player.preroll()?;

    println!(
        "tracks={} is_playing={}",
        sequence.track_count()?,
        player.is_playing()?
    );
    Ok(())
}
