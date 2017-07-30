extern crate vulkano;
extern crate ori_engine;
use ori_engine::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {

    //Start

    //Settings
    let settings = Arc::new(Mutex::new(core::engine_settings::EngineSettings::new()
    .with_dimensions(800, 600)
    .with_name("Teddy the bear")));

    //Input
    let mut input_handler = input::Input::new();
    //Create a renderer with the input system
    let mut render = Arc::new(Mutex::new(render::renderer::Renderer::new(input_handler.get_events_loop(), settings.clone())));
    //Create a asset manager for the renderer
    let mut asset_manager = core::asset_manager::AssetManager::new(render.clone(), settings.clone());

    ///Start the input thread
    input_handler.start();

    //Import the ape
    asset_manager.import_scene("Ape", "Apes.fbx");
    asset_manager.import_scene("Ape_02", "Apes.fbx");
    asset_manager.import_scene("Ape_03", "Apes.fbx");

    let mut adding_status = false;

    loop {

        //Add the ape scene if finished loading. This will be managed by a defined loader later
        if adding_status == false && asset_manager.has_scene("Ape"){
            asset_manager.add_scene_to_main_scene("Ape");
            adding_status = true;
        }

        //Render the scene, this will be offloaded to a render thread later
        let render_instance = render.clone();
        (*render).lock().expect("Failed to lock renderer for rendering").render(&mut asset_manager);

        ///Check if loop should close
        let input_inst = input_handler.key_map.clone();
        let input_lck = input_inst.lock().expect("Failed to lock keymap while reading");

        if input_lck.closed{
            println!("Ending loop", );
            input_handler.end();
            break;
        }

    }
}
