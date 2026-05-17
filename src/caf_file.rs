use crate::{ffi, AudioToolboxError, CAFChunkHeader, CAFFileHeader, Result, CAF_FILE_MAGIC};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CafFile {
    header: CAFFileHeader,
    first_chunk: Option<CAFChunkHeader>,
}

impl CafFile {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 8 {
            return Err(AudioToolboxError::message(
                "CafFile::parse",
                "CAF data is shorter than the 8-byte file header",
            ));
        }
        let data_len = u32::try_from(bytes.len()).unwrap_or(u32::MAX);
        if unsafe { ffi::caf_file::at_caf_is_caf_file(bytes.as_ptr(), data_len) } == 0 {
            return Err(AudioToolboxError::message(
                "CafFile::parse",
                "data does not begin with the 'caff' CAF magic",
            ));
        }

        let header = CAFFileHeader {
            mFileType: u32::from_be_bytes(bytes[0..4].try_into().expect("slice length checked")),
            mFileVersion: u16::from_be_bytes(bytes[4..6].try_into().expect("slice length checked")),
            mFileFlags: u16::from_be_bytes(bytes[6..8].try_into().expect("slice length checked")),
        };

        let first_chunk = if bytes.len() >= 20 {
            Some(CAFChunkHeader {
                mChunkType: u32::from_be_bytes(
                    bytes[8..12].try_into().expect("slice length checked"),
                ),
                mChunkSize: i64::from_be_bytes(
                    bytes[12..20].try_into().expect("slice length checked"),
                ),
            })
        } else {
            None
        };

        Ok(Self {
            header,
            first_chunk,
        })
    }

    pub fn header(&self) -> CAFFileHeader {
        self.header
    }

    pub fn first_chunk(&self) -> Option<CAFChunkHeader> {
        self.first_chunk
    }

    pub fn is_caf(bytes: &[u8]) -> bool {
        let data_len = u32::try_from(bytes.len()).unwrap_or(u32::MAX);
        unsafe { ffi::caf_file::at_caf_is_caf_file(bytes.as_ptr(), data_len) != 0 }
    }

    pub fn file_type(&self) -> u32 {
        self.header.mFileType
    }

    pub fn file_version(&self) -> u16 {
        self.header.mFileVersion
    }

    pub fn file_flags(&self) -> u16 {
        self.header.mFileFlags
    }

    pub fn is_magic_valid(&self) -> bool {
        self.header.mFileType == CAF_FILE_MAGIC
    }
}
