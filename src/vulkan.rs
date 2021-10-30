use std::sync::Arc;
use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer, TypedBufferAccess};
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, SubpassContents};
use vulkano::device::physical::{PhysicalDevice, PhysicalDeviceType, QueueFamily};
use vulkano::device::{Device, DeviceExtensions, Features, Queue};
use vulkano::image::view::ImageView;
use vulkano::image::{ImageUsage, SwapchainImage};
use vulkano::instance::Instance;
use vulkano::instance::InstanceExtensions;
use vulkano::pipeline::viewport::Viewport;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::render_pass::{Framebuffer, FramebufferAbstract, RenderPass, Subpass};
use vulkano::swapchain;
use vulkano::swapchain::{AcquireError, Swapchain, SwapchainCreationError, Surface};
use vulkano::sync;
use vulkano::sync::{FlushError, GpuFuture};
use vulkano::Version;
use vulkano_win::VkSurfaceBuild;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

mod vs;
use crate::vulkan::vs::Shader as VertexShader;

mod fs;
use crate::vulkan::fs::Shader as FragmentShader;


#[derive(Default, Debug, Clone)]
struct Vertex {
    position: [f32; 2],
}
vulkano::impl_vertex!(Vertex, position);



fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn window_size_dependent_setup(
    images: &[Arc<SwapchainImage<Window>>],
    render_pass: Arc<RenderPass>,
    viewport: &mut Viewport,
    ) -> Vec<Arc<dyn FramebufferAbstract>> {
    let dimensions = images[0].dimensions();
    viewport.dimensions = [dimensions[0] as f32, dimensions[1] as f32];

    images
        .iter()
        .map(|image| {
            let view = ImageView::new(image.clone()).unwrap();
            Arc::new(
                Framebuffer::start(render_pass.clone())
                .add(view)
                .unwrap()
                .build()
                .unwrap(),
                ) as Arc<dyn FramebufferAbstract>
        })
    .collect::<Vec<_>>()
}

pub struct Vulkan {
    required_extensions:    Option<InstanceExtensions>,
    instance:               Option<Arc<Instance>>,

    event_loop:             Option<EventLoop<()>>,
    surface:                Option<Arc<Surface<Window>>>,

    device_extensions:      Option<DeviceExtensions>,
    physical_device_index:  Option<usize>,
    queue_family_id:        Option<u32>,

    logical_device:         Option<Arc<Device>>,
    queue:                  Option<Arc<Queue>>,

    swapchain:              Option<Arc<Swapchain<Window>>>,
    images:                 Option<Vec<Arc<SwapchainImage<Window>>>>,

    vertex_buffer:          Option<Arc<CpuAccessibleBuffer<[Vertex]>>>,
    vs:                     Option<VertexShader>,
    fs:                     Option<FragmentShader>,

    render_pass:            Option<Arc<RenderPass>>,
}

impl Vulkan {
    pub fn init() -> Self {
        Self {
            required_extensions:    None,
            instance:               None,

            event_loop:             None,
            surface:                None,

            device_extensions:      None,
            physical_device_index:  None,
            queue_family_id:        None,

            logical_device:         None,
            queue:                  None,

            swapchain:              None,
            images:                 None,

            vertex_buffer:          None,
            vs:                     None,
            fs:                     None,

            render_pass:            None,
        }
    }

    pub fn setup(mut self) {
        self.get_required_extensions();
        self.create_instance();

        self.create_event_loop();
        self.create_window();

        self.choose_device_extensions();
        self.create_physical_device();
        println!("Using device: {} (type: {:?})", self.get_physical_device().properties().device_name, self.get_physical_device().properties().device_type);

        self.create_logical_device();
        self.create_swapchain();


        self.create_vertex_buffer();
        self.create_shaders();

        self.create_render_pass();

        // Before we draw we have to create what is called a pipeline. This is similar to an OpenGL
        // program, but much more specific.
        let pipeline = Arc::new(
            GraphicsPipeline::start()
            // We need to indicate the layout of the vertices.
            // The type `SingleBufferDefinition` actually contains a template parameter corresponding
            // to the type of each vertex. But in this code it is automatically inferred.
            .vertex_input_single_buffer::<Vertex>()
            // A Vulkan shader can in theory contain multiple entry points, so we have to specify
            // which one. The `main` word of `main_entry_point` actually corresponds to the name of
            // the entry point.
            .vertex_shader(self.vs.as_ref().unwrap().main_entry_point(), ())
            // The content of the vertex buffer describes a list of triangles.
            .triangle_list()
            // Use a resizable viewport set to draw over the entire window
            .viewports_dynamic_scissors_irrelevant(1)
            // See `vertex_shader`.
            .fragment_shader(self.fs.as_ref().unwrap().main_entry_point(), ())
            // We have to indicate which subpass of which render pass this pipeline is going to be used
            // in. The pipeline will only be usable from this particular subpass.
            .render_pass(Subpass::from(self.render_pass.as_ref().unwrap().clone(), 0).unwrap())
            // Now that our builder is filled, we call `build()` to obtain an actual pipeline.
            .build(self.logical_device.as_ref().unwrap().clone())
            .unwrap(),
            );

        // Dynamic viewports allow us to recreate just the viewport when the window is resized
        // Otherwise we would have to recreate the whole pipeline.
        let mut viewport = Viewport {
            origin: [0.0, 0.0],
            dimensions: [0.0, 0.0],
            depth_range: 0.0..1.0,
        };

        // The render pass we created above only describes the layout of our framebuffers. Before we
        // can draw we also need to create the actual framebuffers.
        //
        // Since we need to draw to multiple images, we are going to create a different framebuffer for
        // each image.
        let mut framebuffers = window_size_dependent_setup(self.images.as_ref().unwrap(), self.render_pass.as_ref().unwrap().clone(), &mut viewport);

        // Initialization is finally finished!

        // In some situations, the swapchain will become invalid by itself. This includes for example
        // when the window is resized (as the images of the swapchain will no longer match the
        // window's) or, on Android, when the application went to the background and goes back to the
        // foreground.
        //
        // In this situation, acquiring a swapchain image or presenting it will return an error.
        // Rendering to an image of that swapchain will not produce any error, but may or may not work.
        // To continue rendering, we need to recreate the swapchain by creating a new swapchain.
        // Here, we remember that we need to do this for the next loop iteration.
        let mut recreate_swapchain = false;

        // In the loop below we are going to submit commands to the GPU. Submitting a command produces
        // an object that implements the `GpuFuture` trait, which holds the resources for as long as
        // they are in use by the GPU.
        //
        // Destroying the `GpuFuture` blocks until the GPU is finished executing it. In order to avoid
        // that, we store the submission of the previous frame here.
        let mut previous_frame_end = Some(sync::now(self.logical_device.as_ref().unwrap().clone()).boxed());

        self.event_loop.take().unwrap().run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    *control_flow = ControlFlow::Exit;
                }
                Event::WindowEvent {
                    event: WindowEvent::Resized(_),
                    ..
                } => {
                    recreate_swapchain = true;
                }
                Event::RedrawEventsCleared => {
                    // It is important to call this function from time to time, otherwise resources will keep
                    // accumulating and you will eventually reach an out of memory error.
                    // Calling this function polls various fences in order to determine what the GPU has
                    // already processed, and frees the resources that are no longer needed.
                    previous_frame_end.as_mut().unwrap().cleanup_finished();

                    // Whenever the window resizes we need to recreate everything dependent on the window size.
                    // In this example that includes the swapchain, the framebuffers and the dynamic state viewport.
                    if recreate_swapchain {
                        // Get the new dimensions of the window.
                        let dimensions: [u32; 2] = self.surface.as_ref().unwrap().window().inner_size().into();
                        let (new_swapchain, new_images) =
                            match self.swapchain.as_ref().unwrap().recreate().dimensions(dimensions).build() {
                                Ok(r) => r,
                                // This error tends to happen when the user is manually resizing the window.
                                // Simply restarting the loop is the easiest way to fix this issue.
                                Err(SwapchainCreationError::UnsupportedDimensions) => return,
                                Err(e) => panic!("Failed to recreate swapchain: {:?}", e),
                            };

                        self.swapchain = Some(new_swapchain);
                        // Because framebuffers contains an Arc on the old swapchain, we need to
                        // recreate framebuffers as well.
                        framebuffers = window_size_dependent_setup(
                            &new_images,
                            self.render_pass.as_ref().unwrap().clone(),
                            &mut viewport,
                            );
                        recreate_swapchain = false;
                    }

                    // Before we can draw on the output, we have to *acquire* an image from the swapchain. If
                    // no image is available (which happens if you submit draw commands too quickly), then the
                    // function will block.
                    // This operation returns the index of the image that we are allowed to draw upon.
                    //
                    // This function can block if no image is available. The parameter is an optional timeout
                    // after which the function call will return an error.
                    let (image_num, suboptimal, acquire_future) =
                        match swapchain::acquire_next_image(self.swapchain.as_ref().unwrap().clone(), None) {
                            Ok(r) => r,
                            Err(AcquireError::OutOfDate) => {
                                recreate_swapchain = true;
                                return;
                            }
                            Err(e) => panic!("Failed to acquire next image: {:?}", e),
                        };

                    // acquire_next_image can be successful, but suboptimal. This means that the swapchain image
                    // will still work, but it may not display correctly. With some drivers this can be when
                    // the window resizes, but it may not cause the swapchain to become out of date.
                    if suboptimal {
                        recreate_swapchain = true;
                    }

                    // Specify the color to clear the framebuffer with i.e. blue
                    let clear_values = vec![[0.1, 0.1, 0.1, 1.0].into()];

                    // In order to draw, we have to build a *command buffer*. The command buffer object holds
                    // the list of commands that are going to be executed.
                    //
                    // Building a command buffer is an expensive operation (usually a few hundred
                    // microseconds), but it is known to be a hot path in the driver and is expected to be
                    // optimized.
                    //
                    // Note that we have to pass a queue family when we create the command buffer. The command
                    // buffer will only be executable on that given queue family.
                    let mut builder = AutoCommandBufferBuilder::primary(
                        self.logical_device.as_ref().unwrap().clone(),
                        self.queue.as_ref().unwrap().family(),
                        CommandBufferUsage::OneTimeSubmit,
                        )
                        .unwrap();

                    builder
                        // Before we can draw, we have to *enter a render pass*. There are two methods to do
                        // this: `draw_inline` and `draw_secondary`. The latter is a bit more advanced and is
                        // not covered here.
                        //
                        // The third parameter builds the list of values to clear the attachments with. The API
                        // is similar to the list of attachments when building the framebuffers, except that
                        // only the attachments that use `load: Clear` appear in the list.
                        .begin_render_pass(
                            framebuffers[image_num].clone(),
                            SubpassContents::Inline,
                            clear_values,
                            )
                        .unwrap()
                        // We are now inside the first subpass of the render pass. We add a draw command.
                        //
                        // The last two parameters contain the list of resources to pass to the shaders.
                        // Since we used an `EmptyPipeline` object, the objects have to be `()`.
                        .set_viewport(0, [viewport.clone()])
                        .bind_pipeline_graphics(pipeline.clone())
                        .bind_vertex_buffers(0, self.vertex_buffer.as_ref().unwrap().clone())
                        .draw(self.vertex_buffer.as_ref().unwrap().len() as u32, 1, 0, 0)
                        .unwrap()
                        // We leave the render pass by calling `draw_end`. Note that if we had multiple
                        // subpasses we could have called `next_inline` (or `next_secondary`) to jump to the
                        // next subpass.
                        .end_render_pass()
                        .unwrap();

                    // Finish building the command buffer by calling `build`.
                    let command_buffer = builder.build().unwrap();

                    let future = previous_frame_end
                        .take()
                        .unwrap()
                        .join(acquire_future)
                        .then_execute(self.queue.as_ref().unwrap().clone(), command_buffer)
                        .unwrap()
                        // The color output is now expected to contain our triangle. But in order to show it on
                        // the screen, we have to *present* the image by calling `present`.
                        //
                        // This function does not actually present the image immediately. Instead it submits a
                        // present command at the end of the queue. This means that it will only be presented once
                        // the GPU has finished executing the command buffer that draws the triangle.
                        .then_swapchain_present(self.queue.as_ref().unwrap().clone(), self.swapchain.as_ref().unwrap().clone(), image_num)
                        .then_signal_fence_and_flush();

                    match future {
                        Ok(future) => {
                            previous_frame_end = Some(future.boxed());
                        }
                        Err(FlushError::OutOfDate) => {
                            recreate_swapchain = true;
                            previous_frame_end = Some(sync::now(self.logical_device.as_ref().unwrap().clone()).boxed());
                        }
                        Err(e) => {
                            println!("Failed to flush future: {:?}", e);
                            previous_frame_end = Some(sync::now(self.logical_device.as_ref().unwrap().clone()).boxed());
                        }
                    }
                }
                _ => (),
            }
        });

    }

    fn get_required_extensions(&mut self) {
        self.required_extensions = Some(vulkano_win::required_extensions());
    }

    fn create_instance(&mut self) {
        self.instance = Some(Instance::new(None, Version::V1_1, self.required_extensions.as_ref().unwrap(), None).unwrap());
    }

    fn create_event_loop(&mut self) {
        self.event_loop = Some(EventLoop::new());
    }

    fn create_window(&mut self) {
        self.surface = Some(WindowBuilder::new()
                            .build_vk_surface(self.event_loop.as_ref().unwrap(), self.instance.as_ref().unwrap().clone())
                            .unwrap());
    }

    fn choose_device_extensions(&mut self) {
        self.device_extensions = Some(DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::none()
        });
    }

    fn create_physical_device(&mut self) {
        let (physical_device, queue_family) = PhysicalDevice::enumerate(self.instance.as_ref().unwrap())
            .filter(|&p| {
                p.supported_extensions().is_superset_of(self.device_extensions.as_ref().unwrap())
            }).filter_map(|p| {
                p.queue_families()
                    .find(|&q| {
                        q.supports_graphics() && self.surface.as_ref().unwrap().is_supported(q).unwrap_or(false)
                    })
                .map(|q| (p, q))
            }).min_by_key(|(p, _)| {
                match p.properties().device_type {
                    PhysicalDeviceType::DiscreteGpu => 0,
                    PhysicalDeviceType::IntegratedGpu => 1,
                    PhysicalDeviceType::VirtualGpu => 2,
                    PhysicalDeviceType::Cpu => 3,
                    PhysicalDeviceType::Other => 4,
                }
            }).unwrap();

        self.physical_device_index = Some(physical_device.index());
        self.queue_family_id = Some(queue_family.id());
    }

    fn get_physical_device(&self) -> PhysicalDevice {
        PhysicalDevice::from_index(self.instance.as_ref().unwrap(), self.physical_device_index.unwrap()).unwrap()
    }

    fn get_queue_family(&self) -> QueueFamily {
        self.get_physical_device().queue_family_by_id(self.queue_family_id.unwrap()).unwrap()
    }

    fn create_logical_device(&mut self) {
        let (device, mut queues) = Device::new(
            self.get_physical_device(),
            &Features::none(),
            &self.get_physical_device()
            .required_extensions()
            .union(self.device_extensions.as_ref().unwrap()),
            [(self.get_queue_family(), 0.5)].iter().cloned(),
            )
            .unwrap();

        self.logical_device = Some(device);
        self.queue = queues.next();
    }

    fn create_swapchain(&mut self) {
        let (swapchain, images) = {
            let caps = self.surface.as_ref().unwrap().capabilities(self.get_physical_device()).unwrap();
            let composite_alpha = caps.supported_composite_alpha.iter().next().unwrap();
            let format = caps.supported_formats[0].0;
            let dimensions: [u32; 2] = self.surface.as_ref().unwrap().window().inner_size().into();

            Swapchain::start(self.logical_device.as_ref().unwrap().clone(), self.surface.as_ref().unwrap().clone())
                .num_images(caps.min_image_count)
                .format(format)
                .dimensions(dimensions)
                .usage(ImageUsage::color_attachment())
                .sharing_mode(self.queue.as_ref().unwrap())
                .composite_alpha(composite_alpha)
                .build()
                .unwrap()
        };

        self.swapchain = Some(swapchain);
        self.images = Some(images);
    }

    fn create_vertex_buffer(&mut self) {
        self.vertex_buffer = Some(CpuAccessibleBuffer::from_iter(
                self.logical_device.as_ref().unwrap().clone(),
                BufferUsage::all(),
                false,
                [
                Vertex {
                    position: [0.5, 0.5],
                },
                Vertex {
                    position: [-0.5, 0.5],
                },
                Vertex {
                    position: [0.0, -0.5],
                },
                ]
                .iter()
                .cloned(),
                )
            .unwrap());
    }

    fn create_shaders(&mut self) {
        self.vs = Some(vs::Shader::load(self.logical_device.as_ref().unwrap().clone()).unwrap());
        self.fs = Some(fs::Shader::load(self.logical_device.as_ref().unwrap().clone()).unwrap());
    }

    fn create_render_pass(&mut self) {
        self.render_pass = Some(Arc::new(
            vulkano::single_pass_renderpass!(
                self.logical_device.as_ref().unwrap().clone(),
                attachments: {
                    color: {
                        load: Clear,
                        store: Store,
                        format: self.swapchain.as_ref().unwrap().format(),
                        samples: 1,
                    }
                },
                pass: {
                    color: [color],
                    depth_stencil: {}
                }
        )
            .unwrap(),
            ));
    }
}
