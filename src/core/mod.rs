///The scene system mod responsible for handling the scene hirachie and handeling
///queries
pub mod simple_scene_system;
///The scene manager manages all available scene, he is tightly packet with the mesh and light manager(todo)
pub mod scene_manager;
///A high level asset manager which makes it easy to add and remove objects from/to the scene
///Graph. It also handles loading objects in a different thread and assiging materials from a material
///manager.
pub mod asset_manager;
///Definition of a default camera, the momvement has to be controlled by a gameplay-controller
pub mod camera;
///Defines all possible light
///Spot, point and directional light so far
pub mod light;
///Defines a normal mesh along with its properties
pub mod mesh;
///Handels all available meshes as well as the scenes created from an import with several meshes
pub mod mesh_manager;
///An empty can be used if a node should not have any content
pub mod empty;
///Defines a material with all it's properties, NOTE: this might switch to a UE4 like
///node based approach in the future.
pub mod material;
///A high level material manager used for creating, managing and destroying the materials
pub mod material_manager;
///Holds many useful information for different kinds of information
pub mod engine_settings;


use na;
use nc;


///The trait every Node Member should implement
///But you can make your own implementation
pub trait NodeMember {
    ///return the max size of its bound
    fn get_bound_max(&self)-> &na::Point3<f32>;
    ///return the min size of its bound
    fn get_bound_min(&self)-> &na::Point3<f32>;
    ///Sets the bound to the new values (in mesh space)
    fn set_bound(&mut self, min: na::Point3<f32>, max: na::Point3<f32>);
    ///Returns the vertices of the bounding mesh, good for debuging
    fn get_bound_points(&mut self)-> Vec<na::Vector3<f32>>;
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
