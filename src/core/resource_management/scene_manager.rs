use core::simple_scene_system::node;

use std::sync::{Arc, Mutex};

///has a list of all available scenes
pub struct SceneManager {
    scenes: Vec<Arc<Mutex<node::GenericNode>>>,
}

impl SceneManager {
    pub fn new() -> Self{
        SceneManager{
            scenes: Vec::new(),
        }
    }

    //Adds a scene to the scene manager
    pub fn add_scene(&mut self, scene: node::GenericNode){
        self.scenes.push(Arc::new(Mutex::new(scene)));
    }

    ///Returns Some(scene) by name from the `scenes` Vector
    pub fn get_scene(&mut self, name: &str) -> Option<Arc<Mutex<node::GenericNode>>>{

        for i in self.scenes.iter(){

            let scene_lck = i.lock().expect("failed to lock scene in scene Manager");

            if (*scene_lck).name == String::from(name.clone()){
                return Some(i.clone());
            }
        }
        None
    }

    ///Returns the scenes vector as a copy
    pub fn get_scenes_copy(&self) -> Vec<Arc<Mutex<node::GenericNode>>>{
        self.scenes.clone()
    }

    ///Returns true if a scene with `name` as name exists in the `self.scenes` vector
    pub fn has_scene(&self, name: &str) -> bool{

        let mut return_value = false;

        for scene in  self.scenes.iter(){
            let scene_lck = scene.lock().expect("failed to lock scene while testing");
            if (*scene_lck).name == String::from(name.clone()){
                return_value = true;
            }

        }

        return_value
    }
}
