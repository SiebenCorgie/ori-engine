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

///The engine core defines most functions and
///traits needed to feed the renderer and communicate with the physics.
///It also mamanges the scene tree and how to get specific information out of it
pub mod core;

///The engines renderer currently WIP
pub mod render;

///A collection of helpfull tools for integration of data with the engine
pub mod tools;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

/*TODO
Create a asynchron input handler who reads current input stores it and when fully written, writes it
to a shared buffer (simmilar to the object loaders etc.)
*/
