mod vulkan;
use vulkan::Vulkan;

fn main() {
    let mut vulkan = Vulkan::init();

    vulkan.setup(&mut move | vulkan | {
        println!("Hello world");
    });

    vulkan.run();
}
