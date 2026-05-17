use crate::{AUAudioFrameCount, AudioComponentDescription, AudioComponentInstantiationOptions};
use std::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn at_au_audio_unit_new(
        description: *const AudioComponentDescription,
        options: AudioComponentInstantiationOptions,
        out_handle: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> bool;
    pub fn at_au_audio_unit_release(handle: *mut c_void);
    pub fn at_au_audio_unit_component_description(
        handle: *mut c_void,
        out_description: *mut AudioComponentDescription,
    ) -> bool;
    pub fn at_au_audio_unit_copy_component_name(handle: *mut c_void) -> *mut c_char;
    pub fn at_au_audio_unit_copy_audio_unit_name(handle: *mut c_void) -> *mut c_char;
    pub fn at_au_audio_unit_copy_manufacturer_name(handle: *mut c_void) -> *mut c_char;
    pub fn at_au_audio_unit_input_bus_count(handle: *mut c_void) -> u64;
    pub fn at_au_audio_unit_output_bus_count(handle: *mut c_void) -> u64;
    pub fn at_au_audio_unit_allocate_render_resources(
        handle: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> bool;
    pub fn at_au_audio_unit_deallocate_render_resources(handle: *mut c_void);
    pub fn at_au_audio_unit_render_resources_allocated(handle: *mut c_void) -> bool;
    pub fn at_au_audio_unit_reset(handle: *mut c_void);
    pub fn at_au_audio_unit_maximum_frames_to_render(handle: *mut c_void) -> AUAudioFrameCount;
    pub fn at_au_audio_unit_set_maximum_frames_to_render(
        handle: *mut c_void,
        maximum_frames: AUAudioFrameCount,
    );
}
