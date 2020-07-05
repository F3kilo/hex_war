use crate::graphics::backend::vulkan::utils::Version;
use ash::version::EntryV1_0;
use ash::LoadingError;
use std::error::Error;
use std::fmt;
use std::fmt::Display;

#[derive(Clone)]
pub struct Entry {
    handle: ash::Entry,
}

impl Entry {
    pub fn new() -> Result<Self, EntryInitError> {
        let handle = ash::Entry::new()?;
        Ok(Self { handle })
    }

    pub fn try_enumerate_instance_version(&self) -> Result<Version, EnumerateError> {
        let raw_version = self.handle.try_enumerate_instance_version()?;
        Ok(match raw_version {
            None => Version::from_parts(1, 0, 0),
            Some(v) => Version::new(v),
        })
    }

    pub fn enumerate_instance_extension_properties(
        &self,
    ) -> Result<Vec<ash::vk::ExtensionProperties>, EnumerateError> {
        self.handle
            .enumerate_instance_extension_properties()
            .map_err(|e| e.into())
    }

    pub fn enumerate_instance_layer_properties(
        &self,
    ) -> Result<Vec<ash::vk::LayerProperties>, EnumerateError> {
        self.handle
            .enumerate_instance_layer_properties()
            .map_err(|e| e.into())
    }

    pub unsafe fn raw_handle(&self) -> ash::Entry {
        self.handle.clone()
    }
}

pub struct EntryInitError;

impl Error for EntryInitError {}

impl fmt::Display for EntryInitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Can't init Vulkan entry. Try to install VulkanAPI redistributable."
        )
    }
}

impl fmt::Debug for EntryInitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

impl From<LoadingError> for EntryInitError {
    fn from(_: LoadingError) -> Self {
        Self
    }
}

pub struct EnumerateError;

impl Error for EnumerateError {}

impl fmt::Display for EnumerateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Can't enumerate VulkanAPI properties. Try to reinstall VulkanAPI redistributable."
        )
    }
}

impl fmt::Debug for EnumerateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

impl From<ash::vk::Result> for EnumerateError {
    fn from(_: ash::vk::Result) -> Self {
        Self
    }
}
