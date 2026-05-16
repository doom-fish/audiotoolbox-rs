use crate::{
    ffi,
    internal::status_to_result,
    MIDINoteMessage,
    MusicPlayerRef,
    MusicSequenceRef,
    MusicTrackRef,
    AudioToolboxError,
    Result,
};

#[derive(Debug)]
pub struct MusicSequence {
    handle: *mut std::ffi::c_void,
    raw: MusicSequenceRef,
}

#[derive(Debug, Clone, Copy)]
pub struct MusicTrack {
    raw: MusicTrackRef,
}

#[derive(Debug)]
pub struct MusicPlayer {
    handle: *mut std::ffi::c_void,
    raw: MusicPlayerRef,
}

impl MusicSequence {
    pub fn new() -> Result<Self> {
        let mut handle = std::ptr::null_mut();
        let status = unsafe { ffi::music::at_music_sequence_new(&mut handle) };
        status_to_result("NewMusicSequence", status)?;
        let raw: MusicSequenceRef = unsafe { ffi::music::at_music_sequence_raw(handle) }.cast();
        if raw.is_null() {
            return Err(AudioToolboxError::message(
                "NewMusicSequence",
                "framework returned a null MusicSequence",
            ));
        }
        Ok(Self { handle, raw })
    }

    pub fn as_raw(&self) -> MusicSequenceRef {
        self.raw
    }

    pub fn new_track(&self) -> Result<MusicTrack> {
        let mut handle = std::ptr::null_mut();
        let status = unsafe { ffi::music::at_music_sequence_new_track(self.raw.cast(), &mut handle) };
        status_to_result("MusicSequenceNewTrack", status)?;
        let raw: MusicTrackRef = unsafe { ffi::music::at_music_track_raw(handle) }.cast();
        unsafe { ffi::music::at_music_track_release(handle) };
        if raw.is_null() {
            return Err(AudioToolboxError::message(
                "MusicSequenceNewTrack",
                "framework returned a null MusicTrack",
            ));
        }
        Ok(MusicTrack { raw })
    }

    pub fn track_count(&self) -> Result<u32> {
        let mut track_count = 0_u32;
        let status = unsafe {
            ffi::music::at_music_sequence_get_track_count(self.raw.cast(), &mut track_count)
        };
        status_to_result("MusicSequenceGetTrackCount", status)?;
        Ok(track_count)
    }

    pub fn close(mut self) -> Result<()> {
        self.release();
        Ok(())
    }

    fn release(&mut self) {
        if !self.handle.is_null() {
            unsafe { ffi::music::at_music_sequence_release(self.handle) };
            self.handle = std::ptr::null_mut();
            self.raw = std::ptr::null_mut();
        }
    }
}

impl Drop for MusicSequence {
    fn drop(&mut self) {
        self.release();
    }
}

impl MusicTrack {
    pub fn as_raw(&self) -> MusicTrackRef {
        self.raw
    }

    pub fn new_midi_note_event(&self, time_stamp: f64, note_message: MIDINoteMessage) -> Result<()> {
        let status = unsafe {
            ffi::music::at_music_track_new_midi_note_event(
                self.raw.cast(),
                time_stamp,
                std::ptr::from_ref(&note_message),
            )
        };
        status_to_result("MusicTrackNewMIDINoteEvent", status)
    }
}

impl MusicPlayer {
    pub fn new() -> Result<Self> {
        let mut handle = std::ptr::null_mut();
        let status = unsafe { ffi::music::at_music_player_new(&mut handle) };
        status_to_result("NewMusicPlayer", status)?;
        let raw: MusicPlayerRef = unsafe { ffi::music::at_music_player_raw(handle) }.cast();
        if raw.is_null() {
            return Err(AudioToolboxError::message(
                "NewMusicPlayer",
                "framework returned a null MusicPlayer",
            ));
        }
        Ok(Self { handle, raw })
    }

    pub fn as_raw(&self) -> MusicPlayerRef {
        self.raw
    }

    pub fn set_sequence(&self, sequence: &MusicSequence) -> Result<()> {
        let status = unsafe {
            ffi::music::at_music_player_set_sequence(self.raw.cast(), sequence.raw.cast())
        };
        status_to_result("MusicPlayerSetSequence", status)
    }

    pub fn preroll(&self) -> Result<()> {
        let status = unsafe { ffi::music::at_music_player_preroll(self.raw.cast()) };
        status_to_result("MusicPlayerPreroll", status)
    }

    pub fn start(&self) -> Result<()> {
        let status = unsafe { ffi::music::at_music_player_start(self.raw.cast()) };
        status_to_result("MusicPlayerStart", status)
    }

    pub fn stop(&self) -> Result<()> {
        let status = unsafe { ffi::music::at_music_player_stop(self.raw.cast()) };
        status_to_result("MusicPlayerStop", status)
    }

    pub fn is_playing(&self) -> Result<bool> {
        let mut is_playing = 0_u32;
        let status = unsafe {
            ffi::music::at_music_player_is_playing(self.raw.cast(), &mut is_playing)
        };
        status_to_result("MusicPlayerIsPlaying", status)?;
        Ok(is_playing != 0)
    }

    pub fn close(mut self) -> Result<()> {
        self.release();
        Ok(())
    }

    fn release(&mut self) {
        if !self.handle.is_null() {
            unsafe { ffi::music::at_music_player_release(self.handle) };
            self.handle = std::ptr::null_mut();
            self.raw = std::ptr::null_mut();
        }
    }
}

impl Drop for MusicPlayer {
    fn drop(&mut self) {
        self.release();
    }
}
