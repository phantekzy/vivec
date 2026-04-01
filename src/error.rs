use std::fmt;

#[derive(Debug)]
pub enum VanguardError {
    Io(std::io::Error),
    Parse(String),
    Network(String),
    InvalidUrl(String),
    Timeout,
}

impl std::error::Error for VanguardError {}

impl fmt::Display for VanguardError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VanguardError::Io(e) => write!(f, "IO Error: {}", e),
            VanguardError::Parse(s) => write!(f, "Parsing Error: {}", s),
            VanguardError::Network(s) => write!(f, "Network Error: {}", s),
            VanguardError::InvalidUrl(u) => write!(f, "Invalid URL: {}", u),
            VanguardError::Timeout => write!(f, "Request timed out"),
        }
    }
}

impl From<std::io::Error> for VanguardError {
    fn from(err: std::io::Error) -> Self {
        VanguardError::Io(err)
    }
}
