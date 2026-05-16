use crate::{
    ffi,
    internal::{path_to_cstring, status_to_result},
    AudioStreamBasicDescription,
    AudioToolboxError,
    BorrowedAudioConverter,
    ExtAudioFileRef,
    Result,
    AUDIO_FILE_FLAGS_ERASE_FILE,
    EXT_AUDIO_FILE_PROPERTY_CLIENT_DATA_FORMAT,
    EXT_AUDIO_FILE_PROPERTY_FILE_DATA_FORMAT,
    EXT_AUDIO_FILE_PROPERTY_FILE_LENGTH_FRAMES,
};
use std::{mem::MaybeUninit, path::Path};

#[derive(Debug)]
pub struct InterleavedAudioBuffer {
    channels: u32,
    bytes_per_frame: u32,
    storage: Vec<u8>,
    raw: crate::AudioBufferList1,
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
        let raw = crate::AudioBufferList {
            mNumberBuffers: 1,
            mBuffers: [crate::AudioBuffer {
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

    fn raw_mut_ptr(&mut self) -> *mut crate::AudioBufferList1 {
        self.raw.mBuffers[0].mData = self.storage.as_mut_ptr().cast();
        self.raw.mBuffers[0].mDataByteSize = self.storage.len() as u32;
        &mut self.raw
    }

    fn raw_ptr(&self) -> *const crate::AudioBufferList1 {
        &self.raw
    }
}

#[derive(Debug)]
pub struct ExtAudioFile {
    handle: *mut std::ffi::c_void,
    raw: ExtAudioFileRef,
}

impl ExtAudioFile {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path_to_cstring(path.as_ref())?;
        let mut handle = std::ptr::null_mut();
        let status = unsafe { ffi::ext_audio_file::at_ext_audio_file_open(path.as_ptr(), &mut handle) };
        status_to_result("ExtAudioFileOpenURL", status)?;
        let raw: ExtAudioFileRef =
            unsafe { ffi::ext_audio_file::at_ext_audio_file_raw(handle) }.cast();
        if raw.is_null() {
            return Err(AudioToolboxError::message(
                "ExtAudioFileOpenURL",
                "framework returned a null ExtAudioFileRef",
            ));
        }
        Ok(Self { handle, raw })
    }

    pub fn create(
        path: impl AsRef<Path>,
        file_type: crate::AudioFileTypeId,
        format: &AudioStreamBasicDescription,
        flags: u32,
    ) -> Result<Self> {
        let path = path_to_cstring(path.as_ref())?;
        let mut handle = std::ptr::null_mut();
        let status = unsafe {
            ffi::ext_audio_file::at_ext_audio_file_create(
                path.as_ptr(),
                file_type,
                format,
                flags,
                &mut handle,
            )
        };
        status_to_result("ExtAudioFileCreateWithURL", status)?;
        let raw: ExtAudioFileRef =
            unsafe { ffi::ext_audio_file::at_ext_audio_file_raw(handle) }.cast();
        if raw.is_null() {
            return Err(AudioToolboxError::message(
                "ExtAudioFileCreateWithURL",
                "framework returned a null ExtAudioFileRef",
            ));
        }
        Ok(Self { handle, raw })
    }

    pub fn create_erasing(
        path: impl AsRef<Path>,
        file_type: crate::AudioFileTypeId,
        format: &AudioStreamBasicDescription,
    ) -> Result<Self> {
        Self::create(path, file_type, format, AUDIO_FILE_FLAGS_ERASE_FILE)
    }

    pub fn as_raw(&self) -> ExtAudioFileRef {
        self.raw
    }

    pub fn close(mut self) -> Result<()> {
        self.release();
        Ok(())
    }

    pub fn file_data_format(&self) -> Result<AudioStreamBasicDescription> {
        self.get_property_typed(
            EXT_AUDIO_FILE_PROPERTY_FILE_DATA_FORMAT,
            "ExtAudioFileGetProperty(file data format)",
        )
    }

    pub fn client_data_format(&self) -> Result<AudioStreamBasicDescription> {
        self.get_property_typed(
            EXT_AUDIO_FILE_PROPERTY_CLIENT_DATA_FORMAT,
            "ExtAudioFileGetProperty(client data format)",
        )
    }

    pub fn set_client_data_format(&self, format: &AudioStreamBasicDescription) -> Result<()> {
        self.set_property_typed(
            EXT_AUDIO_FILE_PROPERTY_CLIENT_DATA_FORMAT,
            format,
            "ExtAudioFileSetProperty(client data format)",
        )
    }

    pub fn file_length_frames(&self) -> Result<i64> {
        self.get_property_typed(
            EXT_AUDIO_FILE_PROPERTY_FILE_LENGTH_FRAMES,
            "ExtAudioFileGetProperty(file length frames)",
        )
    }

    pub fn audio_converter(&self) -> Result<BorrowedAudioConverter<'_>> {
        let mut handle = std::ptr::null_mut();
        let status = unsafe {
            ffi::ext_audio_file::at_ext_audio_file_copy_audio_converter(self.raw.cast(), &mut handle)
        };
        status_to_result(
            "ExtAudioFileGetProperty(audio converter)",
            status,
        )?;
        let raw = ffi::ext_audio_file::cast_audio_converter_ref(unsafe {
            ffi::ext_audio_file::at_borrowed_audio_converter_raw(handle)
        });
        unsafe { ffi::ext_audio_file::at_borrowed_audio_converter_release(handle) };
        Ok(BorrowedAudioConverter::new(raw))
    }

    pub fn read_interleaved(
        &self,
        buffer: &mut InterleavedAudioBuffer,
        frames: u32,
    ) -> Result<u32> {
        let mut io_number_frames = frames.min(buffer.frame_capacity());
        let raw = buffer.raw_mut_ptr();
        let status = unsafe {
            ffi::ext_audio_file::at_ext_audio_file_read(self.raw.cast(), &mut io_number_frames, raw)
        };
        status_to_result("ExtAudioFileRead", status)?;
        Ok(io_number_frames)
    }

    pub fn write_interleaved(&self, frames: u32, buffer: &InterleavedAudioBuffer) -> Result<()> {
        let status = unsafe {
            ffi::ext_audio_file::at_ext_audio_file_write(self.raw.cast(), frames, buffer.raw_ptr())
        };
        status_to_result("ExtAudioFileWrite", status)
    }

    fn get_property_typed<T: Copy>(
        &self,
        property_id: crate::ExtAudioFilePropertyId,
        operation: &'static str,
    ) -> Result<T> {
        let mut value = MaybeUninit::<T>::uninit();
        let mut size = u32::try_from(std::mem::size_of::<T>()).expect("typed property fits in u32");
        let status = unsafe {
            ffi::ext_audio_file::at_ext_audio_file_get_property(
                self.raw.cast(),
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
        property_id: crate::ExtAudioFilePropertyId,
        value: &T,
        operation: &'static str,
    ) -> Result<()> {
        let size = u32::try_from(std::mem::size_of::<T>()).expect("typed property fits in u32");
        let status = unsafe {
            ffi::ext_audio_file::at_ext_audio_file_set_property(
                self.raw.cast(),
                property_id,
                size,
                std::ptr::from_ref(value).cast(),
            )
        };
        status_to_result(operation, status)
    }

    fn release(&mut self) {
        if !self.handle.is_null() {
            unsafe { ffi::ext_audio_file::at_ext_audio_file_release(self.handle) };
            self.handle = std::ptr::null_mut();
            self.raw = std::ptr::null_mut();
        }
    }
}

impl Drop for ExtAudioFile {
    fn drop(&mut self) {
        self.release();
    }
}
