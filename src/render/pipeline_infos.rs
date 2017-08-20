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


///Describes the components which will be send to the shader for `PointLight`s
pub struct PointLightInfo {
    l_point: [light::LightPoint]
}

///Describes the components which will be send to the shader for `DirectionlLight`s
pub struct DirectionlLightInfo {
    l_directional: [light::LightDirectional]
}

///Describes the components which will be send to the shader for `SportLight`s
pub struct SportLightInfo {
    l_spot: [light::LightSpot]
}
