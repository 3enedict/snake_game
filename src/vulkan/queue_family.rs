extern crate vulkano;
use vulkano::device::physical::PhysicalDevice;
use vulkano::swapchain::Surface;


extern crate winit;

use winit::window::Window;


use std::sync::Arc;

pub struct QueueFamilyIndices {
    pub graphics_family: i32,
    pub present_family: i32,
}

impl QueueFamilyIndices {
    fn new() -> Self {
        Self { graphics_family: -1, present_family: -1 }
    }

    pub fn is_complete(&self) -> bool {
        self.graphics_family >= 0 && self.present_family >= 0
    }
}

pub fn find_queue_families(surface: &Arc<Surface<Window>>, device: &PhysicalDevice) -> QueueFamilyIndices {
    let mut indices = QueueFamilyIndices::new();
    for (i, queue_family) in device.queue_families().enumerate() {
        if queue_family.supports_graphics() {
            indices.graphics_family = i as i32;
        } if surface.is_supported(queue_family).unwrap() {
            indices.present_family = i as i32;
        }

        if indices.is_complete() {
            break;
        }
    }

    indices
}
