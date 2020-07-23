use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum LoadError {}

impl Error for LoadError {}
impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Can't load resource")
    }
}

#[derive(Debug)]
pub enum UnavailableError {
    Loading,
    NotFound,
}

impl Error for UnavailableError {}
impl fmt::Display for UnavailableError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Resource with specified ID is not found.")
    }
}
