
use na;
use std::sync::Arc;
use vulkano;

use render::pipeline;
use core::mesh;

/// an importer via the assimp library
pub mod assimp_importer;
/// an small modeule to handle debug, error and warning messages
pub mod message_handler;
/// an small collection of debug settings, should, and can not be changed after copying
pub mod debug_settings;


///A trait every importer should implement
pub trait Importer {
    ///Returns an importer object
    fn new() -> Self;
    ///Returns a full scene Graph from the data at `path`
    fn import(&mut self, path: &str, name: &str, device: Arc<vulkano::device::Device>,
        queue: Arc<vulkano::device::Queue>)
        -> Vec<mesh::Mesh>;
}

///A helper struct for creating the mesh bound
pub struct BoundCreateInfo {
    pub max_x: f32,
    pub max_y: f32,
    pub max_z: f32,

    pub min_x: f32,
    pub min_y: f32,
    pub min_z: f32,
}


impl BoundCreateInfo{
    ///Creates an zero'ed BoundCreateInfo
    pub fn new() -> Self{
        BoundCreateInfo{
            max_x: 0.0,
            max_y: 0.0,
            max_z: 0.0,

            min_x: 0.0,
            min_y: 0.0,
            min_z: 0.0,
        }
    }

    ///Returns the bound min value
    pub fn get_info_min(&mut self) -> na::Point3<f32>{
        na::Point3::new(self.min_x.clone(), self.min_y.clone(), self.min_z.clone())
    }

    ///Returns the bound max value
    pub fn get_info_max(&mut self) -> na::Point3<f32>{
        na::Point3::new(self.max_x.clone(), self.max_y.clone(), self.max_z.clone())
    }

}
