
use winit;
use vulkano_win;
use vulkano_win::VkSurfaceBuild;
use vulkano;
use std::sync::{Arc, Mutex};

use core::engine_settings;


///Controlles a window created with the renderer
pub struct Window {
    window: vulkano_win::Window,

    engine_settings: Arc<Mutex<engine_settings::EngineSettings>>,
}


impl Window{
    pub fn new(instance: &Arc<vulkano::instance::Instance>,
        events_loop: &winit::EventsLoop,
        engine_settings: Arc<Mutex<engine_settings::EngineSettings>>
    )-> Self{

        let engine_settings_inst = engine_settings.clone();
        let engine_settings_lck = engine_settings_inst.lock().expect("Failed to lock engine settings");

        let window = winit::WindowBuilder::new()
        .with_dimensions(engine_settings_lck.window_width, engine_settings_lck.window_height)
        .with_title("Thingy!")
        .with_decorations(true)
        .build_vk_surface(events_loop, instance.clone()).expect("failed to create window!");

        Window{
            window: window,
            engine_settings: engine_settings.clone(),
        }
    }

    ///Returns the window surface
    pub fn surface(&mut self) -> &Arc<vulkano::swapchain::Surface> {
        self.window.surface()
    }

    ///Returns the window component
    pub fn window(&mut self) -> &winit::Window{
        self.window.window()
    }
}
