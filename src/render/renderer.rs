use render::pipeline_manager;
use render::pipeline_infos;
use core::asset_manager;
use core::camera::Camera;

use na;

use vulkano;
use vulkano::framebuffer::RenderPass;
use vulkano::framebuffer::FramebufferAbstract;
use vulkano::framebuffer::RenderPassAbstract;
use vulkano::sync::GpuFuture;
use vulkano_win;
use vulkano_win::VkSurfaceBuild;
use winit;

use std::sync::{Arc,Mutex};
use std::time::{Duration,Instant};
use time;

///The main renderer
pub struct Renderer  {
    ///Holds the renderers pipeline_manager
    pipeline_manager: Arc<Mutex<pipeline_manager::PipelineManager>>,

    //Vulkano data
    extensions: vulkano::instance::InstanceExtensions,
    instance: Arc<vulkano::instance::Instance>,
    //physical: Arc<vulkano::instance::PhysicalDevice>,
    events_loop: winit::EventsLoop,
    window: vulkano_win::Window,
    device: Arc<vulkano::device::Device>,
    queues: vulkano::device::QueuesIter,
    queue: Arc<vulkano::device::Queue>,
    swapchain: Arc<vulkano::swapchain::Swapchain>,
    images: Vec<Arc<vulkano::image::SwapchainImage>>,
    renderpass: Arc<RenderPassAbstract + Send + Sync>,
    depth_buffer: Arc<vulkano::image::AttachmentImage<vulkano::format::D16Unorm>>,
    framebuffers: Vec<Arc<FramebufferAbstract + Send + Sync>>,

    previous_frame: Option<Box<GpuFuture>>,




}

impl Renderer {
    ///Creates a new renderer with all subsystems
    pub fn new() -> Self{
        //Init Vulkan

        //Check for needed extensions
        let extensions = vulkano_win::required_extensions();
        //Create a vulkan instance from these extensions
        let instance = vulkano::instance::Instance::new(None, &extensions, None).expect("failed to create instance");
        //Get us a graphics card
        let physical = vulkano::instance::PhysicalDevice::enumerate(&instance)
                                .next().expect("no device available");
        println!("Using device: {} (type: {:?})", physical.name(), physical.ty());
        //Create an events loop
        let mut events_loop = winit::EventsLoop::new();
        //and create a window for it TODO bring this in the systen:: module
        let window = winit::WindowBuilder::new()
        .with_dimensions(800, 600)
        .with_title("Thingy!")
        .with_decorations(true)
        .build_vk_surface(&events_loop, instance.clone()).expect("failed to !");

        //Create a queue
        let queue = physical.queue_families().find(|&q| q.supports_graphics() &&
                                                       window.surface().is_supported(q).unwrap_or(false))
                                                    .expect("couldn't find a graphical queue family");
        //select needed device extensions
        let device_ext = vulkano::device::DeviceExtensions {
            khr_swapchain: true,
            .. vulkano::device::DeviceExtensions::none()
        };
        //Create a artificial device and its queue
        let (device, mut queues) = vulkano::device::Device::new(physical, physical.supported_features(),
                                                                &device_ext, [(queue, 0.5)].iter().cloned())
                                   .expect("failed to create device");
        let queue = queues.next().expect("failed to !");

        //Get the swapchain and its images
        let (swapchain, images) = {
            let caps = window.surface().capabilities(physical).expect("failed to get surface capabilities");

            let dimensions = caps.current_extent.unwrap_or([800, 600]);
            let usage = caps.supported_usage_flags;
            let format = caps.supported_formats[0].0;

            vulkano::swapchain::Swapchain::new(device.clone(), window.surface().clone(), caps.min_image_count, format, dimensions, 1,
                                               usage, &queue, vulkano::swapchain::SurfaceTransform::Identity,
                                               vulkano::swapchain::CompositeAlpha::Opaque,
                                               vulkano::swapchain::PresentMode::Fifo, true, None).expect("failed to create swapchain")
        };

        //Create a depth buffer
        let depth_buffer = vulkano::image::attachment::AttachmentImage::transient(device.clone(), images[0].dimensions(), vulkano::format::D16Unorm).expect("failed to !");

        ///Create a uniform buffer with just [[f32; 4]; 4], the buffer will be updated bevore the first loop
        let world = pipeline_infos::Main {
            model : <na::Matrix4<f32>>::identity().into(),
            view : <na::Matrix4<f32>>::identity().into(),
            proj : <na::Matrix4<f32>>::identity().into(),
        };


        //TODO, create custom renderpass with different stages (light computing, final shading (how to loop?),
        //postprogress) => Dig through docs.
        //Create a simple renderpass
        let renderpass = Arc::new(
            single_pass_renderpass!(device.clone(),
                attachments: {
                    color: {
                        load: Clear,
                        store: Store,
                        format: swapchain.format(),
                        samples: 1,
                    },
                    depth: {
                        load: Clear,
                        store: DontCare,
                        format: vulkano::format::Format::D16Unorm,
                        samples: 1,
                    }
                },
                pass: {
                    color: [color],
                    depth_stencil: {depth}
                }
            ).expect("failed to !")
        );

        //Create the frame buffers from all images
        let framebuffers = images.iter().map(|image| {
            Arc::new(vulkano::framebuffer::Framebuffer::start(renderpass.clone())
                //The color pass
                .add(image.clone()).expect("failed to add image to frame buffer!")
                //and its depth pass
                .add(depth_buffer.clone()).expect("failed to add depth to frame buffer!")
                .build().expect("failed to build framebuffer!"))
        }).collect::<Vec<_>>();

        let mut store_framebuffer: Vec<Arc<FramebufferAbstract + Send + Sync>> = Vec::new();
        for i in framebuffers{
            store_framebuffer.push(i.clone());
        }

        let previous_frame = Some(Box::new(vulkano::sync::now(device.clone())) as Box<GpuFuture>);

        //Creates the renderers pipeline manager
        let pipeline_manager = Arc::new(Mutex::new(pipeline_manager::PipelineManager::new(device.clone(), queue.clone(), renderpass.clone(), images.clone(), world)));
        Renderer{
            pipeline_manager: pipeline_manager,

            //Vulkano data
            extensions: extensions,
            instance: instance.clone(),
            //physical: physical,
            events_loop: events_loop,
            window: window,
            device: device,
            queues: queues,
            queue: queue,
            swapchain: swapchain,
            images: images,
            renderpass: renderpass,
            depth_buffer: depth_buffer,
            framebuffers: store_framebuffer,

            previous_frame: previous_frame,
        }
    }

    ///Renders the scene with the parameters supplied by the asset_manager
    pub fn render(&mut self, asset_manager: &mut asset_manager::AssetManager) -> bool{
        //DEBUG
        let start_time = Instant::now();

        self.previous_frame.as_mut().unwrap().cleanup_finished();

        let (image_num, acquire_future) = vulkano::swapchain::acquire_next_image(self.swapchain.clone(), None).expect("failed to !");

        let rotation = na::Rotation3::from_axis_angle(&na::Vector3::y_axis(), time::precise_time_ns() as f32 * 0.000000001);
        let mat_4: na::Matrix4<f32> = na::convert(rotation);
        let uniform_data = pipeline_infos::Main {
            model: mat_4.into(),
            view: asset_manager.get_camera().get_view_matrix().into(),
            proj: asset_manager.get_camera().get_perspective().into(),
        };
        //Lock the pipeline manager and update all uniforms
        let local_pipe_man = self.pipeline_manager.clone();
        {
            (*local_pipe_man).lock().expect("Failed to lock local pipeline manager").update_all_uniform_buffer_01(uniform_data);
        }


        //TODO have to find a nicer way of doing this... later
        let command_buffer = {
            let mut tmp_cmd_buffer = Some(
                vulkano::command_buffer::AutoCommandBufferBuilder::new(
                    self.device.clone(),
                    self.queue.family()).expect("failed to !")
                );

            let build_start = tmp_cmd_buffer.take().expect("failed to take cmd buffer build for start");

            tmp_cmd_buffer = Some(build_start.begin_render_pass(
                self.framebuffers[image_num].clone(), false,
                vec![
                    [0.01, 0.0, 0.1, 1.0].into(),
                    1f32.into()
                ]).expect("failed to clear"));


            //Draw
                //get all meshes, later in view frustum based on camera


            let mut index = 0;
            for i in asset_manager.get_all_meshes().iter(){
                let cb = tmp_cmd_buffer.take().expect("Failed to recive command buffer in loop!");
                let material = asset_manager.get_material_manager().get_material(&i.lock().expect("Could not lock mesh for material").get_material_name());
                let unlocked_material = (*material).lock().expect("Failed to lock material");

                let unlocked_mesh = i.lock().expect("Could not lock mesh for rendering :(");
                let mut unlocked_pipeline_manager = (*local_pipe_man).lock().expect("Failed to lock the pipeline manager while rendering");

                tmp_cmd_buffer = Some(cb
                    .draw_indexed(
                        unlocked_pipeline_manager.get_pipeline_by_name(&unlocked_material.get_pipeline_name().to_string()),
                        vulkano::command_buffer::DynamicState::none(),
                        (*unlocked_mesh).get_vertex_buffer(),
                        (*unlocked_mesh).get_index_buffer(self.device.clone(), self.queue.clone()).clone(),
                        unlocked_pipeline_manager.get_set_01(&unlocked_material.get_pipeline_name().to_string()),
                        ()
                    ).expect("Failed to draw in command buffer!")
                );
            }
            //End renderpass
            tmp_cmd_buffer.take().expect("failed to return command buffer to main buffer")
        }
        .end_render_pass().expect("failed to end")
        .build().expect("failed to end");;

        //TODO find a better methode then Option<Box<GpuFuture>>
        let future = self.previous_frame.take().unwrap().join(acquire_future)
                      .then_execute(self.queue.clone(), command_buffer).expect("failed to !")
                      .then_swapchain_present(self.queue.clone(), self.swapchain.clone(), image_num)
                      .then_signal_fence_and_flush().expect("failed to flush");
        self.previous_frame = Some(Box::new(future) as Box<_>);
        /*
        self.previous_frame = Box::new({
                self.previous_frame.join(acquire_future)
                .then_execute(self.queue.clone(), command_buffer).expect("failed to !")
                .then_swapchain_present(self.queue.clone(), self.swapchain.clone(), image_num)
                .then_signal_fence_and_flush().expect("failed to flush")

        });
        */
        let mut done = false;
        self.events_loop.poll_events(|ev| {
            match ev {
                winit::Event::WindowEvent { event: winit::WindowEvent::Closed, .. } => done = true,
                _ => ()
            }
        });
        if done { return true; }

        //DEBUG
        let fps_time = start_time.elapsed().subsec_nanos();
        println!("FPS: {}", 1.0/ (fps_time as f32 / 1_000_000_000.0) );

        false
    }

    ///Returns the pipeline manager of this renderer
    pub fn get_pipeline_manager(&mut self) -> Arc<Mutex<pipeline_manager::PipelineManager>>{
        self.pipeline_manager.clone()
    }

    ///Starts the rendering loop UNIMPLEMENTED
    pub fn start_loop(){

    }

    ///Returns the device of this renderer
    pub fn get_device(&self) -> Arc<vulkano::device::Device>{
        self.device.clone()
    }

    ///Returns the queue of this renderer
    pub fn get_queue(&self) -> Arc<vulkano::device::Queue>{
        self.queue.clone()
    }
}

/*TODO:
The Functions
Start the renderer
The Renderer is fixed fo now, it will always draw the same frame but will update its content everytime
this will be done via a Arc<content> / clone methode.
For instance the uniform_set 01 will be supplied by the camera system for model and camera info
the set_02 will be supplied by the material system in cooperation with the pipeline system to bind
the correct pipeline and uniform set at the right mesh
the vertex buffer will be copied from each mesh which will be rendered. The scene system will have its own
loop.
Last but not least, at some point the the renderer will calculate the forward+ light pass and give the
info to a ligh handeling system. But this is not implemented yet and won't be so fast. I have
to find out how to calculate this forward pass (ref: https://www.slideshare.net/takahiroharada/forward-34779335
and https://takahiroharada.files.wordpress.com/2015/04/forward_plus.pdf and
https://www.3dgep.com/forward-plus/#Forward)
*/
