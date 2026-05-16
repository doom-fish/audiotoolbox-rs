# audiotoolbox-rs

Safe Rust bindings for Apple’s `AudioToolbox.framework` on macOS via a Swift bridge.

## Covered areas

`audiotoolbox-rs` 0.2.1 now ships bridge-backed wrappers for:

- `AudioFormat`
- `AudioFile`
- `AudioFileComponent`
- `ExtAudioFile`
- `AudioConverter`
- `AudioComponent`
- `AudioUnit`
- `AUAudioUnit`
- `AUGraph`
- `AVFAudio` (`AVAudioEngine` / `AVAudioNode` / `AVAudioFormat`)
- `AudioQueue`
- `MusicSequence` / `MusicPlayer`
- `AudioServices`
- `AudioFileStream`
- `CAFFile`

The original raw C surface is still available behind the `raw-ffi` Cargo feature.

## Installation

```toml
[dependencies]
audiotoolbox = "0.2.1"
```

To reach the legacy raw C bindings as well:

```toml
[dependencies]
audiotoolbox = { version = "0.2.1", features = ["raw-ffi"] }
```

## Quick start

```rust
use audiotoolbox::AudioFile;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = AudioFile::open("/System/Library/Sounds/Glass.aiff")?;
    let format = file.data_format()?;
    let duration = file.estimated_duration()?;

    println!(
        "sample_rate={:.1} channels={} duration={:.3}s",
        format.mSampleRate,
        format.mChannelsPerFrame,
        duration,
    );

    Ok(())
}
```

## Highlights

- ScreenCaptureKit-style Swift bridge with one bridge file per logical area
- Owned Rust handles with `Drop`
- `AudioFile`, `AudioFileComponent`, `ExtAudioFile`, and `AudioFileStream` smoke-tested against `Glass.aiff`
- In-memory `AudioConverter` one-shot conversion helper
- `AudioUnit`, `AUAudioUnit`, `AUGraph`, and `MusicSequence` / `MusicPlayer` creation helpers
- `AVAudioEngine` / `AVAudioNode` / `AVAudioFormat` wrappers for the common `AVFAudio` path
- Pure-Rust `CAFFile` header parsing helpers
- 15 numbered examples and 15 integration smoke tests

## Examples

```bash
for ex in examples/*.rs; do cargo run --example "$(basename "$ex" .rs)"; done
```

Highlights:

- `01_read_glass`
- `04_ext_audio_file_decode`
- `07_audio_unit_converter`
- `08_audio_queue_output`
- `10_audio_file_stream`
- `11_caf_header`
- `12_avfaudio`
- `13_au_audio_unit`
- `14_au_graph`
- `15_audio_file_component`

## Validation

Verified with:

```bash
cargo clippy --all-targets -- -D warnings
cargo test
for ex in examples/*.rs; do cargo run --example "$(basename "$ex" .rs)"; done
cargo check --features raw-ffi
```

## Coverage audit

See [`COVERAGE_AUDIT.md`](COVERAGE_AUDIT.md) for the sampled public-symbol coverage scoreboard and [`COVERAGE.md`](COVERAGE.md) for the header-by-header audit of implemented, partial, and skipped APIs.

## License

Licensed under either of:

- MIT license ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
