use vulkano;
use core::resources::light;
use render::shader_impls::pbr_vertex;
use render::shader_impls::pbr_fragment;

///The definition of the "model info" responsible for handeling the
/// model, view and projection matrix
#[derive(Clone)]
#[repr(C)]
pub struct Main {
    pub model : [[f32;4];4],
    pub view : [[f32;4];4],
    pub proj : [[f32;4];4],
}

/*
#[derive(Clone)]
#[repr(C)]
///A stripped down version of a point light which can be passed to a shader
pub struct LightPointShaderInfo {
    pub color: [f32; 4],
    //pub intensity: [f32; 4],
    /*
    pub intensity: f32,
    pub pad_01: f32,
    pub pad_02: f32,
    pub pad_03: f32,
    */
}

#[derive(Clone)]
#[repr(C)]
///A stripped down version of a directional light which can be passed to a shader
pub struct LightDirectionalShaderInfo {
    pub color: [f32; 4],
    pub direction: [f32; 4],
    //pub intensity: [f32; 4],

    pub intensity: f32,
    pub pad_01: f32,
    pub pad_02: f32,
    pub pad_03: f32,

}

///A stripped down version of a spot light which can be passed to a shader
///NOTE: The this are always vec4 at the start for correct padding (hopfully)
#[derive(Clone)]
#[repr(C)]
pub struct LightSpotShaderInfo {
    pub color: [f32; 4],
    pub direction: [f32; 4],
    //pub int_outer_inner: [f32; 4],
    pub intensity: f32,
    pub outer_radius: f32,
    pub inner_radius: f32,
    pub pad_01: f32,

}
*/
/*
///Describes the components which will be send to the shader for `PointLight`s
#[derive(Clone)]
#[repr(C)]
pub struct PointLightInfo {
    ///Holds an array of all the point lights currently used
    pub l_point: Vec<pbr_fragment::ty::PointLight>
}


///Describes the components which will be send to the shader for `DirectionlLight`s
#[derive(Clone)]
#[repr(C)]
pub struct DirectionlLightInfo {
    ///Holds an array of all the directional lights currently used
    pub l_directional: Vec<LightDirectionalShaderInfo>
}

///Describes the components which will be send to the shader for `SportLight`s
#[derive(Clone)]
#[repr(C)]
pub struct SpotLightInfo {
    ///Holds an array of all the spot lights currently used
    pub l_spot: Vec<LightSpotShaderInfo>
}

///Represents the currently send light count, used to set the loop iterations per light type
///in the shader.
#[derive(Clone)]
#[repr(C)]
pub struct LightCount {
    ///Sets number of currently used point lights
    pub num_point_lights: u32,
    ///Sets number of currently used directional lights
    pub num_directional_lights: u32,
    ///Sets number of currently used spot lights
    pub num_spot_lights: u32,
}
*/
