use std::sync::{Arc, Mutex};
use std::thread;

use core::engine_settings;
use input::KeyMap;
use winit;


#[derive(PartialEq, Eq)]
pub enum InputHandlerStates {
    ShouldEnd,
    Running,
}

pub struct InputHandler {
    key_map: Arc<Mutex<KeyMap>>,
    events_loop: Arc<Mutex<winit::EventsLoop>>,
    pub state: Arc<Mutex<InputHandlerStates>>,

    settings: Arc<Mutex<engine_settings::EngineSettings>>,
}

impl InputHandler{
    ///Creates a new input handler, needs to be started via `start` and ended via `end`
    pub fn new(
        key_map: Arc<Mutex<KeyMap>>,
        events_loop: Arc<Mutex<winit::EventsLoop>>,
        settings: Arc<Mutex<engine_settings::EngineSettings>>,
    ) -> Self{
        InputHandler{
            key_map: key_map,
            events_loop: events_loop,

            settings: settings,

            state: Arc::new(Mutex::new(InputHandlerStates::Running)),
        }
    }

    ///Starts the input reading and saves the current key-map for usage in everything input releated
    pub fn start(&mut self){

        let key_map_inst = self.key_map.clone();
        let events_loop_inst = self.events_loop.clone();
        let state_instance = self.state.clone();
        let settings_ins = self.settings.clone();

        //Start the continues input polling
        let thread = thread::spawn(move ||{
            //Polling all events TODO make a variable input cap for polling



            //Copy our selfs a settings instance to change settings which ... changed
            let mut settings_instance = {
                let tmp = settings_ins.clone();
                let lck = tmp.lock().expect("failed to lock settings in input handler");

                (*lck).clone()
            };

            // And a small flag to prevent to much locking
            let mut b_engine_settings_changed = false;

            //Create a tmp keymap which will overwrite the global keymap in `input`
            let mut current_keys = KeyMap::new();

            //lock the events loop for polling
            let mut events_loop = (*events_loop_inst).lock().expect("Failed to hold lock on eventsloop");

            loop{
                //Check if the thread should end alread, if return
                {
                    let mut state_lck = state_instance.lock().expect("failed to lock thread state");
                    if *state_lck == InputHandlerStates::ShouldEnd{
                        println!("STATUS: INPUT HANDLER: ending input thread", );
                        break;
                    }
                }

                //Now do the events polling
                events_loop.poll_events(|ev| {
                    match ev {
                        //Check the event type
                        //window
                        winit::Event::WindowEvent{window_id, event} =>{
                            //Make life easier and temporarly import all the events
                            use winit::WindowEvent::*;

                            match event{
                                Resized(width , height) =>{

                                    b_engine_settings_changed = true;
                                    settings_instance.set_dimensions(
                                        width.clone() as u32,
                                        height.clone() as u32
                                    );
                                    println!("Resized to {} / {}", width, height );
                                },
                                Moved(width, height) =>{
                                    println!("STATUS: INPUT HANDLER: moved: {} / {}", width, height );

                                },
                                Closed => {
                                    current_keys.closed = true;
                                    println!("STATUS: INPUT HANDLER: closing", );
                                },
                                DroppedFile(file_path) =>{
                                    println!("Droped file with path: {:?}", file_path );
                                },
                                ReceivedCharacter(character) =>{

                                },
                                Focused(b_state) =>{

                                },
                                KeyboardInput {device_id, input} =>{
                                    use winit::KeyboardInput;
                                    match input{
                                        
                                        _ => {},
                                    }

                                },
                                MouseMoved {device_id, position} =>{

                                },
                                MouseEntered{device_id} =>{

                                },
                                MouseLeft{device_id} =>{

                                },
                                MouseWheel{device_id, delta, phase} =>{

                                },
                                MouseInput{device_id, state, button} =>{

                                },
                                TouchpadPressure{device_id, pressure, stage} =>{

                                },
                                AxisMotion{device_id, axis, value} =>{

                                },
                                Refresh =>{

                                },
                                Suspended(b_state) =>{

                                },
                                Touch(touch) =>{

                                },
                            }
                        },
                        //Device
                        winit::Event::DeviceEvent{device_id, event} => {
                            match event{
                                //This could register raw device events, however, not used atm
                                _ => {},
                            }

                        },
                        //Awake (not implemented)
                        winit::Event::Awakened => {},

                    }
                });


                //Overwrite the Arc<Mutex<KeyMap>> with the new capture
                {
                    let mut key_map_unlck = key_map_inst
                    .lock()
                    .expect("failed to hold key_map_inst lock while updating key info");
                    (*key_map_unlck) = current_keys;
                }
            }

        // If some global settings changed, we can push them to the engine_settings instance
        // of this engine run
        if b_engine_settings_changed{
            let l_settings_ins = settings_ins.clone();
            let mut settings_lck = l_settings_ins
            .lock()
            .expect("failed to lock settings for overwrite");

            (*settings_lck) = settings_instance;
        }

        });
    }

    ///Ends the input threa via a end flag
    pub fn end(&mut self){

        let state_inst = self.state.clone();
        let mut state_lck = state_inst
        .lock()
        .expect("Failed to lock input thread state for ending");

         *state_lck = InputHandlerStates::ShouldEnd;
    }
}
