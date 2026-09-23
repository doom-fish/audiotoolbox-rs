use crate::{
    audio_file::validate_packets,
    ffi,
    format::TaggedChannelLayout,
    internal::status_to_result,
    property::{read_property, AudioProperty},
    AudioChannelLayoutTag, AudioQueueBuffer, AudioQueueBufferRef, AudioQueueParameterId,
    AudioQueueParameterValue, AudioQueuePropertyId, AudioQueueRef, AudioStreamBasicDescription,
    AudioStreamPacketDescription, AudioTimeStamp, AudioToolboxError, Result, SMPTETime,
    AUDIO_QUEUE_PARAM_VOLUME, AUDIO_QUEUE_PROPERTY_IS_RUNNING,
    AUDIO_QUEUE_PROPERTY_STREAM_DESCRIPTION,
};
use doom_fish_utils::callback_context::CallbackContext;
use std::{
    cell::Cell,
    ffi::c_void,
    marker::PhantomData,
    sync::{Mutex, PoisonError},
};

type OutputHandler = Mutex<Box<dyn FnMut(&mut AudioQueueOutputBuffer<'_>) -> bool + Send>>;
type InputHandler = Mutex<Box<dyn FnMut(&AudioQueueInputBuffer<'_>) -> bool + Send>>;

enum QueueContext {
    None,
    Output(CallbackContext<OutputHandler>),
    Input(CallbackContext<InputHandler>),
}

impl QueueContext {
    fn deactivate(&self) {
        match self {
            Self::None => {}
            Self::Output(context) => context.deactivate(),
            Self::Input(context) => context.deactivate(),
        }
    }
}

/// Owning wrapper around an AudioToolbox.framework `AudioQueueRef`.
pub struct AudioQueue {
    handle: *mut std::ffi::c_void,
    raw: AudioQueueRef,
    context: QueueContext,
    offline_format: Cell<Option<AudioStreamBasicDescription>>,
}

impl std::fmt::Debug for AudioQueue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AudioQueue")
            .field("raw", &self.raw)
            .field(
                "callback",
                &match self.context {
                    QueueContext::None => "none",
                    QueueContext::Output(_) => "output",
                    QueueContext::Input(_) => "input",
                },
            )
            .finish_non_exhaustive()
    }
}

#[derive(Debug)]
/// Owning wrapper around an AudioToolbox.framework `AudioQueueBufferRef`.
pub struct AudioQueueBufferHandle<'q> {
    raw: AudioQueueBufferRef,
    queue: AudioQueueRef,
    _queue: PhantomData<&'q AudioQueue>,
}

#[derive(Debug)]
pub struct AudioQueueOutputBuffer<'a> {
    raw: AudioQueueBufferRef,
    _buffer: PhantomData<&'a mut AudioQueueBuffer>,
}

#[derive(Debug, Clone, Copy)]
pub struct AudioQueueInputBuffer<'a> {
    pub data: &'a [u8],
    pub packet_descriptions: &'a [AudioStreamPacketDescription],
    pub start_time: Option<AudioTimeStamp>,
}

unsafe fn buffer_bytes_mut<'a>(raw: AudioQueueBufferRef) -> &'a mut [u8] {
    let buffer = unsafe { &*raw };
    if buffer.mAudioData.is_null() || buffer.mAudioDataBytesCapacity == 0 {
        return &mut [];
    }
    unsafe {
        std::slice::from_raw_parts_mut(
            buffer.mAudioData.cast::<u8>(),
            buffer.mAudioDataBytesCapacity as usize,
        )
    }
}

unsafe fn buffer_bytes<'a>(raw: AudioQueueBufferRef) -> &'a [u8] {
    let buffer = unsafe { &*raw };
    let size = buffer
        .mAudioDataByteSize
        .min(buffer.mAudioDataBytesCapacity) as usize;
    if buffer.mAudioData.is_null() || size == 0 {
        return &[];
    }
    unsafe { std::slice::from_raw_parts(buffer.mAudioData.cast::<u8>(), size) }
}

unsafe fn set_buffer_byte_size(raw: AudioQueueBufferRef, byte_size: u32) -> Result<()> {
    let buffer = unsafe { &mut *raw };
    if byte_size > buffer.mAudioDataBytesCapacity {
        return Err(AudioToolboxError::message(
            "AudioQueueBuffer",
            format!(
                "{byte_size} bytes exceed the {}-byte buffer capacity",
                buffer.mAudioDataBytesCapacity
            ),
        ));
    }
    buffer.mAudioDataByteSize = byte_size;
    Ok(())
}

unsafe fn set_buffer_packet_descriptions(
    raw: AudioQueueBufferRef,
    format: &AudioStreamBasicDescription,
    descriptions: &[AudioStreamPacketDescription],
) -> Result<()> {
    let operation = "AudioQueueBuffer::set_packet_descriptions";
    let buffer = unsafe { &mut *raw };
    let count = u32::try_from(descriptions.len())
        .map_err(|_| AudioToolboxError::message(operation, "too many packet descriptions"))?;
    if count > buffer.mPacketDescriptionCapacity || buffer.mPacketDescriptions.is_null() {
        return Err(AudioToolboxError::message(
            operation,
            format!(
                "{count} packet descriptions exceed the buffer's capacity of {}",
                buffer.mPacketDescriptionCapacity
            ),
        ));
    }
    validate_packets(
        operation,
        format,
        buffer.mAudioDataByteSize as usize,
        count,
        Some(descriptions),
    )?;
    unsafe {
        std::ptr::copy_nonoverlapping(
            descriptions.as_ptr(),
            buffer.mPacketDescriptions,
            descriptions.len(),
        );
    }
    buffer.mPacketDescriptionCount = count;
    Ok(())
}

fn deliver_output(handler: &OutputHandler, raw: AudioQueueBufferRef) -> bool {
    if raw.is_null() {
        return false;
    }
    let mut view = AudioQueueOutputBuffer {
        raw,
        _buffer: PhantomData,
    };
    let mut handler = handler.lock().unwrap_or_else(PoisonError::into_inner);
    handler(&mut view)
}

unsafe fn deliver_input(
    handler: &InputHandler,
    raw: AudioQueueBufferRef,
    start_time: *const AudioTimeStamp,
    packet_count: u32,
    packet_descriptions: *const AudioStreamPacketDescription,
) -> bool {
    if raw.is_null() {
        return false;
    }
    let data = unsafe { buffer_bytes(raw) };
    let packet_descriptions = if packet_descriptions.is_null() || packet_count == 0 {
        &[][..]
    } else {
        unsafe { std::slice::from_raw_parts(packet_descriptions, packet_count as usize) }
    };
    let view = AudioQueueInputBuffer {
        data,
        packet_descriptions,
        start_time: unsafe { start_time.as_ref() }.copied(),
    };
    let mut handler = handler.lock().unwrap_or_else(PoisonError::into_inner);
    handler(&view)
}

unsafe extern "C" fn output_trampoline(
    user_data: *mut c_void,
    queue: *mut c_void,
    buffer: AudioQueueBufferRef,
) {
    let enqueue = unsafe {
        CallbackContext::<OutputHandler>::with(
            user_data,
            "audiotoolbox::audio_queue_output",
            |handler| deliver_output(handler, buffer),
        )
    };
    if enqueue == Some(true) {
        let _ = unsafe {
            ffi::audio_queue::at_audio_queue_enqueue_buffer(queue, buffer, 0, std::ptr::null())
        };
    }
}

unsafe extern "C" fn input_trampoline(
    user_data: *mut c_void,
    queue: *mut c_void,
    buffer: AudioQueueBufferRef,
    start_time: *const AudioTimeStamp,
    packet_count: u32,
    packet_descriptions: *const AudioStreamPacketDescription,
) {
    let enqueue = unsafe {
        CallbackContext::<InputHandler>::with(
            user_data,
            "audiotoolbox::audio_queue_input",
            |handler| {
                deliver_input(
                    handler,
                    buffer,
                    start_time,
                    packet_count,
                    packet_descriptions,
                )
            },
        )
    };
    if enqueue == Some(true) {
        let _ = unsafe {
            ffi::audio_queue::at_audio_queue_enqueue_buffer(queue, buffer, 0, std::ptr::null())
        };
    }
}

impl AudioQueue {
    /// Wraps `AudioQueueNewOutput`.
    ///
    /// The returned wrapper owns the underlying AudioToolbox.framework handle and releases it on drop.
    pub fn new_output(format: &AudioStreamBasicDescription) -> Result<Self> {
        let mut handle = std::ptr::null_mut();
        let status =
            unsafe { ffi::audio_queue::at_audio_queue_new_output(format, &raw mut handle) };
        status_to_result("AudioQueueNewOutput", status)?;
        Self::from_handle(handle, QueueContext::None, "AudioQueueNewOutput")
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn new_output_with_callback<F>(
        format: &AudioStreamBasicDescription,
        callback: F,
    ) -> Result<Self>
    where
        F: FnMut(&mut AudioQueueOutputBuffer<'_>) -> bool + Send + 'static,
    {
        let handler: OutputHandler = Mutex::new(Box::new(callback));
        let context = CallbackContext::new(handler);
        let mut handle = std::ptr::null_mut();
        let status = unsafe {
            ffi::audio_queue::at_audio_queue_new_output_with_callback(
                format,
                output_trampoline,
                context.retained_ptr(),
                CallbackContext::<OutputHandler>::RELEASE,
                &raw mut handle,
            )
        };
        status_to_result("AudioQueueNewOutput", status)?;
        Self::from_handle(handle, QueueContext::Output(context), "AudioQueueNewOutput")
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn new_input_with_callback<F>(
        format: &AudioStreamBasicDescription,
        callback: F,
    ) -> Result<Self>
    where
        F: FnMut(&AudioQueueInputBuffer<'_>) -> bool + Send + 'static,
    {
        let handler: InputHandler = Mutex::new(Box::new(callback));
        let context = CallbackContext::new(handler);
        let mut handle = std::ptr::null_mut();
        let status = unsafe {
            ffi::audio_queue::at_audio_queue_new_input_with_callback(
                format,
                input_trampoline,
                context.retained_ptr(),
                CallbackContext::<InputHandler>::RELEASE,
                &raw mut handle,
            )
        };
        status_to_result("AudioQueueNewInput", status)?;
        Self::from_handle(handle, QueueContext::Input(context), "AudioQueueNewInput")
    }

    fn from_handle(
        handle: *mut c_void,
        context: QueueContext,
        operation: &'static str,
    ) -> Result<Self> {
        let raw: AudioQueueRef = unsafe { ffi::audio_queue::at_audio_queue_raw(handle) }.cast();
        let queue = Self {
            handle,
            raw,
            context,
            offline_format: Cell::new(None),
        };
        if raw.is_null() {
            return Err(AudioToolboxError::message(
                operation,
                "framework returned a null AudioQueueRef",
            ));
        }
        Ok(queue)
    }

    /// Returns the wrapped `AudioQueueRef`.
    pub fn as_raw(&self) -> AudioQueueRef {
        self.raw
    }

    /// Wraps `AudioQueueGetProperty`.
    pub fn stream_description(&self) -> Result<AudioStreamBasicDescription> {
        self.get_property_typed(
            AUDIO_QUEUE_PROPERTY_STREAM_DESCRIPTION,
            "AudioQueueGetProperty(stream description)",
        )
    }

    /// Wraps `AudioQueueGetProperty`.
    pub fn is_running(&self) -> Result<bool> {
        Ok(self.get_property_typed::<u32>(
            AUDIO_QUEUE_PROPERTY_IS_RUNNING,
            "AudioQueueGetProperty(is running)",
        )? != 0)
    }

    /// Wraps `AudioQueueGetParameter`.
    pub fn get_parameter(
        &self,
        parameter_id: AudioQueueParameterId,
    ) -> Result<AudioQueueParameterValue> {
        let mut value = 0.0_f32;
        let status = unsafe {
            ffi::audio_queue::at_audio_queue_get_parameter(
                self.raw.cast(),
                parameter_id,
                &raw mut value,
            )
        };
        status_to_result("AudioQueueGetParameter", status)?;
        Ok(value)
    }

    /// Wraps `AudioQueueSetParameter`.
    pub fn set_parameter(
        &self,
        parameter_id: AudioQueueParameterId,
        value: AudioQueueParameterValue,
    ) -> Result<()> {
        let status = unsafe {
            ffi::audio_queue::at_audio_queue_set_parameter(self.raw.cast(), parameter_id, value)
        };
        status_to_result("AudioQueueSetParameter", status)
    }

    /// Wraps `AudioQueueGetParameter` for `kAudioQueueParam_Volume`.
    pub fn volume(&self) -> Result<f32> {
        self.get_parameter(AUDIO_QUEUE_PARAM_VOLUME)
    }

    /// Wraps `AudioQueueSetParameter` for `kAudioQueueParam_Volume`.
    pub fn set_volume(&self, value: f32) -> Result<()> {
        self.set_parameter(AUDIO_QUEUE_PARAM_VOLUME, value)
    }

    /// Wraps `AudioQueueAllocateBuffer`.
    ///
    /// The returned wrapper owns the underlying AudioToolbox.framework handle and releases it on drop.
    pub fn allocate_buffer(&self, byte_size: u32) -> Result<AudioQueueBufferHandle<'_>> {
        let mut raw: AudioQueueBufferRef = std::ptr::null_mut();
        let status = unsafe {
            ffi::audio_queue::at_audio_queue_allocate_buffer(
                self.raw.cast(),
                byte_size,
                &raw mut raw,
            )
        };
        status_to_result("AudioQueueAllocateBuffer", status)?;
        self.buffer_handle(raw, "AudioQueueAllocateBuffer")
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn allocate_buffer_with_packet_descriptions(
        &self,
        byte_size: u32,
        packet_description_capacity: u32,
    ) -> Result<AudioQueueBufferHandle<'_>> {
        let mut raw: AudioQueueBufferRef = std::ptr::null_mut();
        let status = unsafe {
            ffi::audio_queue::at_audio_queue_allocate_buffer_with_packet_descriptions(
                self.raw.cast(),
                byte_size,
                packet_description_capacity,
                &raw mut raw,
            )
        };
        status_to_result("AudioQueueAllocateBufferWithPacketDescriptions", status)?;
        self.buffer_handle(raw, "AudioQueueAllocateBufferWithPacketDescriptions")
    }

    fn buffer_handle(
        &self,
        raw: AudioQueueBufferRef,
        operation: &'static str,
    ) -> Result<AudioQueueBufferHandle<'_>> {
        if raw.is_null() {
            return Err(AudioToolboxError::message(
                operation,
                "framework returned a null AudioQueueBufferRef",
            ));
        }
        Ok(AudioQueueBufferHandle {
            raw,
            queue: self.raw,
            _queue: PhantomData,
        })
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn enqueue_buffer(&self, buffer: AudioQueueBufferHandle<'_>) -> Result<()> {
        if buffer.queue != self.raw {
            return Err(AudioToolboxError::message(
                "AudioQueueEnqueueBuffer",
                "the buffer was allocated by a different queue",
            ));
        }
        let status = unsafe {
            ffi::audio_queue::at_audio_queue_enqueue_buffer(
                self.raw.cast(),
                buffer.raw,
                0,
                std::ptr::null(),
            )
        };
        status_to_result("AudioQueueEnqueueBuffer", status)?;
        std::mem::forget(buffer);
        Ok(())
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn set_offline_render_format(
        &self,
        format: &AudioStreamBasicDescription,
        channel_layout_tag: AudioChannelLayoutTag,
    ) -> Result<()> {
        if !format.is_linear_pcm() || format.mBytesPerFrame == 0 {
            return Err(AudioToolboxError::message(
                "AudioQueueSetOfflineRenderFormat",
                "the offline render format must be linear PCM with a non-zero frame size",
            ));
        }
        let layout = TaggedChannelLayout::new(channel_layout_tag);
        let status = unsafe {
            ffi::audio_queue::at_audio_queue_set_offline_render_format(
                self.raw.cast(),
                format,
                std::ptr::from_ref(&layout).cast(),
            )
        };
        status_to_result("AudioQueueSetOfflineRenderFormat", status)?;
        self.offline_format.set(Some(*format));
        Ok(())
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn offline_render(
        &self,
        sample_time: f64,
        buffer: &mut AudioQueueBufferHandle<'_>,
        frames: u32,
    ) -> Result<()> {
        let operation = "AudioQueueOfflineRender";
        let format = self.offline_format.get().ok_or_else(|| {
            AudioToolboxError::message(operation, "call set_offline_render_format first")
        })?;
        if buffer.queue != self.raw {
            return Err(AudioToolboxError::message(
                operation,
                "the buffer was allocated by a different queue",
            ));
        }
        let needed = u64::from(format.mBytesPerFrame) * u64::from(frames);
        if needed > u64::from(buffer.audio_data_bytes_capacity()) {
            return Err(AudioToolboxError::message(
                operation,
                format!(
                    "{frames} frames need {needed} bytes but the buffer holds {}",
                    buffer.audio_data_bytes_capacity()
                ),
            ));
        }
        let time_stamp = AudioTimeStamp {
            mSampleTime: sample_time,
            mHostTime: 0,
            mRateScalar: 0.0,
            mWordClockTime: 0,
            mSMPTETime: SMPTETime {
                mSubframes: 0,
                mSubframeDivisor: 0,
                mCounter: 0,
                mType: 0,
                mFlags: 0,
                mHours: 0,
                mMinutes: 0,
                mSeconds: 0,
                mFrames: 0,
            },
            mFlags: 1,
            mReserved: 0,
        };
        let status = unsafe {
            ffi::audio_queue::at_audio_queue_offline_render(
                self.raw.cast(),
                &raw const time_stamp,
                buffer.raw,
                frames,
            )
        };
        status_to_result(operation, status)?;
        let size = buffer.byte_size().min(buffer.audio_data_bytes_capacity());
        unsafe { set_buffer_byte_size(buffer.raw, size) }
    }

    /// Wraps `AudioQueueStart`.
    pub fn start(&self) -> Result<()> {
        let status = unsafe { ffi::audio_queue::at_audio_queue_start(self.raw.cast()) };
        status_to_result("AudioQueueStart", status)
    }

    /// Wraps `AudioQueueStop`.
    pub fn stop(&self, immediate: bool) -> Result<()> {
        let status = unsafe { ffi::audio_queue::at_audio_queue_stop(self.raw.cast(), immediate) };
        status_to_result("AudioQueueStop", status)
    }

    /// Wraps `AudioQueueReset`.
    pub fn reset(&self) -> Result<()> {
        let status = unsafe { ffi::audio_queue::at_audio_queue_reset(self.raw.cast()) };
        status_to_result("AudioQueueReset", status)
    }

    /// Wraps `AudioQueueClose`.
    pub fn close(mut self) -> Result<()> {
        self.release();
        Ok(())
    }

    fn get_property_typed<T: AudioProperty>(
        &self,
        property_id: AudioQueuePropertyId,
        operation: &'static str,
    ) -> Result<T> {
        read_property(operation, |data, size| unsafe {
            ffi::audio_queue::at_audio_queue_get_property(self.raw.cast(), property_id, size, data)
        })
    }

    fn release(&mut self) {
        self.context.deactivate();
        if !self.handle.is_null() {
            unsafe { ffi::audio_queue::at_audio_queue_release(self.handle) };
            self.handle = std::ptr::null_mut();
            self.raw = std::ptr::null_mut();
        }
    }
}

impl Drop for AudioQueue {
    fn drop(&mut self) {
        self.release();
    }
}

impl AudioQueueBufferHandle<'_> {
    /// Returns the wrapped `AudioQueueBufferRef`.
    pub fn as_raw(&self) -> AudioQueueBufferRef {
        self.raw
    }

    /// Reads `mAudioDataBytesCapacity` from the wrapped `AudioQueueBufferRef`.
    pub fn audio_data_bytes_capacity(&self) -> u32 {
        unsafe { (*self.raw).mAudioDataBytesCapacity }
    }

    pub fn byte_size(&self) -> u32 {
        unsafe { (*self.raw).mAudioDataByteSize }
    }

    pub fn data(&self) -> &[u8] {
        unsafe { buffer_bytes(self.raw) }
    }

    pub fn data_mut(&mut self) -> &mut [u8] {
        unsafe { buffer_bytes_mut(self.raw) }
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn set_byte_size(&mut self, byte_size: u32) -> Result<()> {
        unsafe { set_buffer_byte_size(self.raw, byte_size) }
    }

    pub fn packet_description_capacity(&self) -> u32 {
        unsafe { (*self.raw).mPacketDescriptionCapacity }
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn set_packet_descriptions(
        &mut self,
        format: &AudioStreamBasicDescription,
        descriptions: &[AudioStreamPacketDescription],
    ) -> Result<()> {
        unsafe { set_buffer_packet_descriptions(self.raw, format, descriptions) }
    }
}

impl Drop for AudioQueueBufferHandle<'_> {
    fn drop(&mut self) {
        let _ =
            unsafe { ffi::audio_queue::at_audio_queue_free_buffer(self.queue.cast(), self.raw) };
    }
}

impl AudioQueueOutputBuffer<'_> {
    pub fn capacity(&self) -> u32 {
        unsafe { (*self.raw).mAudioDataBytesCapacity }
    }

    pub fn byte_size(&self) -> u32 {
        unsafe { (*self.raw).mAudioDataByteSize }
    }

    pub fn data_mut(&mut self) -> &mut [u8] {
        unsafe { buffer_bytes_mut(self.raw) }
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn set_byte_size(&mut self, byte_size: u32) -> Result<()> {
        unsafe { set_buffer_byte_size(self.raw, byte_size) }
    }

    pub fn packet_description_capacity(&self) -> u32 {
        unsafe { (*self.raw).mPacketDescriptionCapacity }
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn set_packet_descriptions(
        &mut self,
        format: &AudioStreamBasicDescription,
        descriptions: &[AudioStreamPacketDescription],
    ) -> Result<()> {
        unsafe { set_buffer_packet_descriptions(self.raw, format, descriptions) }
    }
}

#[cfg(test)]
mod tests {
    use super::{deliver_input, deliver_output, InputHandler, OutputHandler};
    use crate::{AudioQueueBuffer, AudioStreamBasicDescription, AudioStreamPacketDescription};
    use std::sync::{Arc, Mutex};

    fn buffer(
        storage: &mut [u8],
        descriptions: &mut [AudioStreamPacketDescription],
    ) -> AudioQueueBuffer {
        AudioQueueBuffer {
            mAudioDataBytesCapacity: u32::try_from(storage.len()).unwrap(),
            mAudioData: storage.as_mut_ptr().cast(),
            mAudioDataByteSize: 0,
            mUserData: std::ptr::null_mut(),
            mPacketDescriptionCapacity: u32::try_from(descriptions.len()).unwrap(),
            mPacketDescriptions: if descriptions.is_empty() {
                std::ptr::null_mut()
            } else {
                descriptions.as_mut_ptr()
            },
            mPacketDescriptionCount: 0,
        }
    }

    #[test]
    fn output_view_is_bounded_by_the_buffer_capacity() {
        let mut storage = vec![0_u8; 16];
        let mut raw = buffer(&mut storage, &mut []);
        let handler: OutputHandler = Mutex::new(Box::new(|view| {
            assert_eq!(view.capacity(), 16);
            assert_eq!(view.data_mut().len(), 16);
            view.data_mut()[..4].copy_from_slice(&[1, 2, 3, 4]);
            assert!(view.set_byte_size(17).is_err());
            view.set_byte_size(4).unwrap();
            let format = AudioStreamBasicDescription::linear_pcm_i16(44_100.0, 1, true);
            assert!(view
                .set_packet_descriptions(&format, &[AudioStreamPacketDescription::default()])
                .is_err());
            true
        }));
        assert!(deliver_output(&handler, &raw mut raw));
        assert_eq!(raw.mAudioDataByteSize, 4);
        assert_eq!(&storage[..4], &[1, 2, 3, 4]);
        assert!(!deliver_output(&handler, std::ptr::null_mut()));
    }

    #[test]
    fn output_packet_descriptions_are_checked_against_the_payload() {
        let mut storage = vec![0_u8; 32];
        let mut descriptions = vec![AudioStreamPacketDescription::default(); 2];
        let mut raw = buffer(&mut storage, &mut descriptions);
        let mut vbr = AudioStreamBasicDescription::linear_pcm_i16(44_100.0, 1, true);
        vbr.mBytesPerPacket = 0;
        let handler: OutputHandler = Mutex::new(Box::new(move |view| {
            view.set_byte_size(24).unwrap();
            let fits = [
                AudioStreamPacketDescription {
                    mStartOffset: 0,
                    mVariableFramesInPacket: 0,
                    mDataByteSize: 12,
                },
                AudioStreamPacketDescription {
                    mStartOffset: 12,
                    mVariableFramesInPacket: 0,
                    mDataByteSize: 12,
                },
            ];
            view.set_packet_descriptions(&vbr, &fits).unwrap();
            let past_end = [AudioStreamPacketDescription {
                mStartOffset: 20,
                mVariableFramesInPacket: 0,
                mDataByteSize: 8,
            }];
            assert!(view.set_packet_descriptions(&vbr, &past_end).is_err());
            assert!(view
                .set_packet_descriptions(&vbr, &[fits[0], fits[1], fits[0]])
                .is_err());
            false
        }));
        assert!(!deliver_output(&handler, &raw mut raw));
        assert_eq!(raw.mPacketDescriptionCount, 2);
        assert_eq!(descriptions[1].mStartOffset, 12);
    }

    #[test]
    fn input_view_exposes_only_the_recorded_bytes() {
        let mut storage = vec![7_u8; 16];
        let mut raw = buffer(&mut storage, &mut []);
        raw.mAudioDataByteSize = 6;
        let seen = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&seen);
        let handler: InputHandler = Mutex::new(Box::new(move |view| {
            sink.lock().unwrap().extend_from_slice(view.data);
            assert!(view.packet_descriptions.is_empty());
            assert!(view.start_time.is_none());
            true
        }));
        assert!(unsafe {
            deliver_input(
                &handler,
                &raw mut raw,
                std::ptr::null(),
                0,
                std::ptr::null(),
            )
        });
        assert_eq!(*seen.lock().unwrap(), vec![7_u8; 6]);

        raw.mAudioDataByteSize = 64;
        assert!(unsafe {
            deliver_input(
                &handler,
                &raw mut raw,
                std::ptr::null(),
                0,
                std::ptr::null(),
            )
        });
        assert_eq!(seen.lock().unwrap().len(), 6 + 16);
    }
}
