use audiotoolbox::{CafFile, Result, CAF_FILE_MAGIC};

#[test]
fn caf_file_parses_synthetic_header() -> Result<()> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"caff");
    bytes.extend_from_slice(&1_u16.to_be_bytes());
    bytes.extend_from_slice(&0_u16.to_be_bytes());
    bytes.extend_from_slice(b"desc");
    bytes.extend_from_slice(&32_i64.to_be_bytes());

    let caf = CafFile::parse(&bytes)?;
    assert!(caf.is_magic_valid());
    assert_eq!(caf.file_type(), CAF_FILE_MAGIC);
    assert_eq!(caf.file_version(), 1);
    assert_eq!(caf.first_chunk().expect("chunk header").mChunkSize, 32);
    Ok(())
}
