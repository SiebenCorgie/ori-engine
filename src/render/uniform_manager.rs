
use render::pipeline_infos;

use vulkano;

use na;

use std::sync::{Arc,Mutex};

///Handles the public uniforms and an uniform allocator
pub struct UniformManager {
    ///Describes the universal world properties (see `render::pipeline_info`)
    u_world: pipeline_infos::Main,


    ///First uniform buffer pool block, used or model, view and perspecive matrix
    uniform_buffer_pool_01: vulkano::buffer::cpu_pool::CpuBufferPool<pipeline_infos::Main>,
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

        //Create a pool to allocate from
        let tmp_uniform_buffer_pool = vulkano::buffer::cpu_pool::CpuBufferPool::<pipeline_infos::Main>
                                   ::new(device.clone(), vulkano::buffer::BufferUsage::all());


        UniformManager{

            u_world: world,

            uniform_buffer_pool_01: tmp_uniform_buffer_pool,
        }
    }

    ///Returns a subbuffer of the u_world item, can be used to create a u_world_set
    pub fn get_subbuffer_01(&mut self) -> vulkano::buffer::cpu_pool::CpuBufferPoolSubbuffer<pipeline_infos::Main, Arc<vulkano::memory::pool::StdMemoryPool>>{
        self.uniform_buffer_pool_01.next(self.u_world.clone())
    }

    pub fn update(&mut self, new_u_world: pipeline_infos::Main){
        self.u_world = new_u_world;
    }
}
