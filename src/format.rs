use crate::{
    ffi, internal::status_to_result, AudioBalanceFade, AudioClassDescription,
    AudioComponentDescription, AudioFormatFlags, AudioFormatId, AudioFormatInfo,
    AudioFormatListItem, AudioFormatPropertyId, AudioPanningInfo, AudioStreamBasicDescription,
    AudioValueRange, Result, AUDIO_COMPONENT_MANUFACTURER_APPLE, AUDIO_FORMAT_FLAGS_NATIVE_ENDIAN,
    AUDIO_FORMAT_FLAG_IS_NON_INTERLEAVED, AUDIO_FORMAT_FLAG_IS_PACKED,
    AUDIO_FORMAT_FLAG_IS_SIGNED_INTEGER, AUDIO_FORMAT_LINEAR_PCM,
    AUDIO_FORMAT_PROPERTY_AVAILABLE_ENCODE_BIT_RATES,
    AUDIO_FORMAT_PROPERTY_AVAILABLE_ENCODE_SAMPLE_RATES, AUDIO_FORMAT_PROPERTY_BALANCE_FADE,
    AUDIO_FORMAT_PROPERTY_DECODERS, AUDIO_FORMAT_PROPERTY_DECODE_FORMAT_IDS,
    AUDIO_FORMAT_PROPERTY_ENCODERS, AUDIO_FORMAT_PROPERTY_ENCODE_FORMAT_IDS,
    AUDIO_FORMAT_PROPERTY_FIRST_PLAYABLE_FORMAT_FROM_LIST,
    AUDIO_FORMAT_PROPERTY_FORMAT_EMPLOYS_DEPENDENT_PACKETS, AUDIO_FORMAT_PROPERTY_FORMAT_INFO,
    AUDIO_FORMAT_PROPERTY_FORMAT_IS_EXTERNALLY_FRAMED, AUDIO_FORMAT_PROPERTY_FORMAT_IS_VBR,
    AUDIO_FORMAT_PROPERTY_FORMAT_LIST, AUDIO_FORMAT_PROPERTY_OUTPUT_FORMAT_LIST,
    LINEAR_PCM_FORMAT_FLAG_IS_FLOAT,
};
use std::{ffi::c_void, mem::MaybeUninit};

#[derive(Debug)]
/// Namespace wrapper for `AudioFormatGetProperty` and related AudioToolbox.framework APIs.
pub struct AudioFormat;

impl AudioFormat {
    /// Wraps `AudioFormatGetProperty`.
    pub fn format_info(
        mut description: AudioStreamBasicDescription,
    ) -> Result<AudioStreamBasicDescription> {
        let mut size = u32::try_from(std::mem::size_of::<AudioStreamBasicDescription>())
            .expect("AudioStreamBasicDescription fits in u32");
        // SAFETY: Safe FFI call to AudioFormatGetProperty with valid stack-allocated data
        // structures. The mutable reference is valid for the duration of the call, and
        // the result status code is checked before the data is used.
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

    /// Wraps `AudioFormatGetProperty`.
    pub fn format_is_vbr(description: &AudioStreamBasicDescription) -> Result<bool> {
        Ok(get_u32_with_specifier(
            AUDIO_FORMAT_PROPERTY_FORMAT_IS_VBR,
            description,
            "AudioFormatGetProperty(format is VBR)",
        )? != 0)
    }

    /// Wraps `AudioFormatGetProperty`.
    pub fn format_is_externally_framed(description: &AudioStreamBasicDescription) -> Result<bool> {
        Ok(get_u32_with_specifier(
            AUDIO_FORMAT_PROPERTY_FORMAT_IS_EXTERNALLY_FRAMED,
            description,
            "AudioFormatGetProperty(format is externally framed)",
        )? != 0)
    }

    /// Wraps `AudioFormatGetProperty`.
    pub fn format_employs_dependent_packets(
        description: &AudioStreamBasicDescription,
    ) -> Result<bool> {
        Ok(get_u32_with_specifier(
            AUDIO_FORMAT_PROPERTY_FORMAT_EMPLOYS_DEPENDENT_PACKETS,
            description,
            "AudioFormatGetProperty(format employs dependent packets)",
        )? != 0)
    }

    /// Wraps `AudioFormatGetProperty`.
    pub fn encode_format_ids() -> Result<Vec<AudioFormatId>> {
        get_array::<AudioFormatId>(
            AUDIO_FORMAT_PROPERTY_ENCODE_FORMAT_IDS,
            std::ptr::null(),
            0,
            "AudioFormatGetProperty(encode format IDs)",
        )
    }

    /// Wraps `AudioFormatGetProperty`.
    pub fn decode_format_ids() -> Result<Vec<AudioFormatId>> {
        get_array::<AudioFormatId>(
            AUDIO_FORMAT_PROPERTY_DECODE_FORMAT_IDS,
            std::ptr::null(),
            0,
            "AudioFormatGetProperty(decode format IDs)",
        )
    }

    /// Wraps `AudioFormatGetProperty`.
    pub fn encoders(format_id: AudioFormatId) -> Result<Vec<AudioClassDescription>> {
        get_array::<AudioClassDescription>(
            AUDIO_FORMAT_PROPERTY_ENCODERS,
            std::ptr::from_ref(&format_id).cast(),
            u32::try_from(std::mem::size_of::<AudioFormatId>()).expect("format ID fits in u32"),
            "AudioFormatGetProperty(encoders)",
        )
    }

    /// Wraps `AudioFormatGetProperty`.
    pub fn decoders(format_id: AudioFormatId) -> Result<Vec<AudioClassDescription>> {
        get_array::<AudioClassDescription>(
            AUDIO_FORMAT_PROPERTY_DECODERS,
            std::ptr::from_ref(&format_id).cast(),
            u32::try_from(std::mem::size_of::<AudioFormatId>()).expect("format ID fits in u32"),
            "AudioFormatGetProperty(decoders)",
        )
    }

    /// Wraps `AudioFormatGetProperty`.
    pub fn available_encode_bit_rates(format_id: AudioFormatId) -> Result<Vec<AudioValueRange>> {
        get_array::<AudioValueRange>(
            AUDIO_FORMAT_PROPERTY_AVAILABLE_ENCODE_BIT_RATES,
            std::ptr::from_ref(&format_id).cast(),
            u32::try_from(std::mem::size_of::<AudioFormatId>()).expect("format ID fits in u32"),
            "AudioFormatGetProperty(available encode bit rates)",
        )
    }

    /// Wraps `AudioFormatGetProperty`.
    pub fn available_encode_sample_rates(format_id: AudioFormatId) -> Result<Vec<AudioValueRange>> {
        get_array::<AudioValueRange>(
            AUDIO_FORMAT_PROPERTY_AVAILABLE_ENCODE_SAMPLE_RATES,
            std::ptr::from_ref(&format_id).cast(),
            u32::try_from(std::mem::size_of::<AudioFormatId>()).expect("format ID fits in u32"),
            "AudioFormatGetProperty(available encode sample rates)",
        )
    }

    /// Wraps `AudioFormatGetPropertyInfo`.
    pub fn property_info<T>(
        property_id: AudioFormatPropertyId,
        specifier: Option<&T>,
    ) -> Result<u32> {
        let (specifier_ptr, specifier_size) =
            specifier.map_or((std::ptr::null(), 0), |specifier| {
                (
                    std::ptr::from_ref(specifier).cast::<c_void>(),
                    u32::try_from(std::mem::size_of::<T>()).expect("specifier size fits in u32"),
                )
            });
        let mut size = 0_u32;
        let status = unsafe {
            ffi::audio_format::at_audio_format_get_property_info(
                property_id,
                specifier_size,
                specifier_ptr,
                &mut size,
            )
        };
        status_to_result("AudioFormatGetPropertyInfo", status)?;
        Ok(size)
    }

    /// Wraps `AudioFormatGetProperty`.
    pub fn format_list(info: &AudioFormatInfo) -> Result<Vec<AudioFormatListItem>> {
        get_array::<AudioFormatListItem>(
            AUDIO_FORMAT_PROPERTY_FORMAT_LIST,
            std::ptr::from_ref(info).cast(),
            u32::try_from(std::mem::size_of::<AudioFormatInfo>())
                .expect("AudioFormatInfo fits in u32"),
            "AudioFormatGetProperty(format list)",
        )
    }

    /// Wraps `AudioFormatGetProperty`.
    pub fn output_format_list(info: &AudioFormatInfo) -> Result<Vec<AudioFormatListItem>> {
        get_array::<AudioFormatListItem>(
            AUDIO_FORMAT_PROPERTY_OUTPUT_FORMAT_LIST,
            std::ptr::from_ref(info).cast(),
            u32::try_from(std::mem::size_of::<AudioFormatInfo>())
                .expect("AudioFormatInfo fits in u32"),
            "AudioFormatGetProperty(output format list)",
        )
    }

    /// Wraps `AudioFormatGetProperty`.
    pub fn first_playable_format_from_list(items: &[AudioFormatListItem]) -> Result<u32> {
        if items.is_empty() {
            return Err(crate::AudioToolboxError::message(
                "AudioFormatGetProperty(first playable format from list)",
                "at least one AudioFormatListItem is required",
            ));
        }
        let mut index = 0_u32;
        let mut size = u32::try_from(std::mem::size_of::<u32>()).expect("u32 fits in u32");
        let specifier_size = u32::try_from(std::mem::size_of_val(items))
            .expect("AudioFormatListItem slice fits in u32");
        let status = unsafe {
            ffi::audio_format::at_audio_format_get_property(
                AUDIO_FORMAT_PROPERTY_FIRST_PLAYABLE_FORMAT_FROM_LIST,
                specifier_size,
                items.as_ptr().cast(),
                &mut size,
                std::ptr::from_mut(&mut index).cast(),
            )
        };
        status_to_result(
            "AudioFormatGetProperty(first playable format from list)",
            status,
        )?;
        Ok(index)
    }

    /// Wraps `AudioFormatGetProperty`.
    pub fn balance_fade(mut balance_fade: AudioBalanceFade) -> Result<AudioBalanceFade> {
        let mut size = u32::try_from(std::mem::size_of::<AudioBalanceFade>())
            .expect("AudioBalanceFade fits in u32");
        let status = unsafe {
            ffi::audio_format::at_audio_format_get_property(
                AUDIO_FORMAT_PROPERTY_BALANCE_FADE,
                u32::try_from(std::mem::size_of::<AudioBalanceFade>())
                    .expect("AudioBalanceFade fits in u32"),
                std::ptr::from_ref(&balance_fade).cast(),
                &mut size,
                std::ptr::from_mut(&mut balance_fade).cast(),
            )
        };
        status_to_result("AudioFormatGetProperty(balance fade)", status)?;
        Ok(balance_fade)
    }

    /// Wraps `AudioFormatPanningMatrixSize`.
    pub fn panning_matrix_size(info: &AudioPanningInfo) -> Result<u32> {
        Self::property_info(crate::AUDIO_FORMAT_PROPERTY_PANNING_MATRIX, Some(info))
    }
}

fn get_u32_with_specifier<T>(
    property_id: AudioFormatPropertyId,
    specifier: &T,
    operation: &'static str,
) -> Result<u32> {
    let mut value = MaybeUninit::<u32>::uninit();
    let mut size = u32::try_from(std::mem::size_of::<u32>()).expect("u32 fits in u32");
    // SAFETY: Safe FFI call to AudioFormatGetProperty with valid data. The MaybeUninit
    // buffer is properly sized, and result is checked before assume_init().
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
    // SAFETY: Status checked above ensures the value was initialized by AudioToolbox.
    Ok(unsafe { value.assume_init() })
}

fn get_array<T: Copy>(
    property_id: AudioFormatPropertyId,
    specifier: *const c_void,
    specifier_size: u32,
    operation: &'static str,
) -> Result<Vec<T>> {
    let mut byte_size = 0_u32;
    // SAFETY: Safe FFI call to AudioFormatGetPropertyInfo with valid inputs.
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
    // SAFETY: Safe FFI call to AudioFormatGetProperty with properly allocated buffer.
    // Capacity is verified above to be an integral number of T elements.
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

    // SAFETY: Result status checked above; byte_size is verified to be a multiple of element_size.
    unsafe { values.set_len(byte_size as usize / element_size) };
    Ok(values)
}

/// Wraps `AudioFormatIsPrintableFourcc`.
pub fn is_printable_fourcc(value: u32) -> bool {
    value
        .to_be_bytes()
        .iter()
        .all(|byte| byte.is_ascii_graphic() || *byte == b' ')
}

/// Wraps `AudioFormatFourccToString`.
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
    /// Wraps `AudioComponentDescriptionNew`.
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

    /// Wraps `AudioComponentDescriptionWildcard`.
    pub const fn wildcard() -> Self {
        Self::new(0, 0, 0)
    }

    /// Wraps `AudioComponentDescriptionApple`.
    pub const fn apple(component_type: u32, component_sub_type: u32) -> Self {
        Self::new(
            component_type,
            component_sub_type,
            AUDIO_COMPONENT_MANUFACTURER_APPLE,
        )
    }
}

impl AudioStreamBasicDescription {
    /// Wraps `AudioStreamBasicDescriptionLinearPCMF32`.
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

    /// Wraps `AudioStreamBasicDescriptionLinearPCMI16`.
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

    /// Wraps `AudioStreamBasicDescriptionChannelCount`.
    pub const fn channel_count(self) -> u32 {
        self.mChannelsPerFrame
    }

    /// Wraps `AudioStreamBasicDescriptionBytesPerFrame`.
    pub const fn bytes_per_frame(self) -> u32 {
        self.mBytesPerFrame
    }

    /// Wraps `AudioStreamBasicDescriptionBytesPerPacket`.
    pub const fn bytes_per_packet(self) -> u32 {
        self.mBytesPerPacket
    }

    /// Wraps `AudioStreamBasicDescriptionFramesPerPacket`.
    pub const fn frames_per_packet(self) -> u32 {
        self.mFramesPerPacket
    }

    /// Wraps `AudioStreamBasicDescriptionBitsPerChannel`.
    pub const fn bits_per_channel(self) -> u32 {
        self.mBitsPerChannel
    }

    /// Wraps `AudioStreamBasicDescriptionIsLinearPCM`.
    pub const fn is_linear_pcm(self) -> bool {
        self.mFormatID == AUDIO_FORMAT_LINEAR_PCM
    }

    /// Wraps `AudioStreamBasicDescriptionIsInterleaved`.
    pub const fn is_interleaved(self) -> bool {
        (self.mFormatFlags & AUDIO_FORMAT_FLAG_IS_NON_INTERLEAVED) == 0
    }

    /// Wraps `AudioStreamBasicDescriptionUsesPacketDescriptions`.
    pub const fn uses_packet_descriptions(self) -> bool {
        self.mBytesPerPacket == 0 || self.mFramesPerPacket == 0
    }

    /// Wraps `AudioStreamBasicDescriptionInterleavedBytesForFrames`.
    pub const fn interleaved_bytes_for_frames(self, frames: u32) -> Option<usize> {
        if self.mBytesPerFrame == 0 {
            None
        } else {
            Some((self.mBytesPerFrame as usize) * (frames as usize))
        }
    }

    /// Wraps `AudioStreamBasicDescriptionLinearPCMFlags`.
    pub const fn linear_pcm_flags(self) -> AudioFormatFlags {
        self.mFormatFlags
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn printable_fourcc_accepts_graphic_ascii() {
        assert!(is_printable_fourcc(u32::from_be_bytes(*b"lpcm")));
    }

    #[test]
    fn printable_fourcc_rejects_control_bytes() {
        assert!(!is_printable_fourcc(0x0001_0203));
    }

    #[test]
    fn fourcc_to_string_preserves_printable_bytes() {
        assert_eq!(fourcc_to_string(u32::from_be_bytes(*b"lpcm")), "lpcm");
    }

    #[test]
    fn fourcc_to_string_replaces_non_printable_bytes() {
        assert_eq!(fourcc_to_string(0x6c00_706d), "l.pm");
    }

    #[test]
    fn component_description_new_zeroes_flags() {
        let description = AudioComponentDescription::new(1, 2, 3);

        assert_eq!(description.component_type, 1);
        assert_eq!(description.component_sub_type, 2);
        assert_eq!(description.component_manufacturer, 3);
        assert_eq!(description.component_flags, 0);
        assert_eq!(description.component_flags_mask, 0);
    }

    #[test]
    fn component_description_wildcard_zeroes_all_fields() {
        assert_eq!(
            AudioComponentDescription::wildcard(),
            AudioComponentDescription::new(0, 0, 0),
        );
    }

    #[test]
    fn component_description_apple_uses_apple_manufacturer() {
        let description = AudioComponentDescription::apple(11, 22);

        assert_eq!(description.component_type, 11);
        assert_eq!(description.component_sub_type, 22);
        assert_eq!(
            description.component_manufacturer,
            AUDIO_COMPONENT_MANUFACTURER_APPLE,
        );
    }

    #[test]
    fn linear_pcm_f32_interleaved_has_expected_layout() {
        let description = AudioStreamBasicDescription::linear_pcm_f32(48_000.0, 2, true);

        assert!((description.mSampleRate - 48_000.0).abs() < f64::EPSILON);
        assert_eq!(description.mFormatID, AUDIO_FORMAT_LINEAR_PCM);
        assert_eq!(description.channel_count(), 2);
        assert_eq!(description.bytes_per_frame(), 8);
        assert_eq!(description.bytes_per_packet(), 8);
        assert_eq!(description.frames_per_packet(), 1);
        assert_eq!(description.bits_per_channel(), 32);
        assert!(description.is_linear_pcm());
        assert!(description.is_interleaved());
        assert_eq!(description.interleaved_bytes_for_frames(4), Some(32));
        assert_ne!(description.linear_pcm_flags() & LINEAR_PCM_FORMAT_FLAG_IS_FLOAT, 0);
    }

    #[test]
    fn linear_pcm_f32_non_interleaved_sets_non_interleaved_flag() {
        let description = AudioStreamBasicDescription::linear_pcm_f32(48_000.0, 2, false);

        assert!(!description.is_interleaved());
        assert_eq!(description.bytes_per_frame(), 4);
        assert_ne!(
            description.linear_pcm_flags() & AUDIO_FORMAT_FLAG_IS_NON_INTERLEAVED,
            0,
        );
    }

    #[test]
    fn linear_pcm_i16_uses_signed_integer_layout() {
        let description = AudioStreamBasicDescription::linear_pcm_i16(44_100.0, 2, true);

        assert_eq!(description.bits_per_channel(), 16);
        assert_eq!(description.bytes_per_frame(), 4);
        assert_eq!(description.bytes_per_packet(), 4);
        assert_ne!(
            description.linear_pcm_flags() & AUDIO_FORMAT_FLAG_IS_SIGNED_INTEGER,
            0,
        );
        assert!(description.is_linear_pcm());
    }

    #[test]
    fn uses_packet_descriptions_when_packet_fields_are_zero() {
        let bytes_per_packet_zero = AudioStreamBasicDescription {
            mSampleRate: 48_000.0,
            mFormatID: AUDIO_FORMAT_LINEAR_PCM,
            mFormatFlags: 0,
            mBytesPerPacket: 0,
            mFramesPerPacket: 1,
            mBytesPerFrame: 4,
            mChannelsPerFrame: 1,
            mBitsPerChannel: 32,
            mReserved: 0,
        };
        let frames_per_packet_zero = AudioStreamBasicDescription {
            mBytesPerPacket: 4,
            mFramesPerPacket: 0,
            ..bytes_per_packet_zero
        };

        assert!(bytes_per_packet_zero.uses_packet_descriptions());
        assert!(frames_per_packet_zero.uses_packet_descriptions());
    }

    #[test]
    fn interleaved_bytes_for_frames_returns_none_when_bytes_per_frame_is_zero() {
        let description = AudioStreamBasicDescription {
            mSampleRate: 48_000.0,
            mFormatID: AUDIO_FORMAT_LINEAR_PCM,
            mFormatFlags: 0,
            mBytesPerPacket: 0,
            mFramesPerPacket: 1,
            mBytesPerFrame: 0,
            mChannelsPerFrame: 1,
            mBitsPerChannel: 32,
            mReserved: 0,
        };

        assert_eq!(description.interleaved_bytes_for_frames(10), None);
    }
}
