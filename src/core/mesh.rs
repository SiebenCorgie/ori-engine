//TODO Add command buffer creation per mesh
use std::sync::Arc;
///Defines a normal mesh along with its properties
use na;
use nc;

use vulkano;
//use vulkano::impl_vertex;

use core;
use core::NodeMember;
use core::material;
use render::pipeline;

///Defines the information a Vertex should have
#[derive(Clone,Copy)]
pub struct Vertex {
    position: [f32; 3],
    tex_coord: [f32; 2],
    normal: [f32; 3],
    tangent: [f32; 3],
    color: [f32; 3],
}

unsafe impl Send for Vertex {}
unsafe impl Sync for Vertex {}

//Implements the vulkano::vertex trait on Vertex
impl_vertex!(Vertex, position, tex_coord, normal, tangent, color);

//TODO
//Every mesh needs its own indice and vertex buffer plus its pipeline to be drawn

impl Vertex{
    ///Creates a new Vertex
    pub fn new(
        position: [f32; 3],
        tex_coord: [f32; 2],
        normal: [f32; 3],
        tangent: [f32; 3],
        color: [f32; 3]
        ) -> Self
    {
        Vertex{
            position: position,
            tex_coord: tex_coord,
            normal: normal,
            tangent: tangent,
            color: color,
        }
    }
}

#[derive(Clone)]
pub struct Mesh {
    pub name: String,

    ///Holds the raw vertices of this mesh
    vertices: Vec<Vertex>,
    ///Holds the vulkan buffer, gets change if the vertices change
    vertex_buffer: Arc<vulkano::buffer::BufferAccess + Send + Sync>,

    indices: Vec<u32>,

    material: String,

    bound: nc::bounding_volume::AABB<na::Point3<f32>>,
}

impl Mesh {
    ///Returns the Member with the passed `name`
    ///Special parameters light radius or color will have to be set later
    pub fn new(name: &str, device: Arc<vulkano::device::Device>,
        queue: Arc<vulkano::device::Queue>)
        ->Self{
        //Creating the box extend from the location, there might be a better way
        let min = na::Point3::new(0.5, 0.5, 0.5);
        let max = na::Point3::new(0.5, 0.5, 0.5);

        let mut vertices: Vec<Vertex> = Vec::new();
        vertices.push(Vertex::new([0.0; 3], [0.0; 2], [0.0; 3], [0.0; 3], [0.0; 3]));

        let sample_vertex_buffer = vulkano::buffer::cpu_access::CpuAccessibleBuffer
                                    ::from_iter(device.clone(), vulkano::buffer::BufferUsage::all(), Some(queue.family()), vertices.iter().cloned())
                                    .expect("failed to create buffer");

        Mesh{
            name: String::from(name),

            //TODO Create a persistend vertex and indice buffer
            vertices: vertices,
            vertex_buffer: sample_vertex_buffer,

            indices: Vec::new(),

            material: String::from("fallback"),

            bound: nc::bounding_volume::AABB::new(min, max),
        }
    }

    ///Sets the vertex and indice buffer to a new set of `Vertex` and `u32` indices
    ///The device and queue are needed for rebuilding the buffer
    pub fn set_vertices_and_indices(&mut self, vertices: Vec<Vertex>, indices: Vec<u32>,
        device: Arc<vulkano::device::Device>,
        queue: Arc<vulkano::device::Queue>){
        self.vertices = vertices;
        //Rebuild vertex buffer with new vertices
        self.re_create_vertex_buffer(device.clone(), queue.clone());
        self.indices = indices;
    }

    ///Returns the name of the material this mesh uses
    pub fn get_material_name(&self) -> String{
        self.material.clone()
    }

    ///Can be used to set to a new material
    pub fn set_material(&mut self, new_mat: &str){
        self.material = String::from(new_mat);
    }

    ///Returns all indices
    pub fn get_indices(&self) -> Vec<u32>{
        self.indices.clone()
    }

    ///Return all vertices
    pub fn get_all_vertices(&self) -> Vec<Vertex>{
        self.vertices.clone()
    }

    ///Returns all pos data
    pub fn get_all_positions(&self)-> Vec<[f32; 3]>{
        let mut return_vec = Vec::new();
        for i in self.vertices.iter(){
            return_vec.push(i.position);
        }
        return_vec
    }

    ///Returns all pos data
    pub fn get_all_uvs(&self)-> Vec<[f32; 2]>{
        let mut return_vec = Vec::new();
        for i in self.vertices.iter(){
            return_vec.push(i.tex_coord);
        }
        return_vec
    }

    ///Returns all pos data
    pub fn get_all_normals(&self)-> Vec<[f32; 3]>{
        let mut return_vec = Vec::new();
        for i in self.vertices.iter(){
            return_vec.push(i.normal);
        }
        return_vec
    }

    ///Returns all pos data
    pub fn get_all_tangents(&self)-> Vec<[f32; 3]>{
        let mut return_vec = Vec::new();
        for i in self.vertices.iter(){
            return_vec.push(i.tangent);
        }
        return_vec
    }

    ///Returns all pos data
    pub fn get_all_colors(&self)-> Vec<[f32; 3]>{
        let mut return_vec = Vec::new();
        for i in self.vertices.iter(){
            return_vec.push(i.color);
        }
        return_vec
    }

    ///Returns the current vertex buffer of this mesh
    pub fn get_vertex_buffer(&self) -> Vec<Arc<vulkano::buffer::BufferAccess + Send + Sync>>{

        let mut return_vec = Vec::new();
        return_vec.push(self.vertex_buffer.clone());
        return_vec

    }

    ///Recreates the vertex buffer from a specified device and queue
    pub fn re_create_vertex_buffer(&mut self, device: Arc<vulkano::device::Device>,
        queue: Arc<vulkano::device::Queue>)
    {
        let vertex_buffer = vulkano::buffer::cpu_access::CpuAccessibleBuffer
                                    ::from_iter(device.clone(), vulkano::buffer::BufferUsage::all(), Some(queue.family()), self.vertices.iter().cloned())
                                    .expect("failed to create buffer");
        //self.vertex_buffer = vertex_buffer;
        self.vertex_buffer = vertex_buffer;
    }

    ///Returns a index bufffer for this mesh
    pub fn get_index_buffer(&self, device: Arc<vulkano::device::Device>,
        queue: Arc<vulkano::device::Queue>) ->
        Arc<vulkano::buffer::cpu_access::CpuAccessibleBuffer<[u32]>>
    {
        vulkano::buffer::cpu_access::CpuAccessibleBuffer
            ::from_iter(device.clone(), vulkano::buffer::BufferUsage::all(), Some(queue.family()), self.indices.iter().cloned())
            .expect("failed to create index buffer 02")
    }
/*
    ///Returns the current pipeline of this object
    pub fn get_pipeline() -> Arc<vulkano::pipeline::GraphicsPipeline>{
        self.pipeline.clone()
    }
*/

}

///NodeMember for Mesh
impl NodeMember for Mesh{

    ///return the max size of its bound
    fn get_bound_max(&self)-> &na::Point3<f32>{
        self.bound.maxs()
    }
    ///return the min size of its bound
    fn get_bound_min(&self)-> &na::Point3<f32>{
        self.bound.mins()
    }
    ///Sets the bound to the new values (in mesh space)
    fn set_bound(&mut self, min: na::Point3<f32>, max: na::Point3<f32>){
        self.bound = nc::bounding_volume::AABB::new(min, max);
    }

    /*
    ///Set the location of this type and rebuild it
    fn set_location(&mut self, new_location: na::Vector3<f32>){
        let bound_min = self.bound.mins().clone();
        let bound_max = self.bound.maxs().clone();
        let old_location = self.location.clone();

        let obj_space_min = na::Point3::new(
            bound_min[0] - old_location[0],
            bound_min[1] - old_location[1],
            bound_min[2] - old_location[2]
        );

        let obj_space_max = na::Point3::new(
            bound_max[0] - old_location[0],
            bound_max[1] - old_location[1],
            bound_max[2] - old_location[2],
        );

        self.location = new_location;
        self.bound = nc::bounding_volume::AABB::new(
            //min
            na::Point3::new(
                obj_space_min[0] + new_location[0],
                obj_space_min[1] + new_location[1],
                obj_space_min[2] + new_location[2],
            ),
            //max
            na::Point3::new(
                obj_space_max[0] + new_location[0],
                obj_space_max[1] + new_location[1],
                obj_space_max[2] + new_location[2],
            )
        );
    }
    */
    ///Returns the vertices of the bounding mesh, good for debuging
    fn get_bound_points(&mut self)-> Vec<na::Vector3<f32>>{
        let mut return_vector = Vec::new();

        let b_min = self.bound.mins().clone();
        let b_max = self.bound.maxs().clone();

        ///low
        return_vector.push(na::Vector3::new(b_min[0], b_min[1], b_min[2])); //Low
        return_vector.push(na::Vector3::new(b_min[0] + b_max[0], b_min[1], b_min[2])); //+x
        return_vector.push(na::Vector3::new(b_min[0], b_min[1] + b_max[1], b_min[2])); //+y
        return_vector.push(na::Vector3::new(b_min[0],  b_min[1], b_min[2] + b_max[2])); // +z
        return_vector.push(na::Vector3::new(b_min[0] + b_max[0], b_min[1] + b_max[1], b_min[2])); //+xy
        return_vector.push(na::Vector3::new(b_min[0] + b_max[0], b_min[1], b_min[2] + b_max[2])); //+xz
        return_vector.push(na::Vector3::new(b_min[0] , b_min[1] + b_max[1], b_min[2] + b_max[1])); //+yz
        return_vector.push(na::Vector3::new(b_min[0] + b_max[0], b_min[1] + b_max[1], b_min[2] + b_max[2])); //+xyz

        return_vector
    }
}
