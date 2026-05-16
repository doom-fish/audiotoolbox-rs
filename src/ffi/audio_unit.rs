use crate::{
    AudioUnitElement, AudioUnitParameterId, AudioUnitPropertyId, AudioUnitScope,
    AudioUnitParameterValue, OSStatus,
};
use std::ffi::c_void;

unsafe extern "C" {
    pub fn at_audio_unit_new(
        component_type: u32,
        component_sub_type: u32,
        component_manufacturer: u32,
        out_handle: *mut *mut c_void,
    ) -> OSStatus;
    pub fn at_audio_unit_raw(handle: *mut c_void) -> *mut c_void;
    pub fn at_audio_unit_release(handle: *mut c_void);
    pub fn at_audio_unit_initialize(raw_unit: *mut c_void) -> OSStatus;
    pub fn at_audio_unit_uninitialize(raw_unit: *mut c_void) -> OSStatus;
    pub fn at_audio_output_unit_start(raw_unit: *mut c_void) -> OSStatus;
    pub fn at_audio_output_unit_stop(raw_unit: *mut c_void) -> OSStatus;
    pub fn at_audio_unit_get_property(
        raw_unit: *mut c_void,
        property_id: AudioUnitPropertyId,
        scope: AudioUnitScope,
        element: AudioUnitElement,
        io_data_size: *mut u32,
        out_property_data: *mut c_void,
    ) -> OSStatus;
    pub fn at_audio_unit_set_property(
        raw_unit: *mut c_void,
        property_id: AudioUnitPropertyId,
        scope: AudioUnitScope,
        element: AudioUnitElement,
        data_size: u32,
        property_data: *const c_void,
    ) -> OSStatus;
    pub fn at_audio_unit_get_parameter(
        raw_unit: *mut c_void,
        parameter_id: AudioUnitParameterId,
        scope: AudioUnitScope,
        element: AudioUnitElement,
        out_value: *mut AudioUnitParameterValue,
    ) -> OSStatus;
    pub fn at_audio_unit_set_parameter(
        raw_unit: *mut c_void,
        parameter_id: AudioUnitParameterId,
        scope: AudioUnitScope,
        element: AudioUnitElement,
        value: AudioUnitParameterValue,
        buffer_offset_in_frames: u32,
    ) -> OSStatus;
}
