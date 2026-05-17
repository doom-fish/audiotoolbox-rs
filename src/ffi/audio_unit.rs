use crate::{
    AURenderCallback, AudioBufferList1, AudioTimeStamp, AudioUnitElement, AudioUnitParameterEvent,
    AudioUnitParameterId, AudioUnitParameterValue, AudioUnitPropertyId,
    AudioUnitPropertyListenerProc, AudioUnitRenderActionFlags, AudioUnitScope, Boolean, OSStatus,
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
    #[link_name = "AudioUnitGetPropertyInfo"]
    pub fn at_audio_unit_get_property_info(
        raw_unit: *mut c_void,
        property_id: AudioUnitPropertyId,
        scope: AudioUnitScope,
        element: AudioUnitElement,
        out_data_size: *mut u32,
        out_writable: *mut Boolean,
    ) -> OSStatus;
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
    #[link_name = "AudioUnitAddPropertyListener"]
    pub fn at_audio_unit_add_property_listener(
        raw_unit: *mut c_void,
        property_id: AudioUnitPropertyId,
        proc: AudioUnitPropertyListenerProc,
        user_data: *mut c_void,
    ) -> OSStatus;
    #[link_name = "AudioUnitRemovePropertyListenerWithUserData"]
    pub fn at_audio_unit_remove_property_listener_with_user_data(
        raw_unit: *mut c_void,
        property_id: AudioUnitPropertyId,
        proc: AudioUnitPropertyListenerProc,
        user_data: *mut c_void,
    ) -> OSStatus;
    #[link_name = "AudioUnitAddRenderNotify"]
    pub fn at_audio_unit_add_render_notify(
        raw_unit: *mut c_void,
        proc: AURenderCallback,
        user_data: *mut c_void,
    ) -> OSStatus;
    #[link_name = "AudioUnitRemoveRenderNotify"]
    pub fn at_audio_unit_remove_render_notify(
        raw_unit: *mut c_void,
        proc: AURenderCallback,
        user_data: *mut c_void,
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
    #[link_name = "AudioUnitScheduleParameters"]
    pub fn at_audio_unit_schedule_parameters(
        raw_unit: *mut c_void,
        event_count: u32,
        events: *const AudioUnitParameterEvent,
    ) -> OSStatus;
    #[link_name = "AudioUnitRender"]
    pub fn at_audio_unit_render(
        raw_unit: *mut c_void,
        io_action_flags: *mut AudioUnitRenderActionFlags,
        in_time_stamp: *const AudioTimeStamp,
        in_output_bus_number: u32,
        in_number_frames: u32,
        io_data: *mut AudioBufferList1,
    ) -> OSStatus;
}
