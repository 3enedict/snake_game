extern crate vulkano;

use vulkano::instance::Instance;
use vulkano::swapchain::Surface;
use vulkano::device::{
    Device,
    Queue,
    physical::PhysicalDevice,
    Features,
    DeviceExtensions,
};


extern crate winit;

use winit::window::Window;


use std::sync::Arc;
use std::iter::FromIterator;
use std::collections::HashSet;

use crate::vulkan::queue_family::find_queue_families;


pub struct VkLogicalDevice {
    device: Option<Arc<Device>>,

    graphics_queue: Option<Arc<Queue>>,
    present_queue: Option<Arc<Queue>>,
}

impl VkLogicalDevice {
    pub fn new() -> Self {
        Self {
            device: None,

            graphics_queue: None,
            present_queue: None,
        }
    }

    pub fn init(&mut self, instance: &Arc<Instance>, surface: &Arc<Surface<Window>>, physical_device_index: &usize) {
        let physical_device = PhysicalDevice::from_index(&instance, *physical_device_index).unwrap();
        let indices = find_queue_families(&surface, &physical_device);

        let families = [indices.graphics_family, indices.present_family];
        let unique_queue_families: HashSet<&i32> = HashSet::from_iter(families.iter());

        let queue_priority = 1.0;
        let queue_families = unique_queue_families.iter().map(|i| {
            (physical_device.queue_families().nth(**i as usize).unwrap(), queue_priority)
        });

        let (device, mut queues) = Device::new(physical_device, &Features::none(),
        &DeviceExtensions::none(), queue_families)
            .expect("failed to create logical device!");

        self.device = Some(device);
        self.graphics_queue = Some(queues.next().unwrap());
        self.present_queue = Some(queues.next().unwrap_or_else(|| self.graphics_queue.as_ref().unwrap().clone()));
    }

    pub fn get_device(&self) -> &Arc<Device> {
        self.device.as_ref().unwrap()
    }

    pub fn get_graphics_queue(&self) -> &Arc<Queue> {
        self.graphics_queue.as_ref().unwrap()
    }

    pub fn get_present_queue(&self) -> &Arc<Queue> {
        self.present_queue.as_ref().unwrap()
    }
}
