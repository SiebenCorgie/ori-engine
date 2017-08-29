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
    .with_dimensions(1600, 900)
    .with_name("Ori Instance")
    .set_vulkan_silent()
    .with_fullscreen_mode(false)
    .with_cursor_state(winit::CursorState::Grab)
    .with_cursor_visibility(winit::MouseCursor::NoneCursor)
    .with_msaa_factor(4)
    ));

    //Input
    let mut input_handler = input::Input::new(settings.clone()).with_polling_speed(60);

    //Create a renderer with the input system
    let render = Arc::new(
        Mutex::new(
            render::renderer::Renderer::new(
                input_handler.get_events_loop(),
                settings.clone(),
                input_handler.get_key_map(),
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
    asset_manager.import_scene("Helix", "Helix.fbx");
    //asset_manager.import_scene("Ape_02", "Apes.fbx");
    //asset_manager.import_scene("Ape_03", "Apes.fbx");
    {
        //Albedo
        let mut tex_builder_01 = asset_manager.create_texture("/share/3DFiles/TextureLibary/Metal/RustSteal/rustediron2_basecolor.png");
        tex_builder_01 = tex_builder_01.with_flipped_v();
        asset_manager.add_texture_to_manager(tex_builder_01, "metal_albedo").expect("failed to add new_texture");
        //Normal
        let mut tex_builder_02 = asset_manager.create_texture("/share/3DFiles/TextureLibary/Metal/RustSteal/rustediron2_normal.png");
        tex_builder_02 = tex_builder_02.with_flipped_v();
        asset_manager.add_texture_to_manager(tex_builder_02, "metal_normal").expect("failed to add new_texture");
        //Physical
        let mut tex_builder_03 = asset_manager.create_texture("/share/3DFiles/TextureLibary/Metal/RustSteal/rustediron2_physical.png");
        tex_builder_03 = tex_builder_03.with_flipped_v();
        asset_manager.add_texture_to_manager(tex_builder_03, "metal_physical").expect("failed to add new_texture");
        //Creating a new material, currently a bit ugly
        {

            let albedo_in_manager = asset_manager.get_texture_manager().get_texture("metal_albedo");
            let nrm_in_manager = asset_manager.get_texture_manager().get_texture("metal_normal");
            let physical_in_manager = asset_manager.get_texture_manager().get_texture("metal_physical");

            let new_material = core::resources::material::MaterialBuilder::new(
                Some(albedo_in_manager),
                Some(nrm_in_manager),
                Some(physical_in_manager),
                None,
                asset_manager.get_texture_manager().get_none()
            ).with_factors(
                core::resources::material::MaterialFactors::new()
                .with_factor_albedo([1.0, 1.0, 0.0, 1.0])
            );

            asset_manager.add_material_to_manager(new_material, "new_material").expect("failed to add new_material");
        }
    }

    let mut adding_status = false;

    let mut start_time = Instant::now();
    /*
    let mut sun = light::LightDirectional::new("Sun");
    sun.set_direction(na::Vector3::new(1.0, 0.5, 0.5));
    sun.set_color(na::Vector3::new(1.0, 0.75, 0.6));

    let sun_node = Arc::new(
        node_member::SimpleNodeMember::from_light_directional(
            Arc::new(
                Mutex::new(sun)
            )
        )
    );
    asset_manager.get_active_scene().add_child(sun_node);
    */
    //POINT 01 ==================================================================
    let mut point_01 = light::LightPoint::new("Point_01");
    point_01.set_color(na::Vector3::new(150.0, 150.0, 150.0));
    point_01.set_location(na::Vector3::new(-5.0, -5.0, 10.0));

    let point_node_01 = Arc::new(
        node_member::SimpleNodeMember::from_light_point(
            Arc::new(
                Mutex::new(point_01)
            )
        )
    );
    asset_manager.get_active_scene().add_child(point_node_01);
    //POINT 01 ==================================================================

    //POINT 02 ==================================================================
    let mut point_02 = light::LightPoint::new("Point_02");
    point_02.set_color(na::Vector3::new(150.0, 150.0, 150.0));
    point_02.set_location(na::Vector3::new(-5.0, 5.0, 10.0));

    let point_node_02 = Arc::new(
        node_member::SimpleNodeMember::from_light_point(
            Arc::new(
                Mutex::new(point_02)
            )
        )
    );
    asset_manager.get_active_scene().add_child(point_node_02);
    //POINT 02 ==================================================================

    //POINT 03 ==================================================================
    let mut point_03 = light::LightPoint::new("Point_03");
    point_03.set_color(na::Vector3::new(150.0, 150.0, 150.0));
    point_03.set_location(na::Vector3::new(5.0, -5.0, 10.0));

    let point_node_03 = Arc::new(
        node_member::SimpleNodeMember::from_light_point(
            Arc::new(
                Mutex::new(point_03)
            )
        )
    );
    asset_manager.get_active_scene().add_child(point_node_03);
    //POINT 03 ==================================================================

    //POINT 04 ==================================================================
    let mut point_04 = light::LightPoint::new("Point_04");
    point_04.set_color(na::Vector3::new(150.0, 150.0, 150.0));
    point_04.set_location(na::Vector3::new(5.0, 5.0, 10.0));

    let point_node_04 = Arc::new(
        node_member::SimpleNodeMember::from_light_point(
            Arc::new(
                Mutex::new(point_04)
            )
        )
    );
    asset_manager.get_active_scene().add_child(point_node_04);
    //POINT 04 ==================================================================

    asset_manager.get_active_scene().print_member(0);
    println!("Start n stuff", );

    let mut adding_status_helix = false;

    loop {
        //Add the ape scene if finished loading. This will be managed by a defined loader later
        if adding_status == false && asset_manager.has_scene("Ape") && asset_manager.has_scene("Ring"){
            println!("Adding ape", );
            let mut ape_scene ={
                //let scene_manager = asset_manager.get_scene_manager();
                asset_manager.get_scene_manager().get_scene("Ape").expect("no Apes :(")
            };

            for i in (*ape_scene).lock().unwrap().get_all_meshes().iter(){
                //Unwrap the mesh from the tubel
                let mesh = i.0.clone();

                let mesh_inst = mesh.clone();
                let mut mesh_lck = mesh_inst.lock().expect("failed to change material");
                (*mesh_lck).set_material("new_material");
            }
            asset_manager.add_scene_to_main_scene("Ape");
            asset_manager.add_scene_to_main_scene("Ring");

            adding_status = true;
            println!("STATUS: GAME: added all apes", );

        }

        if !adding_status_helix && asset_manager.has_scene("Helix"){
            println!("Adding helix", );

            let helix_scene = asset_manager.get_scene_manager().get_scene("Helix").expect("no Helix :(");

            println!("Set Helix lock", );
            for i in (*helix_scene).lock().unwrap().get_all_meshes().iter(){
                let mesh = i.0.clone();
                println!("Cloned mesh", );
                let mut mesh_lck = mesh.lock().expect("failed to lock helix");
                println!("Locked mesh", );
                (*mesh_lck).set_material("new_material");
                println!("SetMaterial", );
            }

            asset_manager.add_scene_to_main_scene("Helix");

            adding_status_helix = true;
            println!("Finished helix", );
        }
        println!("STATUS: GAME: Starting loop in game", );
        //Update the content of the render_manager
        asset_manager.update();
        println!("STATUS: GAME: Updated all assets", );
        (*render).lock().expect("Failed to lock renderer for rendering").render(&mut asset_manager);
        //Check if loop should close
        if input_handler.get_key_map_copy().closed{
            println!("STATUS: GAME: Shuting down", );
            input_handler.end();
            break;
        }

        if input_handler.get_key_map_copy().escape{
            input_handler.end();
            break;
        }

        if input_handler.get_key_map_copy().t{
            //Get the Ring scene and translate it by 10,10,0
            let mut ape_scene ={
                //Get the reference in the current active scene
                match asset_manager.get_active_scene().get_node("Ape"){
                    Some(scene) => scene,
                    None => continue,
                }
            };
            //Set the translation on this node
            ape_scene.translate(na::Vector3::new(-1.0, -1.0, 0.0));
            println!("Translated", );
        }

        if input_handler.get_key_map_copy().z{
            //Get the Ring scene and translate it by 10,10,0
            let mut helix_scene ={
                //Get the reference in the current active scene
                match asset_manager.get_active_scene().get_node("Helix"){
                    Some(scene) => scene,
                    None => continue,
                }
            };
            //Set the translation on this node
            helix_scene.rotate(na::Rotation3::new(na::Vector3::new(0.05, 0.0, 0.0)));
            //ape_scene.rotate(na::Rotation3::from_euler_angles(0.0, 0.05, 0.0));
            println!("Translated", );
            //input_handler.end();
            //break;
        }

        if input_handler.get_key_map_copy().u{
            //Get the Ring scene and translate it by 10,10,0
            let mut tree_scene ={
                //Get the reference in the current active scene
                match asset_manager.get_active_scene().get_node("Helix_0"){
                    Some(scene) => scene,
                    None => continue,
                }
            };
            //Set the translation on this node
            tree_scene.translate(na::Vector3::new(-0.05, -0.05, 0.0));
            //ape_scene.rotate(na::Rotation3::from_euler_angles(0.0, 0.05, 0.0));
            println!("Translated", );
            //input_handler.end();
            //break;
        }




        asset_manager.get_active_scene().print_member(0);

        let fps_time = start_time.elapsed().subsec_nanos();
        println!("STATUS: RENDER: FPS IN GAME: {}", 1.0/ (fps_time as f32 / 1_000_000_000.0) );
        start_time = Instant::now();
        asset_manager.get_material_manager().print_all_materials();
    }
}
