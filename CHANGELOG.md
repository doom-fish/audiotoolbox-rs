# Changelog

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
