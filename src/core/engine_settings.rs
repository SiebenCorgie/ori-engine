///The struc with the information
pub struct EngineSettings {
    pub app_name: String,

    pub window_width: u32,
    pub window_height: u32,
}

impl EngineSettings{
    pub fn new() -> Self{
        EngineSettings{
            app_name: String::from("Ori-Engine"),
            window_width: 800,
            window_height: 600,
        }
    }

    pub fn with_dimensions(mut self, width: u32, height: u32) -> Self{
        self.window_width = width;
        self.window_height = height;
        self
    }

    pub fn with_name(mut self, name: &str) -> Self{
        self.app_name = String::from(name);
        self
    }

    pub fn set_dimensions(&mut self, width: u32, height: u32){
        self.window_width = width;
        self.window_height = height;
    }

    pub fn get_dimensions(&mut self) -> [u32; 2]{
        [self.window_width.clone(), self.window_height.clone()]
    }
}
