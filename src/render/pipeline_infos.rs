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
    intensity: f32,
    color: [f32; 3],
}

#[derive(Clone)]
///A stripped down version of a directional light which can be passed to a shader
pub struct LightDirectionalShaderInfo {
    intensity: f32,
    color: [f32; 3],

    direction: [f32; 3],
}

#[derive(Clone)]
///A stripped down version of a spot light which can be passed to a shader
pub struct LightSpotShaderInfo {
    intensity: f32,
    color: [f32; 3],

    direction: [f32; 3],

    outer_radius: f32,
    inner_radius: f32,
}

#[derive(Clone)]
///Describes the components which will be send to the shader for `PointLight`s
pub struct PointLightInfo {
    l_point: Vec<LightPointShaderInfo>
}

#[derive(Clone)]
///Describes the components which will be send to the shader for `DirectionlLight`s
pub struct DirectionlLightInfo {
    l_directional: Vec<LightDirectionalShaderInfo>
}

#[derive(Clone)]
///Describes the components which will be send to the shader for `SportLight`s
pub struct SpotLightInfo {
    l_spot: Vec<LightSpotShaderInfo>
}
