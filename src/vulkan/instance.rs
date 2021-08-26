extern crate vulkano;

use vulkano::instance::{
    Instance,
    InstanceExtensions,
    ApplicationInfo,
    Version,
};

use std::sync::Arc;

pub struct VulkanInstance {
    instance: Arc<Instance>,
}

impl VulkanInstance {
    pub fn new(name: &String) -> Self {
        let supported_extensions = InstanceExtensions::supported_by_core()
            .expect("failed to retrieve supported extensions");
        println!("Supported extensions: {:?}", supported_extensions);

        let app_info = ApplicationInfo {
            application_name: Some(name.into()),
            application_version: Some(Version { major: 1, minor: 0, patch: 0 }),
            engine_name: Some("No Engine".into()),
            engine_version: Some(Version { major: 1, minor: 0, patch: 0 }),
        };

        let required_extensions = vulkano_win::required_extensions();

        let instance = Instance::new(Some(&app_info), Version::V1_1, &required_extensions, None)
            .expect("failed to create Vulkan instance");

        Self {
            instance,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::vulkan::VulkanInstance;

    #[test]
    fn verify_instance_creation() {
        VulkanInstance::new(&String::from("Vulkan"));
    }
}
