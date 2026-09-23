// swiftlint:disable function_parameter_count
import AudioToolbox
import AudioUnit
import CoreAudio
import CoreFoundation
import Foundation

private final class MusicSequenceBox {
    let value: MusicSequence
    var graph: AUGraphBox?

    init(_ value: MusicSequence) {
        self.value = value
    }

    deinit {
        DisposeMusicSequence(value)
    }
}

private final class MusicPlayerBox {
    let value: MusicPlayer
    var sequence: MusicSequenceBox?

    init(_ value: MusicPlayer) {
        self.value = value
    }

    deinit {
        DisposeMusicPlayer(value)
    }
}

private final class MusicTrackBox {
    let value: MusicTrack

    init(_ value: MusicTrack) {
        self.value = value
    }
}

private func musicSequence(from raw: UnsafeMutableRawPointer?) -> MusicSequence {
    castOpaque(raw, to: MusicSequence.self)
}

private func musicPlayer(from raw: UnsafeMutableRawPointer?) -> MusicPlayer {
    castOpaque(raw, to: MusicPlayer.self)
}

private func musicTrack(from raw: UnsafeMutableRawPointer?) -> MusicTrack {
    castOpaque(raw, to: MusicTrack.self)
}

@_cdecl("at_music_sequence_new")
public func at_music_sequence_new(_ outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>?) -> Int32 {
    guard let outHandle else {
        return Int32(kAudio_ParamError)
    }

    var sequence: MusicSequence?
    let status = NewMusicSequence(&sequence)
    if status == noErr, let sequence {
        outHandle.pointee = retainObject(MusicSequenceBox(sequence))
    } else {
        outHandle.pointee = nil
    }
    return status
}

@_cdecl("at_music_sequence_raw")
public func at_music_sequence_raw(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let handle else {
        return nil
    }
    let box: MusicSequenceBox = takeUnretained(handle)
    return toRawPointer(box.value)
}

@_cdecl("at_music_sequence_release")
public func at_music_sequence_release(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else {
        return
    }
    releaseObject(handle, as: MusicSequenceBox.self)
}

@_cdecl("at_music_sequence_new_track")
public func at_music_sequence_new_track(
    _ rawSequence: UnsafeMutableRawPointer?,
    _ outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Int32 {
    guard let outHandle else {
        return Int32(kAudio_ParamError)
    }

    var track: MusicTrack?
    let status = MusicSequenceNewTrack(musicSequence(from: rawSequence), &track)
    if status == noErr, let track {
        outHandle.pointee = retainObject(MusicTrackBox(track))
    } else {
        outHandle.pointee = nil
    }
    return status
}

@_cdecl("at_music_sequence_get_track_count")
public func at_music_sequence_get_track_count(
    _ rawSequence: UnsafeMutableRawPointer?,
    _ outTrackCount: UnsafeMutablePointer<UInt32>?
) -> Int32 {
    guard let outTrackCount else {
        return Int32(kAudio_ParamError)
    }
    return MusicSequenceGetTrackCount(musicSequence(from: rawSequence), outTrackCount)
}

@_cdecl("at_music_track_raw")
public func at_music_track_raw(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let handle else {
        return nil
    }
    let box: MusicTrackBox = takeUnretained(handle)
    return toRawPointer(box.value)
}

@_cdecl("at_music_track_release")
public func at_music_track_release(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else {
        return
    }
    releaseObject(handle, as: MusicTrackBox.self)
}

@_cdecl("at_music_track_new_midi_note_event")
public func at_music_track_new_midi_note_event(
    _ rawTrack: UnsafeMutableRawPointer?,
    _ timeStamp: Float64,
    _ noteMessage: UnsafePointer<MIDINoteMessage>?
) -> Int32 {
    guard let noteMessage else {
        return Int32(kAudio_ParamError)
    }
    return MusicTrackNewMIDINoteEvent(musicTrack(from: rawTrack), timeStamp, noteMessage)
}

@_cdecl("at_music_player_new")
public func at_music_player_new(_ outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>?) -> Int32 {
    guard let outHandle else {
        return Int32(kAudio_ParamError)
    }

    var player: MusicPlayer?
    let status = NewMusicPlayer(&player)
    if status == noErr, let player {
        outHandle.pointee = retainObject(MusicPlayerBox(player))
    } else {
        outHandle.pointee = nil
    }
    return status
}

@_cdecl("at_music_player_raw")
public func at_music_player_raw(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let handle else {
        return nil
    }
    let box: MusicPlayerBox = takeUnretained(handle)
    return toRawPointer(box.value)
}

@_cdecl("at_music_player_release")
public func at_music_player_release(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else {
        return
    }
    releaseObject(handle, as: MusicPlayerBox.self)
}

@_cdecl("at_music_player_set_sequence")
public func at_music_player_set_sequence(
    _ playerHandle: UnsafeMutableRawPointer?,
    _ sequenceHandle: UnsafeMutableRawPointer?
) -> Int32 {
    guard let playerHandle else {
        return Int32(kAudio_ParamError)
    }
    let player: MusicPlayerBox = takeUnretained(playerHandle)
    let sequence: MusicSequenceBox? = sequenceHandle.map { takeUnretained($0) }
    let status = MusicPlayerSetSequence(player.value, sequence?.value)
    if status == noErr {
        player.sequence = sequence
    }
    return status
}

@_cdecl("at_music_sequence_set_au_graph")
public func at_music_sequence_set_au_graph(
    _ sequenceHandle: UnsafeMutableRawPointer?,
    _ graphHandle: UnsafeMutableRawPointer?
) -> Int32 {
    guard let sequenceHandle else {
        return Int32(kAudio_ParamError)
    }
    let sequence: MusicSequenceBox = takeUnretained(sequenceHandle)
    let graph: AUGraphBox? = graphHandle.map { takeUnretained($0) }
    let status = MusicSequenceSetAUGraph(sequence.value, graph?.value)
    if status == noErr {
        sequence.graph = graph
    }
    return status
}

@_cdecl("at_music_player_preroll")
public func at_music_player_preroll(_ rawPlayer: UnsafeMutableRawPointer?) -> Int32 {
    MusicPlayerPreroll(musicPlayer(from: rawPlayer))
}

@_cdecl("at_music_player_start")
public func at_music_player_start(_ rawPlayer: UnsafeMutableRawPointer?) -> Int32 {
    MusicPlayerStart(musicPlayer(from: rawPlayer))
}

@_cdecl("at_music_player_stop")
public func at_music_player_stop(_ rawPlayer: UnsafeMutableRawPointer?) -> Int32 {
    MusicPlayerStop(musicPlayer(from: rawPlayer))
}

@_cdecl("at_music_player_is_playing")
public func at_music_player_is_playing(
    _ rawPlayer: UnsafeMutableRawPointer?,
    _ outIsPlaying: UnsafeMutablePointer<UInt32>?
) -> Int32 {
    guard let outIsPlaying else {
        return Int32(kAudio_ParamError)
    }
    var isPlaying: DarwinBoolean = false
    let status = MusicPlayerIsPlaying(musicPlayer(from: rawPlayer), &isPlaying)
    outIsPlaying.pointee = isPlaying.boolValue ? 1 : 0
    return status
}
