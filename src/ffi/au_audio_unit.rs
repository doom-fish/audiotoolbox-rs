use crate::{AUAudioFrameCount, AudioComponentDescription, AudioComponentInstantiationOptions};
use std::ffi::{c_char, c_void};

unsafe extern "C" {
    /// Raw binding for `AUAudioUnitNew`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AUAudioUnitNew`.
    pub fn at_au_audio_unit_new(
        description: *const AudioComponentDescription,
        options: AudioComponentInstantiationOptions,
        out_handle: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> bool;
    /// Raw binding for `AUAudioUnitRelease`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AUAudioUnitRelease`.
    pub fn at_au_audio_unit_release(handle: *mut c_void);
    /// Raw binding for `AUAudioUnitComponentDescription`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AUAudioUnitComponentDescription`.
    pub fn at_au_audio_unit_component_description(
        handle: *mut c_void,
        out_description: *mut AudioComponentDescription,
    ) -> bool;
    /// Raw binding for `AUAudioUnitCopyComponentName`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AUAudioUnitCopyComponentName`.
    pub fn at_au_audio_unit_copy_component_name(handle: *mut c_void) -> *mut c_char;
    /// Raw binding for `AUAudioUnitCopyAudioUnitName`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AUAudioUnitCopyAudioUnitName`.
    pub fn at_au_audio_unit_copy_audio_unit_name(handle: *mut c_void) -> *mut c_char;
    /// Raw binding for `AUAudioUnitCopyManufacturerName`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AUAudioUnitCopyManufacturerName`.
    pub fn at_au_audio_unit_copy_manufacturer_name(handle: *mut c_void) -> *mut c_char;
    /// Raw binding for `AUAudioUnitInputBusCount`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AUAudioUnitInputBusCount`.
    pub fn at_au_audio_unit_input_bus_count(handle: *mut c_void) -> u64;
    /// Raw binding for `AUAudioUnitOutputBusCount`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AUAudioUnitOutputBusCount`.
    pub fn at_au_audio_unit_output_bus_count(handle: *mut c_void) -> u64;
    /// Raw binding for `AUAudioUnitAllocateRenderResources`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AUAudioUnitAllocateRenderResources`.
    pub fn at_au_audio_unit_allocate_render_resources(
        handle: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> bool;
    /// Raw binding for `AUAudioUnitDeallocateRenderResources`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AUAudioUnitDeallocateRenderResources`.
    pub fn at_au_audio_unit_deallocate_render_resources(handle: *mut c_void);
    /// Raw binding for `AUAudioUnitRenderResourcesAllocated`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AUAudioUnitRenderResourcesAllocated`.
    pub fn at_au_audio_unit_render_resources_allocated(handle: *mut c_void) -> bool;
    /// Raw binding for `AUAudioUnitReset`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AUAudioUnitReset`.
    pub fn at_au_audio_unit_reset(handle: *mut c_void);
    /// Raw binding for `AUAudioUnitMaximumFramesToRender`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AUAudioUnitMaximumFramesToRender`.
    pub fn at_au_audio_unit_maximum_frames_to_render(handle: *mut c_void) -> AUAudioFrameCount;
    /// Raw binding for `AUAudioUnitSetMaximumFramesToRender`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AUAudioUnitSetMaximumFramesToRender`.
    pub fn at_au_audio_unit_set_maximum_frames_to_render(
        handle: *mut c_void,
        maximum_frames: AUAudioFrameCount,
    );
}
