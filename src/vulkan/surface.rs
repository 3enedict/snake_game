extern crate vulkano;

use vulkano::swapchain::Surface;
use vulkano::instance::Instance;


extern crate vulkano_win;

use vulkano_win::VkSurfaceBuild;


extern crate winit;
use winit::{
    event_loop::{EventLoop},
    window::{WindowBuilder, Window},
    dpi::LogicalSize,
};

use std::sync::Arc;



pub struct VkSurface {
    event_loop: Option<EventLoop<()>>,
    surface: Option<Arc<Surface<Window>>>,
}

impl VkSurface {
    pub fn new() -> Self {
        Self {
            event_loop: None,
            surface: None,
        }
    }

    pub fn init(&mut self, width: &i32, height: &i32, name: &String, instance: &Arc<Instance>) {
        self.event_loop = Some(EventLoop::new());
        self.surface = Some(WindowBuilder::new()
            .with_title(name)
            .with_inner_size(LogicalSize::new(f64::from(*width), f64::from(*height)))
            .build_vk_surface(self.event_loop.as_ref().unwrap(), instance.clone())
            .expect("failed to create window surface!"));
    }

    pub fn get_surface(&self) -> &Arc<Surface<Window>> {
        self.surface.as_ref().unwrap()
    }
}
