use crate::{
    AUNode, AUNodeInteraction, AURenderCallback, AURenderCallbackStruct, AudioComponentDescription,
    Boolean, OSStatus,
};
use std::ffi::c_void;

unsafe extern "C" {
    pub fn at_au_graph_new(out_handle: *mut *mut c_void) -> OSStatus;
    pub fn at_au_graph_release(handle: *mut c_void);
    pub fn at_au_graph_raw(handle: *mut c_void) -> *mut c_void;
    pub fn at_au_graph_open(handle: *mut c_void) -> OSStatus;
    #[link_name = "AUGraphClose"]
    pub fn at_au_graph_close(handle: *mut c_void) -> OSStatus;
    pub fn at_au_graph_initialize(handle: *mut c_void) -> OSStatus;
    pub fn at_au_graph_uninitialize(handle: *mut c_void) -> OSStatus;
    pub fn at_au_graph_start(handle: *mut c_void) -> OSStatus;
    pub fn at_au_graph_stop(handle: *mut c_void) -> OSStatus;
    #[link_name = "AUGraphIsOpen"]
    pub fn at_au_graph_is_open(handle: *mut c_void, out_is_open: *mut Boolean) -> OSStatus;
    #[link_name = "AUGraphIsInitialized"]
    pub fn at_au_graph_is_initialized(
        handle: *mut c_void,
        out_is_initialized: *mut Boolean,
    ) -> OSStatus;
    #[link_name = "AUGraphIsRunning"]
    pub fn at_au_graph_is_running(handle: *mut c_void, out_is_running: *mut Boolean) -> OSStatus;
    pub fn at_au_graph_get_node_count(
        handle: *mut c_void,
        out_number_of_nodes: *mut u32,
    ) -> OSStatus;
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
    #[link_name = "AUGraphSetNodeInputCallback"]
    pub fn at_au_graph_set_node_input_callback(
        handle: *mut c_void,
        dest_node: AUNode,
        dest_input_number: u32,
        callback_struct: *const AURenderCallbackStruct,
    ) -> OSStatus;
    #[link_name = "AUGraphDisconnectNodeInput"]
    pub fn at_au_graph_disconnect_node_input(
        handle: *mut c_void,
        dest_node: AUNode,
        dest_input_number: u32,
    ) -> OSStatus;
    #[link_name = "AUGraphClearConnections"]
    pub fn at_au_graph_clear_connections(handle: *mut c_void) -> OSStatus;
    #[link_name = "AUGraphGetNumberOfInteractions"]
    pub fn at_au_graph_get_number_of_interactions(
        handle: *mut c_void,
        out_num_interactions: *mut u32,
    ) -> OSStatus;
    #[link_name = "AUGraphGetInteractionInfo"]
    pub fn at_au_graph_get_interaction_info(
        handle: *mut c_void,
        interaction_index: u32,
        out_interaction: *mut AUNodeInteraction,
    ) -> OSStatus;
    #[link_name = "AUGraphCountNodeInteractions"]
    pub fn at_au_graph_count_node_interactions(
        handle: *mut c_void,
        node: AUNode,
        out_num_interactions: *mut u32,
    ) -> OSStatus;
    #[link_name = "AUGraphGetNodeInteractions"]
    pub fn at_au_graph_get_node_interactions(
        handle: *mut c_void,
        node: AUNode,
        io_num_interactions: *mut u32,
        out_interactions: *mut AUNodeInteraction,
    ) -> OSStatus;
    #[link_name = "AUGraphUpdate"]
    pub fn at_au_graph_update(handle: *mut c_void, out_is_updated: *mut Boolean) -> OSStatus;
    #[link_name = "AUGraphGetCPULoad"]
    pub fn at_au_graph_get_cpu_load(
        handle: *mut c_void,
        out_average_cpu_load: *mut f32,
    ) -> OSStatus;
    #[link_name = "AUGraphGetMaxCPULoad"]
    pub fn at_au_graph_get_max_cpu_load(
        handle: *mut c_void,
        out_max_cpu_load: *mut f32,
    ) -> OSStatus;
    #[link_name = "AUGraphAddRenderNotify"]
    pub fn at_au_graph_add_render_notify(
        handle: *mut c_void,
        callback: AURenderCallback,
        ref_con: *mut c_void,
    ) -> OSStatus;
    #[link_name = "AUGraphRemoveRenderNotify"]
    pub fn at_au_graph_remove_render_notify(
        handle: *mut c_void,
        callback: AURenderCallback,
        ref_con: *mut c_void,
    ) -> OSStatus;
    pub fn at_au_graph_node_info(
        handle: *mut c_void,
        node: AUNode,
        out_description: *mut AudioComponentDescription,
    ) -> OSStatus;
}
