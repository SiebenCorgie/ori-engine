///The scene system mod responsible for handling the scene hirachie and handeling
///queries
pub mod simple_scene_system;
///Holds many useful information for different kinds of information
pub mod engine_settings;
///the resource manager is a abstraction to group al management systems together, it is
///not an actuall system
pub mod resource_management;
///Resources holds all loadable resources, they should ussually be managed though one
///of the management systems in `core::resource_management`.
pub mod resources;

use na;
use nc;


///The trait every Node Member should implement
///But you can make your own implementation
pub trait NodeMember {
    ///return the max size of its bound
    fn get_bound_max(&self)-> &na::Point3<f32>;
    ///return the min size of its bound
    fn get_bound_min(&self)-> &na::Point3<f32>;
    ///Sets the bound to the new values (in mesh space)
    fn set_bound(&mut self, min: na::Point3<f32>, max: na::Point3<f32>);
    ///Returns the vertices of the bounding mesh, good for debuging
    fn get_bound_points(&mut self)-> Vec<na::Vector3<f32>>;
}
