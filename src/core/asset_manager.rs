use std::sync::{Mutex, Arc};
use std::thread;

use core::simple_scene_system::node;
use core::material_manager;
use core::light;
use core::mesh;
use core::mesh_manager;
use tools::assimp_importer;
use tools::Importer;
use core::scene_manager;
use core::camera::Camera;
use core::camera::DefaultCamera;

use render::renderer;

use vulkano;

///The main struct for the scene manager
///It is responible for handling the materials and scenes as well as the assets
pub struct AssetManager {
    ///Holds the current active scene
    active_main_scene: node::GenericNode,
    ///Holds the current material manager
    material_manager: material_manager::MaterialManager,

    mesh_manager: mesh_manager::MeshManager,

    scene_manager: scene_manager::SceneManager,

    ///Holds a reference to the renderer
    renderer: Arc<Mutex<renderer::Renderer>>,

    ///A Debug camera, will be removed in favor of a camera_managemant system
    camera: DefaultCamera,



}

impl  AssetManager {
    ///Creates a new idependend scene manager
    pub fn new(renderer: Arc<Mutex<renderer::Renderer>>)->Self{

        let camera = DefaultCamera::new();

        //Make a nice copy so we can retrive the pipeline manager
        let renderer_instance = renderer.clone();
        let pipeline_instance = (*renderer_instance).lock().expect("Failed to hold the lock while getting the pipeline instance").get_pipeline_manager().clone();

        AssetManager{
            active_main_scene: node::GenericNode::new_empty("Empty"),
            material_manager: material_manager::MaterialManager::new(pipeline_instance),
            mesh_manager: mesh_manager::MeshManager::new(),
            scene_manager: scene_manager::SceneManager::new(),
            renderer: renderer_instance,
            camera: camera,
        }
    }

    ///Returns the camera in use TODO this will be managed by a independent camera manager in the future
    pub fn get_camera(&mut self) -> &mut DefaultCamera{
        &mut self.camera
    }

    ///Sets the root scene to a `new_scene_root`
    pub fn set_active_scene(&mut self, new_scene_root: node::GenericNode){
        self.active_main_scene = new_scene_root;
    }

    ///Returns a reference to the active scene
    pub fn get_active_scene(&mut self) -> &mut node::GenericNode{
        &mut self.active_main_scene
    }

    ///Starts the asset thread, responsible for managing all assets
    pub fn start_asset_thread(&mut self){

    }

    ///Returns a reference to the material manager
    pub fn get_material_manager(&mut self) -> &mut material_manager::MaterialManager{
        &mut self.material_manager
    }

    //Returns a raw copy of the meshes in the current active scene tree
    pub fn get_all_meshes(&mut self) -> Vec<Arc<Mutex<mesh::Mesh>>>{
        self.active_main_scene.get_all_meshes()
    }

    pub fn get_meshes_in_frustum(&mut self) -> Vec<Arc<Mutex<mesh::Mesh>>>{
        self.active_main_scene.get_meshes_in_frustum(&self.camera)
    }

    ///Imports a new scene from a file at `path` and saves the scene as `name`
    ///The meshes are stored as Arc<Mutex<T>>'s in the mesh manager the scene Is stored in the scene manager
    pub fn import_scene(&mut self, name: &str, path: &str){

        //lock the renderer
        let render_inst = self.renderer.clone();

        let scene_ref_inst = self.scene_manager.get_scenes_reference();
        let device_inst = {(*render_inst).lock().expect("failed to hold renderer lock").get_device().clone()};
        let queue_inst = {(*render_inst).lock().expect("failed to hold renderer lock").get_queue().clone()};


        //Pass the import params an a scene manager instance to the mesh manager
        self.mesh_manager.import_mesh(
            name, path,
            device_inst, queue_inst,
            scene_ref_inst
        );

    }

    ///Adds a scene from the local scene manager to the local main scene
    pub fn add_scene_to_main_scene(&mut self, name: &str){

        let mut scene = self.scene_manager.get_scene(name);

        match scene{
            Some(sc) =>{
                //TODO make this to an Arc<GenericNode>
                self.active_main_scene.add_node(sc.clone());
            },
            None => println!("Could not find scene with name: {}", name.clone()),
        }
    }

    ///Returns true if a scene with `name` as name exists in the local scene manager
    pub fn has_scene(&self, name: &str) -> bool{
        self.scene_manager.has_scene(name.clone())
    }
}


//Created a mesh manager holding all meshes as Arc<Mutex<T>>
//a scenen manger who holdes sub scenes created form imports as well as user created scenes
//The asset manager holds only a currently active scene, know as the player level
