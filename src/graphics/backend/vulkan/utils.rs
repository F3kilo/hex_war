pub struct Version {
    version: u32,
}

impl Version {
    pub fn new(version: u32) -> Self {
        Self { version }
    }

    pub fn from_parts(major: u32, minor: u32, patch: u32) -> Self {
        let version = ash::vk::make_version(major, minor, patch);
        Self { version }
    }

    pub fn major(&self) -> u32 {
        ash::vk::version_major(self.version)
    }

    pub fn minor(&self) -> u32 {
        ash::vk::version_minor(self.version)
    }

    pub fn patch(&self) -> u32 {
        ash::vk::version_patch(self.version)
    }
}
