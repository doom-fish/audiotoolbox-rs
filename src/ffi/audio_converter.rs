use crate::{
    AudioBufferList1, AudioClassDescription, AudioConverterPropertyId, AudioStreamBasicDescription,
    AudioStreamPacketDescription, Boolean, OSStatus,
};
use std::ffi::c_void;

unsafe extern "C" {
    pub fn at_audio_converter_new(
        source_format: *const AudioStreamBasicDescription,
        destination_format: *const AudioStreamBasicDescription,
        out_handle: *mut *mut c_void,
    ) -> OSStatus;
    pub fn at_audio_converter_new_specific(
        source_format: *const AudioStreamBasicDescription,
        destination_format: *const AudioStreamBasicDescription,
        class_descriptions: *const AudioClassDescription,
        class_description_count: u32,
        out_handle: *mut *mut c_void,
    ) -> OSStatus;
    pub fn at_audio_converter_raw(handle: *mut c_void) -> *mut c_void;
    pub fn at_audio_converter_release(handle: *mut c_void);
    pub fn at_audio_converter_reset(raw_converter: *mut c_void) -> OSStatus;
    pub fn at_audio_converter_get_property_info(
        raw_converter: *mut c_void,
        property_id: AudioConverterPropertyId,
        out_size: *mut u32,
        out_writable: *mut Boolean,
    ) -> OSStatus;
    pub fn at_audio_converter_get_property(
        raw_converter: *mut c_void,
        property_id: AudioConverterPropertyId,
        io_property_data_size: *mut u32,
        out_property_data: *mut c_void,
    ) -> OSStatus;
    pub fn at_audio_converter_set_property(
        raw_converter: *mut c_void,
        property_id: AudioConverterPropertyId,
        property_data_size: u32,
        property_data: *const c_void,
    ) -> OSStatus;
    #[link_name = "AudioConverterConvertBuffer"]
    pub fn at_audio_converter_convert_buffer(
        raw_converter: *mut c_void,
        input_data_size: u32,
        input_data: *const c_void,
        io_output_data_size: *mut u32,
        out_output_data: *mut c_void,
    ) -> OSStatus;
    #[link_name = "AudioConverterConvertComplexBuffer"]
    pub fn at_audio_converter_convert_complex_buffer(
        raw_converter: *mut c_void,
        number_pcm_frames: u32,
        input_data: *const AudioBufferList1,
        out_output_data: *mut AudioBufferList1,
    ) -> OSStatus;
    pub fn at_audio_converter_fill_complex_buffer_once(
        raw_converter: *mut c_void,
        input_data: *const u8,
        input_len: u32,
        packet_count: u32,
        packet_descriptions: *const AudioStreamPacketDescription,
        channels: u32,
        io_output_packet_size: *mut u32,
        out_output_data: *mut AudioBufferList1,
        out_packet_descriptions: *mut AudioStreamPacketDescription,
    ) -> OSStatus;
}
