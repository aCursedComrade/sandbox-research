use std::fmt::{Debug, Display};

/// Represents an exception/failure instance.
#[derive(Debug, Clone)]
pub enum Status {
    ProcessSpawnFailed(String),
    ProcessStopFailed(String),
    NoSuchProfile(u32),
    ConnectionFailed,
    ResponseError,
    ThreadError,
    EmptyError,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ProcessSpawnFailed(name) => write!(f, "Failed to spawn process: {}", name),
            Self::ProcessStopFailed(name) => write!(f, "Failed to stop process: {}", name),
            Self::NoSuchProfile(id) => write!(f, "No such profile exist (internal ID: {})", id),
            Self::ConnectionFailed => write!(f, "Failed to connect to server"),
            Self::ResponseError => write!(f, "Server replied with error"),
            Self::ThreadError => write!(f, "Thread panicked"),
            Self::EmptyError => write!(f, "Last error message was not set"),
        }
    }
}
