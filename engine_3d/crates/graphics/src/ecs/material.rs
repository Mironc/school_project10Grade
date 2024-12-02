use glam::Vec3;
use specs::{Component, VecStorage};

use crate::objects::texture::Texture2D;

#[derive(Debug, Clone)]
pub struct Material {
    pub color: Vec3,
    pub main_texture:Texture2D,
    pub specular: f32,
    pub ambient: f32,
    pub shininess:f32,
    pub transparent:bool,
}
impl Default for Material {
    fn default() -> Self {
        Self {
            color: Vec3::new(1.0, 1.0, 1.0), //White
            main_texture:Texture2D::white(),
            specular: 0.5,
            ambient:0.05,
            shininess:32.0,
            transparent:false,
        }
    }
}
impl Component for Material {
    type Storage = VecStorage<Self>;
}
