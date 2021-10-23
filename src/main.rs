mod vulkan;
use vulkan::Vulkan;

fn main() {
    let mut _vulkan = Vulkan::init()
        .setup();
}
