use vulkano;

///The definition of the "model info" responsible for handeling the
/// model, view and projection matrix
#[derive(Clone)]
pub struct Main {
    pub model : [[f32;4];4],
    pub view : [[f32;4];4],
    pub proj : [[f32;4];4],
}



///The definition of the "material textures"
#[derive(Clone)]
pub struct MaterialTextures {
    pub albedo : f32,
}
