use crate::{
    cf, ffi, AudioStreamBasicDescription, AudioStreamPacketDescription, AudioToolboxError, Result,
};
use std::{mem::MaybeUninit, path::Path};

#[derive(Debug, Clone)]
pub struct PacketData {
    pub data: Vec<u8>,
    pub packet_count: u32,
    pub packet_descriptions: Vec<AudioStreamPacketDescription>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PropertyInfo {
    pub data_size: u32,
    pub writable: bool,
}

#[derive(Debug)]
pub struct AudioFile {
    raw: ffi::AudioFileId,
}

impl AudioFile {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        Self::open_with_permissions(path, ffi::AUDIO_FILE_READ_PERMISSION, 0)
    }

    pub fn open_with_permissions(
        path: impl AsRef<Path>,
        permissions: ffi::AudioFilePermissions,
        file_type_hint: ffi::AudioFileTypeId,
    ) -> Result<Self> {
        let url = cf::path_to_url(path.as_ref())?;
        let mut audio_file = MaybeUninit::uninit();
        let status = unsafe {
            ffi::AudioFileOpenURL(
                url.as_ptr(),
                permissions,
                file_type_hint,
                audio_file.as_mut_ptr(),
            )
        };
        status_to_result("AudioFileOpenURL", status)?;
        Ok(Self {
            raw: unsafe { audio_file.assume_init() },
        })
    }

    pub fn create(
        path: impl AsRef<Path>,
        file_type: ffi::AudioFileTypeId,
        format: &AudioStreamBasicDescription,
        flags: ffi::AudioFileFlags,
    ) -> Result<Self> {
        let url = cf::path_to_url(path.as_ref())?;
        let mut audio_file = MaybeUninit::uninit();
        let status = unsafe {
            ffi::AudioFileCreateWithURL(
                url.as_ptr(),
                file_type,
                format,
                flags,
                audio_file.as_mut_ptr(),
            )
        };
        status_to_result("AudioFileCreateWithURL", status)?;
        Ok(Self {
            raw: unsafe { audio_file.assume_init() },
        })
    }

    pub fn as_raw(&self) -> ffi::AudioFileId {
        self.raw
    }

    pub fn close(mut self) -> Result<()> {
        let raw = self.raw;
        self.raw = std::ptr::null_mut();
        let status = unsafe { ffi::AudioFileClose(raw) };
        status_to_result("AudioFileClose", status)
    }

    pub fn property_info(&self, property_id: ffi::AudioFilePropertyId) -> Result<PropertyInfo> {
        let mut data_size = 0_u32;
        let mut writable = 0_u32;
        let status = unsafe {
            ffi::AudioFileGetPropertyInfo(self.raw, property_id, &mut data_size, &mut writable)
        };
        status_to_result("AudioFileGetPropertyInfo", status)?;
        Ok(PropertyInfo {
            data_size,
            writable: writable != 0,
        })
    }

    pub fn data_format(&self) -> Result<AudioStreamBasicDescription> {
        self.get_property_typed(
            ffi::AUDIO_FILE_PROPERTY_DATA_FORMAT,
            "AudioFileGetProperty(data format)",
        )
    }

    pub fn magic_cookie(&self) -> Result<Vec<u8>> {
        self.get_property_bytes(
            ffi::AUDIO_FILE_PROPERTY_MAGIC_COOKIE_DATA,
            "AudioFileGetProperty(magic cookie)",
        )
    }

    pub fn packet_count(&self) -> Result<i64> {
        self.get_property_typed(
            ffi::AUDIO_FILE_PROPERTY_AUDIO_DATA_PACKET_COUNT,
            "AudioFileGetProperty(packet count)",
        )
    }

    pub fn maximum_packet_size(&self) -> Result<u32> {
        self.get_property_typed(
            ffi::AUDIO_FILE_PROPERTY_MAXIMUM_PACKET_SIZE,
            "AudioFileGetProperty(maximum packet size)",
        )
    }

    pub fn estimated_duration(&self) -> Result<f64> {
        self.get_property_typed(
            ffi::AUDIO_FILE_PROPERTY_ESTIMATED_DURATION,
            "AudioFileGetProperty(estimated duration)",
        )
    }

    pub fn get_property_bytes(
        &self,
        property_id: ffi::AudioFilePropertyId,
        operation: &'static str,
    ) -> Result<Vec<u8>> {
        let info = self.property_info(property_id)?;
        let mut bytes = vec![0_u8; info.data_size as usize];
        let mut size = info.data_size;
        let status = unsafe {
            ffi::AudioFileGetProperty(self.raw, property_id, &mut size, bytes.as_mut_ptr().cast())
        };
        status_to_result(operation, status)?;
        bytes.truncate(size as usize);
        Ok(bytes)
    }

    pub fn set_property_bytes(
        &self,
        property_id: ffi::AudioFilePropertyId,
        bytes: &[u8],
        operation: &'static str,
    ) -> Result<()> {
        let length = u32::try_from(bytes.len()).map_err(|_| {
            AudioToolboxError::message(operation, "property payload exceeds UInt32::MAX bytes")
        })?;
        let status = unsafe {
            ffi::AudioFileSetProperty(self.raw, property_id, length, bytes.as_ptr().cast())
        };
        status_to_result(operation, status)
    }

    pub fn read_packet_data(
        &self,
        starting_packet: i64,
        packet_count: u32,
        use_cache: bool,
    ) -> Result<PacketData> {
        self.read_packet_data_inner(starting_packet, packet_count, use_cache, true)
    }

    pub fn read_packets(
        &self,
        starting_packet: i64,
        packet_count: u32,
        use_cache: bool,
    ) -> Result<PacketData> {
        self.read_packet_data_inner(starting_packet, packet_count, use_cache, false)
    }

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
            ffi::AudioFileWritePackets(
                self.raw,
                u8::from(use_cache),
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

    fn get_property_typed<T: Copy>(
        &self,
        property_id: ffi::AudioFilePropertyId,
        operation: &'static str,
    ) -> Result<T> {
        let mut value = MaybeUninit::<T>::uninit();
        let mut size = u32::try_from(std::mem::size_of::<T>()).expect("typed property fits in u32");
        let status = unsafe {
            ffi::AudioFileGetProperty(self.raw, property_id, &mut size, value.as_mut_ptr().cast())
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
                "AudioFileReadPacketData",
                "read buffer exceeds UInt32::MAX bytes",
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
                ffi::AudioFileReadPacketData(
                    self.raw,
                    u8::from(use_cache),
                    &mut io_num_bytes,
                    packet_description_ptr,
                    starting_packet,
                    &mut io_num_packets,
                    bytes.as_mut_ptr().cast(),
                )
            } else {
                ffi::AudioFileReadPackets(
                    self.raw,
                    u8::from(use_cache),
                    &mut io_num_bytes,
                    packet_description_ptr,
                    starting_packet,
                    &mut io_num_packets,
                    bytes.as_mut_ptr().cast(),
                )
            }
        };

        if status != ffi::NO_ERR && status != ffi::AUDIO_FILE_END_OF_FILE_ERROR {
            status_to_result(
                if use_modern_api {
                    "AudioFileReadPacketData"
                } else {
                    "AudioFileReadPackets"
                },
                status,
            )?;
        }

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
}

impl Drop for AudioFile {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            let _ = unsafe { ffi::AudioFileClose(self.raw) };
        }
    }
}

fn status_to_result(operation: &'static str, status: ffi::OSStatus) -> Result<()> {
    if status == ffi::NO_ERR {
        Ok(())
    } else {
        Err(AudioToolboxError::from_status(operation, status))
    }
}
