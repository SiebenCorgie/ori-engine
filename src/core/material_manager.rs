use std::sync::{Mutex,Arc};
use std::collections::HashMap;
use core::material;
use render::pipeline;
use render::pipeline_manager;

///Handles all available materials
pub struct MaterialManager {
    //TODO comapare if a Vec<material> + search algorith would be faster
    material_vault: HashMap<String, Arc<Mutex<material::Material>>>,
}

impl MaterialManager {
    ///Creates the manager with a default `fallback` material
    pub fn new(pipeline_manager: Arc<Mutex<pipeline_manager::PipelineManager>>)->Self{
        
        let pipeline_copy = pipeline_manager.clone();

        if !(*pipeline_copy).lock().expect("Failed to lock pipeline manager in material manager creation").has_pipeline("DefaultPipeline"){
            println!("Oups, this programm has no default pipeline, PANIC!", );
            panic!();
        }
        println!("Checked pipeline for default pipeline in material manager creation", );
        //Creates a fallback material to which the programm can fallback in case of a "materal not found"
        let fallback_material = Arc::new(Mutex::new(material::Material::new("fallback", "DefaultPipeline")));
        let mut tmp_map = HashMap::new();
        tmp_map.insert(String::from("fallback"), fallback_material);

        MaterialManager{
            material_vault: tmp_map,
        }
    }

    ///Returns the default material of the engine
    fn get_default_material(&mut self) -> Arc<Mutex<material::Material>>{
        self.material_vault.get(&String::from("fallback")).expect("Could not find fallback material, this shouldn't happen, please report this bug").clone()
    }

    ///Returns a metarial-option with this name
    pub fn get_material_by_name(&mut self, name: &str)-> Option<Arc<Mutex<material::Material>>>{
        let mut getter = self.material_vault.get(&String::from(name.clone()));
        match getter{
            Some(material) => return Some(material.clone()),
            None => {
                println!("Could not find material: {}", name.clone());
                return None
            }
        }
        None
    }

    ///Returns a material with this name, or the fallback if it not exists
    pub fn get_material(&mut self, name: &str) -> Arc<Mutex<material::Material>>{
        if self.material_vault.contains_key(&String::from(name.clone())){
            return self.get_material_by_name(name.clone()).expect("The material is in the manager, but unwraping failed").clone();
        }else{
            return self.get_default_material();
        }
    }

    ///Adds a material to this manager
    pub fn add_material(&mut self, material: material::Material){
        self.material_vault.insert(material.get_name(), Arc::new(Mutex::new(material)));
    }
    ///Checks for a material
    pub fn is_available(&self, name: &str) -> bool{
        self.material_vault.contains_key(&String::from(name))
    }

}
