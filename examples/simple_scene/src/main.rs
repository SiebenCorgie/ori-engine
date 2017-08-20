extern crate vulkano;
extern crate ori_engine;
use ori_engine::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Instant, Duration};

extern crate winit;

fn main() {

    //Start

    //Settings
    let settings = Arc::new(Mutex::new(core::engine_settings::EngineSettings::new()
    .with_dimensions(1200, 720)
    .with_name("Teddy the bear")
    .set_vulkan_silent()
    .with_fullscreen_mode(false)
    .with_cursor_state(winit::CursorState::Grab)
    .with_cursor_visibility(winit::MouseCursor::Arrow)
    ));

    //Input
    let mut input_handler = input::Input::new(settings.clone()).with_polling_speed(60);

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
    let mut asset_manager = core::resource_management::asset_manager::AssetManager::new(
        render.clone(),
        settings.clone(),
        input_handler.key_map.clone()
    );

    //Start the input thread
    input_handler.start();

    //Import the ape
    asset_manager.import_scene("Ape", "Apes.fbx");
    asset_manager.import_scene("Ring", "Ring.fbx");
    //asset_manager.import_scene("Ape_02", "Apes.fbx");
    //asset_manager.import_scene("Ape_03", "Apes.fbx");

    ///Creating a new material, currently a bit ugly
    {
        let render_inst = render.clone();
        let mut render_lck = render_inst.lock().expect("failed to hold renderer");

        //Create a second material
        //create new texture
        let new_texture = core::resources::texture::TextureBuilder::from_image(
            "/share/3DFiles/TextureLibary/Comix/Mariuhana.jpg",
            (*render_lck).get_device(),
            (*render_lck).get_queue(),
            settings.clone()
        ).build_with_name("new_texture");

        asset_manager.get_texture_manager().add_texture(new_texture);

        let (_ , normal, physical) = asset_manager.get_texture_manager().get_fallback_textures();

        let texture_in_manager = asset_manager.get_texture_manager().get_texture("new_texture");

        let (pipe, uni_man, device, queue) = (*render_lck).get_material_instances();

        let mut new_material = core::resources::material::MaterialBuilder::new(
            Some(texture_in_manager),
            Some(normal),
            Some(physical),
            None,
            asset_manager.get_texture_manager().get_none()
        ).build(
            "new_material",
            pipe,
            uni_man,
            device,
            queue,
            settings.clone()
        );

        //add to the manager
        asset_manager.get_material_manager().add_material(new_material);
    }

    let mut adding_status = false;

    let mut start_time = Instant::now();

    loop {
        //Add the ape scene if finished loading. This will be managed by a defined loader later
        if adding_status == false && asset_manager.has_scene("Ape") && asset_manager.has_scene("Ring"){

            let mut ape_scene = asset_manager.get_scene_manager().get_scene("Ape").expect("no Apes :(");
            //let mut Ring = asset_manager.get_scene_manager().get_scene("Ring").expect("no Rings :(");

            for i in ape_scene.get_all_meshes().iter(){
                let mesh_inst = i.clone();
                let mut mesh_lck = mesh_inst.lock().expect("failed to change material");
                (*mesh_lck).set_material("new_material");
            }

            asset_manager.add_scene_to_main_scene("Ape");
            asset_manager.add_scene_to_main_scene("Ring");
            adding_status = true;
            //println!("STATUS: GAME: added all apes", );
        }
        //println!("STATUS: GAME: Starting loop in game", );
        //Update the content of the render_manager
        asset_manager.update();
        //println!("STATUS: GAME: Updated all assets", );
        let render_instance = render.clone();
        (*render).lock().expect("Failed to lock renderer for rendering").render(&mut asset_manager);
        //Check if loop should close
        if input_handler.get_key_map_copy().closed{
            //println!("STATUS: GAME: Shuting down", );
            input_handler.end();
            break;
        }

        if input_handler.get_key_map_copy().escape{
            input_handler.end();
            break;
        }

        let fps_time = start_time.elapsed().subsec_nanos();
        //println!("STATUS: RENDER: FPS IN GAME: {}", 1.0/ (fps_time as f32 / 1_000_000_000.0) );
        start_time = Instant::now();
        //asset_manager.get_material_manager().print_all_materials();
    }
}
