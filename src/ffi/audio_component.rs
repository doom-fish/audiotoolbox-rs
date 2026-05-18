use crate::{
    AudioComponentDescription, AudioComponentInstanceRef, AudioComponentRef,
    AudioComponentValidationResult, Boolean, CFDictionaryRef, OSStatus,
};
use std::ffi::{c_char, c_void};

unsafe extern "C" {
    /// Raw binding for `AudioComponentCount`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioComponentCount`.
    pub fn at_audio_component_count(description: *const AudioComponentDescription) -> u32;
    /// Raw binding for `AudioComponentFindNext`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioComponentFindNext`.
    pub fn at_audio_component_find_next(
        previous_raw: *mut c_void,
        description: *const AudioComponentDescription,
    ) -> *mut c_void;
    /// Raw binding for `AudioComponentRaw`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioComponentRaw`.
    pub fn at_audio_component_raw(handle: *mut c_void) -> *mut c_void;
    /// Raw binding for `AudioComponentRelease`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioComponentRelease`.
    pub fn at_audio_component_release(handle: *mut c_void);
    /// Raw binding for `AudioComponentCopyName`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioComponentCopyName`.
    pub fn at_audio_component_copy_name(
        raw_component: *mut c_void,
        out_status: *mut OSStatus,
    ) -> *mut c_char;
    /// Raw binding for `AudioComponentGetDescription`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioComponentGetDescription`.
    pub fn at_audio_component_get_description(
        raw_component: *mut c_void,
        out_description: *mut AudioComponentDescription,
    ) -> OSStatus;
    /// Raw binding for `AudioComponentGetVersion`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioComponentGetVersion`.
    pub fn at_audio_component_get_version(
        raw_component: *mut c_void,
        out_version: *mut u32,
    ) -> OSStatus;
    /// Raw binding for `AudioComponentInstanceNew`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioComponentInstanceNew`.
    pub fn at_audio_component_instance_new(
        raw_component: *mut c_void,
        out_handle: *mut *mut c_void,
    ) -> OSStatus;
    /// Raw binding for `AudioComponentInstanceRaw`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioComponentInstanceRaw`.
    pub fn at_audio_component_instance_raw(handle: *mut c_void) -> *mut c_void;
    /// Raw binding for `AudioComponentInstanceRelease`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioComponentInstanceRelease`.
    pub fn at_audio_component_instance_release(handle: *mut c_void);
    /// Raw binding for `AudioComponentInstanceGetComponent`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioComponentInstanceGetComponent`.
    pub fn at_audio_component_instance_get_component(raw_instance: *mut c_void) -> *mut c_void;
    #[link_name = "AudioComponentInstanceCanDo"]
    /// Raw binding for `AudioComponentInstanceCanDo`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioComponentInstanceCanDo`.
    pub fn at_audio_component_instance_can_do(
        raw_instance: AudioComponentInstanceRef,
        selector_id: i16,
    ) -> Boolean;
    #[link_name = "AudioComponentCopyConfigurationInfo"]
    /// Raw binding for `AudioComponentCopyConfigurationInfo`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioComponentCopyConfigurationInfo`.
    pub fn at_audio_component_copy_configuration_info(
        raw_component: AudioComponentRef,
        out_configuration_info: *mut CFDictionaryRef,
    ) -> OSStatus;
    #[link_name = "AudioComponentValidate"]
    /// Raw binding for `AudioComponentValidate`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AudioComponentValidate`.
    pub fn at_audio_component_validate(
        raw_component: AudioComponentRef,
        validation_parameters: CFDictionaryRef,
        out_validation_result: *mut AudioComponentValidationResult,
    ) -> OSStatus;
}
