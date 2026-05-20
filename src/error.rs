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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_status_preserves_operation_and_status() {
        let error = AudioToolboxError::from_status("AudioQueueStart", -50);

        assert_eq!(error.operation(), "AudioQueueStart");
        assert_eq!(error.status(), Some(-50));
    }

    #[test]
    fn message_constructor_preserves_operation_and_formats_message() {
        let error = AudioToolboxError::message("AudioQueueStart", "device unavailable");

        assert_eq!(error.operation(), "AudioQueueStart");
        assert_eq!(error.status(), None);
        assert_eq!(
            error.to_string(),
            "AudioQueueStart failed: device unavailable"
        );
    }

    #[test]
    fn display_includes_printable_fourcc_for_status_errors() {
        let status = i32::from_be_bytes(*b"lpcm");
        let error = AudioToolboxError::from_status("AudioFormatGetProperty", status);

        assert_eq!(
            error.to_string(),
            format!("AudioFormatGetProperty failed with OSStatus {status} ('lpcm')"),
        );
    }

    #[test]
    fn display_omits_fourcc_for_non_printable_status_errors() {
        let error = AudioToolboxError::from_status("AudioQueueStart", -50);

        assert_eq!(
            error.to_string(),
            "AudioQueueStart failed with OSStatus -50"
        );
    }
}
