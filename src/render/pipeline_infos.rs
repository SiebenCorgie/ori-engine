///The definition of the "model info"
#[derive(Clone)]
pub struct Main {
    pub model : [[f32;4];4],
    pub view : [[f32;4];4],
    pub proj : [[f32;4];4],
}
