# audiotoolbox-rs

Safe Rust bindings for Apple’s `AudioToolbox.framework` on macOS.

`audiotoolbox-rs` wraps the practical low-level pieces you reach for first:

- `AudioFile` for packet-oriented file I/O and property queries
- `ExtAudioFile` for client-format conversion during reads and writes
- `AudioConverter` for one-shot `AudioConverterFillComplexBuffer` pipelines
- `AudioComponent` discovery / instantiation helpers
- `SystemSound` playback and `CAShow*` debug helpers

## Status

Initial `0.1.0` coverage targets pure-C AudioToolbox APIs only. The crate is aimed
at audio-file inspection, simple PCM conversion pipelines, component discovery,
and lightweight system-sound playback.

## Installation

```toml
[dependencies]
audiotoolbox = "0.1"
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

- Pure Rust + C FFI (no Swift bridge required)
- Safe wrappers for the requested AudioToolbox entry points
- Convenience constructors for common linear PCM `AudioStreamBasicDescription`s
- `examples/01_read_glass.rs` smoke test for `/System/Library/Sounds/Glass.aiff`

## API notes

- `AudioFile::read_packet_data` and `AudioFile::read_packets` return `PacketData`
  with both byte payloads and packet descriptions.
- `ExtAudioFile::set_client_data_format` + `InterleavedAudioBuffer` cover the
  common “decode into interleaved PCM” path.
- `AudioConverter::fill_complex_buffer_once` is designed for one-shot in-memory
  conversions; larger streaming scenarios can call into the lower-level APIs in
  repeated chunks.
- `ca_show_to_stdout`, `ca_show_to_stderr`, and `flush_debug_output` provide the
  requested `CAShowFile`-style debugging workflow.

## Smoke example

```bash
cargo run --example 01_read_glass
```

Expected tail output:

```text
✅ audiotoolbox read OK
```

## License

Licensed under either of:

- MIT license ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
