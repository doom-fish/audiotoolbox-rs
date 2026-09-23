use crate::{
    ffi,
    internal::status_to_result,
    property::{read_property, AudioProperty},
    AudioFileStreamId, AudioFileStreamParseFlags, AudioFileStreamPropertyId,
    AudioStreamBasicDescription, AudioStreamPacketDescription, AudioToolboxError, PacketData,
    Result, AUDIO_FILE_STREAM_PROPERTY_AUDIO_DATA_BYTE_COUNT,
    AUDIO_FILE_STREAM_PROPERTY_AUDIO_DATA_PACKET_COUNT, AUDIO_FILE_STREAM_PROPERTY_BIT_RATE,
    AUDIO_FILE_STREAM_PROPERTY_DATA_FORMAT, AUDIO_FILE_STREAM_PROPERTY_FILE_FORMAT,
    AUDIO_FILE_STREAM_PROPERTY_MAGIC_COOKIE_DATA, AUDIO_FILE_STREAM_PROPERTY_MAXIMUM_PACKET_SIZE,
    AUDIO_FILE_STREAM_SEEK_FLAG_OFFSET_IS_ESTIMATED,
};

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
            ffi::audio_file_stream::at_audio_file_stream_open(file_type_hint, &raw mut handle)
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
    pub fn parse_bytes(
        &self,
        data: &[u8],
        parse_flags: AudioFileStreamParseFlags,
    ) -> Result<PacketData> {
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
        let packets = self.take_packets();
        status_to_result("AudioFileStreamParseBytes", status)?;
        packets
    }

    fn take_packets(&self) -> Result<PacketData> {
        let operation = "AudioFileStreamParseBytes(packets)";
        let (mut byte_count, mut description_count, mut packet_count) = (0_u64, 0_u64, 0_u64);
        unsafe {
            ffi::audio_file_stream::at_audio_file_stream_pending_sizes(
                self.handle,
                &raw mut byte_count,
                &raw mut description_count,
                &raw mut packet_count,
            );
        }
        let too_large =
            || AudioToolboxError::message(operation, "parsed packets exceed the address space");
        let mut data = vec![0_u8; usize::try_from(byte_count).map_err(|_| too_large())?];
        let mut packet_descriptions = vec![
            AudioStreamPacketDescription::default();
            usize::try_from(description_count)
                .map_err(|_| too_large())?
        ];
        let taken = unsafe {
            ffi::audio_file_stream::at_audio_file_stream_take_pending(
                self.handle,
                data.as_mut_ptr().cast(),
                byte_count,
                packet_descriptions.as_mut_ptr(),
                description_count,
            )
        };
        if !taken {
            return Err(AudioToolboxError::message(
                operation,
                "the parsed packet buffer changed while it was copied",
            ));
        }
        Ok(PacketData {
            data,
            packet_count: u32::try_from(packet_count).map_err(|_| {
                AudioToolboxError::message(
                    operation,
                    "one parse produced more than UInt32::MAX packets",
                )
            })?,
            packet_descriptions,
        })
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn seek(&self, packet_offset: i64) -> Result<(i64, bool)> {
        let mut byte_offset = 0_i64;
        let mut flags = 0_u32;
        let status = unsafe {
            ffi::audio_file_stream::at_audio_file_stream_seek(
                self.raw.cast(),
                packet_offset,
                &raw mut byte_offset,
                &raw mut flags,
            )
        };
        status_to_result("AudioFileStreamSeek", status)?;
        Ok((
            byte_offset,
            flags & AUDIO_FILE_STREAM_SEEK_FLAG_OFFSET_IS_ESTIMATED != 0,
        ))
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
                &raw mut data_size,
                &raw mut writable,
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

    fn get_property_typed<T: AudioProperty>(
        &self,
        property_id: AudioFileStreamPropertyId,
        operation: &'static str,
    ) -> Result<T> {
        read_property(operation, |data, size| unsafe {
            ffi::audio_file_stream::at_audio_file_stream_get_property(
                self.raw.cast(),
                property_id,
                size,
                data,
            )
        })
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
                &raw mut size,
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
