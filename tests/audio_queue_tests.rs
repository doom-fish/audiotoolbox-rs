use audiotoolbox::{AudioQueue, AudioStreamBasicDescription, Result, AUDIO_FORMAT_LINEAR_PCM};

#[test]
fn audio_queue_allocates_buffer_and_sets_volume() -> Result<()> {
    let format = AudioStreamBasicDescription::linear_pcm_i16(44_100.0, 1, true);
    let queue = AudioQueue::new_output(&format)?;
    let stream = queue.stream_description()?;
    let buffer = queue.allocate_buffer(512)?;

    queue.set_volume(0.25)?;
    assert_eq!(stream.mFormatID, AUDIO_FORMAT_LINEAR_PCM);
    assert!(buffer.audio_data_bytes_capacity() >= 512);
    assert!(queue.volume()? >= 0.0);
    assert!(!queue.is_running()?);
    Ok(())
}
