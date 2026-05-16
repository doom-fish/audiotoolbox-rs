# Changelog

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
