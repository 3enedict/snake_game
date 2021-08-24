mod vulkan;
use vulkan::Vulkan;

fn main() {
    let vulkan = Vulkan::init(1920, 1080, "Vulkan");
}
