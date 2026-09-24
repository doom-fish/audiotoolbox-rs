# Changelog

All notable changes to `audiotoolbox` are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.5.0] - Unreleased

### Security

- Render-notify and property-listener streams could free their context while
  an audio-thread callback was still running (use-after-free and double free
  on the render thread). The context is now kept alive by the audio unit or
  graph until it is disposed, and the ring behind it is freed once no callback
  is inside a push.
- `fill_complex_buffer_once` let the converter keep reading the caller's input
  slice after the call returned (use-after-free on the next call).
- Typed property reads accepted any `T: Copy`, producing invalid `bool`/enum
  values or uninitialized bytes; `MusicTrack` copies outlived their sequence;
  the MIDI raw-data, meta, user and extended-note events made the framework
  read past the Rust object; `render` and `convert_complex_buffer` wrote
  through caller-built `AudioBufferList` pointers.
- Packet reads allocated `max_packet_size × packet_count` taken from the file
  before checking it, so a crafted file could force a huge allocation.
- `AudioQueueBufferHandle` read freed memory after its queue was dropped.

### Fixed

- Chunked conversion no longer ends the stream on every call, which flushed
  encoders and dropped all later input.
- Setting `frameLength` above `frameCapacity`, or asking a node for a bus it
  does not have, raised an Objective-C exception that aborted the process.
- `write_packets` checks the packet count against the descriptions and every
  description against the payload; `ExtAudioFile::write_interleaved` no longer
  reads past the buffer.
- `AudioFile_SMPTE_Time` had the 24-byte `SMPTETime` layout instead of its
  8-byte SDK layout, which also broke `AudioFileMarker`, `AudioFileRegion` and
  their lists.
- `MusicSequence::set_au_graph` and `MusicPlayer::set_sequence` now retain the
  graph and the sequence.
- Apple's DLS synth crashed when instances were initialized or disposed on
  several threads at once; the crate serializes audio unit, graph and music
  player lifecycle calls.
- Non-UTF-8 paths no longer turn into a different file name, `CFData` is
  released on error paths, and the Swift bridge no longer traps on an unknown
  file permission value or force-unwraps its handles.
- `AudioFileStream::parse_bytes` used to throw away the parsed packets.
- `AudioFormat::balance_fade` returned the specifier overwritten with
  coefficients; the coefficients are now returned as a `Vec<f32>`.
- Coverage documents no longer count method-less placeholder handles as
  verified, and say what their numbers measure.
- The 0.4.x hardening (sound batch reads in `AudioFormat`, ABI layout
  assertions) that had not been released is part of this release.
- `build.rs` no longer adds the toolchain's Swift 5.5 back-deployment
  directory (`usr/lib/swift-5.5/macosx`) to the link search path or rpath.
  Its old `libswift_Concurrency.dylib` could shadow the SDK's
  `libswift_Concurrency.tbd` for the whole binary and break linking next to
  Swift bridges that use newer concurrency APIs.

### Changed

- **Breaking:** `get_property_typed` / `get_property_array` require the new
  `AudioProperty` trait and check the returned size; `set_property_typed`,
  `AudioFile::set_property_bytes`, `global_info_size`, `global_info_bytes`,
  `AudioFormat::property_info` and `AUGraph::set_node_input_callback` are
  `unsafe`.
- **Breaking:** `AudioUnit::render` and `convert_complex_buffer` take
  `OwnedAudioBufferList`.
- **Breaking:** `AudioConverter::fill_complex_buffer_once(&self)` is now
  `fill_complex_buffer(&mut self)`, plus `finish` to end the stream;
  `AudioConverter::reset` takes `&mut self`; `BorrowedAudioConverter` no longer
  has the fill method.
- **Breaking:** `MusicTrack<'a>` and `MusicEventIterator<'a>` borrow their
  sequence and are not `Copy`; `dispose_track` takes `&mut self` and an index;
  `track_index`, `copy_insert` and `merge` take `&MusicTrack`;
  `new_midi_raw_data_event`, `new_meta_event`, `new_user_event` and
  `new_extended_note_event` take slices; `new_au_preset_event` takes a
  `CFDictionary`; `set_event_info` takes a `MusicEvent`; `MusicEventInfo` has
  owned `data` instead of a pointer.
- **Breaking:** `AudioFormat::format_list` / `output_format_list` take the
  description and magic cookie; `AudioComponent::validate` takes
  `Option<&CFDictionary>`.
- **Breaking:** `AudioQueueBufferHandle<'q>` borrows its queue and frees its
  buffer on drop; `AudioFileStream::parse_bytes` returns `PacketData`;
  `AVAudioPCMBuffer::set_frame_length` returns `Result`;
  `AURenderCallbackStruct::inputProc` is an `Option`; `AudioFileSmpteTime` is a
  struct; `AudioToolboxDebugObject` is sealed.
- **Breaking:** the async property stream uses the same lock-free ring as the
  render-notify streams, so `next()` returns a `PopFuture`.
- `apple-cf` 0.11 and `doom-fish-utils` 0.4.1 (now a regular dependency);
  `rust-version` is 1.82.

### Added

- `AudioProperty`, `OwnedAudioBufferList`, `AudioConverter::finish`.
- `AudioQueue::new_output_with_callback`, `new_input_with_callback`,
  `enqueue_buffer`, `allocate_buffer_with_packet_descriptions`,
  `set_offline_render_format` and `offline_render`, with
  `AudioQueueOutputBuffer` / `AudioQueueInputBuffer` views.
- `AudioFileStream::seek`, `SystemSound::play_with_completion` and
  `play_alert_with_completion`.
- `MusicEvent`, `ExtendedNote`, typed `MusicTrack` accessors for the loop,
  offset, mute, solo, length and time-resolution properties, and
  `MusicPlayer::clear_sequence`.
- `AudioFormat::balance_fade_coefficients` and `panning_matrix`.
- `CAShow` support for `AUGraph` and `MusicSequence`.

### Removed

- `fill_complex_buffer_once`, `AudioFormat::balance_fade` and
  `panning_matrix_size`, `AudioComponent::copy_configuration_info_raw` and
  `validate_raw` (now `copy_configuration_info` returning an owned
  `CFDictionary`, and `validate`), and `MusicSequence::info_dictionary_raw`
  (now `info_dictionary`).

## [0.4.0] - 2026-05-20

### Added

- `async` feature plus `async_api` stream wrappers for `AudioUnit` property listeners, `AudioUnit` render-notify callbacks, and `AUGraph` render-notify callbacks.
- Real-time-safe SPSC handoff for render-notify callbacks while leaving synchronous pull-render callbacks as manual APIs.

### Notes

- Phase 32 completeness + async sweep.

## [0.3.4] - 2026-05-20

- Added in-`src/` unit tests across `format`, `error`, and `extended_types` (Tier 2 quality polish), providing fast `cargo test --lib` fail-fast signal alongside the existing integration tests under `tests/`.

## [0.3.3] - 2026-05-18

- chore: re-export OS primitives (Boolean, OSStatus, OSType) from apple-cf

## [0.3.2] - 2026-05-18

- Widen apple-cf version bound to `<0.10` so 0.9.x resolves.

## [0.3.1] - 2026-05-18

### Changed

- Added `Debug` coverage for the remaining public structs outside the generated and `ffi` modules, with manual formatters for the callback and union-backed wrappers that cannot derive it directly.
- Bumped the crate to `0.3.1`.

## [0.3.0] - 2026-05-18

### Changed

- Re-exported Core Foundation `CF*Ref` aliases from `apple_cf::raw` instead of duplicating local typedefs and opaque tags.
- Raised the minimum `apple-cf` dependency to `0.8` so the shared raw Core Foundation surface is available.
- Bumped the crate to `0.3.0`.

## [0.2.5] - 2026-05-18

- Widen apple-cf version bound to `<0.9` so the 0.8.0 nested-CGRect dep resolves. No source changes.

## [0.2.4] - 2026-05-18

### Changed

- Added one-line AudioToolbox SDK reference docs across the crate's public surface, including `# Safety` sections for unsafe functions and ownership notes for constructors.
- Raised measured public-item documentation coverage from 0.1% to 100.0%.
- Bumped the crate to `0.2.4`.

## [0.2.3] - 2026-05-17

### Added

- Closed the sampled `COVERAGE_AUDIT.md` to 100% by exposing the remaining exact-name `AudioFile`, `AudioFormat`, `AudioConverter`, `AudioComponent`, `AudioUnit`, `AUGraph`, `AVFAudio`, `MusicSequence`, `AudioFileStream`, and `AudioServices` symbols.
- Added `generated_c_types` for the long-tail C structs, callbacks, and plug-in/clock metadata still missing from the audit surface.
- Added exact-name aliases, opaque Objective-C handle wrappers, protocol markers, and an exhaustive symbol smoke test covering every former audit gap.

### Changed

- Bumped the crate to `0.2.3`.
- Refreshed the README and coverage audit to reflect the exhaustive closure.

## [0.2.2] - 2026-05-17

### Added

- Expanded `AudioFile` with byte I/O, optimize, user-data, and global-info helpers.
- Expanded `AudioFormat`, `AudioConverter`, `AudioComponent`, `AudioUnit`, `AUGraph`, and `MusicSequence` / `MusicPlayer` with more direct C-backed wrappers.
- Added `MusicEventIterator`, `AVAudioPCMBuffer`, and `AVAudioSequencer` wrappers plus smoke-test coverage for the newly exposed APIs.
- Extended the `raw-ffi` feature with additional `AudioFormat`, `AudioFile`, `AudioConverter`, `AudioComponent`, `AudioUnit`, `AUGraph`, and `Music*` entry points.

### Changed

- Bumped the crate to `0.2.2`.
- Refreshed the README to document the broadened wrapper surface.

## [0.2.1] - 2026-05-16

### Added

- Added bridge-backed wrappers for `AVAudioEngine`, `AVAudioNode`, and `AVAudioFormat` from `AVFAudio.framework`.
- Added a bridge-backed `AUAudioUnit` wrapper covering component metadata, bus counts, and frame-budget configuration.
- Added safe wrappers for legacy-but-macOS-available `AUGraph` graph management APIs and `AudioFileComponent` plug-in accessors.
- Added four new numbered examples and four integration smoke tests covering the new surface area.

### Changed

- Bumped the crate to `0.2.1`.
- Refreshed the coverage audit and README to reflect the newly wrapped APIs.

## [0.2.0] - 2026-05-16

### Added

- Migrated the crate to a ScreenCaptureKit-style Swift bridge architecture.
- Preserved the legacy raw C bindings behind the `raw-ffi` Cargo feature.
- Added safe Rust wrappers and Swift bridge files for `AudioFormat`, `AudioUnit`, `AudioQueue`, `MusicSequence` / `MusicPlayer`, `AudioFileStream`, and `CAFFile`.
- Added 11 numbered examples and 11 integration smoke tests spanning every logical area.
- Added `COVERAGE.md` with a header-by-header API audit.

### Changed

- Bumped the crate to `0.2.0`.
- Updated the build to compile and link the Swift bridge automatically.
- Refreshed the README to document the new bridge-backed surface area.

## [0.1.0] - 2026-05-16

### Added

- Safe wrappers for `AudioFile`, `ExtAudioFile`, `AudioConverter`,
  `AudioComponent`, and `AudioServices` entry points used in day-to-day macOS
  audio tooling.
- `AudioStreamBasicDescription` helpers for interleaved and non-interleaved
  linear PCM (`f32` / `i16`).
- One-shot `AudioConverterFillComplexBuffer` wrapper for in-memory conversion.
- `InterleavedAudioBuffer` helper for `ExtAudioFileRead` / `ExtAudioFileWrite`.
- `CAShow` / `CAShowFile` debugging helpers plus a `01_read_glass` smoke example.
