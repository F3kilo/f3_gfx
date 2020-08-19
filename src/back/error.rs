use std::error::Error;
use std::fmt;

#[derive(Debug, Copy, Clone)]
pub enum LoadError {
    FileUnavalabale,
    BadFormat,
    NotEnoughMemory,
}

impl Error for LoadError {}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Self::FileUnavalabale => write!(f, "Resource file is unavailable."),
            Self::BadFormat => write!(f, "Can't parse resource format."),
            Self::NotEnoughMemory => write!(f, "Not enough memory for resource."),
        }
    }
}
