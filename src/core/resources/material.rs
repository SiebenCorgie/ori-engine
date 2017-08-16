use render::renderer;
use render::pipeline;
use render::pipeline_manager;
use render::pipeline_infos;
use render::uniform_manager;
use core::resources::texture;


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


///A Struct used for prototyping the usage flags of the textures
#[derive(Clone)]
pub struct TextureUsageFlags {
    pub albedo: i32,
    pub normal: i32,
    pub metal: i32,
    pub roughness: i32,
    pub occlusion: i32,
    pub emissive: i32
    //TODO Implement additional textures:
    // -
}

impl TextureUsageFlags{
    ///Creates a new flag info where all textures are used
    pub fn new() -> Self{
        TextureUsageFlags{
            albedo: 0,
            normal: 0,
            metal: 0,
            roughness: 0,
            occlusion: 0,
            emissive: 0,
        }
    }

    ///Creates with a set albedo status
    pub fn with_albedo(mut self, albedo: i32) ->Self{
        self.albedo = albedo;
        self
    }

    ///Creates with a set normal status
    pub fn with_normal(mut self, normal: i32) ->Self{
        self.normal = normal;
        self
    }

    ///Creates with a set metal status
    pub fn with_metal(mut self, metal: i32) ->Self{
        self.metal = metal;
        self
    }

    ///Creates with a set roughness status
    pub fn with_roughness(mut self, roughness: i32) ->Self{
        self.roughness = roughness;
        self
    }

    ///Creates with a set occlusion status
    pub fn with_occlusion(mut self, occlusion: i32) ->Self{
        self.occlusion = occlusion;
        self
    }

    ///Creates with a set emissive status
    pub fn with_emissive(mut self, emissive: i32) ->Self{
        self.emissive = emissive;
        self
    }
}

///A Struct defining the the material factors. They are used as Colors/factors if no textures
/// are present
#[derive(Clone)]
pub struct MaterialFactors{
    albedo_factor: [f32; 4],
    normal_factor: [f32; 4],
    metal_factor: f32,
    roughness_factor: f32,
    occlusion_factor: f32,
    emissive_factor: [f32; 4],

}

impl MaterialFactors{
    ///Creates a set of default factors
    pub fn new()-> Self{
        MaterialFactors{
            albedo_factor: [1.0; 4],
            //this needs to be set to just blue for not manipulating the rest
            normal_factor: [0.0, 0.0, 1.0, 1.0],
            metal_factor: 1.0,
            roughness_factor: 1.0,
            occlusion_factor: 1.0,
            emissive_factor: [1.0; 4],
        }
    }

    ///Creates the Factor struct with a given albdeo factor
    pub fn with_factor_albedo(mut self, factor: [f32; 4]) -> Self{
        self.albedo_factor = factor;
        self
    }

    ///Creates the Factor struct with a given normal factor
    pub fn with_factor_normal(mut self, factor: [f32; 4]) -> Self{
        self.normal_factor = factor;
        self
    }

    ///Creates the Factor struct with a given metal factor
    pub fn with_factor_metal(mut self, factor: f32) -> Self{
        self.metal_factor = factor;
        self
    }

    ///Creates the Factor struct with a given roughness factor
    pub fn with_factor_roughness(mut self, factor: f32) -> Self{
        self.roughness_factor = factor;
        self
    }

    ///Creates the Factor struct with a given occlusion factor
    pub fn with_factor_occlusion(mut self, factor: f32) -> Self{
        self.occlusion_factor = factor;
        self
    }

    ///Creates the Factor struct with a given emissive factor
    pub fn with_factor_emmissive(mut self, factor: [f32; 4]) -> Self{
        self.emissive_factor = factor;
        self
    }
}

///The material descibes the physical implementation of the material
///It mostly contains three textures:
/// - albedo: the color representation (without light)
/// - normal: the normal representation of the surface
/// - physical: is a system texture which is split by channels:
///   - Red: Ambient Occlusion
///   - Green: Metallic
///   - Blue: Roughness
///   This is mostly taken from gltf 2.0

///Describes a standart material
pub struct Material {
    name: String,
    //albedo describtion
    t_albedo_path: String,
    sampler_albedo: Arc<vulkano::sampler::Sampler>,
    t_albedo: Arc<vulkano::image::ImmutableImage<vulkano::format::R8G8B8A8Srgb>>,
    //normal
    t_normal_path: String,
    sampler_normal: Arc<vulkano::sampler::Sampler>,
    t_normal: Arc<vulkano::image::ImmutableImage<vulkano::format::R8G8B8A8Srgb>>,
    //Physical
    t_physical_path: String,
    sampler_physical: Arc<vulkano::sampler::Sampler>,
    t_physical: Arc<vulkano::image::ImmutableImage<vulkano::format::R8G8B8A8Srgb>>,
    //Additional textures: TODO implent


    //Technical implementation
    ///Reference to parent pipeline
    pipeline: String,
    pipeline_manager: Arc<Mutex<pipeline_manager::PipelineManager>>,
    device: Arc<vulkano::device::Device>,
    queue: Arc<vulkano::device::Queue>,
    ///A reference to the global uniform manager
    uniform_manager: Arc<Mutex<uniform_manager::UniformManager>>,

    //The set for the u_world information
    set_01: Arc<DescriptorSet + Send + Sync>,

    //A persistent material set which only needs to be alter if a texture changes
    set_02: Arc<DescriptorSet + Send + Sync>,

    //Usage flags of the different buffers, stored in a seperate set
    set_03: Arc<DescriptorSet + Send + Sync>,
    texture_usage_info: TextureUsageFlags,
    usage_info_pool: vulkano::buffer::cpu_pool::CpuBufferPool<TextureUsageFlags>,

    material_factors: MaterialFactors,
    material_factor_pool: vulkano::buffer::cpu_pool::CpuBufferPool<MaterialFactors>,
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
            //Which will create a new maTypeNameterial and replace the old one
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



        //Now load a "nothing" texture, the multiplier might be used instead for colors
        let texture_albedo = {
            let (texture_albedo_tmp, tex_future_albedo) = {
                let image = image::open("/home/siebencorgie/Scripts/Rust/engine/ori-engine/data/fallback_alb.png")
                .expect("failed to load png normal in creation").flipv().to_rgba();

                let (width, height) = image.dimensions();
                let image_data = image.into_raw().clone();

                vulkano::image::immutable::ImmutableImage::from_iter(
                    image_data.iter().cloned(),
                    vulkano::image::Dimensions::Dim2d { width: width, height: height },
                    vulkano::format::R8G8B8A8Srgb,
                    Some(queue.family()),
                    queue.clone()).expect("failed to create immutable image")
            };
            //println!("STATUS: MATERIAL: droping future", );
            texture_albedo_tmp
        };
        //println!("STATUS: MATERIAL: Now with fully loaded albedo", );


        //Now load a default texture
        let texture_nrm = {
            let (texture_nrm_tmp, tex_future_nrm) = {
                let image = image::open("/home/siebencorgie/Scripts/Rust/engine/ori-engine/data/nothing.png")
                .expect("failed to load png normal in creation").to_rgba();

                let (width, height) = image.dimensions();
                let image_data = image.into_raw().clone();

                vulkano::image::immutable::ImmutableImage::from_iter(
                    image_data.iter().cloned(),
                    vulkano::image::Dimensions::Dim2d { width: width, height: height },
                    vulkano::format::R8G8B8A8Srgb,
                    Some(queue.family()),
                    queue.clone()).expect("failed to create immutable image")
            };

            //println!("STATUS: MATERIAL: Returning Normal", );
            texture_nrm_tmp
        };

        //Now load a default texture
        let texture_physical = {
            let (texture_physical_tmp, tex_future_physical) = {
                let image = image::open("/home/siebencorgie/Scripts/Rust/engine/ori-engine/data/nothing.png")
                .expect("failed to load png physical in creation").to_rgba();

                let (width, height) = image.dimensions();
                let image_data = image.into_raw().clone();

                vulkano::image::immutable::ImmutableImage::from_iter(
                    image_data.iter().cloned(),
                    vulkano::image::Dimensions::Dim2d { width: width, height: height },
                    vulkano::format::R8G8B8A8Srgb,
                    Some(queue.family()),
                    queue.clone()).expect("failed to create immutable image")
            };
            //println!("STATUS: MATERIAL: Returning physical", );
            texture_physical_tmp
        };


        //For the 3rd descriptor set create a usage information struct, this might be changed
        let usage_info = TextureUsageFlags::new();
        //and its pool
        //Create a pool to allocate from
        let usage_info_pool = vulkano::buffer::cpu_pool::CpuBufferPool::<TextureUsageFlags>
                                   ::new(device.clone(), vulkano::buffer::BufferUsage::all(), Some(queue.family()));

        //for the 3rd as well the default factors
        let material_factor = MaterialFactors::new();

        let material_factor_pool = vulkano::buffer::cpu_pool::CpuBufferPool::<MaterialFactors>
                                   ::new(device.clone(), vulkano::buffer::BufferUsage::all(), Some(queue.family()));

        //lock the pipe
        let pipe_man_in = pipeline_manager.clone();
        let mut pipe_lck = pipe_man_in.lock().expect("failed to lock pipeline manager");

        //Additionaly lock the uniformanager to get the first global information
        let uniform_manager_isnt = uniform_manager.clone();
        let mut uniform_manager_lck = uniform_manager_isnt.lock().expect("Failed to locj unfiorm_mng");

        //TODO add set 02 for material information
        //println!("STATUS: MATERIAL: Creating set 01 for the first time", );
        let set_01 = Arc::new(PersistentDescriptorSet::start(
            (*pipe_lck).get_pipeline_by_name(&pipeline.clone().to_string()), 0)
            .add_buffer((*uniform_manager_lck).get_subbuffer_01().clone()).expect("Failed to create descriptor set")
            .build().expect("failed to build descriptor")
        );


        //Create the set 02
        //println!("STATUS: MATERIAL: Creating set 02 for the first time", );
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

        //Create the Usage Flag descriptor
        let set_03 = Arc::new(PersistentDescriptorSet::start(
            (*pipe_lck).get_pipeline_by_name(&pipeline.clone().to_string()), 2)
            .add_buffer(usage_info_pool.next(
                usage_info.clone()
            ).clone()).expect("Failed to create descriptor set")
            .add_buffer(material_factor_pool.next(
                material_factor.clone()
            ).clone()).expect("failed to create the first material factor pool")
            .build().expect("failed to build descriptor")
        );

        //println!("STATUS: MATERIAL: Created material!", );

        Material{
            name: String::from(name),

            t_albedo_path: String::from("/home/siebencorgie/Scripts/Rust/engine/ori-engine/data/nothing.png"),
            sampler_albedo: sampler_albedo_tmp,
            t_albedo: texture_albedo,

            t_normal_path: String::from("/home/siebencorgie/Scripts/Rust/engine/ori-engine/data/nothing.png"),
            sampler_normal: sampler_normal_tmp,
            t_normal: texture_nrm,

            t_physical_path: String::from("/home/siebencorgie/Scripts/Rust/engine/ori-engine/data/nothing.png"),
            sampler_physical: sampler_physical_tmp,
            t_physical: texture_physical,

            //All Unifrom infos
            pipeline: String::from(pipeline),
            pipeline_manager: pipeline_manager,
            device: device,
            queue: queue.clone(),
            uniform_manager: uniform_manager,

            set_01: set_01,

            set_02: set_02,

            set_03: set_03,

            texture_usage_info: usage_info,
            usage_info_pool: usage_info_pool,

            material_factors: material_factor,
            material_factor_pool: material_factor_pool,
        }
    }

    ///Adds a albedo texture to the material
    pub fn set_albedo_texture(&mut self, albedo_path: &str){
        self.t_albedo_path = String::from(albedo_path);
        self.texture_usage_info.albedo = 1;
    }

    ///Adds a normal Texture
    pub fn set_normal_texture(&mut self, normal_path: &str){
        self.t_normal_path = String::from(normal_path);
        self.texture_usage_info.normal = 1;
    }

    ///Adds a physical texture
    pub fn set_physical_texture(&mut self, physical_path: &str){
        self.t_physical_path = String::from(physical_path);
    }

    pub fn set_texture_usage_info(&mut self, info: TextureUsageFlags){
        self.texture_usage_info = info;
    }

    ///Sets the material factors
    pub fn set_material_factor_info(&mut self, info: MaterialFactors){
        self.material_factors = info;
    }

    ///Recreates set_02, set_03
    pub fn recreate_static_sets(&mut self){
        //println!("STATUS: MATERIAL: Recreation static sets", );
        //Recreate Texture, this might be moved to a Texture manager which will store
        //MaterialTexture Structs of Type Option<Arc<Texture>> for each type

        //Now load a default texture
        //println!("WARNING: MATERIAL: loading albedo: {}", self.t_albedo_path.clone());
        let texture_albedo = {
            let (texture_albedo_tmp, tex_future_albedo) = {
                let image = image::open(self.t_albedo_path.to_string())
                .expect("failed to load png for albedo").flipv().to_rgba();

                let (width, height) = image.dimensions();
                let image_data = image.into_raw().clone();

                vulkano::image::immutable::ImmutableImage::from_iter(
                    image_data.iter().cloned(),
                    vulkano::image::Dimensions::Dim2d { width: width, height: height },
                    vulkano::format::R8G8B8A8Srgb,
                    Some(self.queue.family()),
                    self.queue.clone()).expect("failed to create immutable image")
            };
            //println!("STATUS: MATERIAL: droping future", );
            texture_albedo_tmp
        };
        self.t_albedo = texture_albedo;
        //println!("STATUS: MATERIAL: Now with fully loaded albedo", );


        //Now load a default texture
        let texture_nrm = {
            let (texture_nrm_tmp, tex_future_nrm) = {
                let image = image::open(self.t_normal_path.to_string())
                .expect("failed to load png for nrm").flipv().to_rgba();

                let (width, height) = image.dimensions();
                let image_data = image.into_raw().clone();

                vulkano::image::immutable::ImmutableImage::from_iter(
                    image_data.iter().cloned(),
                    vulkano::image::Dimensions::Dim2d { width: width, height: height },
                    vulkano::format::R8G8B8A8Srgb,
                    Some(self.queue.family()),
                    self.queue.clone()).expect("failed to create immutable image")
            };

            //println!("STATUS: MATERIAL: Returning Normal", );
            texture_nrm_tmp
        };
        self.t_normal = texture_nrm;

        //Now load a default texture
        let texture_physical = {
            let (texture_physical_tmp, tex_future_physical) = {
                let image = image::open(self.t_physical_path.to_string())
                .expect("failed to load png for physical").flipv().to_rgba();;

                let (width, height) = image.dimensions();
                let image_data = image.into_raw().clone();

                vulkano::image::immutable::ImmutableImage::from_iter(
                    image_data.iter().cloned(),
                    vulkano::image::Dimensions::Dim2d { width: width, height: height },
                    vulkano::format::R8G8B8A8Srgb,
                    Some(self.queue.family()),
                    self.queue.clone()).expect("failed to create immutable image")
            };
            //println!("STATUS: MATERIAL: Returning physical", );
            texture_physical_tmp
        };
        self.t_physical = texture_physical;

        //Lock resources
        //lock the pipe
        let pipe_man_in = self.pipeline_manager.clone();
        let mut pipe_lck = pipe_man_in.lock().expect("failed to lock pipeline manager");

        //Additionaly lock the uniformanager to get the first global information
        let uniform_manager_isnt = self.uniform_manager.clone();
        let mut uniform_manager_lck = uniform_manager_isnt.lock().expect("Failed to locj unfiorm_mng");


        //Create the set 02
        //println!("STATUS: MATERIAL: ReCreating set 02", );
        let set_02 = Arc::new(
            PersistentDescriptorSet::start(
            (*pipe_lck).get_pipeline_by_name(&self.pipeline.clone().to_string()), 1)
            .add_sampled_image(self.t_albedo.clone(), self.sampler_albedo.clone())
            .expect("failed to add sampled albedo")
            .add_sampled_image(self.t_normal.clone(), self.sampler_normal.clone())
            .expect("failed to add sampled nrm")
            .add_sampled_image(self.t_physical.clone(), self.sampler_physical.clone())
            .expect("failed to add sampled physical")
            .build().expect("failed to build set_02")
        );

        self.set_02 = set_02;

        //Create the Usage Flag descriptor
        let set_03 = Arc::new(PersistentDescriptorSet::start(
            (*pipe_lck).get_pipeline_by_name(&self.pipeline.clone().to_string()), 2)
            .add_buffer(
                self.get_usage_info_subbuffer()
            ).expect("Failed to create descriptor set")
            .add_buffer(
                self.get_material_factor_subbuffer()
            ).expect("failed to create the material factor pool")
            .build().expect("failed to build descriptor")
        );

        self.set_03 = set_03;
    }


    //TODO Setup changes of the materials, maybe make possible to only insert colors for albedo etc
    ///Returns the name of the currently used pipeline
    pub fn get_pipeline_name(&self) -> String{
        self.pipeline.clone()
    }

    ///Updates all sets tied to this material
    pub fn update(&mut self){
        //println!("STATUS: MATERIAL: In material, updating now", );
        self.recreate_set_01();
        //println!("STATUS: MATERIAL: Finished updating", );
        //if needed, update the static sets
    }

    ///Recreates set_01 based on the current unfiorm_manager information
    pub fn recreate_set_01(&mut self){
        //println!("STATUS: MATERIAL: Trying to lock pipeline", );
        let pipe_man_in = self.pipeline_manager.clone();
        let mut pipe_lck = pipe_man_in.lock().expect("failed to lock pipeline manager");

        //println!("STATUS: MATERIAL: Trying to locj uniform manager", );
        let uniform_manager_isnt = self.uniform_manager.clone();
        let mut uniform_manager_lck = uniform_manager_isnt.lock().expect("Failed to locj unfiorm_mng");
        //println!("STATUS: MATERIAL: Generation new set_01", );
        //TODO add set 02 for material information
        let new_set = Arc::new(PersistentDescriptorSet::start(
            (*pipe_lck).get_pipeline_by_name(&self.pipeline.clone().to_string()), 0)
            .add_buffer((*uniform_manager_lck).get_subbuffer_01().clone()).expect("Failed to create descriptor set")
            .build().expect("failed to build descriptor")
        );
        //println!("STATUS: MATERIAL: Returning new set to self", );
        //return the new set
        self.set_01 = new_set;
    }

    ///Returns a subbuffer from the `usage_info_pool` to be used when adding a buffer to a set
    fn get_usage_info_subbuffer(&self) ->
     vulkano::buffer::cpu_pool::CpuBufferPoolSubbuffer<TextureUsageFlags,
     Arc<vulkano::memory::pool::StdMemoryPool>>
     {
        self.usage_info_pool.next(self.texture_usage_info.clone())
    }

    ///Returns a subbuffer from the material_factor_pool to be used with the 3rd set
    fn get_material_factor_subbuffer(&self) ->
    vulkano::buffer::cpu_pool::CpuBufferPoolSubbuffer<MaterialFactors,
    Arc<vulkano::memory::pool::StdMemoryPool>>
    {
        self.material_factor_pool.next(self.material_factors.clone())
    }

    ///Returns the u_world_set generated by this call based on a `pipeline` in the
    /// `pipeline_manager` of the used `renderer`
    pub fn get_set_01(&self) -> Arc<DescriptorSet + Send + Sync>{
        self.set_01.clone()
    }

    ///Returns the second set which holds the material textures
    pub fn get_set_02(&self) -> Arc<DescriptorSet + Send + Sync>{
        self.set_02.clone()

    }

    pub fn get_set_03(&self) -> Arc<DescriptorSet + Send + Sync>{
        self.set_03.clone()
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
