use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ChpErrorCode {
    #[error("VERSION_MISMATCH")]
    VersionMismatch,
    #[error("AUTH_REQUIRED")]
    AuthRequired,
    #[error("SESSION_NOT_FOUND")]
    SessionNotFound,
    #[error("INVALID_MESSAGE")]
    InvalidMessage,
    #[error("TOOL_DENIED")]
    ToolDenied,
    #[error("INTERNAL")]
    Internal,
}

impl ChpErrorCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::VersionMismatch => "VERSION_MISMATCH",
            Self::AuthRequired => "AUTH_REQUIRED",
            Self::SessionNotFound => "SESSION_NOT_FOUND",
            Self::InvalidMessage => "INVALID_MESSAGE",
            Self::ToolDenied => "TOOL_DENIED",
            Self::Internal => "INTERNAL",
        }
    }
}
