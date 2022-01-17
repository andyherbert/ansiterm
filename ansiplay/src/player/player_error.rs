#[derive(Debug)]
pub enum PlayerError {
    ThreadError,
}

impl std::fmt::Display for PlayerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayerError::ThreadError => write!(f, "Thread Error"),
        }
    }
}

impl std::error::Error for PlayerError {}
