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
    pub closed: bool,

    //normal keys
    a: bool,
    b: bool,
    c: bool,
    d: bool,
    e: bool,
    f: bool,
    g: bool,
    h: bool,
    i: bool,
    j: bool,
    k: bool,
    l: bool,
    m: bool,
    n: bool,
    o: bool,
    p: bool,
    q: bool,
    r: bool,
    s: bool,
    t: bool,
    u: bool,
    v: bool,
    w: bool,
    x: bool,
    y: bool,
    z: bool,
    //numbers on the top
    t_1: bool,
    t_2: bool,
    t_3: bool,
    t_4: bool,
    t_5: bool,
    t_6: bool,
    t_7: bool,
    t_8: bool,
    t_9: bool,
    t_0: bool,
    //numblock
    num_1: bool,
    num_2: bool,
    num_3: bool,
    num_4: bool,
    num_5: bool,
    num_6: bool,
    num_7: bool,
    num_8: bool,
    num_9: bool,
    num_0: bool,
    //Main controll keys
    ctrl_l: bool,
    ctrl_r: bool,
    alt_l: bool,
    alt_gr: bool,
    super_l: bool,
    super_r: bool,
    caps_lock: bool,
    shift_l: bool,
    shift_r: bool,
    tab: bool,
    space: bool,
    return_l: bool,
    return_num: bool,
    escape: bool,
    //todo addrest
    /*
F1,
F2,
F3,
F4,
F5,
F6,
F7,
F8,
F9,
F10,
F11,
F12,
F13,
F14,
F15,
Snapshot,
Scroll,
Pause,
Insert,
Home,
Delete,
End,
PageDown,
PageUp,
Left,
Up,
Right,
Down,
Back,
Compose,
AbntC1,
AbntC2,
Add,
Apostrophe,
Apps,
At,
Ax,
Backslash,
Calculator,
Colon,
Comma,
Convert,
Decimal,
Divide,
Equals,
Grave,
Kana,
Kanji,
LBracket,
LMenu,
Mail,
MediaSelect,
MediaStop,
Minus,
Multiply,
Mute,
MyComputer,
NavigateForward,
NavigateBackward,
NextTrack,
NoConvert,
NumpadComma,
NumpadEnter,
NumpadEquals,
OEM102,
Period,
PlayPause,
Power,
PrevTrack,
RBracket,
RMenu,
Semicolon,
Slash,
Sleep,
Stop,
Subtract,
Sysrq,
Underline,
Unlabeled,
VolumeDown,
VolumeUp,
Wake,
WebBack,
WebFavorites,
WebForward,
WebHome,
WebRefresh,
WebSearch,
WebStop,
Yen,
    */


}

impl KeyMap{
    pub fn new() -> Self{
        KeyMap{
            //window info
            window_dimensions: [100, 100],
            //state
            closed: false,

            //normal keys
            a: false,
            b: false,
            c: false,
            d: false,
            e: false,
            f: false,
            g: false,
            h: false,
            i: false,
            j: false,
            k: false,
            l: false,
            m: false,
            n: false,
            o: false,
            p: false,
            q: false,
            r: false,
            s: false,
            t: false,
            u: false,
            v: false,
            w: false,
            x: false,
            y: false,
            z: false,
            //numbers on the top
            t_1: false,
            t_2: false,
            t_3: false,
            t_4: false,
            t_5: false,
            t_6: false,
            t_7: false,
            t_8: false,
            t_9: false,
            t_0: false,
            //numblock
            num_1: false,
            num_2: false,
            num_3: false,
            num_4: false,
            num_5: false,
            num_6: false,
            num_7: false,
            num_8: false,
            num_9: false,
            num_0: false,
            //Main controll keys
            ctrl_l: false,
            ctrl_r: false,
            alt_l: false,
            alt_gr: false,
            super_l: false,
            super_r: false,
            caps_lock: false,
            shift_l: false,
            shift_r: false,
            tab: false,
            space: false,
            return_l: false,
            return_num: false,
            escape: false,
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
