use audiotoolbox::{
    AUGraph, AudioComponentDescription, ExtendedNote, MIDIChannelMessage, MIDINoteMessage,
    MusicEvent, MusicPlayer, MusicSequence, MusicTrackLoopInfo, NoteParamsControlValue, Result,
    AUDIO_COMPONENT_TYPE_OUTPUT, AUDIO_UNIT_SUBTYPE_GENERIC_OUTPUT, MUSIC_EVENT_TYPE_EXTENDED_NOTE,
    MUSIC_EVENT_TYPE_META, MUSIC_EVENT_TYPE_MIDI_NOTE_MESSAGE, MUSIC_EVENT_TYPE_MIDI_RAW_DATA,
    MUSIC_EVENT_TYPE_USER,
};

const fn note(velocity: u8) -> MIDINoteMessage {
    MIDINoteMessage {
        channel: 0,
        note: 60,
        velocity,
        releaseVelocity: 0,
        duration: 0.25,
    }
}

#[test]
fn music_sequence_and_player_smoke_test() -> Result<()> {
    let sequence = MusicSequence::new()?;
    let track = sequence.new_track()?;
    track.new_midi_note_event(0.0, note(64))?;
    assert_eq!(sequence.track_count()?, 1);
    assert_eq!(sequence.track_index(&track)?, 0);
    let indexed_track = sequence.track(0)?;
    let tempo_track = sequence.tempo_track()?;
    let iterator = indexed_track.event_iterator()?;
    let event = iterator.event_info()?;

    assert_ne!(tempo_track.as_raw(), indexed_track.as_raw());
    assert!(iterator.has_current_event()?);
    assert!(!iterator.has_previous_event()?);
    assert!(!iterator.has_next_event()?);
    assert_eq!(event.event_type, MUSIC_EVENT_TYPE_MIDI_NOTE_MESSAGE);
    assert_eq!(event.data.len(), std::mem::size_of::<MIDINoteMessage>());
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

#[test]
fn variable_length_events_keep_their_whole_payload() -> Result<()> {
    let sequence = MusicSequence::new()?;
    let track = sequence.new_track()?;
    let sysex = [0xF0, 0x7E, 0x7F, 0x09, 0x01, 0x02, 0x03, 0xF7];
    let text = b"this meta text is longer than one byte";
    let user = [1_u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    track.new_midi_raw_data_event(0.0, &sysex)?;
    track.new_meta_event(1.0, 0x01, text)?;
    track.new_user_event(2.0, &user)?;
    track.new_extended_note_event(
        3.0,
        &ExtendedNote {
            instrument_id: 0,
            group_id: 0,
            duration: 1.0,
            pitch: 60.0,
            velocity: 90.0,
            controls: &[NoteParamsControlValue {
                mID: 7,
                mValue: 0.5,
            }],
        },
    )?;

    let iterator = track.event_iterator()?;
    let raw = iterator.event_info()?;
    assert_eq!(raw.event_type, MUSIC_EVENT_TYPE_MIDI_RAW_DATA);
    assert_eq!(&raw.data[..4], &8_u32.to_ne_bytes());
    assert_eq!(&raw.data[4..12], &sysex);

    iterator.next_event()?;
    let meta = iterator.event_info()?;
    assert_eq!(meta.event_type, MUSIC_EVENT_TYPE_META);
    assert_eq!(meta.data[0], 0x01);
    assert_eq!(&meta.data[8..8 + text.len()], text);

    iterator.next_event()?;
    let user_event = iterator.event_info()?;
    assert_eq!(user_event.event_type, MUSIC_EVENT_TYPE_USER);
    assert_eq!(&user_event.data[4..4 + user.len()], &user);

    iterator.next_event()?;
    let extended = iterator.event_info()?;
    assert_eq!(extended.event_type, MUSIC_EVENT_TYPE_EXTENDED_NOTE);
    assert_eq!(&extended.data[12..16], &3_u32.to_ne_bytes());
    Ok(())
}

#[test]
fn set_event_info_takes_a_typed_event() -> Result<()> {
    let sequence = MusicSequence::new()?;
    let track = sequence.new_track()?;
    track.new_midi_note_event(0.0, note(64))?;
    let iterator = track.event_iterator()?;

    iterator.set_event_info(&MusicEvent::MidiNoteMessage(note(99)))?;
    let event = iterator.event_info()?;
    assert_eq!(event.event_type, MUSIC_EVENT_TYPE_MIDI_NOTE_MESSAGE);
    assert_eq!(event.data[2], 99);

    iterator.set_event_info(&MusicEvent::MidiChannelMessage(MIDIChannelMessage {
        status: 0xB0,
        data1: 7,
        data2: 100,
        reserved: 0,
    }))?;
    let changed = iterator.event_info()?;
    assert_eq!(
        changed.event_type,
        audiotoolbox::MUSIC_EVENT_TYPE_MIDI_CHANNEL_MESSAGE
    );
    assert_eq!(&changed.data[..3], &[0xB0, 7, 100]);
    Ok(())
}

#[test]
fn sequence_keeps_its_graph_alive() -> Result<()> {
    let sequence = MusicSequence::new()?;
    let track = sequence.new_track()?;
    let graph = AUGraph::new()?;
    let node = graph.add_node(AudioComponentDescription::apple(
        AUDIO_COMPONENT_TYPE_OUTPUT,
        AUDIO_UNIT_SUBTYPE_GENERIC_OUTPUT,
    ))?;
    let raw_graph = graph.as_raw();
    sequence.set_au_graph(&graph)?;
    drop(graph);

    assert_eq!(sequence.au_graph_raw()?, raw_graph);
    track.set_dest_node(node)?;
    assert_eq!(track.dest_node()?, node);
    Ok(())
}

#[test]
fn player_keeps_its_sequence_alive() -> Result<()> {
    let player = MusicPlayer::new()?;
    let sequence = MusicSequence::new()?;
    sequence.new_track()?.new_midi_note_event(0.0, note(64))?;
    let raw_sequence = sequence.as_raw();
    player.set_sequence(&sequence)?;
    drop(sequence);

    assert_eq!(player.sequence_raw()?, raw_sequence);
    player.preroll()?;
    player.clear_sequence()?;
    Ok(())
}

#[test]
fn track_properties_and_disposal() -> Result<()> {
    let mut sequence = MusicSequence::new()?;
    {
        let track = sequence.new_track()?;
        track.new_midi_note_event(0.0, note(64))?;
        assert!(!track.is_muted()?);
        track.set_muted(true)?;
        assert!(track.is_muted()?);
        track.set_solo(true)?;
        assert!(track.is_solo()?);
        track.set_offset_time(2.0)?;
        assert!((track.offset_time()? - 2.0).abs() < f64::EPSILON);
        track.set_loop_info(MusicTrackLoopInfo {
            loopDuration: 1.0,
            numberOfLoops: 3,
        })?;
        assert_eq!(track.loop_info()?.numberOfLoops, 3);
        assert!(track.track_length()? > 0.0);
        assert!(track.get_property_typed::<u64>(0).is_err());
        let _second = sequence.new_track()?;
    }
    assert!(sequence.tempo_track()?.time_resolution()? > 0);

    assert_eq!(sequence.track_count()?, 2);
    sequence.dispose_track(0)?;
    assert_eq!(sequence.track_count()?, 1);
    assert!(sequence.dispose_track(5).is_err());
    Ok(())
}
