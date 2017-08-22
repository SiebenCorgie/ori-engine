extern crate vulkano;
extern crate ori_engine;
extern crate nalgebra as na;

use ori_engine::*;
use ori_engine::core::simple_scene_system::node;
use ori_engine::core::simple_scene_system::node_member;
use ori_engine::core::resources::light;
use std::sync::{Arc, Mutex};
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
    let render = Arc::new(
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
    //asset_manager.import_scene("Ape_02", "Apes.fbx");
    //asset_manager.import_scene("Ape_03", "Apes.fbx");

    let mut tex_builder = asset_manager.create_texture("/home/siebencorgie/Pictures/MyPictures/ben_mauro_01.jpg");
    tex_builder = tex_builder.with_flipped_v();
    asset_manager.add_texture_to_manager(tex_builder, "new_texture").expect("failed to add new_texture");

    //Creating a new material, currently a bit ugly
    {

        let (_ , normal, physical) = asset_manager.get_texture_manager().get_fallback_textures();
        let texture_in_manager = asset_manager.get_texture_manager().get_texture("new_texture");

        let new_material = core::resources::material::MaterialBuilder::new(
            Some(texture_in_manager),
            Some(normal),
            Some(physical),
            None,
            asset_manager.get_texture_manager().get_none()
        ).with_factors(
            core::resources::material::MaterialFactors::new()
            .with_factor_albedo([1.0, 1.0, 0.0, 1.0])
        );

        asset_manager.add_material_to_manager(new_material, "new_material").expect("failed to add new_material");
    }

    asset_manager.import_scene("Ring", "Ring.fbx");

    let mut adding_status = false;

    let mut start_time = Instant::now();

    let mut sun = light::LightDirectional::new("Sun");
    sun.set_direction(na::Vector3::new(1.0, -0.5, 0.5));
    sun.set_color(na::Vector3::new(1.0, 0.5, 0.5));

    let sun_node = Arc::new(
        node_member::SimpleNodeMember::from_light_directional(
            Arc::new(
                Mutex::new(sun)
            )
        )
    );
    asset_manager.get_active_scene().add_child(sun_node);


    let mut point = light::LightPoint::new("Point");
    point.set_color(na::Vector3::new(0.0, 0.5, 1.0));

    let point_node = Arc::new(
        node_member::SimpleNodeMember::from_light_point(
            Arc::new(
                Mutex::new(point)
            )
        )
    );
    asset_manager.get_active_scene().add_child(point_node);

    asset_manager.get_active_scene().print_member(0);
    println!("Start n stuff", );

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
        println!("STATUS: RENDER: FPS IN GAME: {}", 1.0/ (fps_time as f32 / 1_000_000_000.0) );
        start_time = Instant::now();
        //asset_manager.get_material_manager().print_all_materials();
    }
}
