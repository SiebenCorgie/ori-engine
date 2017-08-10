use std::collections::HashMap;
use std::sync::{Mutex, Arc};
use std;

use render::pipeline;
use render::pipeline_infos;

use vulkano;
use vulkano::image::swapchain::SwapchainImage;
use vulkano::pipeline::GraphicsPipelineAbstract;
use vulkano::descriptor::descriptor_set::PersistentDescriptorSet;
use vulkano::descriptor::descriptor_set::PersistentDescriptorSetBuf;

///Manages all available pipeline
pub struct PipelineManager {
    pipelines: HashMap<String, pipeline::Pipeline>,
}

impl PipelineManager{

    ///Creates a pipeline Manager with a default pipeline, have a look at the code to see the pipeline type
    pub fn new(
        device: Arc<vulkano::device::Device>, queue: Arc<vulkano::device::Queue>,
        renderpass: Arc<vulkano::framebuffer::RenderPassAbstract + Send + Sync>,
        images: Vec<Arc<SwapchainImage>>,
    ) -> Self
    {
        let mut hashmap = HashMap::new();
        //Creates a default pipeline from a default shader
        let default_pipeline = pipeline::Pipeline::new(device, queue, renderpass, images, "src/defaults/shader/DefShader.vs", "src/defaults/shader/DefShader.fs");
        hashmap.insert(String::from("DefaultPipeline"), default_pipeline);

        PipelineManager{
            pipelines: hashmap,
        }
    }

    ///Returns true if there is a pipeline with this name
    pub fn has_pipeline(&self, name: &str) -> bool{
        if self.pipelines.contains_key(&String::from(name)){
            return true
        }
        false
    }

    ///Should always return the normal PBR pipeline, if it panics, please file a bug report, this should not happen
    pub fn get_default_pipeline(&mut self) -> Arc<GraphicsPipelineAbstract + Send + Sync>{
        match self.pipelines.get_mut(&String::from("DefaultPipeline")){
            Some(ref mut pipe) => return pipe.get_pipeline_ref(),
            None => println!("STATUS: PIPELINE MANAGER: Could not find default pipe this should not happen", ),
        }
        panic!("Crash could not get default pipeline!")
    }

    ///Returns a pipeline by name, if not existend, returns the default pipeline
    pub fn get_pipeline_by_name(&mut self, name: &str) -> Arc<GraphicsPipelineAbstract + Send + Sync>{
        println!("SEARCHING FOR PIPELINE: {}", name.clone() );
        match self.pipelines.get_mut(&String::from(name)){
            Some(ref mut pipe) => return pipe.get_pipeline_ref(),
            None => println!("Could not find pipe {}", name.clone()),
        }
        self.get_default_pipeline()
    }

    ///Adds a pipeline made for the specified shader
    pub fn add_pipeline_from_shader(&mut self, name: &str,device: Arc<vulkano::device::Device>,
        queue: Arc<vulkano::device::Queue>,
        renderpass: Arc<vulkano::framebuffer::RenderPassAbstract + Send + Sync>,
        images: Vec<Arc<SwapchainImage>>,
        uniform_buffer: pipeline_infos::Main,
        vertex_shader: &str,
        fragment_shader: &str)
    {
        let tmp_pipeline = pipeline::Pipeline::new(device, queue, renderpass, images, vertex_shader, fragment_shader);
        self.pipelines.insert(String::from(name), tmp_pipeline);
    }

}
