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


use std::sync::{Arc, Mutex};
use na;
use nc;

///If a object implements this trait, it can be used for any bound operation in the engine
pub trait ReturnBoundInfo {
    ///return the max size of its bound
    fn get_bound_max(&self)-> na::Point3<f32>;
    ///return the min size of its bound
    fn get_bound_min(&self)-> na::Point3<f32>;
    ///Sets the bound to the new values (in mesh space)
    fn set_bound(&mut self, min: na::Point3<f32>, max: na::Point3<f32>);
    ///Returns the vertices of the bounding mesh, good for debuging
    fn get_bound_points(&self)-> Vec<na::Vector3<f32>>;
}

///The trait every Node Member should implement
///But you can make your own implementation
pub trait NodeMember {
    ///return the max size of its bound
    fn get_bound_max(&self)-> na::Point3<f32>;
    ///return the min size of its bound
    fn get_bound_min(&self)-> na::Point3<f32>;
    ///Sets the bound to the new values (in mesh space)
    fn set_bound(&mut self, min: na::Point3<f32>, max: na::Point3<f32>);
    ///Returns the vertices of the bounding mesh, good for debuging
    fn get_bound_points(&self)-> Vec<na::Vector3<f32>>;
    ///Returns the name of this node
    fn get_name(&self) -> String;

    ///Returns `Some(Arc<Mutex<mesh>>)` if this NodeMember contains a mesh tagged as static
    fn get_static_mesh(&self) -> Option<Arc<Mutex<resources::mesh::Mesh>>>;
    ///Returns `Some(Arc<Mutex<mesh>>)` if this NodeMember contains a mesh tagged as dynamic
    fn get_dynamic_mesh(&self) -> Option<Arc<Mutex<resources::mesh::Mesh>>>;
    ///Returns `Some(Arc<Mutex<LightPoint>>)` if this NodeMember contains a light point
    fn get_light_point(&self) -> Option<Arc<Mutex<resources::light::LightPoint>>>;
    ///Returns `Some(Arc<Mutex<LightDirectional>>)` if this NodeMember contains a directional light
    fn get_light_directional(&self) -> Option<Arc<Mutex<resources::light::LightDirectional>>>;
    ///Returns `Some(Arc<Mutex<LightSpot>>)` if this NodeMember contains a light spot
    fn get_light_spot(&self) -> Option<Arc<Mutex<resources::light::LightSpot>>>;
    ///Returns Some({}) if this NodeMemeber is an `empty`
    fn get_empty(&self) -> Option<Arc<Mutex<resources::empty::Empty>>>;
    ///Returns an unfiltered `NodeMemberTypes` object which mus be matched by the user
    fn get_inner(&self) -> simple_scene_system::node_member::NodeMemberTypes;

    ///Returns the type of node this is
    fn get_content_type(&self) -> simple_scene_system::node::ContentTag;
}
