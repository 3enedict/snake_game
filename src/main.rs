mod vulkan;
use vulkan::Vulkan;

fn main() {
    let _vulkan = Vulkan::init()
        .setup();
}
