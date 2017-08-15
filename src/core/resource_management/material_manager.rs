use std::sync::{Mutex,Arc};
use std::collections::HashMap;
use core::material;
use render;
use render::pipeline;
use render::pipeline_manager;

///Handles all available materials
pub struct MaterialManager {
    //TODO comapare if a Vec<material> + search algorith would be faster
    material_vault: HashMap<String, Arc<Mutex<material::Material>>>,
    renderer_inst: Arc<Mutex<render::renderer::Renderer>>,
}

impl MaterialManager {
    ///Creates the manager with a default `fallback` material
    pub fn new(render: Arc<Mutex<render::renderer::Renderer>>)->Self{

        //We'll have to check for a default pipeline, otherwise the Manager creation could fail
        let render_inst = render.clone();
        {
            let mut render_lck = render_inst.lock().expect("Failed to lock renderer");

            let pipeline_copy = (*render_lck).get_pipeline_manager().clone();
            {
                if !(*pipeline_copy).lock()
                    .expect("Failed to lock pipeline manager in material manager creation")
                    .has_pipeline("DefaultPipeline")
                {
                    //println!("STATUS: MATERIAL_MANAGER: Oups, this programm has no default pipeline, PANIC!", );
                    panic!("this engine has no default pipeline :(");
                }
            }
        }


        //println!("STATUS: MATERIAL_MANAGER: Checked pipeline for default pipeline in material manager creation", );
        //Creates a fallback material to which the programm can fallback in case of a "materal not found"

        let mut render_lck = render_inst.lock().expect("Failed to lock renderer");

        let (pipe_man, uni_man, device, queue) = (*render_lck).get_material_instances();

        let fallback_material = Arc::new(Mutex::new(
        {
            let mut tmp_material = material::Material::new(
                        "fallback",
                        "DefaultPipeline",
                        pipe_man,
                        uni_man,
                        device,
                        queue
                    );
            tmp_material.set_albedo_texture("/home/siebencorgie/Scripts/Rust/engine/ori-engine/data/fallback_alb.png");
            tmp_material.set_normal_texture("/home/siebencorgie/Scripts/Rust/engine/ori-engine/data/fallback_nrm.png");
            tmp_material.set_physical_texture("/home/siebencorgie/Scripts/Rust/engine/ori-engine/data/fallback_physical.png");
            tmp_material.set_texture_usage_info({
                //Create a temporary info and add all the info needed
                let mut tmp_info = material::TextureUsageFlags::new()
                .with_albedo(1)
                .with_metal(1)
                .with_normal(1)
                .with_roughness(1)
                .with_occlusion(1)
                .with_emissive(1);
                //Return it cor recretion of the whole material
                tmp_info
            });
            tmp_material.set_material_factor_info({
                let mut tmp_fac_info = material::MaterialFactors::new()
                //.with_factor_albedo([1.0, 0.0, 0.5, 1.0])
                ;
                tmp_fac_info
            });

            tmp_material.recreate_static_sets();
            //And finnally return the material to be used in Arc<Mutex<material>>
            tmp_material
        }
        //now make to a Arc<Mutex<Material>>
        ));

        let mut tmp_map = HashMap::new();
        //and finnaly insert
        tmp_map.insert(String::from("fallback"), fallback_material);

        MaterialManager{
            material_vault: tmp_map,
            renderer_inst: render.clone(),
        }
    }

    ///Updates all materials
    pub fn update(&mut self){
        //println!("STATUS: MATERIAL_MANAGER: In material manager", );
        for (k,i) in self.material_vault.iter_mut(){
            let i_inst = i.clone();
            let mut i_lck = i_inst.lock().expect("failed to lock material for updating");
            //println!("STATUS: MATERIAL_MANAGER: Updating: {}", k);
            (*i_lck).update();
        }
    }

    ///Returns the default material of the engine
    fn get_default_material(&mut self) -> Arc<Mutex<material::Material>>{
        self.material_vault.get(&String::from("fallback"))
        .expect("Could not find fallback material, this shouldn't happen, please report this bug")
        .clone()
    }

    ///Returns a metarial-option with this name
    pub fn get_material_by_name(&mut self, name: &str)-> Option<Arc<Mutex<material::Material>>>{
        let mut getter = self.material_vault.get(&String::from(name.clone()));
        match getter{
            Some(material) => return Some(material.clone()),
            None => {
                //println!("STATUS: MATERIAL_MANAGER: Could not find material: {}", name.clone());
                return None
            }
        }
        None
    }

    ///Returns a material with this name, or the fallback if it not exists
    pub fn get_material(&mut self, name: &str) -> Arc<Mutex<material::Material>>{
        if self.material_vault.contains_key(&String::from(name.clone())){
            return self.get_material_by_name(name.clone())
                .expect("The material is in the manager, but unwraping failed")
                .clone();
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
