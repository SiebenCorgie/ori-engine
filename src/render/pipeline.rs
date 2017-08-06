use vulkano;
use vulkano::pipeline;
use vulkano::image::swapchain::SwapchainImage;
use vulkano::descriptor::descriptor_set::PersistentDescriptorSet;
use vulkano::descriptor::descriptor_set::PersistentDescriptorSetBuf;
use vulkano_shader_derive;

use std::sync::Arc;
use std;
use std::boxed::Box;
use core::mesh;
use render::pipeline_infos;
///Definition of a single pipeline together with its creation and deleting behavoir
pub struct Pipeline {
    ///The main pipeline hold by this struct
    //TODO make this dynamic, or implement a different pipeline struct per type... maybe one graphic, one computing? (<- will do this)
    //TODO change to graphics_pipeline and add a compute_pipeline
    pipeline: Arc<pipeline::GraphicsPipelineAbstract + Send + Sync>,
    ///For reference the shader path this was created from
    vertex_shader_path: String,
    fragment_shader_path: String,
}

impl Pipeline{
    ///Creates a pipeline for a shader, TODO make it possible to create a custom pipeline easily
    pub fn new(
        device: Arc<vulkano::device::Device>,
        queue: Arc<vulkano::device::Queue>,
        renderpass: Arc<vulkano::framebuffer::RenderPassAbstract + Send + Sync>,
        images: Vec<Arc<SwapchainImage>>,
        //uniform_buffer: pipeline_infos::Main,
        vertex_shader_path: &str,
        fragment_shader_path: &str)
        -> Self
    {

        //Currently using a static shader from /data/test.vs/fs
        let vs = vs::Shader::load(device.clone()).expect("failed to create shader module");
        let fs = fs::Shader::load(device.clone()).expect("failed to create shader module");

        //Create the uniform buffers
        //01
        /*
        let tmp_uniform_buffer_pool = vulkano::buffer::cpu_pool::CpuBufferPool::<pipeline_infos::Main>
                                   ::new(device.clone(), vulkano::buffer::BufferUsage::all(), Some(queue.family()));
                                   */
        //Create a pipeline
        let vertex_buffer_definition = vulkano::pipeline::vertex::SingleBufferDefinition::<mesh::Vertex>::new();

        let tmp_pipeline: Arc<pipeline::GraphicsPipelineAbstract + Send + Sync> = Arc::new(vulkano::pipeline::GraphicsPipeline::start()
            .vertex_input(vertex_buffer_definition)
            .vertex_shader(vs.main_entry_point(), ())
            .triangle_list()
            .viewports_dynamic_scissors_irrelevant(1)
            /*
            .viewports(std::iter::once(vulkano::pipeline::viewport::Viewport {
                origin: [0.0, 0.0],
                depth_range: 0.0 .. 1.0,
                dimensions: [images[0].dimensions()[0] as f32, images[0].dimensions()[1] as f32],
            }))
            */
            .fragment_shader(fs.main_entry_point(), ())
            .depth_stencil_simple_depth()
            .render_pass(vulkano::framebuffer::Subpass::from(renderpass.clone(), 0).expect("failed to set render pass at pipe 01!"))
            .build(device.clone())
            .expect("failed to make pipe 01!"));

        /* this will be moved into the uniform_buffer_manager and the materials
        let tmp_uniform_data = tmp_uniform_buffer_pool.next(uniform_buffer);

        //Create Set at frame time from buffer if needed
        let set = Arc::new(PersistentDescriptorSet::start(tmp_pipeline.clone(), 0)
            .add_buffer(tmp_uniform_data).expect("Failed to create descriptor set")
            .build().expect("failed to build descriptor")
        );
        */

        //Create the Struct
        Pipeline{
            pipeline: tmp_pipeline,
            vertex_shader_path: String::from(vertex_shader_path.clone()),
            fragment_shader_path: String::from(fragment_shader_path.clone()),

        }

    }
    ///Returns the vulkano pipline definition
    pub fn get_pipeline_ref(&self) -> Arc<pipeline::GraphicsPipelineAbstract + Send + Sync>
    {
        self.pipeline.clone()
    }

/*
    ///Returns the first uniform set
    pub fn get_set_01(&self) ->
    Arc<PersistentDescriptorSet<Arc<pipeline::GraphicsPipelineAbstract + Send + Sync>,
    (
        (), PersistentDescriptorSetBuf<vulkano::buffer::cpu_pool::CpuBufferPoolSubbuffer
        <pipeline_infos::Main, Arc<vulkano::memory::pool::StdMemoryPool>>>
    )>>
    {
        self.uniform_set_01.clone()
    }

    ///Recreate the local buffer as well as its set
    pub fn update_uniform_buffer_01(&mut self, new_buffer: pipeline_infos::Main){

        let uniform_buffer_subbuffer = self.uniform_buffer_pool_01.next(new_buffer);
        self.recreate_set_01(uniform_buffer_subbuffer);
    }
*/
/*
    ///Recreates the set_01 with the new uniform_buffer, you can use `update_uniform_buffer_01` if you already have
    ///a uniform buffer, or `update_all_uniform_buffer_01` if you are using a pipeline manager
    fn recreate_set_01(&mut self, uniform_subbuffer: vulkano::buffer::cpu_pool::CpuBufferPoolSubbuffer<pipeline_infos::Main, Arc<vulkano::memory::pool::StdMemoryPool>>){
        let new_set = Arc::new(PersistentDescriptorSet::start(self.pipeline.clone(), 0)
            .add_buffer(uniform_subbuffer).expect("Failed to create descriptor set")
            .build().expect("failed to build descriptor")
        );
        self.uniform_set_01 = new_set;
    }
*/
}


//Will be removed hopefully
mod vs {
    #[derive(VulkanoShader)]
    #[ty = "vertex"]
    #[path = "data/test.vs"]
    struct Dummy;
}



mod fs {

    #[derive(VulkanoShader)]
    #[ty = "fragment"]
    #[path = "data/test.fs"]
    struct Dummy;
}
