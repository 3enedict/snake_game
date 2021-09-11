extern crate vulkano;
use vulkano::device::physical::PhysicalDevice;

pub struct QueueFamilyIndices {
    pub graphics_family: i32,
}

impl QueueFamilyIndices {
    fn new() -> Self {
        Self { graphics_family: -1 }
    }

    pub fn is_complete(&self) -> bool {
        self.graphics_family >= 0
    }
}

pub fn find_queue_families(device: &PhysicalDevice) -> QueueFamilyIndices {
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
