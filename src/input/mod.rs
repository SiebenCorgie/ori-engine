use std::sync::{Mutex, Arc};
use std::thread;
use std::time::Duration;

use vulkano_win;
use vulkano_win::VkSurfaceBuild;
use winit;

use core::engine_settings;

///a sub mod who will read the input since the last loop
///and store the key values in a struct
pub mod input_handler;



///The struct stores the current pressed keys
#[derive(Debug, Copy, Clone)]
pub struct KeyMap {

    ///Window info (usually not needed recreation is handled by renderer)
    window_dimensions: [u32; 2],

    ///Global States
    pub closed: bool

}

impl KeyMap{
    pub fn new() -> Self{
        KeyMap{
            //window info
            window_dimensions: [100, 100],
            //state
            closed: false,

        }
    }
}


///Manages all input
pub struct Input {
    input_handler: input_handler::InputHandler,
    events_loop: Arc<Mutex<winit::EventsLoop>>,
    settings: Arc<Mutex<engine_settings::EngineSettings>>,
    pub key_map: Arc<Mutex<KeyMap>>,
}


impl Input{
    ///Creates a new Input instance
    pub fn new(settings: Arc<Mutex<engine_settings::EngineSettings>>) -> Self{

        let key_map_inst = Arc::new(Mutex::new(KeyMap::new()));

        let events_loop = Arc::new(Mutex::new(winit::EventsLoop::new()));

        Input{
            input_handler: input_handler::InputHandler::new(key_map_inst.clone(), events_loop.clone(), settings.clone()),
            events_loop: events_loop,
            settings: settings,
            key_map: key_map_inst.clone(),
        }
    }

    ///Starts the input polling thread
    pub fn start(&mut self){
        self.input_handler.start();
    }

    ///Ends the input polling thread, should be done when exiting the the main loop
    pub fn end(&mut self){

        self.input_handler.end();

        //Wait some mil seconds so the thread has time to end
        thread::sleep(Duration::from_millis(1000));
    }

    ///Returns the Events loop, used for renderer creation
    pub fn get_events_loop(&mut self) -> Arc<Mutex<winit::EventsLoop>>{
        self.events_loop.clone()
    }

    ///Returns the input handler
    pub fn get_input_handler(&mut self) -> &mut input_handler::InputHandler{
        &mut self.input_handler
    }


}
