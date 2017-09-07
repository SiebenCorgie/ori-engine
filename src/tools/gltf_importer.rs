use core::resources::texture;
use core::resources::mesh;
use core::resources::light;
use core::resources::camera;
use core::simple_scene_system::node;
use core::resource_management::{material_manager, mesh_manager, scene_manager, texture_manager};

use vulkano;

use gltf;
use gltf_importer;
use image;

use std::path::Path;
use std::sync::{Arc, Mutex};
///An Import struct used to store information needed to create all meshes, textures etc.
pub struct GltfImporter {
    root_node: node::GenericNode,
    device: Arc<vulkano::device::Device>,
    queue: Arc<vulkano::device::Queue>
}

//TODO everything into struct which also holds a copy of the texture, material, mesh and scene
//manager for easy adding etc.

///Imports a gltf texture
pub fn load_gltf_texture(){

}

///Loads a gltf node into the right node::GenericNode
pub fn load_gltf_node(
    gltf: &gltf::Gltf,
    node: &gltf::Node,
    buffers: &gltf_importer::Buffers,
)// -> node::GenericNode
{

}

///Imports a scene from the file at `path`
pub fn import_gltf(path: &str, name: &str){
    //load the gltf model into a gltf object
    let path = Path::new(path);
    //a default path if `path` doesn't exist, should load a default object in future
    let default = Path::new("");
    //go to the parent directory and load every gltf in this directory
    let base = path.parent().unwrap_or(default);
    //TODO don't panic, load a debug object
    let (gltf, buffers) = gltf_importer::import(path).expect("invalid model for gltf 2.0 loader");


    let mut scene_tree = node::GenericNode::new_empty(name);

    for scene in gltf.scenes(){

    }
}
