extern crate vulkano;

use vulkano::swapchain::{
    Surface,
    Capabilities,
    ColorSpace,
    SupportedPresentModes,
    PresentMode,
    Swapchain,
    CompositeAlpha,
};

use vulkano::format::Format;
use vulkano::image::{ImageUsage, swapchain::SwapchainImage};
use vulkano::sync::SharingMode;

use vulkano::instance::Instance;
use vulkano::device::physical::PhysicalDevice;
use vulkano::device::Device;
use vulkano::device::Queue;


use winit::window::Window;


use std::sync::Arc;


use crate::vulkan::queue_family::find_queue_families;


pub struct VkSwapChain {
    swap_chain: Option<Arc<Swapchain<Window>>>,
    swap_chain_images: Option<Vec<Arc<SwapchainImage<Window>>>>,
}

impl VkSwapChain {
    pub fn new() -> Self {
        Self {
            swap_chain: None,
            swap_chain_images: None,
        }
    }

    pub fn init(
        &mut self,
        instance: &Arc<Instance>,
        surface: &Arc<Surface<Window>>,
        physical_device_index: &usize,
        device: &Arc<Device>,
        graphics_queue: &Arc<Queue>,
        present_queue: &Arc<Queue>,
        width: &i32,
        height: &i32,
        ) {
        let physical_device = PhysicalDevice::from_index(&instance, *physical_device_index).unwrap();
        let capabilities = surface.capabilities(physical_device)
            .expect("failed to get surface capabilities");

        let surface_format = Self::choose_swap_surface_format(&capabilities.supported_formats);
        let present_mode = Self::choose_swap_present_mode(capabilities.present_modes);
        let extent = Self::choose_swap_extent(&capabilities, width, height);

        let mut image_count = capabilities.min_image_count + 1;
        if capabilities.max_image_count.is_some() && image_count > capabilities.max_image_count.unwrap() {
            image_count = capabilities.max_image_count.unwrap();
        }

        let image_usage = ImageUsage {
            color_attachment: true,
            .. ImageUsage::none()
        };

        let indices = find_queue_families(&surface, &physical_device);

        let sharing: SharingMode = if indices.graphics_family != indices.present_family {
            vec![graphics_queue, present_queue].as_slice().into()
        } else {
            graphics_queue.into()
        };

        /*
           let (swap_chain, images) = Swapchain::start(
           device.clone(),
           surface.clone(),
           image_count,
           surface_format.0,
           extent,
           1,
           image_usage,
           sharing,
           capabilities.current_transform,
           CompositeAlpha::Opaque,
           present_mode,
           true,
           None,
           ).expect("failed to create swap chain!");
           */

        let (mut swapchain, images) = {
            let caps = surface.capabilities(physical_device).unwrap();
            let composite_alpha = caps.supported_composite_alpha.iter().next().unwrap();
            let format = caps.supported_formats[0].0;
            let dimensions: [u32; 2] = surface.window().inner_size().into();

            Swapchain::start(device.clone(), surface.clone())
                .num_images(caps.min_image_count)
                .format(format)
                .dimensions(dimensions)
                .usage(ImageUsage::color_attachment())
                .sharing_mode(&queue)
                .composite_alpha(composite_alpha)
                .build()
                .unwrap()
        };

        self.swap_chain = swap_chain;
        self.swap_chain_images = images;
    }

    fn choose_swap_surface_format(available_formats: &[(Format, ColorSpace)]) -> (Format, ColorSpace) {
        *available_formats.iter()
            .find(|(format, color_space)|
                  *format == Format::B8G8R8A8Unorm && *color_space == ColorSpace::SrgbNonLinear
                 )
            .unwrap_or_else(|| &available_formats[0])
    }

    fn choose_swap_present_mode(available_present_modes: SupportedPresentModes) -> PresentMode {
        if available_present_modes.mailbox {
            PresentMode::Mailbox
        } else if available_present_modes.immediate {
            PresentMode::Immediate
        } else {
            PresentMode::Fifo
        }
    }

    fn choose_swap_extent(capabilities: &Capabilities, width: &i32, height: &i32) -> [u32; 2] {
        if let Some(current_extent) = capabilities.current_extent {
            return current_extent
        } else {
            let mut actual_extent = [width, height];
            actual_extent[0] = capabilities.min_image_extent[0]
                .max(capabilities.max_image_extent[0].min(actual_extent[0]));
            actual_extent[1] = capabilities.min_image_extent[1]
                .max(capabilities.max_image_extent[1].min(actual_extent[1]));
            actual_extent
        }
    }

    pub fn get_swap_chain(&self) -> &Arc<Surface<Window>> {
        self.swap_chain.as_ref().unwrap()
    }
}
