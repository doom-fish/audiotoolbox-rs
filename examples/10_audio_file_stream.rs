use audiotoolbox::{AudioFileStream, AUDIO_FILE_AIFF_TYPE};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bytes = std::fs::read("/System/Library/Sounds/Glass.aiff")?;
    let stream = AudioFileStream::open(AUDIO_FILE_AIFF_TYPE)?;
    stream.parse_bytes(&bytes, 0)?;

    println!(
        "format={} ready={} packets_seen={}",
        stream.file_format()?,
        stream.ready_to_produce_packets(),
        stream.packet_count_seen()
    );
    Ok(())
}
