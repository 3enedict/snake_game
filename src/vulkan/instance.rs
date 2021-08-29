extern crate vulkano;

use vulkano::instance::{
    Instance,
    InstanceExtensions,
    ApplicationInfo,
    Version,
    layers_list,
};

use vulkano::instance::debug::{DebugCallback, MessageSeverity, MessageType};

use std::sync::Arc;

use crate::vulkan::{
    DEBUG,
    VALIDATION_LAYERS,
};

pub struct VkInstance {
    instance: Option<Arc<Instance>>,
    debug_callback: Option<DebugCallback>,
}

impl VkInstance {
    pub fn new() -> Self {
        Self {
            instance: None,
            debug_callback: None,
        }
    }

    pub fn init(&mut self, name: &String) {
        let supported_extensions = InstanceExtensions::supported_by_core()
            .expect("failed to retrieve supported extensions");
        println!("Supported extensions: {:?}", supported_extensions);

        if Self::check_validation_layer_support() {
            self.create_instance_with_validation_layers(Self::generate_app_info(name), vulkano_win::required_extensions());
        } else {
            self.create_instance_without_validation_layers(Self::generate_app_info(name), vulkano_win::required_extensions());
        }
    }

    fn create_instance_with_validation_layers(&mut self, app_info: ApplicationInfo, required_extensions: InstanceExtensions) {
        self.instance = Some(Instance::new(Some(&app_info), Version::V1_1, &required_extensions, VALIDATION_LAYERS.iter().cloned())
                             .expect("failed to create Vulkan instance"));

        self.debug_callback = self.setup_debug_callback();
    }

    fn create_instance_without_validation_layers(&mut self, app_info: ApplicationInfo, required_extensions: InstanceExtensions) {
        self.instance = Some(Instance::new(Some(&app_info), Version::V1_1, &required_extensions, None)
                             .expect("failed to create Vulkan instance"));
    }

    fn generate_app_info(name: &String) -> ApplicationInfo {
        ApplicationInfo {
            application_name: Some(name.into()),
            application_version: Some(Version { major: 1, minor: 0, patch: 0 }),
            engine_name: Some("No Engine".into()),
            engine_version: Some(Version { major: 1, minor: 0, patch: 0 }),
        }
    }

    fn check_validation_layer_support() -> bool {
        let layers: Vec<_> = layers_list().unwrap().map(|l| l.name().to_owned()).collect();
        DEBUG && VALIDATION_LAYERS.iter().all(|layer_name| layers.contains(&layer_name.to_string()))
    }

    fn setup_debug_callback(&self) -> Option<DebugCallback> {
        let msg_severity = MessageSeverity { error: true, warning: true, information: false, verbose: true };
        let msg_types = MessageType { general: true, validation: true, performance: true };

        DebugCallback::new(self.instance.as_ref().unwrap(), msg_severity, msg_types, |msg| {
            println!("validation layer: {:?}", msg.description);
        }).ok()
    }
}
