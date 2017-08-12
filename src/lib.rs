///The engines top level

extern crate nalgebra as na;
extern crate ncollide as nc;
extern crate assimp;

extern crate winit;
#[macro_use]
extern crate vulkano;
#[macro_use]
extern crate vulkano_shader_derive;
#[macro_use]
extern crate vulkano_win;
#[macro_use]
extern crate time;
#[macro_use]
extern crate image;

///The engine core defines most functions and
///traits needed to feed the renderer and communicate with the physics.
///It also mamanges the scene tree and how to get specific information out of it
pub mod core;

///The engines renderer currently WIP
pub mod render;

///A collection of helpfull tools for integration of data with the engine
pub mod tools;

///A small thread who will run and administrate the winit window, as well as its input
///processing
pub mod input;


//Some Helper functions
///Returns an runtime error
pub fn rt_error(location: &str, content: &str){
    println!("ERROR AT: {} FOR: {}", location, content);
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}


/*TODO
1st. create a assmip based importer who builds a own scene_tree from a file  //NOTE DONE
2nd create a material structure indipendent from the meshes //NOTE DONE
3rd pair the mesh struct with the material manager maybe with Arc<material> where different meshes
share one material ==> Using a String value for now. //NOTE DONE
//NOTE Currently using a pipeline manager as well, materials with the same shader share one pipeline
    => Much smaller memory footprint compared to the pipeline/material aproach
    But its kinda messy at the moment ... TOO MUCH OF DEM GENERICS ~~~FIXED!~~~~
    //NOTE NOTE: Changed the Uniform System to be more dynamic, Can now create a Uniform from the
    //pipeline_info mod and pass it directly to the pipeline_manager. The manager will rebuild the
    //Uniform buffer and set corresbonding set
    //TODO Might change the Sets to a Vec<Set> and the Unfiorm_pools as well (maybe a fixed [sets; n] vec)
//TODO NOW (2017_07_19) Add material Parameters to make different colors to test material system
//NOTE moved the set Generation to the materials, this way we reuse most sets where possible
//NOw Create a texture manager

2nd Outsource Renderer from the example to the renderer mod DONE
3rd Render on main thread, manage materials on event on different thread,
manage objects on secondary thread, manage loading on n-threads (per object?)
4th then continue

4th create get_*_in_frustum functions for all types in ContentTypes done, needs to be tested
5th create a high level fn collection for adding and removing things from the scene tree
6th build a simple forward renderer with vulkano and test the scene tree //NOTE Done in 3.1-3.4 ?
7th make core, render and later physics independend threads //NOTE Done in 3.1-3.4 ?
8th multithread asset import //NOTE Done in 3.1-3.4 ?
9th add lights to the renderer
10th shadow generation?

9th CREATE A FLUFFY TILED RENDERER WITH ALL THE STUFF
10th PBR ?
11th Editor and templates https://www.youtube.com/watch?v=UWacQrEcMHk , https://www.youtube.com/watch?annotation_id=annotation_661107683&feature=iv&src_vid=UWacQrEcMHk&v=xYiiD-p2q80 , https://www.youtube.com/watch?annotation_id=annotation_2106536565&feature=iv&src_vid=UWacQrEcMHk&v=yIedljapuz0
*/


/*TODO
*/
