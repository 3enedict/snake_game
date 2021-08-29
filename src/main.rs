mod vulkan;
use vulkan::Vulkan;

fn main() {
    let mut vulkan = Vulkan::init();

    vulkan.create_instance();
}
