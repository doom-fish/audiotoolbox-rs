use crate::{
    ffi, internal::status_to_result, AUNode, AUNodeInteraction, AURenderCallback,
    AURenderCallbackStruct, AudioComponentDescription, AudioToolboxError, Result,
};
use std::ffi::c_void;

#[derive(Debug)]
/// Wraps `AUGraph`.
pub struct AUGraph {
    handle: *mut c_void,
    raw: *mut c_void,
}

impl AUGraph {
    /// Wraps `NewAUGraph`.
    pub fn new() -> Result<Self> {
        let mut handle = std::ptr::null_mut();
        let status = unsafe { ffi::au_graph::at_au_graph_new(&mut handle) };
        status_to_result("NewAUGraph", status)?;
        Self::from_handle(handle, "NewAUGraph")
    }

    /// Returns the wrapped `AUGraph` handle.
    pub fn as_raw(&self) -> *mut c_void {
        self.raw
    }

    /// Wraps `AUGraphOpen`.
    pub fn open(&self) -> Result<()> {
        let status = unsafe { ffi::au_graph::at_au_graph_open(self.handle) };
        status_to_result("AUGraphOpen", status)
    }

    /// Wraps `AUGraphClose`.
    pub fn close_graph(&self) -> Result<()> {
        let status = unsafe { ffi::au_graph::at_au_graph_close(self.raw) };
        status_to_result("AUGraphClose", status)
    }

    /// Wraps `AUGraphInitialize`.
    pub fn initialize(&self) -> Result<()> {
        let status = unsafe { ffi::au_graph::at_au_graph_initialize(self.handle) };
        status_to_result("AUGraphInitialize", status)
    }

    /// Wraps `AUGraphUninitialize`.
    pub fn uninitialize(&self) -> Result<()> {
        let status = unsafe { ffi::au_graph::at_au_graph_uninitialize(self.handle) };
        status_to_result("AUGraphUninitialize", status)
    }

    /// Wraps `AUGraphStart`.
    pub fn start(&self) -> Result<()> {
        let status = unsafe { ffi::au_graph::at_au_graph_start(self.handle) };
        status_to_result("AUGraphStart", status)
    }

    /// Wraps `AUGraphStop`.
    pub fn stop(&self) -> Result<()> {
        let status = unsafe { ffi::au_graph::at_au_graph_stop(self.handle) };
        status_to_result("AUGraphStop", status)
    }

    /// Wraps `AUGraphUpdate`.
    pub fn update(&self) -> Result<bool> {
        let mut is_updated = 0_u8;
        let status = unsafe { ffi::au_graph::at_au_graph_update(self.raw, &mut is_updated) };
        status_to_result("AUGraphUpdate", status)?;
        Ok(is_updated != 0)
    }

    /// Wraps `AUGraphIsOpen`.
    pub fn is_open(&self) -> Result<bool> {
        let mut is_open = 0_u8;
        let status = unsafe { ffi::au_graph::at_au_graph_is_open(self.raw, &mut is_open) };
        status_to_result("AUGraphIsOpen", status)?;
        Ok(is_open != 0)
    }

    /// Wraps `AUGraphIsInitialized`.
    pub fn is_initialized(&self) -> Result<bool> {
        let mut is_initialized = 0_u8;
        let status =
            unsafe { ffi::au_graph::at_au_graph_is_initialized(self.raw, &mut is_initialized) };
        status_to_result("AUGraphIsInitialized", status)?;
        Ok(is_initialized != 0)
    }

    /// Wraps `AUGraphIsRunning`.
    pub fn is_running(&self) -> Result<bool> {
        let mut is_running = 0_u8;
        let status = unsafe { ffi::au_graph::at_au_graph_is_running(self.raw, &mut is_running) };
        status_to_result("AUGraphIsRunning", status)?;
        Ok(is_running != 0)
    }

    /// Wraps `AUGraphGetCPULoad`.
    pub fn cpu_load(&self) -> Result<f32> {
        let mut cpu_load = 0.0_f32;
        let status = unsafe { ffi::au_graph::at_au_graph_get_cpu_load(self.raw, &mut cpu_load) };
        status_to_result("AUGraphGetCPULoad", status)?;
        Ok(cpu_load)
    }

    /// Wraps `AUGraphGetMaxCPULoad`.
    pub fn max_cpu_load(&self) -> Result<f32> {
        let mut cpu_load = 0.0_f32;
        let status =
            unsafe { ffi::au_graph::at_au_graph_get_max_cpu_load(self.raw, &mut cpu_load) };
        status_to_result("AUGraphGetMaxCPULoad", status)?;
        Ok(cpu_load)
    }

    /// Wraps `AUGraphGetNodeCount`.
    pub fn node_count(&self) -> Result<u32> {
        let mut node_count = 0_u32;
        let status =
            unsafe { ffi::au_graph::at_au_graph_get_node_count(self.handle, &mut node_count) };
        status_to_result("AUGraphGetNodeCount", status)?;
        Ok(node_count)
    }

    /// Wraps `AUGraphAddNode`.
    pub fn add_node(&self, description: AudioComponentDescription) -> Result<AUNode> {
        let mut node = 0_i32;
        let status = unsafe {
            ffi::au_graph::at_au_graph_add_node(
                self.handle,
                std::ptr::from_ref(&description),
                &mut node,
            )
        };
        status_to_result("AUGraphAddNode", status)?;
        Ok(node)
    }

    /// Wraps `AUGraphConnectNodeInput`.
    pub fn connect_node_input(
        &self,
        source_node: AUNode,
        source_output_number: u32,
        dest_node: AUNode,
        dest_input_number: u32,
    ) -> Result<()> {
        let status = unsafe {
            ffi::au_graph::at_au_graph_connect_node_input(
                self.handle,
                source_node,
                source_output_number,
                dest_node,
                dest_input_number,
            )
        };
        status_to_result("AUGraphConnectNodeInput", status)
    }

    /// Wraps `AUGraphSetNodeInputCallback`.
    pub fn set_node_input_callback(
        &self,
        dest_node: AUNode,
        dest_input_number: u32,
        callback: &AURenderCallbackStruct,
    ) -> Result<()> {
        let status = unsafe {
            ffi::au_graph::at_au_graph_set_node_input_callback(
                self.raw,
                dest_node,
                dest_input_number,
                callback,
            )
        };
        status_to_result("AUGraphSetNodeInputCallback", status)
    }

    /// Wraps `AUGraphDisconnectNodeInput`.
    pub fn disconnect_node_input(&self, dest_node: AUNode, dest_input_number: u32) -> Result<()> {
        let status = unsafe {
            ffi::au_graph::at_au_graph_disconnect_node_input(self.raw, dest_node, dest_input_number)
        };
        status_to_result("AUGraphDisconnectNodeInput", status)
    }

    /// Wraps `AUGraphClearConnections`.
    pub fn clear_connections(&self) -> Result<()> {
        let status = unsafe { ffi::au_graph::at_au_graph_clear_connections(self.raw) };
        status_to_result("AUGraphClearConnections", status)
    }

    /// Wraps `AUGraphGetNumberOfInteractions`.
    pub fn interaction_count(&self) -> Result<u32> {
        let mut interaction_count = 0_u32;
        let status = unsafe {
            ffi::au_graph::at_au_graph_get_number_of_interactions(self.raw, &mut interaction_count)
        };
        status_to_result("AUGraphGetNumberOfInteractions", status)?;
        Ok(interaction_count)
    }

    /// Wraps `AUGraphGetInteractionInfo`.
    pub fn interaction_info(&self, interaction_index: u32) -> Result<AUNodeInteraction> {
        let mut interaction = std::mem::MaybeUninit::<AUNodeInteraction>::uninit();
        let status = unsafe {
            ffi::au_graph::at_au_graph_get_interaction_info(
                self.raw,
                interaction_index,
                interaction.as_mut_ptr(),
            )
        };
        status_to_result("AUGraphGetInteractionInfo", status)?;
        Ok(unsafe { interaction.assume_init() })
    }

    /// Wraps `AUGraphCountNodeInteractions`.
    pub fn count_node_interactions(&self, node: AUNode) -> Result<u32> {
        let mut interaction_count = 0_u32;
        let status = unsafe {
            ffi::au_graph::at_au_graph_count_node_interactions(
                self.raw,
                node,
                &mut interaction_count,
            )
        };
        status_to_result("AUGraphCountNodeInteractions", status)?;
        Ok(interaction_count)
    }

    /// Wraps `AUGraphGetNodeInteractions`.
    pub fn node_interactions(&self, node: AUNode) -> Result<Vec<AUNodeInteraction>> {
        let mut interaction_count = self.count_node_interactions(node)?;
        let mut interactions =
            vec![std::mem::MaybeUninit::<AUNodeInteraction>::uninit(); interaction_count as usize];
        let status = unsafe {
            ffi::au_graph::at_au_graph_get_node_interactions(
                self.raw,
                node,
                &mut interaction_count,
                interactions.as_mut_ptr().cast(),
            )
        };
        status_to_result("AUGraphGetNodeInteractions", status)?;
        interactions.truncate(interaction_count as usize);
        Ok(interactions
            .into_iter()
            .map(|interaction| unsafe { interaction.assume_init() })
            .collect())
    }

    /// Wraps `AUGraphAddRenderNotify`.
    ///
    /// # Safety
    ///
    /// `ref_con` must remain valid for as long as the render notify callback can be invoked.
    pub unsafe fn add_render_notify(
        &self,
        callback: AURenderCallback,
        ref_con: *mut c_void,
    ) -> Result<()> {
        let status =
            unsafe { ffi::au_graph::at_au_graph_add_render_notify(self.raw, callback, ref_con) };
        status_to_result("AUGraphAddRenderNotify", status)
    }

    /// Wraps `AUGraphRemoveRenderNotify`.
    ///
    /// # Safety
    ///
    /// `ref_con` must match the pointer used when the render notify callback was registered.
    pub unsafe fn remove_render_notify(
        &self,
        callback: AURenderCallback,
        ref_con: *mut c_void,
    ) -> Result<()> {
        let status =
            unsafe { ffi::au_graph::at_au_graph_remove_render_notify(self.raw, callback, ref_con) };
        status_to_result("AUGraphRemoveRenderNotify", status)
    }

    /// Wraps `AUGraphNodeInfo`.
    pub fn node_description(&self, node: AUNode) -> Result<AudioComponentDescription> {
        let mut description = AudioComponentDescription::wildcard();
        let status =
            unsafe { ffi::au_graph::at_au_graph_node_info(self.handle, node, &mut description) };
        status_to_result("AUGraphNodeInfo", status)?;
        Ok(description)
    }

    fn from_handle(handle: *mut c_void, operation: &'static str) -> Result<Self> {
        if handle.is_null() {
            return Err(AudioToolboxError::message(
                operation,
                "framework returned a null AUGraph",
            ));
        }
        let raw = unsafe { ffi::au_graph::at_au_graph_raw(handle) };
        if raw.is_null() {
            return Err(AudioToolboxError::message(
                operation,
                "framework returned a null raw AUGraph",
            ));
        }
        Ok(Self { handle, raw })
    }

    fn release(&mut self) {
        if !self.handle.is_null() {
            unsafe { ffi::au_graph::at_au_graph_release(self.handle) };
            self.handle = std::ptr::null_mut();
        }
    }
}

impl Drop for AUGraph {
    fn drop(&mut self) {
        self.release();
    }
}
