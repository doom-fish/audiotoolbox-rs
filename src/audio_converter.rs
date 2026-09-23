use crate::{
    audio_file::validate_packets,
    ffi,
    internal::status_to_result,
    property::{property_byte_size, read_property, read_property_array, AudioProperty},
    AudioBuffer, AudioBufferList, AudioBufferList1, AudioClassDescription, AudioConverterPrimeInfo,
    AudioConverterPropertyId, AudioConverterRef, AudioStreamBasicDescription,
    AudioStreamPacketDescription, AudioToolboxError, AudioValueRange, OSStatus,
    OwnedAudioBufferList, Result, AUDIO_CONVERTER_PROPERTY_APPLICABLE_ENCODE_BIT_RATES,
    AUDIO_CONVERTER_PROPERTY_APPLICABLE_ENCODE_SAMPLE_RATES,
    AUDIO_CONVERTER_PROPERTY_CALCULATE_INPUT_BUFFER_SIZE,
    AUDIO_CONVERTER_PROPERTY_CALCULATE_OUTPUT_BUFFER_SIZE,
    AUDIO_CONVERTER_PROPERTY_CURRENT_INPUT_STREAM_DESCRIPTION,
    AUDIO_CONVERTER_PROPERTY_CURRENT_OUTPUT_STREAM_DESCRIPTION,
    AUDIO_CONVERTER_PROPERTY_ENCODE_BIT_RATE, AUDIO_CONVERTER_PROPERTY_MAXIMUM_OUTPUT_PACKET_SIZE,
    AUDIO_CONVERTER_PROPERTY_PRIME_INFO, NO_ERR,
};
use doom_fish_utils::panic_safe::catch_user_panic_result;
use std::{collections::VecDeque, ffi::c_void, fmt, marker::PhantomData};

const NEED_MORE_INPUT: OSStatus = i32::from_be_bytes(*b"dfni");
const PARAM_ERROR: OSStatus = -50;

#[derive(Debug, Clone, Copy)]
/// Input buffer description used with `AudioConverterFillComplexBuffer`.
pub struct AudioConversionInput<'a> {
    pub data: &'a [u8],
    pub packet_count: u32,
    pub packet_descriptions: Option<&'a [AudioStreamPacketDescription]>,
    pub channels: u32,
}

#[derive(Debug, Clone)]
/// Output buffer returned by `AudioConverterFillComplexBuffer`.
pub struct AudioConversionOutput {
    pub data: Vec<u8>,
    pub packet_count: u32,
    pub packet_descriptions: Vec<AudioStreamPacketDescription>,
}

struct PendingInput {
    data: Vec<u8>,
    packet_count: u32,
    packet_descriptions: Vec<AudioStreamPacketDescription>,
    channels: u32,
}

impl PendingInput {
    fn copy(
        operation: &'static str,
        input: &AudioConversionInput<'_>,
        format: &AudioStreamBasicDescription,
    ) -> Result<Option<Self>> {
        if input.data.is_empty() && input.packet_count == 0 {
            return Ok(None);
        }
        if input.packet_count == 0 {
            return Err(AudioToolboxError::message(
                operation,
                "packet_count must be greater than zero when input data is supplied",
            ));
        }
        if u32::try_from(input.data.len()).is_err() {
            return Err(AudioToolboxError::message(
                operation,
                "input buffer exceeds UInt32::MAX bytes",
            ));
        }
        if needs_separate_buffers(format) {
            return Err(AudioToolboxError::message(
                operation,
                "non-interleaved multichannel input needs one buffer per channel; use an interleaved input format",
            ));
        }
        validate_packets(
            operation,
            format,
            input.data.len(),
            input.packet_count,
            input.packet_descriptions,
        )?;
        Ok(Some(Self {
            data: input.data.to_vec(),
            packet_count: input.packet_count,
            packet_descriptions: input
                .packet_descriptions
                .map_or_else(Vec::new, |descriptions| {
                    descriptions[..input.packet_count as usize].to_vec()
                }),
            channels: input.channels,
        }))
    }
}

#[derive(Default)]
struct FillState {
    in_use: Option<PendingInput>,
    queued: VecDeque<PendingInput>,
    end_of_stream: bool,
}

impl fmt::Debug for FillState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FillState")
            .field("in_use", &self.in_use.is_some())
            .field("queued", &self.queued.len())
            .field("end_of_stream", &self.end_of_stream)
            .finish()
    }
}

fn needs_separate_buffers(format: &AudioStreamBasicDescription) -> bool {
    format.is_linear_pcm() && !format.is_interleaved() && format.mChannelsPerFrame > 1
}

unsafe fn supply_input(
    io_number_data_packets: *mut u32,
    io_data: *mut AudioBufferList1,
    out_data_packet_description: *mut *mut AudioStreamPacketDescription,
    user_data: *mut c_void,
) -> OSStatus {
    let (Some(packets), Some(list), Some(state)) = (unsafe {
        (
            io_number_data_packets.as_mut(),
            io_data.as_mut(),
            user_data.cast::<FillState>().as_mut(),
        )
    }) else {
        return PARAM_ERROR;
    };
    state.in_use = None;
    *packets = 0;
    if list.mNumberBuffers != 1 {
        return PARAM_ERROR;
    }
    let Some(next) = state.queued.pop_front() else {
        list.mBuffers[0].mDataByteSize = 0;
        list.mBuffers[0].mData = std::ptr::null_mut();
        return if state.end_of_stream {
            NO_ERR
        } else {
            NEED_MORE_INPUT
        };
    };
    list.mBuffers[0] = AudioBuffer {
        mNumberChannels: next.channels,
        mDataByteSize: next.data.len() as u32,
        mData: next.data.as_ptr().cast_mut().cast(),
    };
    *packets = next.packet_count;
    if let Some(descriptions) = unsafe { out_data_packet_description.as_mut() } {
        *descriptions = if next.packet_descriptions.is_empty() {
            std::ptr::null_mut()
        } else {
            next.packet_descriptions.as_ptr().cast_mut()
        };
    }
    state.in_use = Some(next);
    NO_ERR
}

unsafe extern "C" fn fill_input_proc(
    _converter: *mut c_void,
    io_number_data_packets: *mut u32,
    io_data: *mut AudioBufferList1,
    out_data_packet_description: *mut *mut AudioStreamPacketDescription,
    user_data: *mut c_void,
) -> OSStatus {
    catch_user_panic_result("audiotoolbox::fill_input_proc", || unsafe {
        supply_input(
            io_number_data_packets,
            io_data,
            out_data_packet_description,
            user_data,
        )
    })
    .unwrap_or(PARAM_ERROR)
}

#[derive(Debug)]
/// Owning wrapper around an AudioToolbox.framework `AudioConverterRef`.
pub struct AudioConverter {
    handle: *mut std::ffi::c_void,
    raw: AudioConverterRef,
    fill: Box<FillState>,
}

#[derive(Debug, Clone, Copy)]
/// Borrowed wrapper around an AudioToolbox.framework `AudioConverterRef`.
pub struct BorrowedAudioConverter<'a> {
    raw: AudioConverterRef,
    _marker: PhantomData<&'a ()>,
}

impl BorrowedAudioConverter<'_> {
    /// Wraps an existing `AudioConverterRef` without changing ownership.
    pub(crate) const fn new(raw: AudioConverterRef) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    /// Returns the wrapped `AudioConverterRef`.
    pub fn as_raw(&self) -> AudioConverterRef {
        self.raw
    }

    /// Wraps `AudioConverterReset`.
    pub fn reset(&self) -> Result<()> {
        let status = unsafe { ffi::audio_converter::at_audio_converter_reset(self.raw.cast()) };
        status_to_result("AudioConverterReset", status)
    }
}

impl AudioConverter {
    /// Wraps `AudioConverterNew`.
    ///
    /// The returned wrapper owns the underlying AudioToolbox.framework handle and releases it on drop.
    pub fn new(
        source_format: &AudioStreamBasicDescription,
        destination_format: &AudioStreamBasicDescription,
    ) -> Result<Self> {
        let mut handle = std::ptr::null_mut();
        let status = unsafe {
            ffi::audio_converter::at_audio_converter_new(
                source_format,
                destination_format,
                &raw mut handle,
            )
        };
        status_to_result("AudioConverterNew", status)?;
        let raw: AudioConverterRef =
            unsafe { ffi::audio_converter::at_audio_converter_raw(handle) }.cast();
        if raw.is_null() {
            return Err(AudioToolboxError::message(
                "AudioConverterNew",
                "framework returned a null AudioConverterRef",
            ));
        }
        Ok(Self {
            handle,
            raw,
            fill: Box::default(),
        })
    }

    /// Wraps `AudioConverterNewSpecific`.
    ///
    /// The returned wrapper owns the underlying AudioToolbox.framework handle and releases it on drop.
    pub fn new_specific(
        source_format: &AudioStreamBasicDescription,
        destination_format: &AudioStreamBasicDescription,
        class_descriptions: &[AudioClassDescription],
    ) -> Result<Self> {
        let mut handle = std::ptr::null_mut();
        let class_count = u32::try_from(class_descriptions.len()).map_err(|_| {
            AudioToolboxError::message(
                "AudioConverterNewSpecific",
                "too many AudioClassDescription entries",
            )
        })?;
        let class_description_ptr = if class_descriptions.is_empty() {
            std::ptr::null()
        } else {
            class_descriptions.as_ptr()
        };
        let status = unsafe {
            ffi::audio_converter::at_audio_converter_new_specific(
                source_format,
                destination_format,
                class_description_ptr,
                class_count,
                &raw mut handle,
            )
        };
        status_to_result("AudioConverterNewSpecific", status)?;
        let raw: AudioConverterRef =
            unsafe { ffi::audio_converter::at_audio_converter_raw(handle) }.cast();
        if raw.is_null() {
            return Err(AudioToolboxError::message(
                "AudioConverterNewSpecific",
                "framework returned a null AudioConverterRef",
            ));
        }
        Ok(Self {
            handle,
            raw,
            fill: Box::default(),
        })
    }

    /// Returns the wrapped `AudioConverterRef`.
    pub fn as_raw(&self) -> AudioConverterRef {
        self.raw
    }

    /// Wraps `AudioConverterReset`.
    pub fn reset(&mut self) -> Result<()> {
        let status = unsafe { ffi::audio_converter::at_audio_converter_reset(self.raw.cast()) };
        status_to_result("AudioConverterReset", status)?;
        *self.fill = FillState::default();
        Ok(())
    }

    /// Wraps `AudioConverterFillComplexBuffer`.
    pub fn fill_complex_buffer(
        &mut self,
        input: AudioConversionInput<'_>,
        output_packet_capacity: u32,
    ) -> Result<AudioConversionOutput> {
        let operation = "AudioConverterFillComplexBuffer";
        if output_packet_capacity == 0 {
            return Err(AudioToolboxError::message(
                operation,
                "output_packet_capacity must be greater than zero",
            ));
        }
        if self.fill.end_of_stream {
            return Err(AudioToolboxError::message(
                operation,
                "the stream has ended; call reset() before converting more input",
            ));
        }
        if let Some(pending) =
            PendingInput::copy(operation, &input, &self.current_input_stream_description()?)?
        {
            self.fill.queued.push_back(pending);
        }
        self.fill_output(operation, output_packet_capacity)
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn finish(&mut self, output_packet_capacity: u32) -> Result<AudioConversionOutput> {
        self.fill.end_of_stream = true;
        self.fill_output(
            "AudioConverterFillComplexBuffer(end of stream)",
            output_packet_capacity,
        )
    }

    fn fill_output(
        &mut self,
        operation: &'static str,
        output_packet_capacity: u32,
    ) -> Result<AudioConversionOutput> {
        if output_packet_capacity == 0 {
            return Err(AudioToolboxError::message(
                operation,
                "output_packet_capacity must be greater than zero",
            ));
        }
        let output_format = self.current_output_stream_description()?;
        if needs_separate_buffers(&output_format) {
            return Err(AudioToolboxError::message(
                operation,
                "non-interleaved multichannel output needs one buffer per channel; use an interleaved output format",
            ));
        }
        let bytes_per_packet = if output_format.mBytesPerPacket == 0 {
            self.maximum_output_packet_size()?
        } else {
            output_format.mBytesPerPacket
        };
        let output_byte_capacity = bytes_per_packet
            .checked_mul(output_packet_capacity)
            .ok_or_else(|| {
                AudioToolboxError::message(operation, "output buffer exceeds UInt32::MAX bytes")
            })?;
        let mut output_bytes = vec![0_u8; output_byte_capacity as usize];
        let uses_descriptions = output_format.uses_packet_descriptions();
        let mut output_packet_descriptions = if uses_descriptions {
            vec![AudioStreamPacketDescription::default(); output_packet_capacity as usize]
        } else {
            Vec::new()
        };
        let mut output_buffer_list = AudioBufferList {
            mNumberBuffers: 1,
            mBuffers: [AudioBuffer {
                mNumberChannels: output_format.mChannelsPerFrame,
                mDataByteSize: output_byte_capacity,
                mData: output_bytes.as_mut_ptr().cast(),
            }],
        };
        let mut output_packets = output_packet_capacity;
        let output_packet_description_ptr = if uses_descriptions {
            output_packet_descriptions.as_mut_ptr()
        } else {
            std::ptr::null_mut()
        };

        let status = unsafe {
            ffi::audio_converter::at_audio_converter_fill_complex_buffer(
                self.raw.cast(),
                fill_input_proc,
                (&raw mut *self.fill).cast(),
                &raw mut output_packets,
                &raw mut output_buffer_list,
                output_packet_description_ptr,
            )
        };
        if status != NEED_MORE_INPUT {
            status_to_result(operation, status)?;
        }

        let output_packets = output_packets.min(output_packet_capacity);
        output_bytes.truncate(
            output_buffer_list.mBuffers[0]
                .mDataByteSize
                .min(output_byte_capacity) as usize,
        );
        output_packet_descriptions.truncate(output_packets as usize);

        Ok(AudioConversionOutput {
            data: output_bytes,
            packet_count: output_packets,
            packet_descriptions: output_packet_descriptions,
        })
    }

    /// Wraps `AudioConverterClose`.
    pub fn close(mut self) -> Result<()> {
        self.release();
        Ok(())
    }

    fn release(&mut self) {
        if !self.handle.is_null() {
            unsafe { ffi::audio_converter::at_audio_converter_release(self.handle) };
            self.handle = std::ptr::null_mut();
            self.raw = std::ptr::null_mut();
        }
    }
}

macro_rules! impl_converter_methods {
    ($ty:ty) => {
        impl $ty {
            /// Wraps `AudioConverterGetPropertyInfo`.
            pub fn property_info(
                &self,
                property_id: AudioConverterPropertyId,
            ) -> Result<(u32, bool)> {
                let mut size = 0_u32;
                let mut writable = 0_u8;
                let status = unsafe {
                    ffi::audio_converter::at_audio_converter_get_property_info(
                        self.raw.cast(),
                        property_id,
                        &mut size,
                        &mut writable,
                    )
                };
                status_to_result("AudioConverterGetPropertyInfo", status)?;
                Ok((size, writable != 0))
            }

            /// Wraps `AudioConverterGetProperty`.
            pub fn current_input_stream_description(&self) -> Result<AudioStreamBasicDescription> {
                get_property_typed(
                    self.raw,
                    AUDIO_CONVERTER_PROPERTY_CURRENT_INPUT_STREAM_DESCRIPTION,
                    "AudioConverterGetProperty(current input stream description)",
                )
            }

            /// Wraps `AudioConverterGetProperty`.
            pub fn current_output_stream_description(&self) -> Result<AudioStreamBasicDescription> {
                get_property_typed(
                    self.raw,
                    AUDIO_CONVERTER_PROPERTY_CURRENT_OUTPUT_STREAM_DESCRIPTION,
                    "AudioConverterGetProperty(current output stream description)",
                )
            }

            /// Wraps `AudioConverterGetProperty`.
            pub fn encode_bit_rate(&self) -> Result<u32> {
                get_property_typed(
                    self.raw,
                    AUDIO_CONVERTER_PROPERTY_ENCODE_BIT_RATE,
                    "AudioConverterGetProperty(encode bit rate)",
                )
            }

            /// Wraps `AudioConverterSetProperty`.
            pub fn set_encode_bit_rate(&self, bits_per_second: u32) -> Result<()> {
                set_property_typed(
                    self.raw,
                    AUDIO_CONVERTER_PROPERTY_ENCODE_BIT_RATE,
                    &bits_per_second,
                    "AudioConverterSetProperty(encode bit rate)",
                )
            }

            /// Wraps `AudioConverterGetProperty`.
            pub fn maximum_output_packet_size(&self) -> Result<u32> {
                get_property_typed(
                    self.raw,
                    AUDIO_CONVERTER_PROPERTY_MAXIMUM_OUTPUT_PACKET_SIZE,
                    "AudioConverterGetProperty(maximum output packet size)",
                )
            }

            /// Wraps `AudioConverterGetProperty`.
            pub fn applicable_encode_bit_rates(&self) -> Result<Vec<AudioValueRange>> {
                get_property_array(
                    self.raw,
                    AUDIO_CONVERTER_PROPERTY_APPLICABLE_ENCODE_BIT_RATES,
                    "AudioConverterGetProperty(applicable encode bit rates)",
                )
            }

            /// Wraps `AudioConverterGetProperty`.
            pub fn applicable_encode_sample_rates(&self) -> Result<Vec<AudioValueRange>> {
                get_property_array(
                    self.raw,
                    AUDIO_CONVERTER_PROPERTY_APPLICABLE_ENCODE_SAMPLE_RATES,
                    "AudioConverterGetProperty(applicable encode sample rates)",
                )
            }

            /// Wraps `AudioConverterGetProperty`.
            pub fn prime_info(&self) -> Result<AudioConverterPrimeInfo> {
                get_property_typed(
                    self.raw,
                    AUDIO_CONVERTER_PROPERTY_PRIME_INFO,
                    "AudioConverterGetProperty(prime info)",
                )
            }

            /// Wraps `AudioConverterSetProperty`.
            pub fn set_prime_info(&self, prime_info: &AudioConverterPrimeInfo) -> Result<()> {
                set_property_typed(
                    self.raw,
                    AUDIO_CONVERTER_PROPERTY_PRIME_INFO,
                    prime_info,
                    "AudioConverterSetProperty(prime info)",
                )
            }

            /// Wraps `AudioConverterGetProperty`.
            pub fn calculate_input_buffer_size(&self, output_byte_size: u32) -> Result<u32> {
                calculate_buffer_size(
                    self.raw,
                    AUDIO_CONVERTER_PROPERTY_CALCULATE_INPUT_BUFFER_SIZE,
                    output_byte_size,
                    "AudioConverterGetProperty(calculate input buffer size)",
                )
            }

            /// Wraps `AudioConverterGetProperty`.
            pub fn calculate_output_buffer_size(&self, input_byte_size: u32) -> Result<u32> {
                calculate_buffer_size(
                    self.raw,
                    AUDIO_CONVERTER_PROPERTY_CALCULATE_OUTPUT_BUFFER_SIZE,
                    input_byte_size,
                    "AudioConverterGetProperty(calculate output buffer size)",
                )
            }

            /// Wraps `AudioConverterConvertBuffer`.
            pub fn convert_buffer(
                &self,
                input: &[u8],
                output_byte_capacity: u32,
            ) -> Result<Vec<u8>> {
                let input_len = u32::try_from(input.len()).map_err(|_| {
                    AudioToolboxError::message(
                        "AudioConverterConvertBuffer",
                        "input buffer exceeds UInt32::MAX bytes",
                    )
                })?;
                let mut output_len = output_byte_capacity;
                let mut output = vec![0_u8; output_byte_capacity as usize];
                let status = unsafe {
                    ffi::audio_converter::at_audio_converter_convert_buffer(
                        self.raw.cast(),
                        input_len,
                        input.as_ptr().cast(),
                        &mut output_len,
                        output.as_mut_ptr().cast(),
                    )
                };
                status_to_result("AudioConverterConvertBuffer", status)?;
                output.truncate(output_len as usize);
                Ok(output)
            }

            /// Wraps `AudioConverterConvertComplexBuffer`.
            pub fn convert_complex_buffer(
                &self,
                number_pcm_frames: u32,
                input_data: &OwnedAudioBufferList,
                output_data: &mut OwnedAudioBufferList,
            ) -> Result<()> {
                let operation = "AudioConverterConvertComplexBuffer";
                input_data.check_valid_frames(
                    operation,
                    &self.current_input_stream_description()?,
                    number_pcm_frames,
                )?;
                let byte_size = output_data.check_frames(
                    operation,
                    &self.current_output_stream_description()?,
                    number_pcm_frames,
                )?;
                output_data.set_all_data_byte_sizes(byte_size);
                let input = input_data.raw_const();
                let mut output = output_data.raw_mut();
                let status = unsafe {
                    ffi::audio_converter::at_audio_converter_convert_complex_buffer(
                        self.raw.cast(),
                        number_pcm_frames,
                        input.as_ptr(),
                        output.as_mut_ptr(),
                    )
                };
                output_data.absorb(&output);
                status_to_result(operation, status)
            }
        }
    };
}

impl_converter_methods!(AudioConverter);
impl_converter_methods!(BorrowedAudioConverter<'_>);

impl Drop for AudioConverter {
    fn drop(&mut self) {
        self.release();
    }
}

fn get_property_typed<T: AudioProperty>(
    raw: AudioConverterRef,
    property_id: AudioConverterPropertyId,
    operation: &'static str,
) -> Result<T> {
    read_property(operation, |data, size| unsafe {
        ffi::audio_converter::at_audio_converter_get_property(raw.cast(), property_id, size, data)
    })
}

fn set_property_typed<T: AudioProperty>(
    raw: AudioConverterRef,
    property_id: AudioConverterPropertyId,
    value: &T,
    operation: &'static str,
) -> Result<()> {
    let size = property_byte_size::<T>(operation)?;
    let status = unsafe {
        ffi::audio_converter::at_audio_converter_set_property(
            raw.cast(),
            property_id,
            size,
            std::ptr::from_ref(value).cast(),
        )
    };
    status_to_result(operation, status)
}

fn calculate_buffer_size(
    raw: AudioConverterRef,
    property_id: AudioConverterPropertyId,
    byte_size: u32,
    operation: &'static str,
) -> Result<u32> {
    let mut value = byte_size;
    let expected = property_byte_size::<u32>(operation)?;
    let mut size = expected;
    let status = unsafe {
        ffi::audio_converter::at_audio_converter_get_property(
            raw.cast(),
            property_id,
            &raw mut size,
            std::ptr::from_mut(&mut value).cast(),
        )
    };
    status_to_result(operation, status)?;
    if size != expected {
        return Err(AudioToolboxError::message(
            operation,
            format!("property returned {size} bytes, expected {expected}"),
        ));
    }
    Ok(value)
}

fn get_property_array<T: AudioProperty>(
    raw: AudioConverterRef,
    property_id: AudioConverterPropertyId,
    operation: &'static str,
) -> Result<Vec<T>> {
    let mut byte_size = 0_u32;
    let status = unsafe {
        ffi::audio_converter::at_audio_converter_get_property_info(
            raw.cast(),
            property_id,
            &raw mut byte_size,
            std::ptr::null_mut(),
        )
    };
    status_to_result(operation, status)?;
    read_property_array(operation, byte_size, |data, size| unsafe {
        ffi::audio_converter::at_audio_converter_get_property(raw.cast(), property_id, size, data)
    })
}
