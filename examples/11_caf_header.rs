use audiotoolbox::CafFile;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"caff");
    bytes.extend_from_slice(&1_u16.to_be_bytes());
    bytes.extend_from_slice(&0_u16.to_be_bytes());
    bytes.extend_from_slice(b"desc");
    bytes.extend_from_slice(&32_i64.to_be_bytes());

    let caf = CafFile::parse(&bytes)?;
    println!("caf_version={} first_chunk={:?}", caf.file_version(), caf.first_chunk());
    Ok(())
}
