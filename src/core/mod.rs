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
use cgmath::*;
use collision::*;

///If a object implements this trait, it can be used for any bound operation in the engine
pub trait ReturnBoundInfo {
    ///return the max size of its bound
    fn get_bound_max(&self)-> Point3<f32>;
    ///return the min size of its bound
    fn get_bound_min(&self)-> Point3<f32>;
    ///Sets the bound to the new values (in mesh space)
    fn set_bound(&mut self, min: Point3<f32>, max: Point3<f32>);
    ///Returns the vertices of the bounding mesh, good for debuging
    fn get_bound_points(&self)-> Vec<Vector3<f32>>;
}

///The trait every Node Member should implement
///But you can make your own implementation
pub trait NodeMember {
    ///return the max size of its bound
    fn get_bound_max(&self)-> Point3<f32>;
    ///return the min size of its bound
    fn get_bound_min(&self)-> Point3<f32>;
    ///Sets the bound to the new values (in mesh space)
    fn set_bound(&mut self, min: Point3<f32>, max: Point3<f32>);
    ///Returns the vertices of the bounding mesh, good for debuging
    fn get_bound_points(&self)-> Vec<Vector3<f32>>;
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


///tmp trait for intersction
pub trait AABB3Intersection {
    fn intersects(&self, other: &Aabb3<f32>) -> bool;
    fn half_extend(&self) -> Vector3<f32>;
}

impl AABB3Intersection for Aabb3<f32>{
    fn intersects(&self, other: &Aabb3<f32>) -> bool{
        let (a0, a1) = (self.center(), self.half_extend());
        let (b0, b1) = (other.center(), other.half_extend());

        let x = (a0.x - b0.x) <= (a1.x + b1.x);
        let y = (a0.y - b0.y) <= (a1.y + b1.y);
        let z = (a0.z - b0.z) <= (a1.z + b1.z);

        if x && y && z{
            return true;
        }else{
            return false;
        }

        //a1.x > b0.x && a0.x < b1.x && a1.y > b0.y && a0.y < b1.y
    }

    fn half_extend(&self) -> Vector3<f32>{
        let mins = self.min();
        let maxs = self.max();

        let x = (maxs.x - mins.x) / 2.0;
        let y = (maxs.y - mins.y) / 2.0;
        let z = (maxs.z - mins.z) / 2.0;

        Vector3::new(x, y, z)
    }
}
/*
///Temporary AABB intersection check till its mainlined into the collision crate
impl<S: BaseFloat> Discrete<Aabb3<S>> for Aabb3<S> {
    fn intersects(&self, other: &Aabb2<S>) -> bool {
        let (a0, a1) = (self.min(), self.max());
        let (b0, b1) = (other.min(), other.max());

        a1.x > b0.x && a0.x < b1.x && a1.y > b0.y && a0.y < b1.y
    }
}
*/
