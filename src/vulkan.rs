extern crate vulkano;
extern crate winit;

mod instance;
use instance::VulkanInstance;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder, 
    dpi::LogicalSize,
};

pub struct Vulkan {
    width: i32,
    height: i32,
    name: String,

    event_loop: EventLoop<()>,
    instance: Option<VulkanInstance>,
}

impl Vulkan {
    pub fn init() -> Self {
        Self {
          width: 1920,
          height: 1080,
          name: String::from("Vulkan"),

          event_loop: EventLoop::new(),
          instance: None,
        }
    }

    fn init_window(&self) {
        let _window = WindowBuilder::new()
            .with_title(&self.name)
            .with_inner_size(winit::dpi::LogicalSize::new(self.width, self.height))
            .build(&self.event_loop);
    }

    fn create_instance(&mut self) {
        self.instance = Some(VulkanInstance::new(&self.name));
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
    fn verify_window_creation() {
        let vulkan = Vulkan::init();

        vulkan.init_window();
    }
}
