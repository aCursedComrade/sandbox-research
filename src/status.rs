use std::fmt::{Debug, Display};

/// Represents an exception/failure instance.
#[derive(Clone, Copy)]
pub enum Status {
    ConnectionFailed,
    ResponseError,
    ThreadError,
    EmptyError,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::ConnectionFailed => write!(f, "Failed to connect to server"),
            Self::ResponseError => write!(f, "Server replied with error"),
            Self::ThreadError => write!(f, "Thread panicked"),
            Self::EmptyError => write!(f, "Last error message was not set"),
        }
    }
}

impl Debug for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
