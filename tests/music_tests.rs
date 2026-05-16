use audiotoolbox::{MIDINoteMessage, MusicPlayer, MusicSequence, Result};

#[test]
fn music_sequence_and_player_smoke_test() -> Result<()> {
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
    assert_eq!(sequence.track_count()?, 1);

    let player = MusicPlayer::new()?;
    player.set_sequence(&sequence)?;
    player.preroll()?;
    assert!(!player.is_playing()?);
    Ok(())
}
