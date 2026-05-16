use crate::{
    ffi,
    internal::status_to_result,
    AudioComponentDescription,
    AudioToolboxError,
    Result,
    AUNode,
};
use std::ffi::c_void;

#[derive(Debug)]
pub struct AUGraph {
    handle: *mut c_void,
}

impl AUGraph {
    pub fn new() -> Result<Self> {
        let mut handle = std::ptr::null_mut();
        let status = unsafe { ffi::au_graph::at_au_graph_new(&mut handle) };
        status_to_result("NewAUGraph", status)?;
        Self::from_handle(handle, "NewAUGraph")
    }

    pub fn open(&self) -> Result<()> {
        let status = unsafe { ffi::au_graph::at_au_graph_open(self.handle) };
        status_to_result("AUGraphOpen", status)
    }

    pub fn initialize(&self) -> Result<()> {
        let status = unsafe { ffi::au_graph::at_au_graph_initialize(self.handle) };
        status_to_result("AUGraphInitialize", status)
    }

    pub fn uninitialize(&self) -> Result<()> {
        let status = unsafe { ffi::au_graph::at_au_graph_uninitialize(self.handle) };
        status_to_result("AUGraphUninitialize", status)
    }

    pub fn start(&self) -> Result<()> {
        let status = unsafe { ffi::au_graph::at_au_graph_start(self.handle) };
        status_to_result("AUGraphStart", status)
    }

    pub fn stop(&self) -> Result<()> {
        let status = unsafe { ffi::au_graph::at_au_graph_stop(self.handle) };
        status_to_result("AUGraphStop", status)
    }

    pub fn node_count(&self) -> Result<u32> {
        let mut node_count = 0_u32;
        let status =
            unsafe { ffi::au_graph::at_au_graph_get_node_count(self.handle, &mut node_count) };
        status_to_result("AUGraphGetNodeCount", status)?;
        Ok(node_count)
    }

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

    pub fn node_description(&self, node: AUNode) -> Result<AudioComponentDescription> {
        let mut description = AudioComponentDescription::wildcard();
        let status =
            unsafe { ffi::au_graph::at_au_graph_node_info(self.handle, node, &mut description) };
        status_to_result("AUGraphNodeInfo", status)?;
        Ok(description)
    }

    fn from_handle(handle: *mut c_void, operation: &'static str) -> Result<Self> {
        if handle.is_null() {
            Err(AudioToolboxError::message(
                operation,
                "framework returned a null AUGraph",
            ))
        } else {
            Ok(Self { handle })
        }
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
