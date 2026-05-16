use audiotoolbox::{AudioQueue, AudioStreamBasicDescription};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let format = AudioStreamBasicDescription::linear_pcm_i16(44_100.0, 1, true);
    let queue = AudioQueue::new_output(&format)?;
    let buffer = queue.allocate_buffer(512)?;
    queue.set_volume(0.25)?;

    println!(
        "volume={:.3} capacity={} running={}",
        queue.volume()?,
        buffer.audio_data_bytes_capacity(),
        queue.is_running()?
    );
    Ok(())
}
