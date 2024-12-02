use crate::objects::{model::InstancedModel, shader::Shader};
use specs::*;

#[derive(Debug)]
pub struct MeshRenderer {
    pub model: InstancedModel,
    pub shader: Option<Shader>,
}
impl MeshRenderer {
    pub fn new(model: InstancedModel, shader: Option<Shader>) -> Self {
        Self { model, shader }
    }
}
impl Component for MeshRenderer {
    type Storage = VecStorage<Self>;
}
