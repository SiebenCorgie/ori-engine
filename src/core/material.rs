use render::renderer;
use render::pipeline;
use render::pipeline_manager;
use render::pipeline_infos;
use render::uniform_manager;

use vulkano::descriptor::descriptor_set::PersistentDescriptorSet;
use vulkano::image::ImmutableImage;
use vulkano::descriptor::descriptor_set::StdDescriptorPoolAlloc;
use vulkano::descriptor::descriptor_set::PersistentDescriptorSetBuf;
use vulkano::descriptor::descriptor_set::PersistentDescriptorSetImg;
use vulkano::descriptor::descriptor_set::PersistentDescriptorSetSampler;
use vulkano::descriptor::descriptor_set::DescriptorSet;
use vulkano::pipeline::GraphicsPipelineAbstract;
use vulkano;

use image;

use std::sync::{Mutex,Arc};


///The material descibes the physical implementation of the material
///It mostly contains three textures:
/// - albedo: the color representation (without light)
/// - normal: the normal representation of the surface
/// - physical: is a system texture which is split by channels:
///   - Green: Metallic
///   - Blue: Roughness
///   This is directly taken from gltf


///Describes a standart material
pub struct Material {
    name: String,
    //Physical describtion
    t_albedo_path: String,
    sampler_albedo: Arc<vulkano::sampler::Sampler>,
    t_albedo: Arc<vulkano::image::ImmutableImage<vulkano::format::R8G8B8A8Srgb>>,

    t_normal_path: String,
    sampler_normal: Arc<vulkano::sampler::Sampler>,
    t_normal: Arc<vulkano::image::ImmutableImage<vulkano::format::R8G8B8A8Srgb>>,

    t_physical_path: String,
    sampler_physical: Arc<vulkano::sampler::Sampler>,
    t_physical: Arc<vulkano::image::ImmutableImage<vulkano::format::R8G8B8A8Srgb>>,

    //Technical implementation
    ///Reference to parent pipeline
    pipeline: String,
    pipeline_manager: Arc<Mutex<pipeline_manager::PipelineManager>>,
    device: Arc<vulkano::device::Device>,
    ///A reference to the global uniform manager
    uniform_manager: Arc<Mutex<uniform_manager::UniformManager>>,

    //The set for the u_world information
    set_01: Arc<DescriptorSet + Send + Sync>,

    //A persistent material set which only needs to be alter if a texture changes
    set_02: Arc<DescriptorSet + Send + Sync>,
}


impl Material {
    ///Creates an empty material with standart parameter for a pipeline
    pub fn new(name: &str, pipeline: &str,
        pipeline_manager: Arc<Mutex<pipeline_manager::PipelineManager>>,
        uniform_manager: Arc<Mutex<uniform_manager::UniformManager>>,
        device: Arc<vulkano::device::Device>,
        queue: Arc<vulkano::device::Queue>,
    )-> Self{

        //Generate all the samplers for the different textures
            //NOTE this might be changed to be configurable
            //Either with an update function or a builder type
            //Which will create a new material and replace the old one
            //However, the material properties might stay static on runtime

            //Antistrophical filtering level as well as lod levels might be moved to
            //a configuration file
        ///Create the samplers

        let sampler_albedo_tmp = vulkano::sampler::Sampler::new(
            device.clone(),
            vulkano::sampler::Filter::Linear,
            vulkano::sampler::Filter::Linear,
            vulkano::sampler::MipmapMode::Nearest,
            vulkano::sampler::SamplerAddressMode::Repeat,
            vulkano::sampler::SamplerAddressMode::Repeat,
            vulkano::sampler::SamplerAddressMode::Repeat,
            0.0, 1.0, 0.0, 0.0
        ).expect("Failed to generate albedo sampler");

        let sampler_normal_tmp = vulkano::sampler::Sampler::new(
            device.clone(),
            vulkano::sampler::Filter::Linear,
            vulkano::sampler::Filter::Linear,
            vulkano::sampler::MipmapMode::Nearest,
            vulkano::sampler::SamplerAddressMode::Repeat,
            vulkano::sampler::SamplerAddressMode::Repeat,
            vulkano::sampler::SamplerAddressMode::Repeat,
            0.0, 1.0, 0.0, 0.0
        ).expect("Failed to generate normal sampler");

        let sampler_physical_tmp= vulkano::sampler::Sampler::new(
            device.clone(),
            vulkano::sampler::Filter::Linear,
            vulkano::sampler::Filter::Linear,
            vulkano::sampler::MipmapMode::Nearest,
            vulkano::sampler::SamplerAddressMode::Repeat,
            vulkano::sampler::SamplerAddressMode::Repeat,
            vulkano::sampler::SamplerAddressMode::Repeat,
            0.0, 1.0, 0.0, 0.0
        ).expect("Failed to generate physical sampler");



        //Now load a default texture
        let texture_albedo = {
            let (texture_albedo_tmp, tex_future_albedo) = {
                let image = image::load_from_memory_with_format(include_bytes!("/home/siebencorgie/Scripts/Rust/engine/ori-engine/data/fallback_alb.png"),
                    image::ImageFormat::PNG).expect("failed to load png").to_rgba();

                let (width, height) = image.dimensions();
                let image_data = image.into_raw().clone();

                vulkano::image::immutable::ImmutableImage::from_iter(
                    image_data.iter().cloned(),
                    vulkano::image::Dimensions::Dim2d { width: width, height: height },
                    vulkano::format::R8G8B8A8Srgb,
                    Some(queue.family()),
                    queue.clone()).expect("failed to create immutable image")
            };
            println!("STATUS: MATERIAL: droping future", );
            texture_albedo_tmp
        };
        println!("STATUS: MATERIAL: Now with fully loaded albedo", );


        //Now load a default texture
        let texture_nrm = {
            let (texture_nrm_tmp, tex_future_nrm) = {
                let image = image::load_from_memory_with_format(include_bytes!("/home/siebencorgie/Scripts/Rust/engine/ori-engine/data/fallback_nrm.png"),
                    image::ImageFormat::PNG).expect("failed to load png").to_rgba();

                let (width, height) = image.dimensions();
                let image_data = image.into_raw().clone();

                vulkano::image::immutable::ImmutableImage::from_iter(
                    image_data.iter().cloned(),
                    vulkano::image::Dimensions::Dim2d { width: width, height: height },
                    vulkano::format::R8G8B8A8Srgb,
                    Some(queue.family()),
                    queue.clone()).expect("failed to create immutable image")
            };

            println!("STATUS: MATERIAL: Returning Normal", );
            texture_nrm_tmp
        };

        //Now load a default texture
        let texture_physical = {
            let (texture_physical_tmp, tex_future_physical) = {
                let image = image::load_from_memory_with_format(include_bytes!("/home/siebencorgie/Scripts/Rust/engine/ori-engine/data/fallback_rough.png"),
                    image::ImageFormat::PNG).expect("failed to load png").to_rgba();

                let (width, height) = image.dimensions();
                let image_data = image.into_raw().clone();

                vulkano::image::immutable::ImmutableImage::from_iter(
                    image_data.iter().cloned(),
                    vulkano::image::Dimensions::Dim2d { width: width, height: height },
                    vulkano::format::R8G8B8A8Srgb,
                    Some(queue.family()),
                    queue.clone()).expect("failed to create immutable image")
            };
            println!("STATUS: MATERIAL: Returning physical", );
            texture_physical_tmp
        };



        //Create the set 02
        //lock the pipe
        let pipe_man_in = pipeline_manager.clone();
        let mut pipe_lck = pipe_man_in.lock().expect("failed to lock pipeline manager");

        println!("STATUS: MATERIAL: Creating set 02 for the first time", );
        let set_02 = Arc::new(
            PersistentDescriptorSet::start(
            (*pipe_lck).get_pipeline_by_name(&pipeline.clone().to_string()), 1)
            .add_sampled_image(texture_albedo.clone(), sampler_albedo_tmp.clone())
            .expect("failed to add sampled albedo")
            .add_sampled_image(texture_nrm.clone(), sampler_normal_tmp.clone())
            .expect("failed to add sampled nrm")
            .add_sampled_image(texture_physical.clone(), sampler_physical_tmp.clone())
            .expect("failed to add sampled physical")
            .build().expect("failed to build set_02")
        );

        //now drop the future to wait for gpu
        println!("STATUS: MATERIAL: Waiting for texture on gpu", );
        //tex_future_albedo.cleanup_finished();
        println!("STATUS: MATERIAL: Finished waiting for all textures", );

        let uniform_manager_isnt = uniform_manager.clone();
        let mut uniform_manager_lck = uniform_manager_isnt.lock().expect("Failed to locj unfiorm_mng");

        //TODO add set 02 for material information
        println!("STATUS: MATERIAL: Creating set 01 for the first time", );
        let set_01 = Arc::new(PersistentDescriptorSet::start(
            (*pipe_lck).get_pipeline_by_name(&pipeline.clone().to_string()), 0)
            .add_buffer((*uniform_manager_lck).get_subbuffer_01().clone()).expect("Failed to create descriptor set")
            .build().expect("failed to build descriptor")
        );

        println!("STATUS: MATERIAL: Created material!", );

        Material{
            name: String::from(name),

            t_albedo_path: String::from("data/fallback_alb.png"),
            sampler_albedo: sampler_albedo_tmp,
            t_albedo: texture_albedo,

            t_normal_path: String::from("data/fallback_nrm.png"),
            sampler_normal: sampler_normal_tmp,
            t_normal: texture_nrm,

            t_physical_path: String::from("data/fallback_metal.png"),
            sampler_physical: sampler_physical_tmp,
            t_physical: texture_physical,

            //All Unifrom infos
            pipeline: String::from(pipeline),
            pipeline_manager: pipeline_manager,
            device: device,
            uniform_manager: uniform_manager,

            set_01: set_01,

            set_02: set_02,

        }
    }

    //TODO Setup changes of the materials, maybe make possible to only insert colors for albedo etc
    pub fn get_pipeline_name(&self) -> String{
        self.pipeline.clone()
    }

    ///Updates all sets tied to this material
    pub fn update(&mut self){
        println!("STATUS: MATERIAL: In material, updating now", );
        self.recreate_set_01();
        println!("STATUS: MATERIAL: Finished updating", );
        //self.recreate_textures_and_samplers();
    }

    ///Recreates set_01 based on the current unfiorm_manager information
    pub fn recreate_set_01(&mut self){
        println!("STATUS: MATERIAL: Trying to lock pipeline", );
        let pipe_man_in = self.pipeline_manager.clone();
        let mut pipe_lck = pipe_man_in.lock().expect("failed to lock pipeline manager");

        println!("STATUS: MATERIAL: Trying to locj uniform manager", );
        let uniform_manager_isnt = self.uniform_manager.clone();
        let mut uniform_manager_lck = uniform_manager_isnt.lock().expect("Failed to locj unfiorm_mng");
        println!("STATUS: MATERIAL: Generation new set_01", );
        //TODO add set 02 for material information
        let new_set = Arc::new(PersistentDescriptorSet::start(
            (*pipe_lck).get_pipeline_by_name(&self.pipeline.clone().to_string()), 0)
            .add_buffer((*uniform_manager_lck).get_subbuffer_01().clone()).expect("Failed to create descriptor set")
            .build().expect("failed to build descriptor")
        );
        println!("STATUS: MATERIAL: Returning new set to self", );
        //return the new set
        self.set_01 = new_set;
    }

    ///Returns the u_world_set generated by this call based on a `pipeline` in the
    /// `pipeline_manager` of the used `renderer`
    pub fn get_set_01(&self) -> Arc<DescriptorSet + Send + Sync>
    {

        /*
        let pipe_man_in = self.pipeline_manager.clone();
        let mut pipe_lck = pipe_man_in.lock().expect("failed to lock pipeline manager");

        let uniform_manager_isnt = self.uniform_manager.clone();
        let mut uniform_manager_lck = uniform_manager_isnt.lock().expect("Failed to locj unfiorm_mng");

        //TODO add set 02 for material information
        let new_set = Arc::new(PersistentDescriptorSet::start(
            (*pipe_lck).get_pipeline_by_name(&self.pipeline.clone().to_string()), 0)
            .add_buffer((*uniform_manager_lck).get_subbuffer_01().clone()).expect("Failed to create descriptor set")
            .build().expect("failed to build descriptor")
        );

        println!("Returned set 01", );
        //return the new set
        new_set
        */
        println!("STATUS: MATERIAL: Returning saved set_01", );
        self.set_01.clone()
    }

    ///Returns the second set which holds the material textures
    pub fn get_set_02(&self) -> Arc<DescriptorSet + Send + Sync>
    {

        /*
        //lock the pipe
        let pipe_man_in = self.pipeline_manager.clone();
        let mut pipe_lck = pipe_man_in.lock().expect("failed to lock pipeline manager");


        let set = Arc::new(
            PersistentDescriptorSet::start(
            (*pipe_lck).get_pipeline_by_name(&self.pipeline.clone().to_string()), 1)
            .add_sampled_image(self.t_albedo.clone(), self.sampler_albedo.clone())
            .expect("failed to add sampled albedo")
            .build().expect("failed to build set_02")
        );

        println!("Return set 02", );
        //return the set
        set
        */
        println!("STATUS: MATERIAL: Returning saved set_02", );

        self.set_02.clone()

    }

    ///Cleans the future objects of the textures
    pub fn clean(&self){

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
