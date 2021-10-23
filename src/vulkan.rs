extern crate vulkano;
extern crate winit;

mod instance;
use instance::VkInstance;

mod surface;
use surface::VkSurface;

mod physical_device;
use physical_device::VkPhysicalDevice;

mod logical_device;
use logical_device::VkLogicalDevice;

mod queue_family;

mod swap_chain;
use swap_chain::VkSwapChain;


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

    instance: VkInstance,
    surface: VkSurface,
    physical_device: VkPhysicalDevice,
    logical_device: VkLogicalDevice,
    swap_chain: VkSwapChain,
}

impl Vulkan {
    pub fn init() -> Self {
        Self {
            width: 1920,
            height: 1080,
            name: String::from("Vulkan"),

            instance: VkInstance::new(),
            surface: VkSurface::new(),
            physical_device: VkPhysicalDevice::new(),
            logical_device: VkLogicalDevice::new(),
            swap_chain: VkSwapChain::new(),
        }
    }

    pub fn setup(&mut self) {
        self.instance.init(
            &self.name,
            );

        self.surface.init(
            &self.width, 
            &self.height, 
            &self.name, 
            self.instance.get_instance(),
            );

        self.physical_device.init(
            self.instance.get_instance(), 
            self.surface.get_surface(),
            );

        self.logical_device.init(
            self.instance.get_instance(), 
            self.surface.get_surface(), 
            self.physical_device.get_index(),
            );

        self.swap_chain.init(
            self.instance.get_instance(),
            self.surface.get_surface(),
            self.physical_device.get_index(),
            self.logical_device.get_device(),
            self.logical_device.get_graphics_queue(),
            self.logical_device.get_present_queue(),
            &self.width,
            &self.height,
            );
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
