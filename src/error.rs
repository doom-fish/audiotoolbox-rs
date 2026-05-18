use crate::OSStatus;
use std::{borrow::Cow, error::Error, fmt};

/// Result alias used by AudioToolbox.framework wrappers in this crate.
pub type Result<T> = std::result::Result<T, AudioToolboxError>;

#[derive(Debug, Clone)]
/// Error wrapper for `OSStatus` values returned by AudioToolbox.framework.
pub struct AudioToolboxError {
    operation: &'static str,
    status: Option<OSStatus>,
    message: Option<Cow<'static, str>>,
}

impl AudioToolboxError {
    /// Wraps `AudioToolboxErrorFromStatus`.
    pub(crate) fn from_status(operation: &'static str, status: OSStatus) -> Self {
        Self {
            operation,
            status: Some(status),
            message: None,
        }
    }

    /// Wraps `AudioToolboxErrorMessage`.
    pub(crate) fn message(operation: &'static str, message: impl Into<Cow<'static, str>>) -> Self {
        Self {
            operation,
            status: None,
            message: Some(message.into()),
        }
    }

    /// Wraps `AudioToolboxErrorOperation`.
    pub fn operation(&self) -> &'static str {
        self.operation
    }

    /// Wraps `AudioToolboxErrorStatus`.
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
