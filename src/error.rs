use std::error::Error;
use std::fmt;

#[derive(Debug, Copy, Clone, Default)]
pub struct NotFoundError {}

impl Error for NotFoundError {}

impl fmt::Display for NotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Resource with specified id is not found.")
    }
}
