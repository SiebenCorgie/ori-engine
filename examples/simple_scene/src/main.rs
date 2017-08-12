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
    .with_name("Teddy the bear")
    .set_vulkan_silent()
    ));

    //Input
    let mut input_handler = input::Input::new(settings.clone());
    //Create a renderer with the input system
    let mut render = Arc::new(
        Mutex::new(
            render::renderer::Renderer::new(
                input_handler.get_events_loop(),
                settings.clone()
            )
        )
    );
    //Create a asset manager for the renderer
    let mut asset_manager = core::asset_manager::AssetManager::new(
        render.clone(),
        settings.clone(),
        input_handler.key_map.clone()
    );

    ///Start the input thread
    input_handler.start();

    //Import the ape
    asset_manager.import_scene("Ape", "Apes.fbx");
    //asset_manager.import_scene("Ape_02", "Apes.fbx");
    //asset_manager.import_scene("Ape_03", "Apes.fbx");

    let mut adding_status = false;

    loop {
        //Add the ape scene if finished loading. This will be managed by a defined loader later
        if adding_status == false && asset_manager.has_scene("Ape"){
            asset_manager.add_scene_to_main_scene("Ape");
            adding_status = true;
            println!("STATUS: GAME: added all apes", );
        }
        println!("STATUS: GAME: Starting loop in game", );
        //Update the content of the render_manager
        asset_manager.update();
        println!("STATUS: GAME: Updated all assets", );
        let render_instance = render.clone();
        (*render).lock().expect("Failed to lock renderer for rendering").render(&mut asset_manager);
        ///Check if loop should close
        println!("STATUS: GAME: Rendered!", );
        let input_inst = input_handler.key_map.clone();
        let input_lck = input_inst.lock().expect("Failed to lock keymap while reading");
        println!("STATUS: GAME: Processed Input", );
        if input_lck.closed{
            println!("STATUS: GAME: Shuting down", );
            input_handler.end();
            break;
        }
    }
}
