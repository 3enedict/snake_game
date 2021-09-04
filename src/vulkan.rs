extern crate vulkano;
extern crate winit;

mod instance;
use instance::VkInstance;

mod physical_device;
use physical_device::VkPhysicalDevice;

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
    physical_device: VkPhysicalDevice,
}

impl Vulkan {
    pub fn init() -> Self {
        Self {
            width: 1920,
            height: 1080,
            name: String::from("Vulkan"),

            event_loop: EventLoop::new(),
            instance: VkInstance::new(),
            physical_device: VkPhysicalDevice::new(),
        }
    }

    pub fn setup(&mut self) {
        self.init_window();

        self.instance.init(&self.name);
        self.physical_device.init(self.instance.get_instance());
    }

    fn init_window(&self) {
        let _window = WindowBuilder::new()
            .with_title(&self.name)
            .with_inner_size(winit::dpi::LogicalSize::new(self.width, self.height))
            .build(&self.event_loop);
    }





    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn with_width(mut self, width: i32) -> Self {
        self.width = width;
        self
    }

    pub fn with_height(mut self, height: i32) -> Self {
        self.height = height;
        self
    }


    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use crate::Vulkan;

    #[test]
    fn verify_default_width() {
        let vulkan = Vulkan::init();

        assert_eq!(vulkan.get_width(), 1920);
    }

    #[test]
    fn verify_default_height() {
        let vulkan = Vulkan::init();

        assert_eq!(vulkan.get_height(), 1080);
    }

    #[test]
    fn verify_default_name() {
        let vulkan = Vulkan::init();

        assert_eq!(vulkan.get_name(), "Vulkan");
    }

    #[test]
    fn verify_width() {
        let vulkan = Vulkan::init()
            .with_width(1280);

        assert_eq!(vulkan.get_width(), 1280);
    }

    #[test]
    fn verify_height() {
        let vulkan = Vulkan::init()
            .with_height(720);

        assert_eq!(vulkan.get_height(), 720);
    }

    #[test]
    fn verify_name() {
        let vulkan = Vulkan::init()
            .with_name(String::from("Other name"));

        assert_eq!(vulkan.get_name(), "Other name");
    }

    #[test]
    fn verify_instance_creation() {
        let _vulkan = Vulkan::init()
            .setup();
    }
}
