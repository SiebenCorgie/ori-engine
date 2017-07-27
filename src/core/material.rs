use render::pipeline;
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
}


impl Material {
    ///Creates an empty material with standart parameter for a pipeline
    pub fn new(name: &str, pipeline: &str)-> Self{
        Material{
            name: String::from(name),

            t_albedo: String::from("data/fallback_alb.png"),
            t_roughness: String::from("data/fallback_rough.png"),
            t_metal: String::from("data/fallback_metal.png"),
            t_normal: String::from("data/fallback_nrm.png"),

            pipeline: String::from(pipeline),
        }
    }
    //TODO Setup changes of the materials, maybe make possible to only insert colors for albedo etc
    //NOTE Changed to String based pipeline system for easier soft changes of the pipeline in a material
    ///Returns the pipeline this material uses
    pub fn get_pipeline_name(&self) -> String{
        self.pipeline.clone()
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
