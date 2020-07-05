use crate::graphics::backend::vulkan::utils::Version;
use crate::graphics::backend::vulkan::vk::entry::Entry;
use ash::version::{EntryV1_0, InstanceV1_0};
use std::error::Error;
use std::ffi::CString;
use std::fmt;
use std::os::raw::c_char;

pub struct CreateInfo {
    pub app_name: CString,
    pub app_version: Version,
    pub engine_name: CString,
    pub engine_version: Version,
    pub api_version: Version,
    pub extensions: Vec<ash::vk::ExtensionProperties>,
    pub layers: Vec<ash::vk::LayerProperties>,
}

pub struct Instance {
    handle: ash::Instance,
    entry: Entry,
}

impl Instance {
    pub fn new(entry: Entry, create_info: &CreateInfo) -> Result<Self, InstanceInitError> {
        let mut ext_names = Vec::new();
        let mut layer_names = Vec::new();
        unsafe {
            let app_info = Self::get_raw_application_info(create_info);
            let raw_create_info =
                Self::get_raw_create_info(create_info, &app_info, &mut ext_names, &mut layer_names);
            let handle = entry.raw_handle().create_instance(&raw_create_info, None)?;
            Ok(Self { entry, handle })
        }
    }

    unsafe fn get_raw_create_info(
        create_info: &CreateInfo,
        app_info: &ash::vk::ApplicationInfo,
        ext_names: &mut Vec<*const c_char>,
        layer_names: &mut Vec<*const c_char>,
    ) -> ash::vk::InstanceCreateInfo {
        ext_names.extend(
            create_info
                .extensions
                .iter()
                .map(|i| i.extension_name.as_ptr()),
        );

        layer_names.extend(create_info.layers.iter().map(|i| i.layer_name.as_ptr()));

        let raw_create_info = ash::vk::InstanceCreateInfo::builder()
            .enabled_extension_names(ext_names.as_slice())
            .enabled_layer_names(layer_names)
            .application_info(app_info);

        raw_create_info.build()
    }

    unsafe fn get_raw_application_info(create_info: &CreateInfo) -> ash::vk::ApplicationInfo {
        let raw_app_info = ash::vk::ApplicationInfo::builder()
            .application_name(create_info.app_name.as_c_str())
            .application_version(create_info.app_version.raw())
            .engine_name(create_info.engine_name.as_c_str())
            .engine_version(create_info.engine_version.raw())
            .api_version(create_info.api_version.raw());

        raw_app_info.build()
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe { self.handle.destroy_instance(None) }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum InstanceInitError {
    LoadError,
    InstanceError(InstanceError),
}

impl Error for InstanceInitError {}

impl fmt::Display for InstanceInitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InstanceInitError::LoadError => write!(
                f,
                "Can't init Vulkan instance. Try to install VulkanAPI redistributable."
            ),
            InstanceInitError::InstanceError(e) => fmt::Display::fmt(e, f),
        }
    }
}

impl From<ash::InstanceError> for InstanceInitError {
    fn from(e: ash::InstanceError) -> Self {
        match e {
            ash::InstanceError::LoadError(_) => InstanceInitError::LoadError,
            ash::InstanceError::VkError(e) => InstanceInitError::InstanceError(e.into()),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum InstanceError {
    OutOfHostMemory,
    OutOfDeviceMemory,
    InitializationFailed,
    LayerNotPresent,
    ExtensionNotPresent,
    IncompatibleDriver,
    Unexpected,
}

impl fmt::Display for InstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f) // TODO: advices for diffirent error types
    }
}

impl From<ash::vk::Result> for InstanceError {
    fn from(r: ash::vk::Result) -> Self {
        match r {
            ash::vk::Result::ERROR_OUT_OF_HOST_MEMORY => InstanceError::OutOfHostMemory,
            ash::vk::Result::ERROR_OUT_OF_DEVICE_MEMORY => InstanceError::OutOfDeviceMemory,
            ash::vk::Result::ERROR_INITIALIZATION_FAILED => InstanceError::InitializationFailed,
            ash::vk::Result::ERROR_LAYER_NOT_PRESENT => InstanceError::LayerNotPresent,
            ash::vk::Result::ERROR_EXTENSION_NOT_PRESENT => InstanceError::ExtensionNotPresent,
            ash::vk::Result::ERROR_INCOMPATIBLE_DRIVER => InstanceError::IncompatibleDriver,
            _ => InstanceError::Unexpected,
        }
    }
}
