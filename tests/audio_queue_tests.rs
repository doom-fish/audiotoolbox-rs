use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use audiotoolbox::{AudioQueue, AudioStreamBasicDescription, Result, AUDIO_FORMAT_LINEAR_PCM};

const MONO_LAYOUT: u32 = (100 << 16) | 1;
const LEVEL: i16 = 1000;

fn fill_level(data: &mut [u8]) {
    for sample in data.chunks_exact_mut(2) {
        sample.copy_from_slice(&LEVEL.to_ne_bytes());
    }
}

#[test]
fn audio_queue_allocates_buffer_and_sets_volume() -> Result<()> {
    let format = AudioStreamBasicDescription::linear_pcm_i16(44_100.0, 1, true);
    let queue = AudioQueue::new_output(&format)?;
    let stream = queue.stream_description()?;
    let mut buffer = queue.allocate_buffer(512)?;

    queue.set_volume(0.25)?;
    assert_eq!(stream.mFormatID, AUDIO_FORMAT_LINEAR_PCM);
    assert!(buffer.audio_data_bytes_capacity() >= 512);
    assert_eq!(
        buffer.data_mut().len(),
        buffer.audio_data_bytes_capacity() as usize
    );
    assert!(buffer
        .set_byte_size(buffer.audio_data_bytes_capacity() + 1)
        .is_err());
    buffer.set_byte_size(16)?;
    assert_eq!(buffer.data().len(), 16);
    assert!(queue.volume()? >= 0.0);
    assert!(!queue.is_running()?);
    Ok(())
}

#[test]
fn buffers_cannot_move_between_queues() -> Result<()> {
    let format = AudioStreamBasicDescription::linear_pcm_i16(44_100.0, 1, true);
    let first = AudioQueue::new_output(&format)?;
    let second = AudioQueue::new_output(&format)?;
    let mut buffer = first.allocate_buffer(64)?;
    buffer.set_byte_size(64)?;
    assert!(second.enqueue_buffer(buffer).is_err());
    let mut output = first.allocate_buffer(64)?;
    assert!(second.offline_render(0.0, &mut output, 1).is_err());
    assert!(first.offline_render(0.0, &mut output, 1).is_err());
    Ok(())
}

#[test]
fn output_callback_refills_buffers_during_offline_render() -> Result<()> {
    let format = AudioStreamBasicDescription::linear_pcm_i16(44_100.0, 1, true);
    let calls = Arc::new(AtomicUsize::new(0));
    let counter = Arc::clone(&calls);
    let queue = AudioQueue::new_output_with_callback(&format, move |buffer| {
        let call = counter.fetch_add(1, Ordering::SeqCst);
        fill_level(buffer.data_mut());
        let capacity = buffer.capacity();
        buffer.set_byte_size(capacity).is_ok() && call < 32
    })?;
    for _ in 0..3 {
        let mut buffer = queue.allocate_buffer(512)?;
        fill_level(buffer.data_mut());
        buffer.set_byte_size(512)?;
        queue.enqueue_buffer(buffer)?;
    }
    queue.set_offline_render_format(&format, MONO_LAYOUT)?;
    queue.start()?;

    let mut output = queue.allocate_buffer(512)?;
    queue.offline_render(0.0, &mut output, 0)?;
    assert!(queue.offline_render(0.0, &mut output, 257).is_err());
    let mut rendered_level = false;
    let mut sample_time = 0.0;
    for _ in 0..16 {
        queue.offline_render(sample_time, &mut output, 256)?;
        rendered_level |= output
            .data()
            .chunks_exact(2)
            .any(|sample| i16::from_ne_bytes([sample[0], sample[1]]) == LEVEL);
        sample_time += 256.0;
    }
    queue.stop(true)?;

    assert!(
        calls.load(Ordering::SeqCst) > 0,
        "the output callback never ran"
    );
    assert!(
        rendered_level,
        "the offline render never produced the queued samples"
    );
    Ok(())
}
