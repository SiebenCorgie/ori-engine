///The scene system mod responsible for handling the scene hirachie and handeling
///queries
pub mod simple_scene_system;
///The scene manager manages all available scene, he is tightly packet with the mesh and light manager(todo)
pub mod scene_manager;
///A high level asset manager which makes it easy to add and remove objects from/to the scene
///Graph. It also handles loading objects in a different thread and assiging materials from a material
///manager.
pub mod asset_manager;
///Definition of a default camera, the momvement has to be controlled by a gameplay-controller
pub mod camera;
///Defines all possible light
///Spot, point and directional light so far
pub mod light;
///Defines a normal mesh along with its properties
pub mod mesh;
///Handels all available meshes as well as the scenes created from an import with several meshes
pub mod mesh_manager;
///An empty can be used if a node should not have any content
pub mod empty;
///Defines a material with all it's properties, NOTE: this might switch to a UE4 like
///node based approach in the future.
pub mod material;
///A high level material manager used for creating, managing and destroying the materials
pub mod material_manager;
///Holds many useful information for different kinds of information
pub mod engine_settings;


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
