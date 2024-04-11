use std::fmt::{Debug, Display};

pub enum FailStatus {
    ConnectionFailed,
    ResponseError,
    ThreadError,
}

impl Display for FailStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::ConnectionFailed => write!(f, "Failed to connect to server"),
            Self::ResponseError => write!(f, "Server replied with error"),
            Self::ThreadError => write!(f, "Thread panicked"),
        }
    }
}

impl Debug for FailStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
