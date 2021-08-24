extern crate vulkano;

extern crate winit;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder, 
    dpi::LogicalSize,
};

pub struct Vulkan {
    width: i32,
    height: i32,
    event_loop: EventLoop<()>,
}

impl Vulkan {
    pub fn init(width: i32, height: i32, name: &str) -> Self {
        let event_loop = Self::init_window(name, width, height);

        Self {
            width,
            height,
            event_loop,
        }
    }

    fn init_window(name: &str, width: i32, height: i32) -> EventLoop<()> {
        let event_loop = EventLoop::new();
        let _window = WindowBuilder::new()
            .with_title(name)
            .with_inner_size(winit::dpi::LogicalSize::new(width, height))
            .build(&event_loop);

        event_loop
    }


    pub fn get_width(&self) -> i32 {
        return self.width;
    }

    pub fn get_height(&self) -> i32 {
        return self.height;
    }
}

#[cfg(test)]
mod tests {
    use crate::Vulkan;

    #[test]
    fn verify_dimensions() {
        let vulkan = Vulkan::init(1920, 1080, "Vulkan");

        assert_eq!(vulkan.get_width(), 1920);
        assert_eq!(vulkan.get_height(), 1080);
    }
}
