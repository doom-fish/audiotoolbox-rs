use crate::{
    ffi, internal::status_to_result, AudioFileStreamId, AudioFileStreamParseFlags,
    AudioFileStreamPropertyId, AudioStreamBasicDescription, AudioToolboxError, Result,
    AUDIO_FILE_STREAM_PROPERTY_AUDIO_DATA_BYTE_COUNT,
    AUDIO_FILE_STREAM_PROPERTY_AUDIO_DATA_PACKET_COUNT, AUDIO_FILE_STREAM_PROPERTY_BIT_RATE,
    AUDIO_FILE_STREAM_PROPERTY_DATA_FORMAT, AUDIO_FILE_STREAM_PROPERTY_FILE_FORMAT,
    AUDIO_FILE_STREAM_PROPERTY_MAGIC_COOKIE_DATA, AUDIO_FILE_STREAM_PROPERTY_MAXIMUM_PACKET_SIZE,
};
use std::mem::MaybeUninit;

#[derive(Debug)]
/// Owning wrapper around an AudioToolbox.framework `AudioFileStreamID`.
pub struct AudioFileStream {
    handle: *mut std::ffi::c_void,
    raw: AudioFileStreamId,
}

impl AudioFileStream {
    /// Wraps `AudioFileStreamOpen`.
    ///
    /// The returned wrapper owns the underlying AudioToolbox.framework handle and releases it on drop.
    pub fn open(file_type_hint: u32) -> Result<Self> {
        let mut handle = std::ptr::null_mut();
        let status = unsafe {
            ffi::audio_file_stream::at_audio_file_stream_open(file_type_hint, &mut handle)
        };
        status_to_result("AudioFileStreamOpen", status)?;
        let raw: AudioFileStreamId =
            unsafe { ffi::audio_file_stream::at_audio_file_stream_raw(handle) }.cast();
        if raw.is_null() {
            return Err(AudioToolboxError::message(
                "AudioFileStreamOpen",
                "framework returned a null AudioFileStreamID",
            ));
        }
        Ok(Self { handle, raw })
    }

    /// Returns the wrapped `AudioFileStreamID`.
    pub fn as_raw(&self) -> AudioFileStreamId {
        self.raw
    }

    /// Wraps `AudioFileStreamParseBytes`.
    pub fn parse_bytes(&self, data: &[u8], parse_flags: AudioFileStreamParseFlags) -> Result<()> {
        let data_len = u32::try_from(data.len()).map_err(|_| {
            AudioToolboxError::message(
                "AudioFileStreamParseBytes",
                "payload exceeds UInt32::MAX bytes",
            )
        })?;
        let status = unsafe {
            ffi::audio_file_stream::at_audio_file_stream_parse_bytes(
                self.raw.cast(),
                data.as_ptr().cast(),
                data_len,
                parse_flags,
            )
        };
        status_to_result("AudioFileStreamParseBytes", status)
    }

    /// Wraps `AudioFileStreamReadyToProducePackets`.
    pub fn ready_to_produce_packets(&self) -> bool {
        unsafe {
            ffi::audio_file_stream::at_audio_file_stream_ready_to_produce_packets(self.handle) != 0
        }
    }

    /// Wraps `AudioFileStreamPacketCountSeen`.
    pub fn packet_count_seen(&self) -> u64 {
        unsafe { ffi::audio_file_stream::at_audio_file_stream_packet_count_seen(self.handle) }
    }

    /// Wraps `AudioFileStreamGetProperty`.
    pub fn file_format(&self) -> Result<u32> {
        self.get_property_typed(
            AUDIO_FILE_STREAM_PROPERTY_FILE_FORMAT,
            "AudioFileStreamGetProperty(file format)",
        )
    }

    /// Wraps `AudioFileStreamGetProperty`.
    pub fn data_format(&self) -> Result<AudioStreamBasicDescription> {
        self.get_property_typed(
            AUDIO_FILE_STREAM_PROPERTY_DATA_FORMAT,
            "AudioFileStreamGetProperty(data format)",
        )
    }

    /// Wraps `AudioFileStreamGetProperty`.
    pub fn maximum_packet_size(&self) -> Result<u32> {
        self.get_property_typed(
            AUDIO_FILE_STREAM_PROPERTY_MAXIMUM_PACKET_SIZE,
            "AudioFileStreamGetProperty(maximum packet size)",
        )
    }

    /// Wraps `AudioFileStreamGetProperty`.
    pub fn bit_rate(&self) -> Result<u32> {
        self.get_property_typed(
            AUDIO_FILE_STREAM_PROPERTY_BIT_RATE,
            "AudioFileStreamGetProperty(bit rate)",
        )
    }

    /// Wraps `AudioFileStreamGetProperty`.
    pub fn audio_data_byte_count(&self) -> Result<i64> {
        self.get_property_typed(
            AUDIO_FILE_STREAM_PROPERTY_AUDIO_DATA_BYTE_COUNT,
            "AudioFileStreamGetProperty(audio data byte count)",
        )
    }

    /// Wraps `AudioFileStreamGetProperty`.
    pub fn audio_data_packet_count(&self) -> Result<i64> {
        self.get_property_typed(
            AUDIO_FILE_STREAM_PROPERTY_AUDIO_DATA_PACKET_COUNT,
            "AudioFileStreamGetProperty(audio data packet count)",
        )
    }

    /// Wraps `AudioFileStreamGetProperty`.
    pub fn magic_cookie(&self) -> Result<Vec<u8>> {
        self.get_property_bytes(
            AUDIO_FILE_STREAM_PROPERTY_MAGIC_COOKIE_DATA,
            "AudioFileStreamGetProperty(magic cookie)",
        )
    }

    /// Wraps `AudioFileStreamGetPropertyInfo`.
    pub fn property_info(&self, property_id: AudioFileStreamPropertyId) -> Result<(u32, bool)> {
        let mut data_size = 0_u32;
        let mut writable = 0_u8;
        let status = unsafe {
            ffi::audio_file_stream::at_audio_file_stream_get_property_info(
                self.raw.cast(),
                property_id,
                &mut data_size,
                &mut writable,
            )
        };
        status_to_result("AudioFileStreamGetPropertyInfo", status)?;
        Ok((data_size, writable != 0))
    }

    /// Wraps `AudioFileStreamClose`.
    pub fn close(mut self) -> Result<()> {
        self.release();
        Ok(())
    }

    fn get_property_typed<T: Copy>(
        &self,
        property_id: AudioFileStreamPropertyId,
        operation: &'static str,
    ) -> Result<T> {
        let mut value = MaybeUninit::<T>::uninit();
        let mut size = u32::try_from(std::mem::size_of::<T>()).expect("typed property fits in u32");
        let status = unsafe {
            ffi::audio_file_stream::at_audio_file_stream_get_property(
                self.raw.cast(),
                property_id,
                &mut size,
                value.as_mut_ptr().cast(),
            )
        };
        status_to_result(operation, status)?;
        Ok(unsafe { value.assume_init() })
    }

    fn get_property_bytes(
        &self,
        property_id: AudioFileStreamPropertyId,
        operation: &'static str,
    ) -> Result<Vec<u8>> {
        let (data_size, _) = self.property_info(property_id)?;
        if data_size == 0 {
            return Ok(Vec::new());
        }
        let mut bytes = vec![0_u8; data_size as usize];
        let mut size = data_size;
        let status = unsafe {
            ffi::audio_file_stream::at_audio_file_stream_get_property(
                self.raw.cast(),
                property_id,
                &mut size,
                bytes.as_mut_ptr().cast(),
            )
        };
        status_to_result(operation, status)?;
        bytes.truncate(size as usize);
        Ok(bytes)
    }

    fn release(&mut self) {
        if !self.handle.is_null() {
            unsafe { ffi::audio_file_stream::at_audio_file_stream_release(self.handle) };
            self.handle = std::ptr::null_mut();
            self.raw = std::ptr::null_mut();
        }
    }
}

impl Drop for AudioFileStream {
    fn drop(&mut self) {
        self.release();
    }
}
