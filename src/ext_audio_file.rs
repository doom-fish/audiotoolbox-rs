use crate::{
    audio_converter::BorrowedAudioConverter, cf, ffi, AudioStreamBasicDescription,
    AudioToolboxError, Result,
};
use std::{mem::MaybeUninit, path::Path};

#[derive(Debug)]
pub struct InterleavedAudioBuffer {
    channels: u32,
    bytes_per_frame: u32,
    storage: Vec<u8>,
    raw: ffi::AudioBufferList1,
}

impl InterleavedAudioBuffer {
    pub fn new(channels: u32, bytes_per_frame: u32, frame_capacity: u32) -> Result<Self> {
        let byte_capacity = (bytes_per_frame as usize)
            .checked_mul(frame_capacity as usize)
            .ok_or_else(|| {
                AudioToolboxError::message(
                    "InterleavedAudioBuffer::new",
                    "buffer size overflowed usize",
                )
            })?;
        let mut storage = vec![0_u8; byte_capacity];
        let raw = ffi::AudioBufferList {
            mNumberBuffers: 1,
            mBuffers: [ffi::AudioBuffer {
                mNumberChannels: channels,
                mDataByteSize: u32::try_from(byte_capacity).map_err(|_| {
                    AudioToolboxError::message(
                        "InterleavedAudioBuffer::new",
                        "buffer size exceeds UInt32::MAX bytes",
                    )
                })?,
                mData: storage.as_mut_ptr().cast(),
            }],
        };
        Ok(Self {
            channels,
            bytes_per_frame,
            storage,
            raw,
        })
    }

    pub fn channels(&self) -> u32 {
        self.channels
    }

    pub fn bytes_per_frame(&self) -> u32 {
        self.bytes_per_frame
    }

    pub fn frame_capacity(&self) -> u32 {
        (self.storage.len() as u32) / self.bytes_per_frame.max(1)
    }

    pub fn valid_byte_size(&self) -> u32 {
        self.raw.mBuffers[0].mDataByteSize
    }

    pub fn valid_frames(&self) -> u32 {
        self.valid_byte_size() / self.bytes_per_frame.max(1)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.storage[..self.valid_byte_size() as usize]
    }

    pub fn as_mut_bytes(&mut self) -> &mut [u8] {
        let len = self.valid_byte_size() as usize;
        &mut self.storage[..len]
    }

    pub fn set_valid_byte_size(&mut self, byte_size: u32) {
        self.raw.mBuffers[0].mDataByteSize = byte_size.min(self.storage.len() as u32);
    }

    fn raw_mut_ptr(&mut self) -> *mut ffi::AudioBufferList1 {
        self.raw.mBuffers[0].mData = self.storage.as_mut_ptr().cast();
        self.raw.mBuffers[0].mDataByteSize = self.storage.len() as u32;
        &mut self.raw
    }

    fn raw_ptr(&self) -> *const ffi::AudioBufferList1 {
        &self.raw
    }
}

#[derive(Debug)]
pub struct ExtAudioFile {
    raw: ffi::ExtAudioFileRef,
}

impl ExtAudioFile {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let url = cf::path_to_url(path.as_ref())?;
        let mut ext_audio_file = MaybeUninit::uninit();
        let status = unsafe { ffi::ExtAudioFileOpenURL(url.as_ptr(), ext_audio_file.as_mut_ptr()) };
        status_to_result("ExtAudioFileOpenURL", status)?;
        Ok(Self {
            raw: unsafe { ext_audio_file.assume_init() },
        })
    }

    pub fn create(
        path: impl AsRef<Path>,
        file_type: ffi::AudioFileTypeId,
        format: &AudioStreamBasicDescription,
        flags: ffi::AudioFileFlags,
    ) -> Result<Self> {
        let url = cf::path_to_url(path.as_ref())?;
        let mut ext_audio_file = MaybeUninit::uninit();
        let status = unsafe {
            ffi::ExtAudioFileCreateWithURL(
                url.as_ptr(),
                file_type,
                format,
                std::ptr::null(),
                flags,
                ext_audio_file.as_mut_ptr(),
            )
        };
        status_to_result("ExtAudioFileCreateWithURL", status)?;
        Ok(Self {
            raw: unsafe { ext_audio_file.assume_init() },
        })
    }

    pub fn as_raw(&self) -> ffi::ExtAudioFileRef {
        self.raw
    }

    pub fn close(mut self) -> Result<()> {
        let raw = self.raw;
        self.raw = std::ptr::null_mut();
        let status = unsafe { ffi::ExtAudioFileDispose(raw) };
        status_to_result("ExtAudioFileDispose", status)
    }

    pub fn file_data_format(&self) -> Result<AudioStreamBasicDescription> {
        self.get_property_typed(
            ffi::EXT_AUDIO_FILE_PROPERTY_FILE_DATA_FORMAT,
            "ExtAudioFileGetProperty(file data format)",
        )
    }

    pub fn client_data_format(&self) -> Result<AudioStreamBasicDescription> {
        self.get_property_typed(
            ffi::EXT_AUDIO_FILE_PROPERTY_CLIENT_DATA_FORMAT,
            "ExtAudioFileGetProperty(client data format)",
        )
    }

    pub fn set_client_data_format(&self, format: &AudioStreamBasicDescription) -> Result<()> {
        self.set_property_typed(
            ffi::EXT_AUDIO_FILE_PROPERTY_CLIENT_DATA_FORMAT,
            format,
            "ExtAudioFileSetProperty(client data format)",
        )
    }

    pub fn file_length_frames(&self) -> Result<i64> {
        self.get_property_typed(
            ffi::EXT_AUDIO_FILE_PROPERTY_FILE_LENGTH_FRAMES,
            "ExtAudioFileGetProperty(file length frames)",
        )
    }

    pub fn audio_converter(&self) -> Result<BorrowedAudioConverter<'_>> {
        let mut raw = MaybeUninit::<ffi::AudioConverterRef>::uninit();
        let mut size = u32::try_from(std::mem::size_of::<ffi::AudioConverterRef>())
            .expect("pointer size fits in u32");
        let status = unsafe {
            ffi::ExtAudioFileGetProperty(
                self.raw,
                ffi::EXT_AUDIO_FILE_PROPERTY_AUDIO_CONVERTER,
                &mut size,
                raw.as_mut_ptr().cast(),
            )
        };
        status_to_result("ExtAudioFileGetProperty(audio converter)", status)?;
        Ok(BorrowedAudioConverter::new(unsafe { raw.assume_init() }))
    }

    pub fn read_interleaved(
        &self,
        buffer: &mut InterleavedAudioBuffer,
        frames: u32,
    ) -> Result<u32> {
        let mut io_number_frames = frames.min(buffer.frame_capacity());
        let raw = buffer.raw_mut_ptr();
        let status = unsafe { ffi::ExtAudioFileRead(self.raw, &mut io_number_frames, raw) };
        status_to_result("ExtAudioFileRead", status)?;
        Ok(io_number_frames)
    }

    pub fn write_interleaved(&self, frames: u32, buffer: &InterleavedAudioBuffer) -> Result<()> {
        let status = unsafe { ffi::ExtAudioFileWrite(self.raw, frames, buffer.raw_ptr()) };
        status_to_result("ExtAudioFileWrite", status)
    }

    fn get_property_typed<T: Copy>(
        &self,
        property_id: ffi::ExtAudioFilePropertyId,
        operation: &'static str,
    ) -> Result<T> {
        let mut value = MaybeUninit::<T>::uninit();
        let mut size = u32::try_from(std::mem::size_of::<T>()).expect("typed property fits in u32");
        let status = unsafe {
            ffi::ExtAudioFileGetProperty(
                self.raw,
                property_id,
                &mut size,
                value.as_mut_ptr().cast(),
            )
        };
        status_to_result(operation, status)?;
        Ok(unsafe { value.assume_init() })
    }

    fn set_property_typed<T: Copy>(
        &self,
        property_id: ffi::ExtAudioFilePropertyId,
        value: &T,
        operation: &'static str,
    ) -> Result<()> {
        let size = u32::try_from(std::mem::size_of::<T>()).expect("typed property fits in u32");
        let status = unsafe {
            ffi::ExtAudioFileSetProperty(
                self.raw,
                property_id,
                size,
                std::ptr::from_ref(value).cast(),
            )
        };
        status_to_result(operation, status)
    }
}

impl Drop for ExtAudioFile {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            let _ = unsafe { ffi::ExtAudioFileDispose(self.raw) };
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
