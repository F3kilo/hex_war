use crate::graphics::backend::vulkan::vk::instance::Instance;
use ash::version::InstanceV1_0;
use std::fmt;
use std::rc::Rc;

#[derive(Clone)]
pub struct PhysicalDevice {
    instance: Rc<Instance>,
    handle: ash::vk::PhysicalDevice,
}

impl PhysicalDevice {
    pub fn enumerate(instance: Rc<Instance>) -> Result<Vec<Self>, EnumerateError> {
        let pdevices = unsafe { instance.raw_handle().enumerate_physical_devices()? };
        let selfs = pdevices
            .into_iter()
            .map(|handle| Self {
                handle,
                instance: instance.clone(),
            })
            .collect();
        Ok(selfs)
    }

    pub fn enumerate_device_extension_properties(
        &self,
    ) -> Result<Vec<ash::vk::ExtensionProperties>, EnumerateExtensionsError> {
        let props = unsafe {
            self.instance
                .raw_handle()
                .enumerate_device_extension_properties(self.handle)
        }?;
        Ok(props)
    }

    pub fn get_physical_device_features(&self) -> ash::vk::PhysicalDeviceFeatures {
        unsafe {
            self.instance
                .raw_handle()
                .get_physical_device_features(self.handle)
        }
    }

    pub fn get_physical_device_format_properties(
        &self,
        format: ash::vk::Format,
    ) -> ash::vk::FormatProperties {
        unsafe {
            self.instance
                .raw_handle()
                .get_physical_device_format_properties(self.handle, format)
        }
    }

    pub fn get_physical_device_image_format_properties(
        &self,
        format: ash::vk::Format,
        typ: ash::vk::ImageType,
        tiling: ash::vk::ImageTiling,
        usage: ash::vk::ImageUsageFlags,
        flags: ash::vk::ImageCreateFlags,
    ) -> Result<ash::vk::ImageFormatProperties, GetImageFormatError> {
        unsafe {
            let props = self
                .instance
                .raw_handle()
                .get_physical_device_image_format_properties(
                    self.handle,
                    format,
                    typ,
                    tiling,
                    usage,
                    flags,
                )?;
            Ok(props)
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum EnumerateError {
    OutOfHostMemory,
    OutOfDeviceMemory,
    InitializationFailed,
    Unexpected,
}

impl fmt::Display for EnumerateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f) // TODO: advices for diffirent error types
    }
}

impl From<ash::vk::Result> for EnumerateError {
    fn from(r: ash::vk::Result) -> Self {
        match r {
            ash::vk::Result::ERROR_OUT_OF_HOST_MEMORY => EnumerateError::OutOfHostMemory,
            ash::vk::Result::ERROR_OUT_OF_DEVICE_MEMORY => EnumerateError::OutOfDeviceMemory,
            ash::vk::Result::ERROR_INITIALIZATION_FAILED => EnumerateError::InitializationFailed,
            _ => EnumerateError::Unexpected,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum EnumerateExtensionsError {
    OutOfHostMemory,
    OutOfDeviceMemory,
    LayerNotPresent,
    Unexpected,
}

impl fmt::Display for EnumerateExtensionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f) // TODO: advices for diffirent error types
    }
}

impl From<ash::vk::Result> for EnumerateExtensionsError {
    fn from(r: ash::vk::Result) -> Self {
        match r {
            ash::vk::Result::ERROR_OUT_OF_HOST_MEMORY => EnumerateExtensionsError::OutOfHostMemory,
            ash::vk::Result::ERROR_OUT_OF_DEVICE_MEMORY => {
                EnumerateExtensionsError::OutOfDeviceMemory
            }
            ash::vk::Result::ERROR_LAYER_NOT_PRESENT => EnumerateExtensionsError::LayerNotPresent,
            _ => EnumerateExtensionsError::Unexpected,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GetImageFormatError {
    OutOfHostMemory,
    OutOfDeviceMemory,
    FormatNotSupported,
    Unexpected,
}

impl fmt::Display for GetImageFormatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f) // TODO: advices for diffirent error types
    }
}

impl From<ash::vk::Result> for GetImageFormatError {
    fn from(r: ash::vk::Result) -> Self {
        match r {
            ash::vk::Result::ERROR_OUT_OF_HOST_MEMORY => GetImageFormatError::OutOfHostMemory,
            ash::vk::Result::ERROR_OUT_OF_DEVICE_MEMORY => GetImageFormatError::OutOfDeviceMemory,
            ash::vk::Result::ERROR_FORMAT_NOT_SUPPORTED => GetImageFormatError::FormatNotSupported,
            _ => GetImageFormatError::Unexpected,
        }
    }
}
