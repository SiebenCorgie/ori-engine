use std::sync::{Arc, Mutex};
use std::thread;

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
}

impl InputHandler{
    pub fn new(key_map: Arc<Mutex<KeyMap>>, events_loop: Arc<Mutex<winit::EventsLoop>>) -> Self{
        InputHandler{
            key_map: key_map,
            events_loop: events_loop,

            state: Arc::new(Mutex::new(InputHandlerStates::Running)),
        }
    }

    pub fn start(&mut self){

        let key_map_inst = self.key_map.clone();
        let events_loop_inst = self.events_loop.clone();
        let state_instance = self.state.clone();

        //Start the continues input polling
        let thread = thread::spawn(move ||{
            //Polling all events TODO make a variable input cap for polling

            //Create a tmp keymap which will overwrite the global keymap in `input`
            let mut current_keys = KeyMap::new();

            //lock the events loop for polling
            let mut events_loop = (*events_loop_inst).lock().expect("Failed to hold lock on eventsloop");

            loop{
                //Check if the thread should end alread, if return
                {
                    let mut state_lck = state_instance.lock().expect("failed to lock thread state");
                    if *state_lck == InputHandlerStates::ShouldEnd{
                        //println!("STATUS: INPUT HANDLER: ending input thread", );
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

                                },
                                Moved(width, height) =>{
                                    //println!("STATUS: INPUT HANDLER: moved: {} / {}", width, height );

                                },
                                Closed => {
                                    current_keys.closed = true;
                                    //println!("STATUS: INPUT HANDLER: closing", );
                                },
                                DroppedFile(file_path) =>{

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
                    let mut key_map_unlck = key_map_inst.lock().expect("failed to hold key_map_inst lock while updating key info");
                    (*key_map_unlck) = current_keys;
                }
            }
        });
    }

    pub fn end(&mut self){

        let state_inst = self.state.clone();
        let mut state_lck = state_inst
        .lock()
        .expect("Failed to lock input thread state for ending");

         *state_lck = InputHandlerStates::ShouldEnd;
    }
}
