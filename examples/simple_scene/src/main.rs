extern crate vulkano;
extern crate ori_engine;
use ori-engine::*;
use std::sync::{Arc, Mutex};

fn main() {

    //Start a renderer
    let mut render = Arc::new(Mutex::new(render::renderer::Renderer::new()));
    let mut asset_manager = core::asset_manager::AssetManager::new(render.clone());

    asset_manager.import_scene("Ape", "Apes.fbx");
    asset_manager.add_scene_to_main_scene("Ape");

    let mut adding_status = false;

    loop {

        //Add the ape scene if finished loading. This will be managed by a defined loader later
        if adding_status == false && asset_manager.has_scene("Ape"){
            asset_manager.add_scene_to_main_scene("Ape");
            adding_status = true;
        }

        let render_instance = render.clone();
        (*render).lock().expect("Failed to lock renderer for rendering").render(&mut asset_manager);
    }

    //TODO why no mesh?


    println!("Hello, world!");
}
