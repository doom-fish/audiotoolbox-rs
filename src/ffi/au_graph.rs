use crate::{AudioComponentDescription, AUNode, OSStatus};
use std::ffi::c_void;

unsafe extern "C" {
    pub fn at_au_graph_new(out_handle: *mut *mut c_void) -> OSStatus;
    pub fn at_au_graph_release(handle: *mut c_void);
    pub fn at_au_graph_open(handle: *mut c_void) -> OSStatus;
    pub fn at_au_graph_initialize(handle: *mut c_void) -> OSStatus;
    pub fn at_au_graph_uninitialize(handle: *mut c_void) -> OSStatus;
    pub fn at_au_graph_start(handle: *mut c_void) -> OSStatus;
    pub fn at_au_graph_stop(handle: *mut c_void) -> OSStatus;
    pub fn at_au_graph_get_node_count(handle: *mut c_void, out_number_of_nodes: *mut u32)
        -> OSStatus;
    pub fn at_au_graph_add_node(
        handle: *mut c_void,
        description: *const AudioComponentDescription,
        out_node: *mut AUNode,
    ) -> OSStatus;
    pub fn at_au_graph_connect_node_input(
        handle: *mut c_void,
        source_node: AUNode,
        source_output_number: u32,
        dest_node: AUNode,
        dest_input_number: u32,
    ) -> OSStatus;
    pub fn at_au_graph_node_info(
        handle: *mut c_void,
        node: AUNode,
        out_description: *mut AudioComponentDescription,
    ) -> OSStatus;
}
