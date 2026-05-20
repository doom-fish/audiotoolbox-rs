use crate::{
    AUNode, AUNodeInteraction, AURenderCallback, AURenderCallbackStruct, AudioComponentDescription,
    Boolean, OSStatus,
};
use std::ffi::c_void;

unsafe extern "C" {
    /// Raw binding for `AUGraphNew`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AUGraphNew`.
    pub fn at_au_graph_new(out_handle: *mut *mut c_void) -> OSStatus;
    /// Raw binding for `AUGraphRelease`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AUGraphRelease`.
    pub fn at_au_graph_release(handle: *mut c_void);
    /// Raw binding for `AUGraphRaw`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AUGraphRaw`.
    pub fn at_au_graph_raw(handle: *mut c_void) -> *mut c_void;
    /// Raw binding for `AUGraphRetain`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AUGraphRetain`.
    pub fn at_au_graph_retain(handle: *mut c_void) -> *mut c_void;
    /// Raw binding for `AUGraphOpen`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AUGraphOpen`.
    pub fn at_au_graph_open(handle: *mut c_void) -> OSStatus;
    #[link_name = "AUGraphClose"]
    /// Raw binding for `AUGraphClose`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AUGraphClose`.
    pub fn at_au_graph_close(handle: *mut c_void) -> OSStatus;
    /// Raw binding for `AUGraphInitialize`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AUGraphInitialize`.
    pub fn at_au_graph_initialize(handle: *mut c_void) -> OSStatus;
    /// Raw binding for `AUGraphUninitialize`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AUGraphUninitialize`.
    pub fn at_au_graph_uninitialize(handle: *mut c_void) -> OSStatus;
    /// Raw binding for `AUGraphStart`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AUGraphStart`.
    pub fn at_au_graph_start(handle: *mut c_void) -> OSStatus;
    /// Raw binding for `AUGraphStop`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AUGraphStop`.
    pub fn at_au_graph_stop(handle: *mut c_void) -> OSStatus;
    #[link_name = "AUGraphIsOpen"]
    /// Raw binding for `AUGraphIsOpen`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AUGraphIsOpen`.
    pub fn at_au_graph_is_open(handle: *mut c_void, out_is_open: *mut Boolean) -> OSStatus;
    #[link_name = "AUGraphIsInitialized"]
    /// Raw binding for `AUGraphIsInitialized`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AUGraphIsInitialized`.
    pub fn at_au_graph_is_initialized(
        handle: *mut c_void,
        out_is_initialized: *mut Boolean,
    ) -> OSStatus;
    #[link_name = "AUGraphIsRunning"]
    /// Raw binding for `AUGraphIsRunning`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AUGraphIsRunning`.
    pub fn at_au_graph_is_running(handle: *mut c_void, out_is_running: *mut Boolean) -> OSStatus;
    /// Raw binding for `AUGraphGetNodeCount`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AUGraphGetNodeCount`.
    pub fn at_au_graph_get_node_count(
        handle: *mut c_void,
        out_number_of_nodes: *mut u32,
    ) -> OSStatus;
    /// Raw binding for `AUGraphAddNode`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AUGraphAddNode`.
    pub fn at_au_graph_add_node(
        handle: *mut c_void,
        description: *const AudioComponentDescription,
        out_node: *mut AUNode,
    ) -> OSStatus;
    /// Raw binding for `AUGraphConnectNodeInput`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AUGraphConnectNodeInput`.
    pub fn at_au_graph_connect_node_input(
        handle: *mut c_void,
        source_node: AUNode,
        source_output_number: u32,
        dest_node: AUNode,
        dest_input_number: u32,
    ) -> OSStatus;
    #[link_name = "AUGraphSetNodeInputCallback"]
    /// Raw binding for `AUGraphSetNodeInputCallback`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AUGraphSetNodeInputCallback`.
    pub fn at_au_graph_set_node_input_callback(
        handle: *mut c_void,
        dest_node: AUNode,
        dest_input_number: u32,
        callback_struct: *const AURenderCallbackStruct,
    ) -> OSStatus;
    #[link_name = "AUGraphDisconnectNodeInput"]
    /// Raw binding for `AUGraphDisconnectNodeInput`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AUGraphDisconnectNodeInput`.
    pub fn at_au_graph_disconnect_node_input(
        handle: *mut c_void,
        dest_node: AUNode,
        dest_input_number: u32,
    ) -> OSStatus;
    #[link_name = "AUGraphClearConnections"]
    /// Raw binding for `AUGraphClearConnections`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AUGraphClearConnections`.
    pub fn at_au_graph_clear_connections(handle: *mut c_void) -> OSStatus;
    #[link_name = "AUGraphGetNumberOfInteractions"]
    /// Raw binding for `AUGraphGetNumberOfInteractions`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AUGraphGetNumberOfInteractions`.
    pub fn at_au_graph_get_number_of_interactions(
        handle: *mut c_void,
        out_num_interactions: *mut u32,
    ) -> OSStatus;
    #[link_name = "AUGraphGetInteractionInfo"]
    /// Raw binding for `AUGraphGetInteractionInfo`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AUGraphGetInteractionInfo`.
    pub fn at_au_graph_get_interaction_info(
        handle: *mut c_void,
        interaction_index: u32,
        out_interaction: *mut AUNodeInteraction,
    ) -> OSStatus;
    #[link_name = "AUGraphCountNodeInteractions"]
    /// Raw binding for `AUGraphCountNodeInteractions`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AUGraphCountNodeInteractions`.
    pub fn at_au_graph_count_node_interactions(
        handle: *mut c_void,
        node: AUNode,
        out_num_interactions: *mut u32,
    ) -> OSStatus;
    #[link_name = "AUGraphGetNodeInteractions"]
    /// Raw binding for `AUGraphGetNodeInteractions`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AUGraphGetNodeInteractions`.
    pub fn at_au_graph_get_node_interactions(
        handle: *mut c_void,
        node: AUNode,
        io_num_interactions: *mut u32,
        out_interactions: *mut AUNodeInteraction,
    ) -> OSStatus;
    #[link_name = "AUGraphUpdate"]
    /// Raw binding for `AUGraphUpdate`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AUGraphUpdate`.
    pub fn at_au_graph_update(handle: *mut c_void, out_is_updated: *mut Boolean) -> OSStatus;
    #[link_name = "AUGraphGetCPULoad"]
    /// Raw binding for `AUGraphGetCPULoad`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AUGraphGetCPULoad`.
    pub fn at_au_graph_get_cpu_load(
        handle: *mut c_void,
        out_average_cpu_load: *mut f32,
    ) -> OSStatus;
    #[link_name = "AUGraphGetMaxCPULoad"]
    /// Raw binding for `AUGraphGetMaxCPULoad`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AUGraphGetMaxCPULoad`.
    pub fn at_au_graph_get_max_cpu_load(
        handle: *mut c_void,
        out_max_cpu_load: *mut f32,
    ) -> OSStatus;
    #[link_name = "AUGraphAddRenderNotify"]
    /// Raw binding for `AUGraphAddRenderNotify`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AUGraphAddRenderNotify`.
    pub fn at_au_graph_add_render_notify(
        handle: *mut c_void,
        callback: AURenderCallback,
        ref_con: *mut c_void,
    ) -> OSStatus;
    #[link_name = "AUGraphRemoveRenderNotify"]
    /// Raw binding for `AUGraphRemoveRenderNotify`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AUGraphRemoveRenderNotify`.
    pub fn at_au_graph_remove_render_notify(
        handle: *mut c_void,
        callback: AURenderCallback,
        ref_con: *mut c_void,
    ) -> OSStatus;
    /// Raw binding for `AUGraphNodeInfo`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the pointer, lifetime, and callback requirements of `AUGraphNodeInfo`.
    pub fn at_au_graph_node_info(
        handle: *mut c_void,
        node: AUNode,
        out_description: *mut AudioComponentDescription,
    ) -> OSStatus;
}
