use std::{error, fmt};

/// An error representing a variety of outcomes.
#[derive(Debug)]
pub enum StdInReceiverError {
    ThreadDisconnected,
    UnableToJoin,
    UnableToSend,
    UnableToReadFromStdIn,
}

impl fmt::Display for StdInReceiverError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StdInReceiverError::ThreadDisconnected => write!(f, "Thread disconnected"),
            StdInReceiverError::UnableToJoin => write!(f, "Unable to join with thread"),
            StdInReceiverError::UnableToSend => write!(f, "Unable to send to thread"),
            StdInReceiverError::UnableToReadFromStdIn => write!(f, "Unable to read from stdin"),
        }
    }
}

impl error::Error for StdInReceiverError {}
