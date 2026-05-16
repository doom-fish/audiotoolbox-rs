use crate::{
    ffi,
    internal::status_to_result,
    AudioClassDescription,
    AudioComponentDescription,
    AudioFormatFlags,
    AudioFormatId,
    AudioFormatPropertyId,
    AudioStreamBasicDescription,
    AudioValueRange,
    Result,
    AUDIO_COMPONENT_MANUFACTURER_APPLE,
    AUDIO_FORMAT_FLAG_IS_NON_INTERLEAVED,
    AUDIO_FORMAT_FLAG_IS_PACKED,
    AUDIO_FORMAT_FLAG_IS_SIGNED_INTEGER,
    AUDIO_FORMAT_FLAGS_NATIVE_ENDIAN,
    AUDIO_FORMAT_LINEAR_PCM,
    AUDIO_FORMAT_PROPERTY_AVAILABLE_ENCODE_BIT_RATES,
    AUDIO_FORMAT_PROPERTY_AVAILABLE_ENCODE_SAMPLE_RATES,
    AUDIO_FORMAT_PROPERTY_DECODE_FORMAT_IDS,
    AUDIO_FORMAT_PROPERTY_DECODERS,
    AUDIO_FORMAT_PROPERTY_ENCODE_FORMAT_IDS,
    AUDIO_FORMAT_PROPERTY_ENCODERS,
    AUDIO_FORMAT_PROPERTY_FORMAT_EMPLOYS_DEPENDENT_PACKETS,
    AUDIO_FORMAT_PROPERTY_FORMAT_INFO,
    AUDIO_FORMAT_PROPERTY_FORMAT_IS_EXTERNALLY_FRAMED,
    AUDIO_FORMAT_PROPERTY_FORMAT_IS_VBR,
    LINEAR_PCM_FORMAT_FLAG_IS_FLOAT,
};
use std::{ffi::c_void, mem::MaybeUninit};

pub struct AudioFormat;

impl AudioFormat {
    pub fn format_info(
        mut description: AudioStreamBasicDescription,
    ) -> Result<AudioStreamBasicDescription> {
        let mut size = u32::try_from(std::mem::size_of::<AudioStreamBasicDescription>())
            .expect("AudioStreamBasicDescription fits in u32");
        let status = unsafe {
            ffi::audio_format::at_audio_format_get_property(
                AUDIO_FORMAT_PROPERTY_FORMAT_INFO,
                0,
                std::ptr::null(),
                &mut size,
                std::ptr::from_mut(&mut description).cast::<c_void>(),
            )
        };
        status_to_result("AudioFormatGetProperty(format info)", status)?;
        Ok(description)
    }

    pub fn format_is_vbr(description: &AudioStreamBasicDescription) -> Result<bool> {
        Ok(get_u32_with_specifier(
            AUDIO_FORMAT_PROPERTY_FORMAT_IS_VBR,
            description,
            "AudioFormatGetProperty(format is VBR)",
        )? != 0)
    }

    pub fn format_is_externally_framed(description: &AudioStreamBasicDescription) -> Result<bool> {
        Ok(get_u32_with_specifier(
            AUDIO_FORMAT_PROPERTY_FORMAT_IS_EXTERNALLY_FRAMED,
            description,
            "AudioFormatGetProperty(format is externally framed)",
        )? != 0)
    }

    pub fn format_employs_dependent_packets(
        description: &AudioStreamBasicDescription,
    ) -> Result<bool> {
        Ok(get_u32_with_specifier(
            AUDIO_FORMAT_PROPERTY_FORMAT_EMPLOYS_DEPENDENT_PACKETS,
            description,
            "AudioFormatGetProperty(format employs dependent packets)",
        )? != 0)
    }

    pub fn encode_format_ids() -> Result<Vec<AudioFormatId>> {
        get_array::<AudioFormatId>(
            AUDIO_FORMAT_PROPERTY_ENCODE_FORMAT_IDS,
            std::ptr::null(),
            0,
            "AudioFormatGetProperty(encode format IDs)",
        )
    }

    pub fn decode_format_ids() -> Result<Vec<AudioFormatId>> {
        get_array::<AudioFormatId>(
            AUDIO_FORMAT_PROPERTY_DECODE_FORMAT_IDS,
            std::ptr::null(),
            0,
            "AudioFormatGetProperty(decode format IDs)",
        )
    }

    pub fn encoders(format_id: AudioFormatId) -> Result<Vec<AudioClassDescription>> {
        get_array::<AudioClassDescription>(
            AUDIO_FORMAT_PROPERTY_ENCODERS,
            std::ptr::from_ref(&format_id).cast(),
            u32::try_from(std::mem::size_of::<AudioFormatId>()).expect("format ID fits in u32"),
            "AudioFormatGetProperty(encoders)",
        )
    }

    pub fn decoders(format_id: AudioFormatId) -> Result<Vec<AudioClassDescription>> {
        get_array::<AudioClassDescription>(
            AUDIO_FORMAT_PROPERTY_DECODERS,
            std::ptr::from_ref(&format_id).cast(),
            u32::try_from(std::mem::size_of::<AudioFormatId>()).expect("format ID fits in u32"),
            "AudioFormatGetProperty(decoders)",
        )
    }

    pub fn available_encode_bit_rates(format_id: AudioFormatId) -> Result<Vec<AudioValueRange>> {
        get_array::<AudioValueRange>(
            AUDIO_FORMAT_PROPERTY_AVAILABLE_ENCODE_BIT_RATES,
            std::ptr::from_ref(&format_id).cast(),
            u32::try_from(std::mem::size_of::<AudioFormatId>()).expect("format ID fits in u32"),
            "AudioFormatGetProperty(available encode bit rates)",
        )
    }

    pub fn available_encode_sample_rates(format_id: AudioFormatId) -> Result<Vec<AudioValueRange>> {
        get_array::<AudioValueRange>(
            AUDIO_FORMAT_PROPERTY_AVAILABLE_ENCODE_SAMPLE_RATES,
            std::ptr::from_ref(&format_id).cast(),
            u32::try_from(std::mem::size_of::<AudioFormatId>()).expect("format ID fits in u32"),
            "AudioFormatGetProperty(available encode sample rates)",
        )
    }
}

fn get_u32_with_specifier<T>(
    property_id: AudioFormatPropertyId,
    specifier: &T,
    operation: &'static str,
) -> Result<u32> {
    let mut value = MaybeUninit::<u32>::uninit();
    let mut size = u32::try_from(std::mem::size_of::<u32>()).expect("u32 fits in u32");
    let status = unsafe {
        ffi::audio_format::at_audio_format_get_property(
            property_id,
            u32::try_from(std::mem::size_of::<T>()).expect("specifier size fits in u32"),
            std::ptr::from_ref(specifier).cast(),
            &mut size,
            value.as_mut_ptr().cast(),
        )
    };
    status_to_result(operation, status)?;
    Ok(unsafe { value.assume_init() })
}

fn get_array<T: Copy>(
    property_id: AudioFormatPropertyId,
    specifier: *const c_void,
    specifier_size: u32,
    operation: &'static str,
) -> Result<Vec<T>> {
    let mut byte_size = 0_u32;
    let status = unsafe {
        ffi::audio_format::at_audio_format_get_property_info(
            property_id,
            specifier_size,
            specifier,
            &mut byte_size,
        )
    };
    status_to_result(operation, status)?;

    if byte_size == 0 {
        return Ok(Vec::new());
    }

    let element_size = std::mem::size_of::<T>();
    if byte_size as usize % element_size != 0 {
        return Err(crate::AudioToolboxError::message(
            operation,
            "property payload is not an integral number of elements",
        ));
    }

    let mut values = Vec::<T>::with_capacity(byte_size as usize / element_size);
    let status = unsafe {
        ffi::audio_format::at_audio_format_get_property(
            property_id,
            specifier_size,
            specifier,
            &mut byte_size,
            values.as_mut_ptr().cast(),
        )
    };
    status_to_result(operation, status)?;

    unsafe { values.set_len(byte_size as usize / element_size) };
    Ok(values)
}

pub fn is_printable_fourcc(value: u32) -> bool {
    value
        .to_be_bytes()
        .iter()
        .all(|byte| byte.is_ascii_graphic() || *byte == b' ')
}

pub fn fourcc_to_string(value: u32) -> String {
    value
        .to_be_bytes()
        .iter()
        .map(|byte| {
            if byte.is_ascii_graphic() || *byte == b' ' {
                char::from(*byte)
            } else {
                '.'
            }
        })
        .collect()
}

impl AudioComponentDescription {
    pub const fn new(
        component_type: u32,
        component_sub_type: u32,
        component_manufacturer: u32,
    ) -> Self {
        Self {
            component_type,
            component_sub_type,
            component_manufacturer,
            component_flags: 0,
            component_flags_mask: 0,
        }
    }

    pub const fn wildcard() -> Self {
        Self::new(0, 0, 0)
    }

    pub const fn apple(component_type: u32, component_sub_type: u32) -> Self {
        Self::new(
            component_type,
            component_sub_type,
            AUDIO_COMPONENT_MANUFACTURER_APPLE,
        )
    }
}

impl AudioStreamBasicDescription {
    pub const fn linear_pcm_f32(sample_rate: f64, channels: u32, interleaved: bool) -> Self {
        let bytes_per_channel = 4;
        let bytes_per_frame = if interleaved {
            bytes_per_channel * channels
        } else {
            bytes_per_channel
        };
        let format_flags = LINEAR_PCM_FORMAT_FLAG_IS_FLOAT
            | AUDIO_FORMAT_FLAG_IS_PACKED
            | AUDIO_FORMAT_FLAGS_NATIVE_ENDIAN
            | if interleaved {
                0
            } else {
                AUDIO_FORMAT_FLAG_IS_NON_INTERLEAVED
            };
        Self {
            mSampleRate: sample_rate,
            mFormatID: AUDIO_FORMAT_LINEAR_PCM,
            mFormatFlags: format_flags,
            mBytesPerPacket: bytes_per_frame,
            mFramesPerPacket: 1,
            mBytesPerFrame: bytes_per_frame,
            mChannelsPerFrame: channels,
            mBitsPerChannel: 32,
            mReserved: 0,
        }
    }

    pub const fn linear_pcm_i16(sample_rate: f64, channels: u32, interleaved: bool) -> Self {
        let bytes_per_channel = 2;
        let bytes_per_frame = if interleaved {
            bytes_per_channel * channels
        } else {
            bytes_per_channel
        };
        let format_flags = AUDIO_FORMAT_FLAG_IS_SIGNED_INTEGER
            | AUDIO_FORMAT_FLAG_IS_PACKED
            | AUDIO_FORMAT_FLAGS_NATIVE_ENDIAN
            | if interleaved {
                0
            } else {
                AUDIO_FORMAT_FLAG_IS_NON_INTERLEAVED
            };
        Self {
            mSampleRate: sample_rate,
            mFormatID: AUDIO_FORMAT_LINEAR_PCM,
            mFormatFlags: format_flags,
            mBytesPerPacket: bytes_per_frame,
            mFramesPerPacket: 1,
            mBytesPerFrame: bytes_per_frame,
            mChannelsPerFrame: channels,
            mBitsPerChannel: 16,
            mReserved: 0,
        }
    }

    pub const fn channel_count(self) -> u32 {
        self.mChannelsPerFrame
    }

    pub const fn bytes_per_frame(self) -> u32 {
        self.mBytesPerFrame
    }

    pub const fn bytes_per_packet(self) -> u32 {
        self.mBytesPerPacket
    }

    pub const fn frames_per_packet(self) -> u32 {
        self.mFramesPerPacket
    }

    pub const fn bits_per_channel(self) -> u32 {
        self.mBitsPerChannel
    }

    pub const fn is_linear_pcm(self) -> bool {
        self.mFormatID == AUDIO_FORMAT_LINEAR_PCM
    }

    pub const fn is_interleaved(self) -> bool {
        (self.mFormatFlags & AUDIO_FORMAT_FLAG_IS_NON_INTERLEAVED) == 0
    }

    pub const fn uses_packet_descriptions(self) -> bool {
        self.mBytesPerPacket == 0 || self.mFramesPerPacket == 0
    }

    pub const fn interleaved_bytes_for_frames(self, frames: u32) -> Option<usize> {
        if self.mBytesPerFrame == 0 {
            None
        } else {
            Some((self.mBytesPerFrame as usize) * (frames as usize))
        }
    }

    pub const fn linear_pcm_flags(self) -> AudioFormatFlags {
        self.mFormatFlags
    }
}
