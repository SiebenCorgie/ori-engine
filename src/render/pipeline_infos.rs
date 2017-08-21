use vulkano;
use core::resources::light;
///The definition of the "model info" responsible for handeling the
/// model, view and projection matrix
#[derive(Clone)]
pub struct Main {
    pub model : [[f32;4];4],
    pub view : [[f32;4];4],
    pub proj : [[f32;4];4],
}

#[derive(Clone)]
///A stripped down version of a point light which can be passed to a shader
pub struct LightPointShaderInfo {
    pub color: [f32; 3],
    pub intensity: f32,
}

#[derive(Clone)]
///A stripped down version of a directional light which can be passed to a shader
pub struct LightDirectionalShaderInfo {
    pub color: [f32; 3],
    pub direction: [f32; 3],
    pub intensity: f32,
}

#[derive(Clone)]
///A stripped down version of a spot light which can be passed to a shader
pub struct LightSpotShaderInfo {
    pub color: [f32; 3],
    pub direction: [f32; 3],
    pub intensity: f32,
    pub outer_radius: f32,
    pub inner_radius: f32,
}

#[derive(Clone)]
///Describes the components which will be send to the shader for `PointLight`s
pub struct PointLightInfo {
    ///Holds an array of all the point lights currently used
    pub l_point: Vec<LightPointShaderInfo>
}

#[derive(Clone)]
///Describes the components which will be send to the shader for `DirectionlLight`s
pub struct DirectionlLightInfo {
    ///Holds an array of all the directional lights currently used
    pub l_directional: Vec<LightDirectionalShaderInfo>
}

#[derive(Clone)]
///Describes the components which will be send to the shader for `SportLight`s
pub struct SpotLightInfo {
    ///Holds an array of all the spot lights currently used
    pub l_spot: Vec<LightSpotShaderInfo>
}

#[derive(Clone)]
pub struct LightCount {
    ///Sets number of currently used point lights
    pub num_point_lights: u32,
    ///Sets number of currently used directional lights
    pub num_directional_lights: u32,
    ///Sets number of currently used spot lights
    pub num_spot_lights: u32,
}
