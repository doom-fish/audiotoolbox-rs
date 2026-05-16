use crate::{
    ffi, AudioStreamBasicDescription, AudioStreamPacketDescription, AudioToolboxError, Result,
};
use std::{ffi::c_void, marker::PhantomData, mem::MaybeUninit};

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
    raw: ffi::AudioConverterRef,
}

#[derive(Debug, Clone, Copy)]
pub struct BorrowedAudioConverter<'a> {
    raw: ffi::AudioConverterRef,
    _marker: PhantomData<&'a ()>,
}

impl BorrowedAudioConverter<'_> {
    pub(crate) const fn new(raw: ffi::AudioConverterRef) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    pub fn as_raw(&self) -> ffi::AudioConverterRef {
        self.raw
    }
}

impl AudioConverter {
    pub fn new(
        source_format: &AudioStreamBasicDescription,
        destination_format: &AudioStreamBasicDescription,
    ) -> Result<Self> {
        let mut raw = MaybeUninit::uninit();
        let status =
            unsafe { ffi::AudioConverterNew(source_format, destination_format, raw.as_mut_ptr()) };
        status_to_result("AudioConverterNew", status)?;
        Ok(Self {
            raw: unsafe { raw.assume_init() },
        })
    }

    pub fn new_specific(
        source_format: &AudioStreamBasicDescription,
        destination_format: &AudioStreamBasicDescription,
        class_descriptions: &[ffi::AudioClassDescription],
    ) -> Result<Self> {
        let mut raw = MaybeUninit::uninit();
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
            ffi::AudioConverterNewSpecific(
                source_format,
                destination_format,
                class_count,
                class_description_ptr,
                raw.as_mut_ptr(),
            )
        };
        status_to_result("AudioConverterNewSpecific", status)?;
        Ok(Self {
            raw: unsafe { raw.assume_init() },
        })
    }

    pub fn as_raw(&self) -> ffi::AudioConverterRef {
        self.raw
    }

    pub fn close(mut self) -> Result<()> {
        let raw = self.raw;
        self.raw = std::ptr::null_mut();
        let status = unsafe { ffi::AudioConverterDispose(raw) };
        status_to_result("AudioConverterDispose", status)
    }
}

macro_rules! impl_converter_methods {
    ($ty:ty) => {
        impl $ty {
            pub fn reset(&self) -> Result<()> {
                let status = unsafe { ffi::AudioConverterReset(self.raw) };
                status_to_result("AudioConverterReset", status)
            }

            pub fn property_info(
                &self,
                property_id: ffi::AudioConverterPropertyId,
            ) -> Result<(u32, bool)> {
                let mut size = 0_u32;
                let mut writable = 0_u8;
                let status = unsafe {
                    ffi::AudioConverterGetPropertyInfo(
                        self.raw,
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
                    ffi::AUDIO_CONVERTER_PROPERTY_CURRENT_INPUT_STREAM_DESCRIPTION,
                    "AudioConverterGetProperty(current input stream description)",
                )
            }

            pub fn current_output_stream_description(&self) -> Result<AudioStreamBasicDescription> {
                get_property_typed(
                    self.raw,
                    ffi::AUDIO_CONVERTER_PROPERTY_CURRENT_OUTPUT_STREAM_DESCRIPTION,
                    "AudioConverterGetProperty(current output stream description)",
                )
            }

            pub fn encode_bit_rate(&self) -> Result<u32> {
                get_property_typed(
                    self.raw,
                    ffi::AUDIO_CONVERTER_PROPERTY_ENCODE_BIT_RATE,
                    "AudioConverterGetProperty(encode bit rate)",
                )
            }

            pub fn set_encode_bit_rate(&self, bits_per_second: u32) -> Result<()> {
                set_property_typed(
                    self.raw,
                    ffi::AUDIO_CONVERTER_PROPERTY_ENCODE_BIT_RATE,
                    &bits_per_second,
                    "AudioConverterSetProperty(encode bit rate)",
                )
            }

            pub fn maximum_output_packet_size(&self) -> Result<u32> {
                get_property_typed(
                    self.raw,
                    ffi::AUDIO_CONVERTER_PROPERTY_MAXIMUM_OUTPUT_PACKET_SIZE,
                    "AudioConverterGetProperty(maximum output packet size)",
                )
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
                let mut output_buffer_list = ffi::AudioBufferList {
                    mNumberBuffers: 1,
                    mBuffers: [ffi::AudioBuffer {
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
                let mut input_state = OneShotInputState::from_input(input);
                let output_packet_description_ptr = if output_format.uses_packet_descriptions() {
                    output_packet_descriptions.as_mut_ptr()
                } else {
                    std::ptr::null_mut()
                };

                let status = unsafe {
                    ffi::AudioConverterFillComplexBuffer(
                        self.raw,
                        one_shot_input_proc,
                        std::ptr::from_mut(&mut input_state).cast::<c_void>(),
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
        if !self.raw.is_null() {
            let _ = unsafe { ffi::AudioConverterDispose(self.raw) };
        }
    }
}

#[derive(Debug)]
struct OneShotInputState {
    data_ptr: *const u8,
    data_len: usize,
    packet_count: u32,
    packet_descriptions_ptr: *const AudioStreamPacketDescription,
    channels: u32,
    provided: bool,
}

impl OneShotInputState {
    fn from_input(input: AudioConversionInput<'_>) -> Self {
        Self {
            data_ptr: input.data.as_ptr(),
            data_len: input.data.len(),
            packet_count: input.packet_count,
            packet_descriptions_ptr: input
                .packet_descriptions
                .map_or(std::ptr::null(), <[AudioStreamPacketDescription]>::as_ptr),
            channels: input.channels,
            provided: false,
        }
    }
}

extern "C" fn one_shot_input_proc(
    _converter: ffi::AudioConverterRef,
    io_number_data_packets: *mut u32,
    io_data: *mut ffi::AudioBufferList1,
    out_data_packet_description: *mut *mut AudioStreamPacketDescription,
    in_user_data: *mut c_void,
) -> ffi::OSStatus {
    let state = unsafe { &mut *in_user_data.cast::<OneShotInputState>() };
    if state.provided || state.data_len == 0 {
        unsafe { *io_number_data_packets = 0 };
        return ffi::NO_ERR;
    }

    state.provided = true;
    unsafe {
        *io_number_data_packets = state.packet_count;
        (*io_data).mNumberBuffers = 1;
        (*io_data).mBuffers[0].mNumberChannels = state.channels;
        (*io_data).mBuffers[0].mDataByteSize = state.data_len as u32;
        (*io_data).mBuffers[0].mData = state.data_ptr.cast_mut().cast();
        if !out_data_packet_description.is_null() {
            *out_data_packet_description = state.packet_descriptions_ptr.cast_mut();
        }
    }
    ffi::NO_ERR
}

fn get_property_typed<T: Copy>(
    raw: ffi::AudioConverterRef,
    property_id: ffi::AudioConverterPropertyId,
    operation: &'static str,
) -> Result<T> {
    let mut value = MaybeUninit::<T>::uninit();
    let mut size = u32::try_from(std::mem::size_of::<T>()).expect("typed property fits in u32");
    let status = unsafe {
        ffi::AudioConverterGetProperty(raw, property_id, &mut size, value.as_mut_ptr().cast())
    };
    status_to_result(operation, status)?;
    Ok(unsafe { value.assume_init() })
}

fn set_property_typed<T: Copy>(
    raw: ffi::AudioConverterRef,
    property_id: ffi::AudioConverterPropertyId,
    value: &T,
    operation: &'static str,
) -> Result<()> {
    let size = u32::try_from(std::mem::size_of::<T>()).expect("typed property fits in u32");
    let status = unsafe {
        ffi::AudioConverterSetProperty(raw, property_id, size, std::ptr::from_ref(value).cast())
    };
    status_to_result(operation, status)
}

fn status_to_result(operation: &'static str, status: ffi::OSStatus) -> Result<()> {
    if status == ffi::NO_ERR {
        Ok(())
    } else {
        Err(AudioToolboxError::from_status(operation, status))
    }
}
