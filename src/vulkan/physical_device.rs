extern crate vulkano;

use vulkano::instance::Instance;
use vulkano::device::physical::PhysicalDevice;
use vulkano::swapchain::Surface;


extern crate winit;

use winit::window::Window;


use std::sync::Arc;

use crate::vulkan::queue_family::find_queue_families;

pub struct VkPhysicalDevice {
    physical_device_index: Option<usize>,
}

impl VkPhysicalDevice {
    pub fn new() -> Self {
        Self {
            physical_device_index: None,
        }
    }

    pub fn init(&mut self, instance: &Arc<Instance>, surface: &Arc<Surface<Window>>) {
        self.physical_device_index = Some(PhysicalDevice::enumerate(&instance)
            .position(|device| Self::is_device_suitable(surface, &device))
            .expect("failed to find a suitable GPU!"));
    }

    fn is_device_suitable(surface: &Arc<Surface<Window>>, device: &PhysicalDevice) -> bool {
        let indices = find_queue_families(surface, device);
        indices.is_complete()
    }

    pub fn get_index(&self) -> &usize {
        self.physical_device_index.as_ref().unwrap()
    }
}
