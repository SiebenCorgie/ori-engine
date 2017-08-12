
///The struc with the information
#[derive(Clone)]
pub struct EngineSettings {
    //Displayed name
    pub app_name: String,
    //Dimensions in pixel
    pub window_dimensions: [u32; 2],
    //location in pixel
    pub window_location: [u32; 2],
    //Debug settings:
    silent_vulkan: bool,

}

impl EngineSettings{
    /// Creates a `EngineSettings` with default values.
    /// You can change some of them like this at creation time:
    /// # Examples
    ///  ```
    /// use ori-engine::core::engine_settings;
    ///
    /// let settings = core::engine_settings::EngineSettings::new()
    ///     .with_dimensions(800, 600)
    ///     .with_name("Teddy the bear")
    ///     .set_vulkan_silent()
    ///     ));
    ///  ```
    pub fn new() -> Self{



        EngineSettings{
            app_name: String::from("Ori-Engine"),
            window_dimensions: [800, 600],
            window_location: [100, 100],

            silent_vulkan: false,
        }
    }

    /// Sets vulkan silent, vulkan won't print any validation layer infos anymore
    pub fn set_vulkan_silent(mut self) -> Self{
        self.silent_vulkan = true;
        self
    }
    ///returns the silent status of vulkan
    pub fn vulkan_silence(&self) -> bool{
        self.silent_vulkan.clone()
    }
    ///Sets the dimensions of `self` to `width` and `height`
    pub fn with_dimensions(mut self, width: u32, height: u32) -> Self{
        self.window_dimensions = [width, height];
        self
    }
    ///Sets the Location of `self` to `width` and `height`
    pub fn at_location(mut self, width: u32, height: u32) -> Self{
        self.window_location = [width, height];
        self
    }
    ///Sets the name of this settings
    pub fn with_name(mut self, name: &str) -> Self{
        self.app_name = String::from(name);
        self
    }
    ///Sets the dimensions of a currently used instance of `EngineSettings`
    pub fn set_dimensions(&mut self, width: u32, height: u32){
        self.window_dimensions = [width, height];
    }
    ///Returns the dimensions
    pub fn get_dimensions(&self) -> [u32; 2]{
        self.window_dimensions.clone()
    }
}
