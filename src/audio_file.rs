use crate::{
    ffi,
    internal::{path_to_cstring, status_to_result},
    AudioFileId, AudioFilePermissions, AudioFilePropertyId, AudioFileTypeId,
    AudioStreamBasicDescription, AudioStreamPacketDescription, AudioToolboxError, Result,
    AUDIO_FILE_PROPERTY_AUDIO_DATA_BYTE_COUNT, AUDIO_FILE_PROPERTY_AUDIO_DATA_PACKET_COUNT,
    AUDIO_FILE_PROPERTY_DATA_FORMAT, AUDIO_FILE_PROPERTY_DATA_OFFSET,
    AUDIO_FILE_PROPERTY_ESTIMATED_DURATION, AUDIO_FILE_PROPERTY_MAGIC_COOKIE_DATA,
    AUDIO_FILE_PROPERTY_MAXIMUM_PACKET_SIZE, AUDIO_FILE_READ_PERMISSION,
};
use std::{ffi::c_void, mem::MaybeUninit, path::Path};

#[derive(Debug, Clone)]
/// Rust-owned packet payload returned by `AudioFileReadPacketData` and `AudioFileReadPackets`.
pub struct PacketData {
    pub data: Vec<u8>,
    pub packet_count: u32,
    pub packet_descriptions: Vec<AudioStreamPacketDescription>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Rust representation of `AudioFileGetPropertyInfo` results.
pub struct PropertyInfo {
    pub data_size: u32,
    pub writable: bool,
}

#[derive(Debug)]
/// Owning wrapper around an AudioToolbox.framework `AudioFileID`.
pub struct AudioFile {
    handle: *mut std::ffi::c_void,
    raw: AudioFileId,
}

impl AudioFile {
    /// Wraps `AudioFileOpenURL`.
    ///
    /// The returned wrapper owns the underlying AudioToolbox.framework handle and releases it on drop.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        Self::open_with_permissions(path, AUDIO_FILE_READ_PERMISSION, 0)
    }

    /// Wraps `AudioFileOpenURL`.
    ///
    /// The returned wrapper owns the underlying AudioToolbox.framework handle and releases it on drop.
    pub fn open_with_permissions(
        path: impl AsRef<Path>,
        permissions: AudioFilePermissions,
        file_type_hint: AudioFileTypeId,
    ) -> Result<Self> {
        let path = path_to_cstring(path.as_ref())?;
        let mut handle = std::ptr::null_mut();
        // SAFETY: Safe FFI call to AudioFileOpenURL with valid C string and output handle pointer.
        let status = unsafe {
            ffi::audio_file::at_audio_file_open(
                path.as_ptr(),
                permissions,
                file_type_hint,
                &mut handle,
            )
        };
        status_to_result("AudioFileOpenURL", status)?;
        // SAFETY: handle is valid after successful status check; cast to opaque type.
        let raw = ffi::audio_file::cast_audio_file_id(unsafe {
            ffi::audio_file::at_audio_file_raw(handle)
        });
        if raw.is_null() {
            return Err(AudioToolboxError::message(
                "AudioFileOpenURL",
                "framework returned a null AudioFileID",
            ));
        }
        Ok(Self { handle, raw })
    }

    /// Wraps `AudioFileCreateWithURL`.
    ///
    /// The returned wrapper owns the underlying AudioToolbox.framework handle and releases it on drop.
    pub fn create(
        path: impl AsRef<Path>,
        file_type: AudioFileTypeId,
        format: &AudioStreamBasicDescription,
        flags: u32,
    ) -> Result<Self> {
        let path = path_to_cstring(path.as_ref())?;
        let mut handle = std::ptr::null_mut();
        let status = unsafe {
            ffi::audio_file::at_audio_file_create(
                path.as_ptr(),
                file_type,
                format,
                flags,
                &mut handle,
            )
        };
        status_to_result("AudioFileCreateWithURL", status)?;
        let raw = ffi::audio_file::cast_audio_file_id(unsafe {
            ffi::audio_file::at_audio_file_raw(handle)
        });
        if raw.is_null() {
            return Err(AudioToolboxError::message(
                "AudioFileCreateWithURL",
                "framework returned a null AudioFileID",
            ));
        }
        Ok(Self { handle, raw })
    }

    /// Returns the wrapped `AudioFileID`.
    pub fn as_raw(&self) -> AudioFileId {
        self.raw
    }

    /// Wraps `AudioFileClose`.
    pub fn close(mut self) -> Result<()> {
        self.release();
        Ok(())
    }

    /// Wraps `AudioFileGetPropertyInfo`.
    pub fn property_info(&self, property_id: AudioFilePropertyId) -> Result<PropertyInfo> {
        let mut data_size = 0_u32;
        let mut writable = 0_u32;
        let status = unsafe {
            ffi::audio_file::at_audio_file_get_property_info(
                self.raw.cast(),
                property_id,
                &mut data_size,
                &mut writable,
            )
        };
        status_to_result("AudioFileGetPropertyInfo", status)?;
        Ok(PropertyInfo {
            data_size,
            writable: writable != 0,
        })
    }

    /// Wraps `AudioFileGetProperty`.
    pub fn data_format(&self) -> Result<AudioStreamBasicDescription> {
        self.get_property_typed(
            AUDIO_FILE_PROPERTY_DATA_FORMAT,
            "AudioFileGetProperty(data format)",
        )
    }

    /// Wraps `AudioFileGetProperty`.
    pub fn magic_cookie(&self) -> Result<Vec<u8>> {
        self.get_property_bytes(
            AUDIO_FILE_PROPERTY_MAGIC_COOKIE_DATA,
            "AudioFileGetProperty(magic cookie)",
        )
    }

    /// Wraps `AudioFileGetProperty`.
    pub fn packet_count(&self) -> Result<i64> {
        self.get_property_typed(
            AUDIO_FILE_PROPERTY_AUDIO_DATA_PACKET_COUNT,
            "AudioFileGetProperty(packet count)",
        )
    }

    /// Wraps `AudioFileGetProperty`.
    pub fn maximum_packet_size(&self) -> Result<u32> {
        self.get_property_typed(
            AUDIO_FILE_PROPERTY_MAXIMUM_PACKET_SIZE,
            "AudioFileGetProperty(maximum packet size)",
        )
    }

    /// Wraps `AudioFileGetProperty`.
    pub fn estimated_duration(&self) -> Result<f64> {
        self.get_property_typed(
            AUDIO_FILE_PROPERTY_ESTIMATED_DURATION,
            "AudioFileGetProperty(estimated duration)",
        )
    }

    /// Wraps `AudioFileGetProperty`.
    pub fn audio_data_byte_count(&self) -> Result<i64> {
        self.get_property_typed(
            AUDIO_FILE_PROPERTY_AUDIO_DATA_BYTE_COUNT,
            "AudioFileGetProperty(audio data byte count)",
        )
    }

    /// Wraps `AudioFileGetProperty`.
    pub fn data_offset(&self) -> Result<i64> {
        self.get_property_typed(
            AUDIO_FILE_PROPERTY_DATA_OFFSET,
            "AudioFileGetProperty(data offset)",
        )
    }

    /// Wraps `AudioFileOptimize`.
    pub fn optimize(&self) -> Result<()> {
        let status = unsafe { ffi::audio_file::at_audio_file_optimize(self.raw) };
        status_to_result("AudioFileOptimize", status)
    }

    /// Wraps `AudioFileReadBytes`.
    pub fn read_bytes(
        &self,
        starting_byte: i64,
        byte_count: u32,
        use_cache: bool,
    ) -> Result<Vec<u8>> {
        let mut bytes = vec![0_u8; byte_count as usize];
        let mut actual_byte_count = byte_count;
        let status = unsafe {
            ffi::audio_file::at_audio_file_read_bytes(
                self.raw,
                u8::from(use_cache),
                starting_byte,
                &mut actual_byte_count,
                bytes.as_mut_ptr().cast(),
            )
        };
        status_to_result("AudioFileReadBytes", status)?;
        bytes.truncate(actual_byte_count as usize);
        Ok(bytes)
    }

    /// Wraps `AudioFileWriteBytes`.
    pub fn write_bytes(&self, starting_byte: i64, data: &[u8], use_cache: bool) -> Result<u32> {
        let mut actual_byte_count = u32::try_from(data.len()).map_err(|_| {
            AudioToolboxError::message(
                "AudioFileWriteBytes",
                "byte payload exceeds UInt32::MAX bytes",
            )
        })?;
        let status = unsafe {
            ffi::audio_file::at_audio_file_write_bytes(
                self.raw,
                u8::from(use_cache),
                starting_byte,
                &mut actual_byte_count,
                data.as_ptr().cast(),
            )
        };
        status_to_result("AudioFileWriteBytes", status)?;
        Ok(actual_byte_count)
    }

    /// Wraps `AudioFileCountUserData`.
    pub fn count_user_data(&self, user_data_id: u32) -> Result<u32> {
        let mut count = 0_u32;
        let status = unsafe {
            ffi::audio_file::at_audio_file_count_user_data(self.raw, user_data_id, &mut count)
        };
        status_to_result("AudioFileCountUserData", status)?;
        Ok(count)
    }

    /// Wraps `AudioFileGetUserDataSize`.
    pub fn user_data_size(&self, user_data_id: u32, index: u32) -> Result<u32> {
        let mut size = 0_u32;
        let status = unsafe {
            ffi::audio_file::at_audio_file_get_user_data_size(
                self.raw,
                user_data_id,
                index,
                &mut size,
            )
        };
        status_to_result("AudioFileGetUserDataSize", status)?;
        Ok(size)
    }

    /// Wraps `AudioFileGetUserDataSize64`.
    pub fn user_data_size64(&self, user_data_id: u32, index: u32) -> Result<u64> {
        let mut size = 0_u64;
        let status = unsafe {
            ffi::audio_file::at_audio_file_get_user_data_size64(
                self.raw,
                user_data_id,
                index,
                &mut size,
            )
        };
        status_to_result("AudioFileGetUserDataSize64", status)?;
        Ok(size)
    }

    /// Wraps `AudioFileGetUserData`.
    pub fn user_data(&self, user_data_id: u32, index: u32) -> Result<Vec<u8>> {
        let mut size = self.user_data_size(user_data_id, index)?;
        let mut bytes = vec![0_u8; size as usize];
        let status = unsafe {
            ffi::audio_file::at_audio_file_get_user_data(
                self.raw,
                user_data_id,
                index,
                &mut size,
                bytes.as_mut_ptr().cast(),
            )
        };
        status_to_result("AudioFileGetUserData", status)?;
        bytes.truncate(size as usize);
        Ok(bytes)
    }

    /// Wraps `AudioFileGetUserDataAtOffset`.
    pub fn user_data_at_offset(
        &self,
        user_data_id: u32,
        index: u32,
        offset: i64,
        byte_count: u32,
    ) -> Result<Vec<u8>> {
        let mut size = byte_count;
        let mut bytes = vec![0_u8; size as usize];
        let status = unsafe {
            ffi::audio_file::at_audio_file_get_user_data_at_offset(
                self.raw,
                user_data_id,
                index,
                offset,
                &mut size,
                bytes.as_mut_ptr().cast(),
            )
        };
        status_to_result("AudioFileGetUserDataAtOffset", status)?;
        bytes.truncate(size as usize);
        Ok(bytes)
    }

    /// Wraps `AudioFileSetUserData`.
    pub fn set_user_data(&self, user_data_id: u32, index: u32, data: &[u8]) -> Result<()> {
        let size = u32::try_from(data.len()).map_err(|_| {
            AudioToolboxError::message(
                "AudioFileSetUserData",
                "user data payload exceeds UInt32::MAX bytes",
            )
        })?;
        let status = unsafe {
            ffi::audio_file::at_audio_file_set_user_data(
                self.raw,
                user_data_id,
                index,
                size,
                data.as_ptr().cast(),
            )
        };
        status_to_result("AudioFileSetUserData", status)
    }

    /// Wraps `AudioFileRemoveUserData`.
    pub fn remove_user_data(&self, user_data_id: u32, index: u32) -> Result<()> {
        let status = unsafe {
            ffi::audio_file::at_audio_file_remove_user_data(self.raw, user_data_id, index)
        };
        status_to_result("AudioFileRemoveUserData", status)
    }

    /// Wraps `AudioFileGetGlobalInfoSize`.
    pub fn global_info_size<T>(
        property_id: AudioFilePropertyId,
        specifier: Option<&T>,
    ) -> Result<u32> {
        let (specifier_ptr, specifier_size) =
            specifier.map_or((std::ptr::null(), 0), |specifier| {
                (
                    std::ptr::from_ref(specifier).cast::<c_void>(),
                    u32::try_from(std::mem::size_of::<T>()).expect("specifier fits in u32"),
                )
            });
        let mut size = 0_u32;
        let status = unsafe {
            ffi::audio_file::at_audio_file_get_global_info_size(
                property_id,
                specifier_size,
                specifier_ptr,
                &mut size,
            )
        };
        status_to_result("AudioFileGetGlobalInfoSize", status)?;
        Ok(size)
    }

    /// Wraps `AudioFileGetGlobalInfo`.
    pub fn global_info_bytes<T>(
        property_id: AudioFilePropertyId,
        specifier: Option<&T>,
    ) -> Result<Vec<u8>> {
        let (specifier_ptr, specifier_size) =
            specifier.map_or((std::ptr::null(), 0), |specifier| {
                (
                    std::ptr::from_ref(specifier).cast::<c_void>(),
                    u32::try_from(std::mem::size_of::<T>()).expect("specifier fits in u32"),
                )
            });
        let mut size = Self::global_info_size(property_id, specifier)?;
        let mut bytes = vec![0_u8; size as usize];
        let status = unsafe {
            ffi::audio_file::at_audio_file_get_global_info(
                property_id,
                specifier_size,
                specifier_ptr,
                &mut size,
                bytes.as_mut_ptr().cast(),
            )
        };
        status_to_result("AudioFileGetGlobalInfo", status)?;
        bytes.truncate(size as usize);
        Ok(bytes)
    }

    /// Wraps `AudioFileGetProperty`.
    pub fn get_property_bytes(
        &self,
        property_id: AudioFilePropertyId,
        operation: &'static str,
    ) -> Result<Vec<u8>> {
        let info = self.property_info(property_id)?;
        let mut bytes = vec![0_u8; info.data_size as usize];
        let mut size = info.data_size;
        let status = unsafe {
            ffi::audio_file::at_audio_file_get_property(
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

    /// Wraps `AudioFileSetProperty`.
    pub fn set_property_bytes(
        &self,
        property_id: AudioFilePropertyId,
        bytes: &[u8],
        operation: &'static str,
    ) -> Result<()> {
        let length = u32::try_from(bytes.len()).map_err(|_| {
            AudioToolboxError::message(operation, "property payload exceeds UInt32::MAX bytes")
        })?;
        let status = unsafe {
            ffi::audio_file::at_audio_file_set_property(
                self.raw.cast(),
                property_id,
                length,
                bytes.as_ptr().cast(),
            )
        };
        status_to_result(operation, status)
    }

    /// Wraps `AudioFileGetPropertyArray`.
    pub fn get_property_array<T: Copy>(
        &self,
        property_id: AudioFilePropertyId,
        operation: &'static str,
    ) -> Result<Vec<T>> {
        let bytes = self.get_property_bytes(property_id, operation)?;
        let element_size = std::mem::size_of::<T>();
        if element_size == 0 || bytes.len() % element_size != 0 {
            return Err(AudioToolboxError::message(
                operation,
                "property payload is not an integral number of elements",
            ));
        }
        let (prefix, values, suffix) = unsafe { bytes.align_to::<T>() };
        if !prefix.is_empty() || !suffix.is_empty() {
            return Err(AudioToolboxError::message(
                operation,
                "property payload is not aligned for the requested element type",
            ));
        }
        Ok(values.to_vec())
    }

    /// Wraps `AudioFileSetProperty`.
    pub fn set_property_typed<T: Copy>(
        &self,
        property_id: AudioFilePropertyId,
        value: &T,
        operation: &'static str,
    ) -> Result<()> {
        let size = u32::try_from(std::mem::size_of::<T>()).expect("typed property fits in u32");
        let status = unsafe {
            ffi::audio_file::at_audio_file_set_property(
                self.raw.cast(),
                property_id,
                size,
                std::ptr::from_ref(value).cast(),
            )
        };
        status_to_result(operation, status)
    }

    /// Wraps `AudioFileReadPacketData`.
    pub fn read_packet_data(
        &self,
        starting_packet: i64,
        packet_count: u32,
        use_cache: bool,
    ) -> Result<PacketData> {
        self.read_packet_data_inner(starting_packet, packet_count, use_cache, true)
    }

    /// Wraps `AudioFileReadPackets`.
    pub fn read_packets(
        &self,
        starting_packet: i64,
        packet_count: u32,
        use_cache: bool,
    ) -> Result<PacketData> {
        self.read_packet_data_inner(starting_packet, packet_count, use_cache, false)
    }

    /// Wraps `AudioFileWritePackets`.
    pub fn write_packets(
        &self,
        starting_packet: i64,
        data: &[u8],
        packet_count: u32,
        packet_descriptions: Option<&[AudioStreamPacketDescription]>,
        use_cache: bool,
    ) -> Result<u32> {
        let in_num_bytes = u32::try_from(data.len()).map_err(|_| {
            AudioToolboxError::message(
                "AudioFileWritePackets",
                "packet payload exceeds UInt32::MAX bytes",
            )
        })?;
        let mut io_num_packets = packet_count;
        let packet_description_ptr =
            packet_descriptions.map_or(std::ptr::null(), <[AudioStreamPacketDescription]>::as_ptr);
        let status = unsafe {
            ffi::audio_file::at_audio_file_write_packets(
                self.raw.cast(),
                use_cache,
                in_num_bytes,
                packet_description_ptr,
                starting_packet,
                &mut io_num_packets,
                data.as_ptr().cast(),
            )
        };
        status_to_result("AudioFileWritePackets", status)?;
        Ok(io_num_packets)
    }

    /// Wraps `AudioFileGetProperty`.
    pub fn get_property_typed<T: Copy>(
        &self,
        property_id: AudioFilePropertyId,
        operation: &'static str,
    ) -> Result<T> {
        let mut value = MaybeUninit::<T>::uninit();
        let mut size = u32::try_from(std::mem::size_of::<T>()).expect("typed property fits in u32");
        let status = unsafe {
            ffi::audio_file::at_audio_file_get_property(
                self.raw.cast(),
                property_id,
                &mut size,
                value.as_mut_ptr().cast(),
            )
        };
        status_to_result(operation, status)?;
        Ok(unsafe { value.assume_init() })
    }

    fn read_packet_data_inner(
        &self,
        starting_packet: i64,
        packet_count: u32,
        use_cache: bool,
        use_modern_api: bool,
    ) -> Result<PacketData> {
        if packet_count == 0 {
            return Ok(PacketData {
                data: Vec::new(),
                packet_count: 0,
                packet_descriptions: Vec::new(),
            });
        }

        let format = self.data_format()?;
        let max_packet_size = self
            .maximum_packet_size()
            .unwrap_or_else(|_| format.mBytesPerPacket.max(1));
        let mut bytes =
            vec![0_u8; (max_packet_size as usize).saturating_mul(packet_count as usize)];
        let mut packet_descriptions =
            vec![AudioStreamPacketDescription::default(); packet_count as usize];
        let mut io_num_bytes = u32::try_from(bytes.len()).map_err(|_| {
            AudioToolboxError::message(
                if use_modern_api {
                    "AudioFileReadPacketData"
                } else {
                    "AudioFileReadPackets"
                },
                "requested packet buffer exceeds UInt32::MAX bytes",
            )
        })?;
        let mut io_num_packets = packet_count;
        let packet_description_ptr = if format.uses_packet_descriptions() {
            packet_descriptions.as_mut_ptr()
        } else {
            std::ptr::null_mut()
        };

        let status = unsafe {
            if use_modern_api {
                ffi::audio_file::at_audio_file_read_packet_data(
                    self.raw.cast(),
                    use_cache,
                    &mut io_num_bytes,
                    packet_description_ptr,
                    starting_packet,
                    &mut io_num_packets,
                    bytes.as_mut_ptr().cast(),
                )
            } else {
                ffi::audio_file::at_audio_file_read_packets(
                    self.raw.cast(),
                    use_cache,
                    &mut io_num_bytes,
                    packet_description_ptr,
                    starting_packet,
                    &mut io_num_packets,
                    bytes.as_mut_ptr().cast(),
                )
            }
        };
        status_to_result(
            if use_modern_api {
                "AudioFileReadPacketData"
            } else {
                "AudioFileReadPackets"
            },
            status,
        )?;

        bytes.truncate(io_num_bytes as usize);
        if format.uses_packet_descriptions() {
            packet_descriptions.truncate(io_num_packets as usize);
        } else {
            packet_descriptions.clear();
        }

        Ok(PacketData {
            data: bytes,
            packet_count: io_num_packets,
            packet_descriptions,
        })
    }

    fn release(&mut self) {
        if !self.handle.is_null() {
            unsafe { ffi::audio_file::at_audio_file_release(self.handle) };
            self.handle = std::ptr::null_mut();
            self.raw = std::ptr::null_mut();
        }
    }
}

impl Drop for AudioFile {
    fn drop(&mut self) {
        self.release();
    }
}
