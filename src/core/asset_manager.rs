use std::sync::{Mutex, Arc};
use std::thread;
use time;

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
use core::engine_settings;

use rt_error;

use render::renderer;
use render::pipeline;
use render::pipeline_manager;
use render::pipeline_infos;

use input::KeyMap;

use na;
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

    settings: Arc<Mutex<engine_settings::EngineSettings>>,

    /// a copy of the keymap to be used for passing to everything gameplay related
    key_map: Arc<Mutex<KeyMap>>,

}

impl  AssetManager {
    ///Creates a new idependend scene manager
    pub fn new(
        renderer: Arc<Mutex<renderer::Renderer>>,
        settings: Arc<Mutex<engine_settings::EngineSettings>>,
        key_map: Arc<Mutex<KeyMap>>,
    )->Self{

        //The camera will be moved to a camera manager
        let camera = DefaultCamera::new(settings.clone(), key_map.clone());

        //Make a nice copy so we can retrive the pipeline manager
        let renderer_instance = renderer.clone();
        let pipeline_instance = (*renderer_instance).lock().expect("Failed to hold the lock while getting the pipeline instance").get_pipeline_manager().clone();


        AssetManager{
            active_main_scene: node::GenericNode::new_empty("Empty"),
            material_manager: material_manager::MaterialManager::new(renderer.clone()),
            mesh_manager: mesh_manager::MeshManager::new(),
            scene_manager: scene_manager::SceneManager::new(),
            renderer: renderer_instance,
            camera: camera,

            settings: settings,

            key_map: key_map.clone(),
        }
    }

    ///Updates all child components
    pub fn update(&mut self){

        println!("STATUS: ASSET_MANAGER: Trying to update", );
        //Update uniform manager
        let render_int = self.renderer.clone();
        let render_lck = render_int.lock().expect("failed to lock renderer");

        //Debug stuff which will be handled by the application later
        let rotation = na::Rotation3::from_axis_angle(&na::Vector3::y_axis(), time::precise_time_ns() as f32 * 0.000000001);
        let mat_4: na::Matrix4<f32> = na::convert(rotation);

        let uniform_data = pipeline_infos::Main {
            model: mat_4.into(),
            view: self.get_camera().get_view_matrix().into(),
            proj: self.get_camera().get_perspective().into(),
        };
        //in scope to prevent dead lock while updating material manager
        {
            let uniform_manager = (*render_lck).get_uniform_manager();
            let mut uniform_manager_lck = uniform_manager.lock().expect("failed to lock uniform_man.");
            (*uniform_manager_lck).update(uniform_data);
        }


        println!("STATUS: ASSET_MANAGER: Now I'll update the materials", );
        //Update materials
        self.material_manager.update();
        println!("STATUS: ASSET_MANAGER: Finished materials", );
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
    ///Might be removed because not neccessary
    pub fn start_asset_thread(&mut self){
        /// NOTE has to be implemented
        return
    }

    ///Returns a reference to the material manager
    pub fn get_material_manager(&mut self) -> &mut material_manager::MaterialManager{
        &mut self.material_manager
    }

    //Returns a raw copy of the meshes in the current active scene tree
    pub fn get_all_meshes(&mut self) -> Vec<Arc<Mutex<mesh::Mesh>>>{
        self.active_main_scene.get_all_meshes()
    }

    ///Returns all meshes in the view frustum of the currently active camera
    pub fn get_meshes_in_frustum(&mut self) -> Vec<Arc<Mutex<mesh::Mesh>>>{
        self.active_main_scene.get_meshes_in_frustum(&self.camera)
    }

    ///Imports a new scene from a file at `path` and saves the scene as `name`
    ///The meshes are stored as Arc<Mutex<T>>'s in the mesh manager the scene Is stored in the scene manager
    pub fn import_scene(&mut self, name: &str, path: &str){

        let render_inst = self.renderer.clone();
        ///Lock in scope to prevent dead lock while importing
        let scene_ref_inst = self.scene_manager.get_scenes_reference();

        let device_inst = {
            (*render_inst).lock().expect("failed to hold renderer lock").get_device().clone()
        };
        let queue_inst = {
            (*render_inst).lock().expect("failed to hold renderer lock").get_queue().clone()
        };


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
            None => rt_error("ASSET_MANAGER", &("Could not find scene with name".to_string() + name.clone()).to_string()),
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
