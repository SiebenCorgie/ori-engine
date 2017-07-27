use core::simple_scene_system::node;

use std::sync::{Arc, Mutex};

///has a list of all available scenes
pub struct SceneManager {
    scenes: Arc<Mutex<Vec<node::GenericNode>>>,
}

impl SceneManager {
    pub fn new() -> Self{
        SceneManager{
            scenes: Arc::new(Mutex::new(Vec::new())),
        }
    }

    //Adds a scene to the scene manager
    pub fn add_scene(&self, scene: node::GenericNode){
        //lock and add
        let scenes_instance = self.scenes.clone();
        {
            (*scenes_instance).lock().expect("failed to hold lock while adding scene to scene manager").push(scene);
        }
    }

    ///Returns Some(scene) by name from the `scenes` Vector
    pub fn get_scene(&self, name: &str) -> Option<node::GenericNode>{

        //get a copy
        let scenes_instance = self.scenes.clone();

        for i in (*scenes_instance).lock().expect("could not lock scenes").iter(){
            if i.name == String::from(name.clone()){
                //should be save to clone this because the content in the scenes
                //meshes light etc are Arc<Mutex<T>>
                return Some(i.clone())
            }
        }
        None
    }

    ///Returns the scenes vector
    pub fn get_scenes_reference(&self) -> Arc<Mutex<Vec<node::GenericNode>>>{
        self.scenes.clone()
    }

    ///Returns true if a scene with `name` as name exists in the `self.scenes` vector
    pub fn has_scene(&self, name: &str) -> bool{

        let mut return_value = false;

        let local_scenes = self.scenes.clone();
        for scene in (*local_scenes).lock().expect("Failed to hold lock while testing for a scene in the scene manager").iter(){

            if scene.name == String::from(name.clone()){
                return_value = true;
            }

        }

        return_value
    }
}
