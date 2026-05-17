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
    assert_eq!(sequence.track_index(track)?, 0);
    let indexed_track = sequence.track(0)?;
    let tempo_track = sequence.tempo_track()?;
    let iterator = indexed_track.event_iterator()?;
    let event = iterator.event_info()?;

    assert_ne!(tempo_track.as_raw(), indexed_track.as_raw());
    assert!(iterator.has_current_event()?);
    assert!(!iterator.has_previous_event()?);
    assert!(!iterator.has_next_event()?);
    assert_eq!(
        event.event_type,
        audiotoolbox::MUSIC_EVENT_TYPE_MIDI_NOTE_MESSAGE
    );
    assert!(event.time_stamp.abs() < f64::EPSILON);
    assert!((sequence.seconds_for_beats(1.0)? - 0.5).abs() < f64::EPSILON);
    assert!((sequence.beats_for_seconds(0.5)? - 1.0).abs() < f64::EPSILON);

    let player = MusicPlayer::new()?;
    player.set_sequence(&sequence)?;
    player.set_play_rate_scalar(1.25)?;
    player.preroll()?;
    assert!(!player.is_playing()?);
    assert!((player.play_rate_scalar()? - 1.25).abs() < f64::EPSILON);
    Ok(())
}
