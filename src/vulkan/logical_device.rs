extern crate vulkano;

use vulkano::instance::Instance;
use vulkano::device::{
    Device,
    Queue,
    physical::PhysicalDevice,
    Features,
    DeviceExtensions,
};

use std::sync::Arc;

use crate::vulkan::queue_family::find_queue_families;


pub struct VkLogicalDevice {
    device: Option<Arc<Device>>,
    graphics_queue: Option<Arc<Queue>>,
}

impl VkLogicalDevice {
    pub fn new() -> Self {
        Self {
            device: None,
            graphics_queue: None,
        }
    }

    pub fn init(&mut self, instance: &Arc<Instance>, physical_device_index: &usize) {
        let physical_device = PhysicalDevice::from_index(&instance, *physical_device_index).unwrap();
        let indices = find_queue_families(&physical_device);

        let queue_family = physical_device.queue_families()
            .nth(indices.graphics_family as usize).unwrap();

        let queue_priority = 1.0;

        let (device, mut queues) = Device::new(physical_device, &Features::none(), &DeviceExtensions::none(),
                                               [(queue_family, queue_priority)].iter().cloned())
            .expect("failed to create logical device!");

        self.graphics_queue = Some(queues.next().unwrap());
        self.device = Some(device);
    }
}
