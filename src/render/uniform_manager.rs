
use render::pipeline_infos;

use vulkano::buffer::cpu_pool::CpuBufferPoolSubbuffer;
use vulkano::buffer::cpu_pool::CpuBufferPool;
use vulkano;

use na;

use std::sync::{Arc,Mutex};

///Handles the public uniforms and an uniform allocator
pub struct UniformManager {
    ///Describes the universal world properties (see `render::pipeline_info`)
    u_world: pipeline_infos::Main,

    u_point_lights: pipeline_infos::PointLightInfo,
    u_directional_lights: pipeline_infos::DirectionlLightInfo,
    u_spot_lights: pipeline_infos::SpotLightInfo,
    u_light_count: pipeline_infos::LightCount,


    ///First uniform buffer pool block, used or model, view and perspecive matrix
    buffer_pool_01_mvp: vulkano::buffer::cpu_pool::CpuBufferPool<pipeline_infos::Main>,

    ///4th uniform buffer pool block, used for point lights
    buffer_pool_02_point: vulkano::buffer::cpu_pool::CpuBufferPool<pipeline_infos::PointLightInfo>,

    ///4th uniform buffer pool block, used for directional lights
    buffer_pool_03_dir: vulkano::buffer::cpu_pool::CpuBufferPool<pipeline_infos::DirectionlLightInfo>,

    ///4th uniform buffer pool block, used for spot lights
    buffer_pool_04_spot: vulkano::buffer::cpu_pool::CpuBufferPool<pipeline_infos::SpotLightInfo>,

    ///4th uniform buffer pool block, used for spot lights
    buffer_pool_05_light_num: vulkano::buffer::cpu_pool::CpuBufferPool<pipeline_infos::LightCount>,
}

//Create a buffer and the pool
//Recreate set in material not pipeline
//
impl UniformManager{
    pub fn new(device: Arc<vulkano::device::Device>) -> Self{

        //Create a uniform buffer with just [[f32; 4]; 4], the buffer will be updated bevore the first loop
        let world = pipeline_infos::Main {
            model : <na::Matrix4<f32>>::identity().into(),
            view : <na::Matrix4<f32>>::identity().into(),
            proj : <na::Matrix4<f32>>::identity().into(),
        };

        let points = pipeline_infos::PointLightInfo{
            l_point: Vec::new(),
        };

        let direct = pipeline_infos::DirectionlLightInfo{
            l_directional: Vec::new(),
        };

        let spots = pipeline_infos::SpotLightInfo{
            l_spot: Vec::new(),
        };

        //init a light count
        let light_count = pipeline_infos::LightCount{
            num_point_lights: 0,
            num_directional_lights: 0,
            num_spot_lights: 0,
        };



        //Create some pools to allocate from
        let tmp_uniform_buffer_pool_01 = CpuBufferPool::<pipeline_infos::Main>::new(
            device.clone(), vulkano::buffer::BufferUsage::all()
        );

        let tmp_uniform_buffer_pool_02 = CpuBufferPool::<pipeline_infos::PointLightInfo>::new(
            device.clone(), vulkano::buffer::BufferUsage::all()
        );

        let tmp_uniform_buffer_pool_03 = CpuBufferPool::<pipeline_infos::DirectionlLightInfo>::new(
            device.clone(), vulkano::buffer::BufferUsage::all()
        );

        let tmp_uniform_buffer_pool_04 = CpuBufferPool::<pipeline_infos::SpotLightInfo>::new(
            device.clone(), vulkano::buffer::BufferUsage::all()
        );

        let tmp_uniform_buffer_pool_05 = CpuBufferPool::<pipeline_infos::LightCount>::new(
            device.clone(), vulkano::buffer::BufferUsage::all()
        );



        UniformManager{

            u_world: world,

            u_point_lights: points,
            u_directional_lights: direct,
            u_spot_lights: spots,
            u_light_count: light_count,

            ///First uniform buffer pool block, used or model, view and perspecive matrix
            buffer_pool_01_mvp: tmp_uniform_buffer_pool_01,

            ///4th uniform buffer pool block, used for point lights
            buffer_pool_02_point: tmp_uniform_buffer_pool_02,

            ///4th uniform buffer pool block, used for directional lights
            buffer_pool_03_dir: tmp_uniform_buffer_pool_03,

            ///4th uniform buffer pool block, used for spot lights
            buffer_pool_04_spot: tmp_uniform_buffer_pool_04,

            buffer_pool_05_light_num: tmp_uniform_buffer_pool_05,
        }
    }

    ///Returns a subbuffer of the u_world item, can be used to create a u_world_set
    pub fn get_subbuffer_01 (&mut self) ->
    CpuBufferPoolSubbuffer<pipeline_infos::Main, Arc<vulkano::memory::pool::StdMemoryPool>>{
        self.buffer_pool_01_mvp.next(self.u_world.clone())
    }

    ///Returns a subbuffer of the u_point_light
    pub fn get_subbuffer_02 (&mut self) ->
    CpuBufferPoolSubbuffer<pipeline_infos::PointLightInfo, Arc<vulkano::memory::pool::StdMemoryPool>>{
        self.buffer_pool_02_point.next(self.u_point_lights.clone())
    }

    ///Returns a subbuffer of the u_directional_light
    pub fn get_subbuffer_03 (&mut self) ->
    CpuBufferPoolSubbuffer<pipeline_infos::DirectionlLightInfo, Arc<vulkano::memory::pool::StdMemoryPool>>{
        self.buffer_pool_03_dir.next(self.u_directional_lights.clone())
    }

    ///Returns a subbuffer of the u_spot_light
    pub fn get_subbuffer_04 (&mut self) ->
    CpuBufferPoolSubbuffer<pipeline_infos::SpotLightInfo, Arc<vulkano::memory::pool::StdMemoryPool>>{
        self.buffer_pool_04_spot.next(self.u_spot_lights.clone())
    }

    ///Returns a subbuffer of the u_spot_light
    pub fn get_subbuffer_05 (&mut self) ->
    CpuBufferPoolSubbuffer<pipeline_infos::LightCount, Arc<vulkano::memory::pool::StdMemoryPool>>{
        self.buffer_pool_05_light_num.next(self.u_light_count.clone())
    }

    ///Updates the internal data used for the uniform buffer creation
    pub fn update(
        &mut self, new_u_world: pipeline_infos::Main,
        new_point: pipeline_infos::PointLightInfo,
        new_dir: pipeline_infos::DirectionlLightInfo,
        new_spot: pipeline_infos::SpotLightInfo,
        new_light_count: pipeline_infos::LightCount,
    ){
        self.u_world = new_u_world;
        self.u_point_lights = new_point;
        self.u_directional_lights = new_dir;
        self.u_spot_lights = new_spot;
        self.u_light_count = new_light_count;
    }
}
