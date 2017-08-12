
///The main renderer responsible for the coordination of all render work in its own render loop
pub mod renderer;
///Manages all available pipeline, you'll mostly just need the default one
pub mod pipeline_manager;
///Defines the pipeline an renderable object can have, must be stored in the pipeline_manager
pub mod pipeline;
///Defines several types a pipeline can implement and pass to a shader
pub mod pipeline_infos;
///Handles a window which was created for the renderer
pub mod window;
///manages all universal accesible uniforms, like lights and world info
pub mod uniform_manager;
