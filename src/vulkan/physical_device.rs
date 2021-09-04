extern crate vulkano;

use vulkano::instance::Instance;
use vulkano::device::physical::PhysicalDevice;

use std::sync::Arc;

struct QueueFamilyIndices {
    graphics_family: i32,
}
impl QueueFamilyIndices {
    fn new() -> Self {
        Self { graphics_family: -1 }
    }

    fn is_complete(&self) -> bool {
        self.graphics_family >= 0
    }
}

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
        let indices = Self::find_queue_families(device);
        indices.is_complete()
    }

    fn find_queue_families(device: &PhysicalDevice) -> QueueFamilyIndices {
        let mut indices = QueueFamilyIndices::new();
        for (i, queue_family) in device.queue_families().enumerate() {
            if queue_family.supports_graphics() {
                indices.graphics_family = i as i32;
            }

            if indices.is_complete() {
                break;
            }
        }

        indices
    }
}
