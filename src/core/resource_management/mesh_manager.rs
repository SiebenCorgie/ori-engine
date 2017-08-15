use core::resources::mesh;
use core::simple_scene_system::node;
use tools::Importer;
use tools::assimp_importer;

use vulkano;

use std::sync::{Arc, Mutex};
use std::thread;

///The structure containing all meshes and created scenes
pub struct MeshManager {
    meshes: Arc<Mutex<Vec<Arc<Mutex<mesh::Mesh>>>>>,

}

impl MeshManager {
    pub fn new() -> Self{
        MeshManager{
            meshes: Arc::new(Mutex::new(Vec::new())),
        }
    }

    ///Adds a mesh to the manager
    pub fn add_mesh(&mut self, mesh: mesh::Mesh){

        let meshes_instance = self.meshes.clone();
        {
            (*meshes_instance).lock().expect("Failed to hold while adding mesh to mesh manager")
            .push(Arc::new(Mutex::new(mesh)));
        }

    }

    ///Imports a mesh in a seperate thread.
    ///This will do two things:
    ///
    /// 1st. Import all sub meshes of this file in seperate `Arc<Mutex<Mesh>>` objects
    ///
    /// 2nd. Create a scene with all meshes stack as children below the root node
    ///
    /// By doing this the sub.meshes can be reused to create new scene and a complex scene with
    /// different objects stays in one sub-scene
    pub fn import_mesh(&mut self, name: &str, path: &str, device: Arc<vulkano::device::Device>,
        queue: Arc<vulkano::device::Queue>,
        scene_manager_scenes: Arc<Mutex<Vec<node::GenericNode>>>
    )
    {

        let mut meshes_instance = self.meshes.clone();
        let mut scene_manager_instance = scene_manager_scenes.clone();
        let device_instance = device.clone();
        let queue_instance = queue.clone();
        let name_instance = name.to_owned();
        let path_instance = path.to_owned();

        let thread = thread::spawn(move ||{

            //println!("STATUS: MESH_MANAGER: Spawned thread with id: {:?}", thread::current().id());

            let mut importer = assimp_importer::AssimpImporter::new();
            let new_meshes = importer.import(&path_instance, &name_instance, device_instance.clone(), queue_instance.clone());

            //Convert the meshes to Arc<Mutex<mesh::Mesh>>

            let mut arc_meshes = Vec::new();
            for mesh in new_meshes.iter(){
                arc_meshes.push(Arc::new(Mutex::new(mesh.clone())));
            }


            //Now add the mesh[s] to the meshes vector and after that build a scene from it and at the scene to
            //the scenes Vec

            {
                let mut meshes_editor = (*meshes_instance).lock().expect("failed to lock meshes vec");
                for mesh in arc_meshes.iter(){
                    meshes_editor.push(mesh.clone());
                }
            }

            //now lock the scene Vec and add a scene with an empty root with the name of this mesh
            //println!("STATUS: MESH_MANAGER: Adding scene with name: {}", &name_instance.clone());
            let mut root_node = node::GenericNode::new_empty(&name_instance.clone());
            for i in arc_meshes.iter(){
                let mesh_node = node::ContentTypes::StaticMesh(i.clone());
                root_node.add_child(mesh_node);
            }

            //now lock the scene reference and add the subscene

            {
                (*scene_manager_instance).lock().expect("failed to lock the scene manager reference while importing")
                .push(root_node);
            }


            //println!("STATUS: MESH_MANAGER: Finshed importing {}", name_instance.clone());
        });

    }
}
