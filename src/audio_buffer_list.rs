use std::ffi::c_void;

use crate::{
    AudioBuffer, AudioBufferList1, AudioStreamBasicDescription, AudioToolboxError, Result,
};

#[derive(Debug)]
struct OwnedBuffer {
    storage: Box<[u64]>,
    capacity: u32,
    data_byte_size: u32,
    number_channels: u32,
}

impl OwnedBuffer {
    fn new(number_channels: u32, capacity: u32) -> Self {
        let words = (capacity as usize).div_ceil(8);
        Self {
            storage: vec![0_u64; words].into_boxed_slice(),
            capacity,
            data_byte_size: capacity,
            number_channels,
        }
    }

    fn bytes(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.storage.as_ptr().cast(), self.capacity as usize) }
    }

    fn bytes_mut(&mut self) -> &mut [u8] {
        unsafe {
            std::slice::from_raw_parts_mut(self.storage.as_mut_ptr().cast(), self.capacity as usize)
        }
    }
}

#[derive(Debug)]
pub struct OwnedAudioBufferList {
    buffers: Vec<OwnedBuffer>,
}

pub(crate) struct RawAudioBufferList {
    words: Vec<u64>,
}

impl RawAudioBufferList {
    fn new(buffers: impl ExactSizeIterator<Item = AudioBuffer>) -> Self {
        let count = buffers.len();
        let mut words = vec![0_u64; 1 + 2 * count.max(1)];
        let base = words.as_mut_ptr();
        unsafe {
            base.cast::<u32>().write(count as u32);
            for (index, buffer) in buffers.enumerate() {
                base.add(1 + 2 * index).cast::<AudioBuffer>().write(buffer);
            }
        }
        Self { words }
    }

    pub(crate) fn as_mut_ptr(&mut self) -> *mut AudioBufferList1 {
        self.words.as_mut_ptr().cast()
    }

    pub(crate) fn as_ptr(&self) -> *const AudioBufferList1 {
        self.words.as_ptr().cast()
    }

    fn buffer(&self, index: usize) -> AudioBuffer {
        unsafe {
            self.words
                .as_ptr()
                .add(1 + 2 * index)
                .cast::<AudioBuffer>()
                .read()
        }
    }
}

impl OwnedAudioBufferList {
    #[allow(clippy::missing_errors_doc)]
    pub fn new(
        buffer_count: usize,
        channels_per_buffer: u32,
        bytes_per_buffer: u32,
    ) -> Result<Self> {
        if u32::try_from(buffer_count).is_err() {
            return Err(AudioToolboxError::message(
                "OwnedAudioBufferList::new",
                "buffer count exceeds UInt32::MAX",
            ));
        }
        Ok(Self {
            buffers: (0..buffer_count)
                .map(|_| OwnedBuffer::new(channels_per_buffer, bytes_per_buffer))
                .collect(),
        })
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn for_format(format: &AudioStreamBasicDescription, frames: u32) -> Result<Self> {
        let operation = "OwnedAudioBufferList::for_format";
        let (buffer_count, channels_per_buffer) = pcm_layout(operation, format)?;
        let bytes = frame_bytes(operation, format, frames)?;
        Self::new(buffer_count, channels_per_buffer, bytes)
    }

    pub fn buffer_count(&self) -> usize {
        self.buffers.len()
    }

    pub fn number_channels(&self, index: usize) -> Option<u32> {
        self.buffers.get(index).map(|buffer| buffer.number_channels)
    }

    pub fn capacity(&self, index: usize) -> Option<u32> {
        self.buffers.get(index).map(|buffer| buffer.capacity)
    }

    pub fn data_byte_size(&self, index: usize) -> Option<u32> {
        self.buffers.get(index).map(|buffer| buffer.data_byte_size)
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn set_data_byte_size(&mut self, index: usize, byte_size: u32) -> Result<()> {
        let buffer = self.buffers.get_mut(index).ok_or_else(|| {
            AudioToolboxError::message(
                "OwnedAudioBufferList::set_data_byte_size",
                format!("buffer index {index} is out of range"),
            )
        })?;
        if byte_size > buffer.capacity {
            return Err(AudioToolboxError::message(
                "OwnedAudioBufferList::set_data_byte_size",
                format!(
                    "{byte_size} bytes exceed the {}-byte buffer capacity",
                    buffer.capacity
                ),
            ));
        }
        buffer.data_byte_size = byte_size;
        Ok(())
    }

    pub fn data(&self, index: usize) -> Option<&[u8]> {
        self.buffers
            .get(index)
            .map(|buffer| &buffer.bytes()[..buffer.data_byte_size as usize])
    }

    pub fn data_mut(&mut self, index: usize) -> Option<&mut [u8]> {
        self.buffers.get_mut(index).map(OwnedBuffer::bytes_mut)
    }

    pub(crate) fn check_frames(
        &self,
        operation: &'static str,
        format: &AudioStreamBasicDescription,
        frames: u32,
    ) -> Result<u32> {
        let (buffer_count, _) = pcm_layout(operation, format)?;
        if self.buffers.len() != buffer_count {
            return Err(AudioToolboxError::message(
                operation,
                format!(
                    "the buffer list has {} buffers but the stream format needs {buffer_count}",
                    self.buffers.len()
                ),
            ));
        }
        let bytes = frame_bytes(operation, format, frames)?;
        if let Some(index) = self
            .buffers
            .iter()
            .position(|buffer| buffer.capacity < bytes)
        {
            return Err(AudioToolboxError::message(
                operation,
                format!(
                    "buffer {index} holds {} bytes but {frames} frames need {bytes}",
                    self.buffers[index].capacity
                ),
            ));
        }
        Ok(bytes)
    }

    pub(crate) fn check_valid_frames(
        &self,
        operation: &'static str,
        format: &AudioStreamBasicDescription,
        frames: u32,
    ) -> Result<()> {
        let bytes = self.check_frames(operation, format, frames)?;
        if let Some(index) = self
            .buffers
            .iter()
            .position(|buffer| buffer.data_byte_size < bytes)
        {
            return Err(AudioToolboxError::message(
                operation,
                format!(
                    "buffer {index} has {} valid bytes but {frames} frames need {bytes}",
                    self.buffers[index].data_byte_size
                ),
            ));
        }
        Ok(())
    }

    pub(crate) fn set_all_data_byte_sizes(&mut self, byte_size: u32) {
        for buffer in &mut self.buffers {
            buffer.data_byte_size = byte_size.min(buffer.capacity);
        }
    }

    pub(crate) fn raw_mut(&mut self) -> RawAudioBufferList {
        RawAudioBufferList::new(self.buffers.iter_mut().map(|buffer| AudioBuffer {
            mNumberChannels: buffer.number_channels,
            mDataByteSize: buffer.data_byte_size,
            mData: buffer.storage.as_mut_ptr().cast(),
        }))
    }

    pub(crate) fn raw_const(&self) -> RawAudioBufferList {
        RawAudioBufferList::new(self.buffers.iter().map(|buffer| AudioBuffer {
            mNumberChannels: buffer.number_channels,
            mDataByteSize: buffer.data_byte_size,
            mData: buffer.storage.as_ptr().cast_mut().cast::<c_void>(),
        }))
    }

    pub(crate) fn absorb(&mut self, raw: &RawAudioBufferList) {
        for (index, buffer) in self.buffers.iter_mut().enumerate() {
            let written = raw.buffer(index);
            let size = written.mDataByteSize.min(buffer.capacity);
            let own = buffer.storage.as_mut_ptr().cast::<c_void>();
            if !written.mData.is_null() && written.mData != own {
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        written.mData.cast::<u8>(),
                        own.cast::<u8>(),
                        size as usize,
                    );
                }
            }
            buffer.data_byte_size = size;
        }
    }
}

fn pcm_layout(
    operation: &'static str,
    format: &AudioStreamBasicDescription,
) -> Result<(usize, u32)> {
    if !format.is_linear_pcm() || format.mBytesPerFrame == 0 || format.mChannelsPerFrame == 0 {
        return Err(AudioToolboxError::message(
            operation,
            "the stream format must be linear PCM with a non-zero frame size",
        ));
    }
    Ok(if format.is_interleaved() {
        (1, format.mChannelsPerFrame)
    } else {
        (format.mChannelsPerFrame as usize, 1)
    })
}

fn frame_bytes(
    operation: &'static str,
    format: &AudioStreamBasicDescription,
    frames: u32,
) -> Result<u32> {
    format.mBytesPerFrame.checked_mul(frames).ok_or_else(|| {
        AudioToolboxError::message(
            operation,
            format!(
                "{frames} frames of {} bytes exceed UInt32::MAX bytes",
                format.mBytesPerFrame
            ),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::OwnedAudioBufferList;
    use crate::{AudioBuffer, AudioStreamBasicDescription};

    #[test]
    fn for_format_follows_the_interleaving_of_the_format() {
        let interleaved = AudioStreamBasicDescription::linear_pcm_f32(48_000.0, 2, true);
        let list = OwnedAudioBufferList::for_format(&interleaved, 16).expect("interleaved");
        assert_eq!(list.buffer_count(), 1);
        assert_eq!(list.number_channels(0), Some(2));
        assert_eq!(list.capacity(0), Some(128));

        let planar = AudioStreamBasicDescription::linear_pcm_f32(48_000.0, 3, false);
        let list = OwnedAudioBufferList::for_format(&planar, 16).expect("planar");
        assert_eq!(list.buffer_count(), 3);
        assert_eq!(list.number_channels(2), Some(1));
        assert_eq!(list.capacity(2), Some(64));
    }

    #[test]
    fn check_frames_rejects_a_mismatched_buffer_count_or_capacity() {
        let planar = AudioStreamBasicDescription::linear_pcm_f32(48_000.0, 2, false);
        let one = OwnedAudioBufferList::new(1, 2, 1024).expect("list");
        assert!(one.check_frames("test", &planar, 16).is_err());

        let small = OwnedAudioBufferList::new(2, 1, 32).expect("list");
        assert_eq!(small.check_frames("test", &planar, 8).ok(), Some(32));
        assert!(small.check_frames("test", &planar, 9).is_err());
        assert!(small.check_frames("test", &planar, u32::MAX).is_err());
    }

    #[test]
    fn raw_list_counts_every_buffer_and_absorbs_replaced_pointers() {
        let mut list = OwnedAudioBufferList::new(2, 1, 8).expect("list");
        list.data_mut(1).expect("buffer 1").copy_from_slice(&[9; 8]);
        let mut raw = list.raw_mut();
        let header = unsafe { raw.as_ptr().cast::<u32>().read() };
        assert_eq!(header, 2);

        let foreign = [7_u8; 8];
        unsafe {
            raw.as_mut_ptr()
                .cast::<u64>()
                .add(1)
                .cast::<AudioBuffer>()
                .write(AudioBuffer {
                    mNumberChannels: 1,
                    mDataByteSize: 4,
                    mData: foreign.as_ptr().cast_mut().cast(),
                });
        }
        list.absorb(&raw);
        assert_eq!(list.data(0), Some(&[7_u8; 4][..]));
        assert_eq!(list.data(1), Some(&[9_u8; 8][..]));
    }

    #[test]
    fn set_data_byte_size_is_bounded_by_capacity() {
        let mut list = OwnedAudioBufferList::new(1, 1, 8).expect("list");
        assert!(list.set_data_byte_size(0, 9).is_err());
        assert!(list.set_data_byte_size(1, 1).is_err());
        list.set_data_byte_size(0, 3).expect("in range");
        assert_eq!(list.data(0).map(<[u8]>::len), Some(3));
        assert_eq!(list.data_mut(0).map(|data| data.len()), Some(8));
    }

    #[test]
    fn non_pcm_formats_are_rejected() {
        let mut compressed = AudioStreamBasicDescription::linear_pcm_f32(48_000.0, 2, true);
        compressed.mFormatID = crate::AUDIO_FORMAT_MPEG4_AAC;
        assert!(OwnedAudioBufferList::for_format(&compressed, 1).is_err());
    }
}
