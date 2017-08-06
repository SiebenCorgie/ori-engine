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


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

/*TODO
- Create a uniformbuffer manager which stores the public buffers (everything except textures and materials)

- Todo FIX ALL THE ERROROS

Create a texture manager and move the set generator to the materials for a per frame
set generation for texture set in a material
*/
