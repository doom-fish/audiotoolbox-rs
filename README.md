# audiotoolbox-rs

Safe Rust bindings for Apple’s `AudioToolbox.framework` on macOS via a Swift bridge.

Requires macOS 10.15 or later.

## Covered areas

`audiotoolbox-rs` ships bridge-backed wrappers for:

- `AudioFormat`
- `AudioFile`
- `AudioFileComponent`
- `ExtAudioFile`
- `AudioConverter`
- `AudioComponent`
- `AudioUnit`
- `AUAudioUnit`
- `AUGraph`
- `AVFAudio` (`AVAudioEngine` / `AVAudioNode` / `AVAudioFormat` / `AVAudioPCMBuffer` / `AVAudioSequencer`)
- `AudioQueue`
- `MusicSequence` / `MusicPlayer` / `MusicEventIterator`
- `AudioServices`
- `AudioFileStream`
- `CAFFile`

The original raw C surface is still available behind the `raw-ffi` Cargo feature, and the audit-filling long-tail C declarations are available under `audiotoolbox::generated_c_types`. Many Objective-C classes from the audit list (for example `AVSpeechSynthesizer` or `AVAudioPlayerNode`) exist only as opaque placeholder handles with no methods.

## Installation

```toml
[dependencies]
audiotoolbox = "0.5.0"
```

To reach the legacy raw C bindings as well:

```toml
[dependencies]
audiotoolbox = { version = "0.5.0", features = ["raw-ffi"] }
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

## Safety model

- Typed property getters (`AudioUnit::get_property_typed`, `AudioFile::get_property_typed` and `get_property_array`, `MusicTrack::get_property_typed`) accept only types that implement the `AudioProperty` marker trait: plain-old-data types for which every bit pattern is valid. They fail unless the framework returns exactly `size_of::<T>()` bytes. `AudioProperty` is an `unsafe` trait; implement it for your own `#[repr(C)]` types only if they meet that rule.
- Calls whose meaning depends on a property id or on caller-owned pointers are `unsafe`: the `set_property_typed` methods, `AudioFile::set_property_bytes` and `global_info_*`, `AudioFormat::property_info`, `AUGraph::set_node_input_callback`, and the raw `add_render_notify` / `add_property_listener` registrations.
- `AudioUnit::render` and `AudioConverter::convert_complex_buffer` take `OwnedAudioBufferList` values; the buffer count, channel counts and capacities are checked against the stream formats before the call.
- `AudioConverter::fill_complex_buffer` copies each input chunk and keeps it alive until the converter asks for more input. It never signals end of stream; call `finish` (until it returns no packets) at the end of the input, and `reset` before converting a new stream.
- `MusicTrack` and `MusicEventIterator` borrow their `MusicSequence`, and tracks are disposed through `MusicSequence::dispose_track(&mut self, index)`. Variable-length events (raw MIDI data, meta, user and extended note events) are built from slices.
- `AudioQueueBufferHandle` borrows its `AudioQueue` and frees its buffer when dropped.
- Apple's DLS synth crashes when instances are initialized or uninitialized on several threads at once, even separate instances. The crate serializes audio unit, `AUGraph`, `MusicPlayer` and `MusicSequence` creation, initialization, start/stop and disposal behind one process-wide lock; code outside this crate that does the same on other threads is not covered by that lock.

## Callbacks

- `AudioQueue::new_output_with_callback` and `new_input_with_callback` run a closure on the queue's own thread for every returned buffer; return `true` to enqueue the buffer again. `set_offline_render_format` and `offline_render` render an output queue without an output device.
- `SystemSound::play_with_completion` and `play_alert_with_completion` run a closure once the sound has finished.
- `AudioFileStream::parse_bytes` returns the packets parsed by each call.

## Async callback streams

Enable the `async` feature for executor-agnostic wrappers over property listeners and render-notify callbacks. Both hand events from the callback to the stream through a lock-free ring that drops the oldest event when it is full, and the callback path takes no locks and allocates nothing. The callback context stays alive until the audio unit or graph is disposed, so a callback that is already running when a stream is dropped never touches freed memory. `try_next` never involves the executor; awaiting `next()` registers a waker, and the render thread then runs your executor's wake code. Synchronous pull-render callbacks remain manual.

```toml
[dependencies]
audiotoolbox = { version = "0.5.0", features = ["async"] }
```

```rust,no_run
use audiotoolbox::{
    AudioUnit, AUDIO_COMPONENT_TYPE_FORMAT_CONVERTER, AUDIO_UNIT_PROPERTY_STREAM_FORMAT,
    AUDIO_UNIT_SUBTYPE_AU_CONVERTER,
};

# async fn run() -> Result<(), audiotoolbox::AudioToolboxError> {
let unit = AudioUnit::new_apple(
    AUDIO_COMPONENT_TYPE_FORMAT_CONVERTER,
    AUDIO_UNIT_SUBTYPE_AU_CONVERTER,
)?;
let property_stream = unit.property_events(AUDIO_UNIT_PROPERTY_STREAM_FORMAT, 16)?;
let render_stream = unit.render_notify_stream(128)?;
let _ = property_stream.buffered_count() + render_stream.buffered_count();
# Ok(())
# }
```

## Highlights

- ScreenCaptureKit-style Swift bridge with one bridge file per logical area
- Owned Rust handles with `Drop`
- `AudioFile`, `AudioFileComponent`, `ExtAudioFile`, and `AudioFileStream` smoke-tested against `Glass.aiff`
- In-memory `AudioConverter` streaming conversion with owned input
- `AudioUnit`, `AUAudioUnit`, `AUGraph`, and `MusicSequence` / `MusicPlayer` / `MusicEventIterator` creation helpers
- `AVAudioEngine` / `AVAudioNode` / `AVAudioFormat` / `AVAudioPCMBuffer` / `AVAudioSequencer` wrappers for the common `AVFAudio` path plus exact-name audit aliases for the remaining sampled surface
- Pure-Rust `CAFFile` header parsing helpers
- 15 numbered examples and 18 integration test files, including a check of every former audit symbol's size against the SDK headers

## Not wrapped yet

- The macOS 26 converter additions (`AudioConverterFillComplexBufferRealtimeSafe`, `AudioConverterNewWithOptions`, `AudioConverterPrepare`).
- AudioQueue property listeners, timelines, processing taps and the dispatch-queue constructors.
- AVFAudio beyond `AVAudioEngine`, `AVAudioNode`, `AVAudioFormat`, `AVAudioPCMBuffer` and `AVAudioSequencer`.

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
cargo build --all-targets --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo +1.82.0 check --lib --all-features
```

## Coverage audit

See [`COVERAGE.md`](COVERAGE.md) for the header-by-header audit of implemented, partial, and skipped APIs. [`COVERAGE_AUDIT.md`](COVERAGE_AUDIT.md) is a scoreboard over a self-selected sample of 300 symbols; it only shows that a Rust item of each name exists, and lists method-less placeholders separately.

## License

Licensed under either of:

- MIT license ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
