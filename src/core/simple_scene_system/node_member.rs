use std::sync::{Arc, Mutex};

use core::resources::mesh;
use core::resources::light;
use core::resources::empty;
use core::NodeMember;
use core::ReturnBoundInfo;
use core::simple_scene_system::node::ContentTag;

use na;

///An internal type which stores a node member and can return the original type which might
///be managed by a different manager
#[derive(Clone)]
pub struct SimpleNodeMember {
    inner: NodeMemberTypes,
    content_tag: ContentTag,
}

///An enum representing all possible `inner` types this node_members can have
#[derive(Clone)]
pub enum NodeMemberTypes {
    StaticMesh(Arc<Mutex<mesh::Mesh>>),
    LightPoint(Arc<Mutex<light::LightPoint>>),
    LightDir(Arc<Mutex<light::LightDirectional>>),
    LightSpot(Arc<Mutex<light::LightSpot>>),
    Empty(Arc<Mutex<empty::Empty>>),
}


impl SimpleNodeMember{
    ///Creates a new `SImpleNodeMember` from a supplied mesh
    pub fn from_static_mesh(mesh: Arc<Mutex<mesh::Mesh>>) -> Self{
        let inner = NodeMemberTypes::StaticMesh(mesh);
        SimpleNodeMember{
            inner: inner,
            content_tag: ContentTag::StaticMesh,
        }
    }

    ///Creates a new `SimpleNodeMember` from a supplied mesh
    pub fn from_light_point(light: Arc<Mutex<light::LightPoint>>) -> Self{
        let inner = NodeMemberTypes::LightPoint(light);
        SimpleNodeMember{
            inner: inner,
            content_tag: ContentTag::LightPoint,
        }
    }

    ///Creates a new `SimpleNodeMember` from a supplied mesh
    pub fn from_light_directional(light: Arc<Mutex<light::LightDirectional>>) -> Self{
        let inner = NodeMemberTypes::LightDir(light);
        SimpleNodeMember{
            inner: inner,
            content_tag: ContentTag::LightDirectional,
        }
    }

    ///Creates a new `SimpleNodeMember` from a supplied mesh
    pub fn from_light_spot(light: Arc<Mutex<light::LightSpot>>) -> Self{
        let inner = NodeMemberTypes::LightSpot(light);
        SimpleNodeMember{
            inner: inner,
            content_tag: ContentTag::LightSpot,
        }
    }

    ///Creates a new `SimpleNodeMember` from a supplied mesh
    pub fn from_empty(empty: Arc<Mutex<empty::Empty>>) -> Self{
        let inner = NodeMemberTypes::Empty(empty);
        SimpleNodeMember{
            inner: inner,
            content_tag: ContentTag::Empty,
        }
    }
}


impl NodeMember for SimpleNodeMember{
    ///return the max size of its bound
    fn get_bound_max(&self)-> na::Point3<f32>{
        //match inner to return the right
        match self.inner{
            NodeMemberTypes::StaticMesh(ref mesh) => {
                let mesh_inst = mesh.clone();
                let mesh_lck = mesh_inst.lock().expect("failed to lock mesh");

                return (*mesh_lck).get_bound_max();
            },

            NodeMemberTypes::LightPoint(ref light) => {
                let light_inst = light.clone();
                let light_lck = light_inst.lock().expect("failed to lock mesh");

                return (*light_lck).get_bound_max();
            },

            NodeMemberTypes::LightDir(ref light) => {
                let light_inst = light.clone();
                let light_lck = light_inst.lock().expect("failed to lock mesh");

                return (*light_lck).get_bound_max();
            },

            NodeMemberTypes::LightSpot(ref light) => {
                let light_inst = light.clone();
                let light_lck = light_inst.lock().expect("failed to lock mesh");

                return (*light_lck).get_bound_max();
            },

            NodeMemberTypes::Empty(ref empty) => {
                let empty_inst = empty.clone();
                let empty_lck = empty_inst.lock().expect("failed to lock mesh");

                return (*empty_lck).get_bound_max();
            },

        }
    }
    ///return the min size of its bound
    fn get_bound_min(&self)-> na::Point3<f32>{
        //match inner to return the right
        match self.inner{
            NodeMemberTypes::StaticMesh(ref mesh) => {
                let mesh_inst = mesh.clone();
                let mesh_lck = mesh_inst.lock().expect("failed to lock mesh");

                return (*mesh_lck).get_bound_min();
            },

            NodeMemberTypes::LightPoint(ref light) => {
                let light_inst = light.clone();
                let light_lck = light_inst.lock().expect("failed to lock mesh");

                return (*light_lck).get_bound_min();
            },

            NodeMemberTypes::LightDir(ref light) => {
                let light_inst = light.clone();
                let light_lck = light_inst.lock().expect("failed to lock mesh");

                return (*light_lck).get_bound_min();
            },

            NodeMemberTypes::LightSpot(ref light) => {
                let light_inst = light.clone();
                let light_lck = light_inst.lock().expect("failed to lock mesh");

                return (*light_lck).get_bound_min();
            },

            NodeMemberTypes::Empty(ref empty) => {
                let empty_inst = empty.clone();
                let empty_lck = empty_inst.lock().expect("failed to lock mesh");

                return (*empty_lck).get_bound_min();
            },

        }
    }
    ///Sets the bound to the new values (in mesh space)
    fn set_bound(&mut self, min: na::Point3<f32>, max: na::Point3<f32>){

    }
    ///Returns the vertices of the bounding mesh, good for debuging
    fn get_bound_points(&self)-> Vec<na::Vector3<f32>>{
        //match inner to return the right
        match self.inner{
            NodeMemberTypes::StaticMesh(ref mesh) => {
                let mesh_inst = mesh.clone();
                let mesh_lck = mesh_inst.lock().expect("failed to lock mesh");

                return (*mesh_lck).get_bound_points();
            },

            NodeMemberTypes::LightPoint(ref light) => {
                let light_inst = light.clone();
                let light_lck = light_inst.lock().expect("failed to lock mesh");

                return (*light_lck).get_bound_points();
            },

            NodeMemberTypes::LightDir(ref light) => {
                let light_inst = light.clone();
                let light_lck = light_inst.lock().expect("failed to lock mesh");

                return (*light_lck).get_bound_points();
            },

            NodeMemberTypes::LightSpot(ref light) => {
                let light_inst = light.clone();
                let light_lck = light_inst.lock().expect("failed to lock mesh");

                return (*light_lck).get_bound_points();
            },

            NodeMemberTypes::Empty(ref empty) => {
                let empty_inst = empty.clone();
                let empty_lck = empty_inst.lock().expect("failed to lock mesh");

                return (*empty_lck).get_bound_points();
            },
        }
    }
    ///Returns the name of this node
    fn get_name(&self) -> String{
        //match inner to return the right
        match self.inner{
            NodeMemberTypes::StaticMesh(ref mesh) => {
                let mesh_inst = mesh.clone();
                let mesh_lck = mesh_inst.lock().expect("failed to lock mesh");

                return (*mesh_lck).name.clone();
            },

            NodeMemberTypes::LightPoint(ref light) => {
                let light_inst = light.clone();
                let light_lck = light_inst.lock().expect("failed to lock mesh");

                return (*light_lck).name.clone();
            },

            NodeMemberTypes::LightDir(ref light) => {
                let light_inst = light.clone();
                let light_lck = light_inst.lock().expect("failed to lock mesh");

                return (*light_lck).name.clone();
            },

            NodeMemberTypes::LightSpot(ref light) => {
                let light_inst = light.clone();
                let light_lck = light_inst.lock().expect("failed to lock mesh");

                return (*light_lck).name.clone();
            },

            NodeMemberTypes::Empty(ref empty) => {
                let empty_inst = empty.clone();
                let empty_lck = empty_inst.lock().expect("failed to lock mesh");

                return (*empty_lck).name.clone();
            },
        }
    }

    ///Returns `Some(Arc<Mutex<mesh>>)` if this NodeMember contains a mesh tagged as static
    fn get_static_mesh(&self) -> Option<Arc<Mutex<mesh::Mesh>>>{
        //match inner to return the right
        match self.inner{
            NodeMemberTypes::StaticMesh(ref mesh) => {
                return Some(mesh.clone());
            },

            _ => return None,
        }
    }
    ///Returns `Some(Arc<Mutex<mesh>>)` if this NodeMember contains a mesh tagged as dynamic
    fn get_dynamic_mesh(&self) -> Option<Arc<Mutex<mesh::Mesh>>>{
        match self.inner{
            NodeMemberTypes::StaticMesh(ref mesh) => {
                return Some(mesh.clone());
            },

            _ => return None,
        }
    }
    ///Returns `Some(Arc<Mutex<LightPoint>>)` if this NodeMember contains a light point
    fn get_light_point(&self) -> Option<Arc<Mutex<light::LightPoint>>>{
        match self.inner{
            NodeMemberTypes::LightPoint(ref light) => {
                return Some(light.clone());
            },

            _ => return None,
        }
    }
    ///Returns `Some(Arc<Mutex<LightDirectional>>)` if this NodeMember contains a directional light
    fn get_light_directional(&self) -> Option<Arc<Mutex<light::LightDirectional>>>{
        match self.inner{
            NodeMemberTypes::LightDir(ref light) => {
                return Some(light.clone());
            },

            _ => return None,
        }
    }
    ///Returns `Some(Arc<Mutex<LightSpot>>)` if this NodeMember contains a light spot
    fn get_light_spot(&self) -> Option<Arc<Mutex<light::LightSpot>>>{
        match self.inner{
            NodeMemberTypes::LightSpot(ref light) => {
                return Some(light.clone());
            },

            _ => return None,
        }
    }
    ///Returns Some({}) if this NodeMemeber is an `empty`
    fn get_empty(&self) -> Option<Arc<Mutex<empty::Empty>>>{
        match self.inner{
            NodeMemberTypes::Empty(ref empty) => {
                return Some(empty.clone());
            },

            _ => return None,
        }
    }
    ///Returns an unfiltered `NodeMemberTypes` object which must be matched by the user
    fn get_inner(&self) -> NodeMemberTypes{
        self.inner.clone()
    }

    ///Returns the type of node this is
    fn get_content_type(&self) -> ContentTag{
        self.content_tag.clone()
    }
}
