import AudioToolbox
import AudioUnit
import CoreAudio
import CoreFoundation
import Foundation

private final class AUGraphBox {
    var value: AUGraph?

    init(_ value: AUGraph) {
        self.value = value
    }

    deinit {
        if let value {
            DisposeAUGraph(value)
        }
    }
}

private func auGraph(from handle: UnsafeMutableRawPointer) -> AUGraph {
    let box: AUGraphBox = takeUnretained(handle)
    return box.value!
}

@_cdecl("at_au_graph_new")
public func at_au_graph_new(_ outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>?) -> Int32 {
    guard let outHandle else {
        return Int32(kAudio_ParamError)
    }

    var graph: AUGraph?
    let status = NewAUGraph(&graph)
    if status == noErr, let graph {
        outHandle.pointee = retainObject(AUGraphBox(graph))
    } else {
        outHandle.pointee = nil
    }
    return status
}

@_cdecl("at_au_graph_release")
public func at_au_graph_release(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else {
        return
    }
    releaseObject(handle, as: AUGraphBox.self)
}

@_cdecl("at_au_graph_open")
public func at_au_graph_open(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let handle else {
        return Int32(kAudio_ParamError)
    }
    return AUGraphOpen(auGraph(from: handle))
}

@_cdecl("at_au_graph_initialize")
public func at_au_graph_initialize(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let handle else {
        return Int32(kAudio_ParamError)
    }
    return AUGraphInitialize(auGraph(from: handle))
}

@_cdecl("at_au_graph_uninitialize")
public func at_au_graph_uninitialize(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let handle else {
        return Int32(kAudio_ParamError)
    }
    return AUGraphUninitialize(auGraph(from: handle))
}

@_cdecl("at_au_graph_start")
public func at_au_graph_start(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let handle else {
        return Int32(kAudio_ParamError)
    }
    return AUGraphStart(auGraph(from: handle))
}

@_cdecl("at_au_graph_stop")
public func at_au_graph_stop(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let handle else {
        return Int32(kAudio_ParamError)
    }
    return AUGraphStop(auGraph(from: handle))
}

@_cdecl("at_au_graph_get_node_count")
public func at_au_graph_get_node_count(
    _ handle: UnsafeMutableRawPointer?,
    _ outNumberOfNodes: UnsafeMutablePointer<UInt32>?
) -> Int32 {
    guard let handle, let outNumberOfNodes else {
        return Int32(kAudio_ParamError)
    }
    return AUGraphGetNodeCount(auGraph(from: handle), outNumberOfNodes)
}

@_cdecl("at_au_graph_add_node")
public func at_au_graph_add_node(
    _ handle: UnsafeMutableRawPointer?,
    _ description: UnsafePointer<AudioComponentDescription>?,
    _ outNode: UnsafeMutablePointer<AUNode>?
) -> Int32 {
    guard let handle, let description, let outNode else {
        return Int32(kAudio_ParamError)
    }
    return AUGraphAddNode(auGraph(from: handle), description, outNode)
}

@_cdecl("at_au_graph_connect_node_input")
public func at_au_graph_connect_node_input(
    _ handle: UnsafeMutableRawPointer?,
    _ sourceNode: AUNode,
    _ sourceOutputNumber: UInt32,
    _ destNode: AUNode,
    _ destInputNumber: UInt32
) -> Int32 {
    guard let handle else {
        return Int32(kAudio_ParamError)
    }
    return AUGraphConnectNodeInput(
        auGraph(from: handle),
        sourceNode,
        sourceOutputNumber,
        destNode,
        destInputNumber
    )
}

@_cdecl("at_au_graph_node_info")
public func at_au_graph_node_info(
    _ handle: UnsafeMutableRawPointer?,
    _ node: AUNode,
    _ outDescription: UnsafeMutablePointer<AudioComponentDescription>?
) -> Int32 {
    guard let handle, let outDescription else {
        return Int32(kAudio_ParamError)
    }
    return AUGraphNodeInfo(auGraph(from: handle), node, outDescription, nil)
}
