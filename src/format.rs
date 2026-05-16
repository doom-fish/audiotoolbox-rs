use crate::ffi::{
    fourcc, AudioComponentDescription, AudioFormatFlags, AudioStreamBasicDescription,
    AUDIO_FORMAT_FLAGS_NATIVE_ENDIAN, AUDIO_FORMAT_FLAG_IS_NON_INTERLEAVED,
    AUDIO_FORMAT_FLAG_IS_PACKED, AUDIO_FORMAT_FLAG_IS_SIGNED_INTEGER, AUDIO_FORMAT_LINEAR_PCM,
    LINEAR_PCM_FORMAT_FLAG_IS_FLOAT,
};

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
        Self::new(component_type, component_sub_type, fourcc(*b"appl"))
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
