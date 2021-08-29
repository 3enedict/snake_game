extern crate vulkano;
extern crate winit;

mod instance;
use instance::VkInstance;

use winit::{
    event_loop::{EventLoop},
    window::WindowBuilder, 
};


#[cfg(all(debug_assertions))]
const DEBUG: bool = true;
#[cfg(not(debug_assertions))]
const DEBUG: bool = false;

const VALIDATION_LAYERS: &[&str] =  &[
    "VK_LAYER_KHRONOS_validation"
];


pub struct Vulkan {
    width: i32,
    height: i32,
    name: String,

    event_loop: EventLoop<()>,
    instance: VkInstance,
}

impl Vulkan {
    pub fn init() -> Self {
        Self {
            width: 1920,
            height: 1080,
            name: String::from("Vulkan"),

            event_loop: EventLoop::new(),
            instance: VkInstance::new(),
        }
    }

    pub fn create_instance(&mut self) {
        self.init_window();

        self.instance.init(&self.name);
    }

    fn init_window(&self) {
        let _window = WindowBuilder::new()
            .with_title(&self.name)
            .with_inner_size(winit::dpi::LogicalSize::new(self.width, self.height))
            .build(&self.event_loop);
    }
}

#[cfg(test)]
mod tests {
    use crate::Vulkan;

    #[test]
    fn verify_width() {
        let vulkan = Vulkan::init();

        assert_eq!(vulkan.width, 1920);
    }

    #[test]
    fn verify_height() {
        let vulkan = Vulkan::init();

        assert_eq!(vulkan.height, 1080);
    }

    #[test]
    fn verify_name() {
        let vulkan = Vulkan::init();

        assert_eq!(vulkan.name, "Vulkan");
    }

    #[test]
    fn verify_instance_creation() {
        let mut vulkan = Vulkan::init();

        vulkan.create_instance();
    }
}
