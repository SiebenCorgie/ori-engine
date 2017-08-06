use render::renderer;
use render::pipeline;
use render::pipeline_manager;
use render::pipeline_infos;
use render::uniform_manager;

use vulkano::descriptor::descriptor_set::PersistentDescriptorSet;
use vulkano::descriptor::descriptor_set::PersistentDescriptorSetBuf;
use vulkano::pipeline::GraphicsPipelineAbstract;
use vulkano;

use std::sync::{Mutex,Arc};


///Describes a standart material
pub struct Material {
    name: String,
    //Physical describtion
    t_albedo: String,
    t_roughness: String,
    t_metal: String,
    t_normal: String,

    //Technical implementation
    ///Reference to parent pipeline
    pipeline: String,
    pipeline_manager: Arc<Mutex<pipeline_manager::PipelineManager>>,
    device: Arc<vulkano::device::Device>,
    uniform_manager: Arc<Mutex<uniform_manager::UniformManager>>,
}


impl Material {
    ///Creates an empty material with standart parameter for a pipeline
    pub fn new(name: &str, pipeline: &str,
        pipeline_manager: Arc<Mutex<pipeline_manager::PipelineManager>>,
        uniform_manager: Arc<Mutex<uniform_manager::UniformManager>>,
        device: Arc<vulkano::device::Device>,
    )-> Self{




        Material{
            name: String::from(name),

            t_albedo: String::from("data/fallback_alb.png"),
            t_roughness: String::from("data/fallback_rough.png"),
            t_metal: String::from("data/fallback_metal.png"),
            t_normal: String::from("data/fallback_nrm.png"),

            //All Unifrom infos
            pipeline: String::from(pipeline),
            pipeline_manager: pipeline_manager,
            uniform_manager: uniform_manager,
            device: device,

        }
    }
    //TODO Setup changes of the materials, maybe make possible to only insert colors for albedo etc
    pub fn get_pipeline_name(&self) -> String{
        self.pipeline.clone()
    }

    ///Returns the u_world_set generated by this call based on a `pipeline` in the
    /// `pipeline_manager` of the used `renderer`
    pub fn get_set_01(&self) ->
        Arc<PersistentDescriptorSet<Arc<GraphicsPipelineAbstract + Send + Sync>,
        (
            (), PersistentDescriptorSetBuf<vulkano::buffer::cpu_pool::CpuBufferPoolSubbuffer
            <pipeline_infos::Main, Arc<vulkano::memory::pool::StdMemoryPool>>>
        )>>
    {


        let pipe_man_in = self.pipeline_manager.clone();
        let mut pipe_lck = pipe_man_in.lock().expect("failed to lock pipeline manager");

        let uniform_manager_isnt = self.uniform_manager.clone();
        let mut uniform_manager_lck = uniform_manager_isnt.lock().expect("Failed to locj unfiorm_mng");

        //TODO add set 02 for material information
        let new_set = Arc::new(PersistentDescriptorSet::start((*pipe_lck).get_pipeline_by_name(&self.pipeline.clone().to_string()), 0)
            .add_buffer((*uniform_manager_lck).get_subbuffer_01().clone()).expect("Failed to create descriptor set")
            .build().expect("failed to build descriptor")
        );

        //return the new set
        new_set
    }

    ///Sets a new pipeline
    pub fn set_pipeline(&mut self, new_pipe: &str){
        self.pipeline = String::from(new_pipe);
    }

    ///Returns a copy/clone of this name
    pub fn get_name(&self) -> String{
        self.name.clone()
    }
}
