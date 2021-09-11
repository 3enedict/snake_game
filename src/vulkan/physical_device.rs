extern crate vulkano;

use crate::vulkan::queue_family::find_queue_families;

use vulkano::instance::Instance;
use vulkano::device::physical::PhysicalDevice;

use std::sync::Arc;

pub struct VkPhysicalDevice {
    physical_device_index: Option<usize>,
}

impl VkPhysicalDevice {
    pub fn new() -> Self {
        Self {
            physical_device_index: None,
        }
    }

    pub fn init(&mut self, instance: &Arc<Instance>) {
        self.physical_device_index = Some(PhysicalDevice::enumerate(&instance)
            .position(|device| Self::is_device_suitable(&device))
            .expect("failed to find a suitable GPU!"));
    }

    fn is_device_suitable(device: &PhysicalDevice) -> bool {
        let indices = find_queue_families(device);
        indices.is_complete()
    }

    pub fn get_index(&self) -> &usize {
        self.physical_device_index.as_ref().unwrap()
    }
}
