use crate::{
    ffi, internal::status_to_result, AudioBuffer, AudioBufferList, AudioClassDescription,
    AudioConverterPrimeInfo, AudioConverterPropertyId, AudioConverterRef,
    AudioStreamBasicDescription, AudioStreamPacketDescription, AudioToolboxError, AudioValueRange,
    Result, AUDIO_CONVERTER_PROPERTY_APPLICABLE_ENCODE_BIT_RATES,
    AUDIO_CONVERTER_PROPERTY_APPLICABLE_ENCODE_SAMPLE_RATES,
    AUDIO_CONVERTER_PROPERTY_CALCULATE_INPUT_BUFFER_SIZE,
    AUDIO_CONVERTER_PROPERTY_CALCULATE_OUTPUT_BUFFER_SIZE,
    AUDIO_CONVERTER_PROPERTY_CURRENT_INPUT_STREAM_DESCRIPTION,
    AUDIO_CONVERTER_PROPERTY_CURRENT_OUTPUT_STREAM_DESCRIPTION,
    AUDIO_CONVERTER_PROPERTY_ENCODE_BIT_RATE, AUDIO_CONVERTER_PROPERTY_MAXIMUM_OUTPUT_PACKET_SIZE,
    AUDIO_CONVERTER_PROPERTY_PRIME_INFO,
};
use std::{marker::PhantomData, mem::MaybeUninit};

#[derive(Debug, Clone, Copy)]
pub struct AudioConversionInput<'a> {
    pub data: &'a [u8],
    pub packet_count: u32,
    pub packet_descriptions: Option<&'a [AudioStreamPacketDescription]>,
    pub channels: u32,
}

#[derive(Debug, Clone)]
pub struct AudioConversionOutput {
    pub data: Vec<u8>,
    pub packet_count: u32,
    pub packet_descriptions: Vec<AudioStreamPacketDescription>,
}

#[derive(Debug)]
pub struct AudioConverter {
    handle: *mut std::ffi::c_void,
    raw: AudioConverterRef,
}

#[derive(Debug, Clone, Copy)]
pub struct BorrowedAudioConverter<'a> {
    raw: AudioConverterRef,
    _marker: PhantomData<&'a ()>,
}

impl BorrowedAudioConverter<'_> {
    pub(crate) const fn new(raw: AudioConverterRef) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    pub fn as_raw(&self) -> AudioConverterRef {
        self.raw
    }
}

impl AudioConverter {
    pub fn new(
        source_format: &AudioStreamBasicDescription,
        destination_format: &AudioStreamBasicDescription,
    ) -> Result<Self> {
        let mut handle = std::ptr::null_mut();
        let status = unsafe {
            ffi::audio_converter::at_audio_converter_new(
                source_format,
                destination_format,
                &mut handle,
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
        Ok(Self { handle, raw })
    }

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
                &mut handle,
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
        Ok(Self { handle, raw })
    }

    pub fn as_raw(&self) -> AudioConverterRef {
        self.raw
    }

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
            pub fn reset(&self) -> Result<()> {
                let status =
                    unsafe { ffi::audio_converter::at_audio_converter_reset(self.raw.cast()) };
                status_to_result("AudioConverterReset", status)
            }

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

            pub fn current_input_stream_description(&self) -> Result<AudioStreamBasicDescription> {
                get_property_typed(
                    self.raw,
                    AUDIO_CONVERTER_PROPERTY_CURRENT_INPUT_STREAM_DESCRIPTION,
                    "AudioConverterGetProperty(current input stream description)",
                )
            }

            pub fn current_output_stream_description(&self) -> Result<AudioStreamBasicDescription> {
                get_property_typed(
                    self.raw,
                    AUDIO_CONVERTER_PROPERTY_CURRENT_OUTPUT_STREAM_DESCRIPTION,
                    "AudioConverterGetProperty(current output stream description)",
                )
            }

            pub fn encode_bit_rate(&self) -> Result<u32> {
                get_property_typed(
                    self.raw,
                    AUDIO_CONVERTER_PROPERTY_ENCODE_BIT_RATE,
                    "AudioConverterGetProperty(encode bit rate)",
                )
            }

            pub fn set_encode_bit_rate(&self, bits_per_second: u32) -> Result<()> {
                set_property_typed(
                    self.raw,
                    AUDIO_CONVERTER_PROPERTY_ENCODE_BIT_RATE,
                    &bits_per_second,
                    "AudioConverterSetProperty(encode bit rate)",
                )
            }

            pub fn maximum_output_packet_size(&self) -> Result<u32> {
                get_property_typed(
                    self.raw,
                    AUDIO_CONVERTER_PROPERTY_MAXIMUM_OUTPUT_PACKET_SIZE,
                    "AudioConverterGetProperty(maximum output packet size)",
                )
            }

            pub fn applicable_encode_bit_rates(&self) -> Result<Vec<AudioValueRange>> {
                get_property_array(
                    self.raw,
                    AUDIO_CONVERTER_PROPERTY_APPLICABLE_ENCODE_BIT_RATES,
                    "AudioConverterGetProperty(applicable encode bit rates)",
                )
            }

            pub fn applicable_encode_sample_rates(&self) -> Result<Vec<AudioValueRange>> {
                get_property_array(
                    self.raw,
                    AUDIO_CONVERTER_PROPERTY_APPLICABLE_ENCODE_SAMPLE_RATES,
                    "AudioConverterGetProperty(applicable encode sample rates)",
                )
            }

            pub fn prime_info(&self) -> Result<AudioConverterPrimeInfo> {
                get_property_typed(
                    self.raw,
                    AUDIO_CONVERTER_PROPERTY_PRIME_INFO,
                    "AudioConverterGetProperty(prime info)",
                )
            }

            pub fn set_prime_info(&self, prime_info: &AudioConverterPrimeInfo) -> Result<()> {
                set_property_typed(
                    self.raw,
                    AUDIO_CONVERTER_PROPERTY_PRIME_INFO,
                    prime_info,
                    "AudioConverterSetProperty(prime info)",
                )
            }

            pub fn calculate_input_buffer_size(&self, output_byte_size: u32) -> Result<u32> {
                let mut byte_size = output_byte_size;
                let mut size = u32::try_from(std::mem::size_of::<u32>()).expect("u32 fits in u32");
                let status = unsafe {
                    ffi::audio_converter::at_audio_converter_get_property(
                        self.raw.cast(),
                        AUDIO_CONVERTER_PROPERTY_CALCULATE_INPUT_BUFFER_SIZE,
                        &mut size,
                        std::ptr::from_mut(&mut byte_size).cast(),
                    )
                };
                status_to_result(
                    "AudioConverterGetProperty(calculate input buffer size)",
                    status,
                )?;
                Ok(byte_size)
            }

            pub fn calculate_output_buffer_size(&self, input_byte_size: u32) -> Result<u32> {
                let mut byte_size = input_byte_size;
                let mut size = u32::try_from(std::mem::size_of::<u32>()).expect("u32 fits in u32");
                let status = unsafe {
                    ffi::audio_converter::at_audio_converter_get_property(
                        self.raw.cast(),
                        AUDIO_CONVERTER_PROPERTY_CALCULATE_OUTPUT_BUFFER_SIZE,
                        &mut size,
                        std::ptr::from_mut(&mut byte_size).cast(),
                    )
                };
                status_to_result(
                    "AudioConverterGetProperty(calculate output buffer size)",
                    status,
                )?;
                Ok(byte_size)
            }

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

            pub fn convert_complex_buffer(
                &self,
                number_pcm_frames: u32,
                input_data: &AudioBufferList<1>,
                output_data: &mut AudioBufferList<1>,
            ) -> Result<()> {
                let status = unsafe {
                    ffi::audio_converter::at_audio_converter_convert_complex_buffer(
                        self.raw.cast(),
                        number_pcm_frames,
                        input_data,
                        output_data,
                    )
                };
                status_to_result("AudioConverterConvertComplexBuffer", status)
            }

            pub fn fill_complex_buffer_once(
                &self,
                input: AudioConversionInput<'_>,
                output_packet_capacity: u32,
            ) -> Result<AudioConversionOutput> {
                if output_packet_capacity == 0 {
                    return Err(AudioToolboxError::message(
                        "AudioConverterFillComplexBuffer",
                        "output_packet_capacity must be greater than zero",
                    ));
                }
                if let Some(packet_descriptions) = input.packet_descriptions {
                    if packet_descriptions.len() < input.packet_count as usize {
                        return Err(AudioToolboxError::message(
                            "AudioConverterFillComplexBuffer",
                            "packet description count is smaller than packet_count",
                        ));
                    }
                }

                let output_format = self.current_output_stream_description()?;
                let bytes_per_packet = if output_format.mBytesPerPacket == 0 {
                    self.maximum_output_packet_size()?
                } else {
                    output_format.mBytesPerPacket
                };
                let output_byte_capacity = (bytes_per_packet as usize)
                    .checked_mul(output_packet_capacity as usize)
                    .ok_or_else(|| {
                        AudioToolboxError::message(
                            "AudioConverterFillComplexBuffer",
                            "output buffer size overflowed usize",
                        )
                    })?;
                let mut output_bytes = vec![0_u8; output_byte_capacity];
                let mut output_packet_descriptions =
                    vec![AudioStreamPacketDescription::default(); output_packet_capacity as usize];
                let mut output_buffer_list = AudioBufferList {
                    mNumberBuffers: 1,
                    mBuffers: [AudioBuffer {
                        mNumberChannels: output_format.mChannelsPerFrame,
                        mDataByteSize: u32::try_from(output_byte_capacity).map_err(|_| {
                            AudioToolboxError::message(
                                "AudioConverterFillComplexBuffer",
                                "output buffer exceeds UInt32::MAX bytes",
                            )
                        })?,
                        mData: output_bytes.as_mut_ptr().cast(),
                    }],
                };
                let mut output_packets = output_packet_capacity;
                let input_len = u32::try_from(input.data.len()).map_err(|_| {
                    AudioToolboxError::message(
                        "AudioConverterFillComplexBuffer",
                        "input buffer exceeds UInt32::MAX bytes",
                    )
                })?;
                let input_packet_descriptions = input
                    .packet_descriptions
                    .map_or(std::ptr::null(), <[AudioStreamPacketDescription]>::as_ptr);
                let output_packet_description_ptr = if output_format.uses_packet_descriptions() {
                    output_packet_descriptions.as_mut_ptr()
                } else {
                    std::ptr::null_mut()
                };

                let status = unsafe {
                    ffi::audio_converter::at_audio_converter_fill_complex_buffer_once(
                        self.raw.cast(),
                        input.data.as_ptr(),
                        input_len,
                        input.packet_count,
                        input_packet_descriptions,
                        input.channels,
                        &mut output_packets,
                        &mut output_buffer_list,
                        output_packet_description_ptr,
                    )
                };
                status_to_result("AudioConverterFillComplexBuffer", status)?;

                output_bytes.truncate(output_buffer_list.mBuffers[0].mDataByteSize as usize);
                if output_format.uses_packet_descriptions() {
                    output_packet_descriptions.truncate(output_packets as usize);
                } else {
                    output_packet_descriptions.clear();
                }

                Ok(AudioConversionOutput {
                    data: output_bytes,
                    packet_count: output_packets,
                    packet_descriptions: output_packet_descriptions,
                })
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

fn get_property_typed<T: Copy>(
    raw: AudioConverterRef,
    property_id: AudioConverterPropertyId,
    operation: &'static str,
) -> Result<T> {
    let mut value = MaybeUninit::<T>::uninit();
    let mut size = u32::try_from(std::mem::size_of::<T>()).expect("typed property fits in u32");
    let status = unsafe {
        ffi::audio_converter::at_audio_converter_get_property(
            raw.cast(),
            property_id,
            &mut size,
            value.as_mut_ptr().cast(),
        )
    };
    status_to_result(operation, status)?;
    Ok(unsafe { value.assume_init() })
}

fn set_property_typed<T: Copy>(
    raw: AudioConverterRef,
    property_id: AudioConverterPropertyId,
    value: &T,
    operation: &'static str,
) -> Result<()> {
    let size = u32::try_from(std::mem::size_of::<T>()).expect("typed property fits in u32");
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

fn get_property_array<T: Copy>(
    raw: AudioConverterRef,
    property_id: AudioConverterPropertyId,
    operation: &'static str,
) -> Result<Vec<T>> {
    let mut byte_size = 0_u32;
    let status = unsafe {
        ffi::audio_converter::at_audio_converter_get_property_info(
            raw.cast(),
            property_id,
            &mut byte_size,
            std::ptr::null_mut(),
        )
    };
    status_to_result(operation, status)?;
    if byte_size == 0 {
        return Ok(Vec::new());
    }

    let element_size = std::mem::size_of::<T>();
    if byte_size as usize % element_size != 0 {
        return Err(AudioToolboxError::message(
            operation,
            "property payload is not an integral number of elements",
        ));
    }

    let mut bytes = vec![0_u8; byte_size as usize];
    let status = unsafe {
        ffi::audio_converter::at_audio_converter_get_property(
            raw.cast(),
            property_id,
            &mut byte_size,
            bytes.as_mut_ptr().cast(),
        )
    };
    status_to_result(operation, status)?;
    let (prefix, values, suffix) = unsafe { bytes.align_to::<T>() };
    if !prefix.is_empty() || !suffix.is_empty() {
        return Err(AudioToolboxError::message(
            operation,
            "property payload is not aligned for the requested element type",
        ));
    }
    Ok(values.to_vec())
}
