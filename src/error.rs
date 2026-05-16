use crate::ffi::OSStatus;
use std::{borrow::Cow, error::Error, fmt};

pub type Result<T> = std::result::Result<T, AudioToolboxError>;

#[derive(Debug, Clone)]
pub struct AudioToolboxError {
    operation: &'static str,
    status: Option<OSStatus>,
    message: Option<Cow<'static, str>>,
}

impl AudioToolboxError {
    pub(crate) fn from_status(operation: &'static str, status: OSStatus) -> Self {
        Self {
            operation,
            status: Some(status),
            message: None,
        }
    }

    pub(crate) fn message(operation: &'static str, message: impl Into<Cow<'static, str>>) -> Self {
        Self {
            operation,
            status: None,
            message: Some(message.into()),
        }
    }

    pub fn operation(&self) -> &'static str {
        self.operation
    }

    pub fn status(&self) -> Option<OSStatus> {
        self.status
    }
}

impl fmt::Display for AudioToolboxError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(status) = self.status {
            let fourcc = crate::format::fourcc_to_string(status as u32);
            if crate::format::is_printable_fourcc(status as u32) {
                write!(
                    formatter,
                    "{} failed with OSStatus {status} ('{fourcc}')",
                    self.operation
                )
            } else {
                write!(
                    formatter,
                    "{} failed with OSStatus {status}",
                    self.operation
                )
            }
        } else if let Some(message) = &self.message {
            write!(formatter, "{} failed: {message}", self.operation)
        } else {
            write!(formatter, "{} failed", self.operation)
        }
    }
}

impl Error for AudioToolboxError {}
