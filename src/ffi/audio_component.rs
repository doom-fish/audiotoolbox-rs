use crate::{
    AudioComponentDescription, AudioComponentInstanceRef, AudioComponentRef,
    AudioComponentValidationResult, Boolean, CFDictionaryRef, OSStatus,
};
use std::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn at_audio_component_count(description: *const AudioComponentDescription) -> u32;
    pub fn at_audio_component_find_next(
        previous_raw: *mut c_void,
        description: *const AudioComponentDescription,
    ) -> *mut c_void;
    pub fn at_audio_component_raw(handle: *mut c_void) -> *mut c_void;
    pub fn at_audio_component_release(handle: *mut c_void);
    pub fn at_audio_component_copy_name(
        raw_component: *mut c_void,
        out_status: *mut OSStatus,
    ) -> *mut c_char;
    pub fn at_audio_component_get_description(
        raw_component: *mut c_void,
        out_description: *mut AudioComponentDescription,
    ) -> OSStatus;
    pub fn at_audio_component_get_version(
        raw_component: *mut c_void,
        out_version: *mut u32,
    ) -> OSStatus;
    pub fn at_audio_component_instance_new(
        raw_component: *mut c_void,
        out_handle: *mut *mut c_void,
    ) -> OSStatus;
    pub fn at_audio_component_instance_raw(handle: *mut c_void) -> *mut c_void;
    pub fn at_audio_component_instance_release(handle: *mut c_void);
    pub fn at_audio_component_instance_get_component(raw_instance: *mut c_void) -> *mut c_void;
    #[link_name = "AudioComponentInstanceCanDo"]
    pub fn at_audio_component_instance_can_do(
        raw_instance: AudioComponentInstanceRef,
        selector_id: i16,
    ) -> Boolean;
    #[link_name = "AudioComponentCopyConfigurationInfo"]
    pub fn at_audio_component_copy_configuration_info(
        raw_component: AudioComponentRef,
        out_configuration_info: *mut CFDictionaryRef,
    ) -> OSStatus;
    #[link_name = "AudioComponentValidate"]
    pub fn at_audio_component_validate(
        raw_component: AudioComponentRef,
        validation_parameters: CFDictionaryRef,
        out_validation_result: *mut AudioComponentValidationResult,
    ) -> OSStatus;
}
