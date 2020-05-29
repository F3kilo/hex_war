use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum AppInitError {
    CantCreateWindow(WindowCreateError),
}

impl Error for AppInitError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AppInitError::CantCreateWindow(e) => Some(e),
        }
    }
}

impl Display for AppInitError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            AppInitError::CantCreateWindow(e) => e.fmt(f),
        }
    }
}

impl From<WindowCreateError> for AppInitError {
    fn from(source: WindowCreateError) -> Self {
        AppInitError::CantCreateWindow(source)
    }
}

pub type WindowCreateError = winit::error::OsError;
