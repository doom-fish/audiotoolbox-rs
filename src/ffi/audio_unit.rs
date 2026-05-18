use crate::{
    AURenderCallback, AudioBufferList1, AudioTimeStamp, AudioUnitElement, AudioUnitParameterEvent,
    AudioUnitParameterId, AudioUnitParameterValue, AudioUnitPropertyId,
    AudioUnitPropertyListenerProc, AudioUnitRenderActionFlags, AudioUnitScope, Boolean, OSStatus,
};
use std::ffi::c_void;

unsafe extern "C" {
    /// Raw binding for `AudioUnitNew`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioUnitNew`.
    pub fn at_audio_unit_new(
        component_type: u32,
        component_sub_type: u32,
        component_manufacturer: u32,
        out_handle: *mut *mut c_void,
    ) -> OSStatus;
    /// Raw binding for `AudioUnitRaw`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioUnitRaw`.
    pub fn at_audio_unit_raw(handle: *mut c_void) -> *mut c_void;
    /// Raw binding for `AudioUnitRelease`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioUnitRelease`.
    pub fn at_audio_unit_release(handle: *mut c_void);
    /// Raw binding for `AudioUnitInitialize`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioUnitInitialize`.
    pub fn at_audio_unit_initialize(raw_unit: *mut c_void) -> OSStatus;
    /// Raw binding for `AudioUnitUninitialize`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioUnitUninitialize`.
    pub fn at_audio_unit_uninitialize(raw_unit: *mut c_void) -> OSStatus;
    /// Raw binding for `AudioOutputUnitStart`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioOutputUnitStart`.
    pub fn at_audio_output_unit_start(raw_unit: *mut c_void) -> OSStatus;
    /// Raw binding for `AudioOutputUnitStop`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioOutputUnitStop`.
    pub fn at_audio_output_unit_stop(raw_unit: *mut c_void) -> OSStatus;
    #[link_name = "AudioUnitGetPropertyInfo"]
    /// Raw binding for `AudioUnitGetPropertyInfo`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioUnitGetPropertyInfo`.
    pub fn at_audio_unit_get_property_info(
        raw_unit: *mut c_void,
        property_id: AudioUnitPropertyId,
        scope: AudioUnitScope,
        element: AudioUnitElement,
        out_data_size: *mut u32,
        out_writable: *mut Boolean,
    ) -> OSStatus;
    /// Raw binding for `AudioUnitGetProperty`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioUnitGetProperty`.
    pub fn at_audio_unit_get_property(
        raw_unit: *mut c_void,
        property_id: AudioUnitPropertyId,
        scope: AudioUnitScope,
        element: AudioUnitElement,
        io_data_size: *mut u32,
        out_property_data: *mut c_void,
    ) -> OSStatus;
    /// Raw binding for `AudioUnitSetProperty`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioUnitSetProperty`.
    pub fn at_audio_unit_set_property(
        raw_unit: *mut c_void,
        property_id: AudioUnitPropertyId,
        scope: AudioUnitScope,
        element: AudioUnitElement,
        data_size: u32,
        property_data: *const c_void,
    ) -> OSStatus;
    #[link_name = "AudioUnitAddPropertyListener"]
    /// Raw binding for `AudioUnitAddPropertyListener`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioUnitAddPropertyListener`.
    pub fn at_audio_unit_add_property_listener(
        raw_unit: *mut c_void,
        property_id: AudioUnitPropertyId,
        proc: AudioUnitPropertyListenerProc,
        user_data: *mut c_void,
    ) -> OSStatus;
    #[link_name = "AudioUnitRemovePropertyListenerWithUserData"]
    /// Raw binding for `AudioUnitRemovePropertyListenerWithUserData`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioUnitRemovePropertyListenerWithUserData`.
    pub fn at_audio_unit_remove_property_listener_with_user_data(
        raw_unit: *mut c_void,
        property_id: AudioUnitPropertyId,
        proc: AudioUnitPropertyListenerProc,
        user_data: *mut c_void,
    ) -> OSStatus;
    #[link_name = "AudioUnitAddRenderNotify"]
    /// Raw binding for `AudioUnitAddRenderNotify`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioUnitAddRenderNotify`.
    pub fn at_audio_unit_add_render_notify(
        raw_unit: *mut c_void,
        proc: AURenderCallback,
        user_data: *mut c_void,
    ) -> OSStatus;
    #[link_name = "AudioUnitRemoveRenderNotify"]
    /// Raw binding for `AudioUnitRemoveRenderNotify`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioUnitRemoveRenderNotify`.
    pub fn at_audio_unit_remove_render_notify(
        raw_unit: *mut c_void,
        proc: AURenderCallback,
        user_data: *mut c_void,
    ) -> OSStatus;
    /// Raw binding for `AudioUnitGetParameter`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioUnitGetParameter`.
    pub fn at_audio_unit_get_parameter(
        raw_unit: *mut c_void,
        parameter_id: AudioUnitParameterId,
        scope: AudioUnitScope,
        element: AudioUnitElement,
        out_value: *mut AudioUnitParameterValue,
    ) -> OSStatus;
    /// Raw binding for `AudioUnitSetParameter`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioUnitSetParameter`.
    pub fn at_audio_unit_set_parameter(
        raw_unit: *mut c_void,
        parameter_id: AudioUnitParameterId,
        scope: AudioUnitScope,
        element: AudioUnitElement,
        value: AudioUnitParameterValue,
        buffer_offset_in_frames: u32,
    ) -> OSStatus;
    #[link_name = "AudioUnitScheduleParameters"]
    /// Raw binding for `AudioUnitScheduleParameters`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioUnitScheduleParameters`.
    pub fn at_audio_unit_schedule_parameters(
        raw_unit: *mut c_void,
        event_count: u32,
        events: *const AudioUnitParameterEvent,
    ) -> OSStatus;
    #[link_name = "AudioUnitRender"]
    /// Raw binding for `AudioUnitRender`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioUnitRender`.
    pub fn at_audio_unit_render(
        raw_unit: *mut c_void,
        io_action_flags: *mut AudioUnitRenderActionFlags,
        in_time_stamp: *const AudioTimeStamp,
        in_output_bus_number: u32,
        in_number_frames: u32,
        io_data: *mut AudioBufferList1,
    ) -> OSStatus;
}
