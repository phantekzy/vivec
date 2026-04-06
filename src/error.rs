use std::fmt;

#[derive(Debug)]
pub enum XsusError {
    Io(std::io::Error),
    Parse(String),
    Network(String),
    InvalidUrl(String),
    Timeout,
}

impl std::error::Error for XsusError {}

impl fmt::Display for XsusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            XsusError::Io(e) => write!(f, "IO Error : {}", e),
            XsusError::Parse(s) => write!(f, "Parsing Error : {}", s),
            XsusError::Network(s) => writeln!(f, "Network Error : {}", s),
            XsusError::InvalidUrl(u) => writeln!(f, "Invalid URL : {}", u),
            XsusError::Timeout => writeln!(f, "Request timed out"),
        }
    }
}

impl From<std::io::Error> for XsusError {
    fn from(err: std::io::Error) -> Self {
        XsusError::Io(err)
    }
}
