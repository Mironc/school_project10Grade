use glam::Vec3;
use specs::{Component,HashMapStorage};

#[derive(Debug,Clone, Copy)]
pub enum Light{
    Point(LightProperties),
    Spotlight(LightProperties,Vec3),
}
impl Light {
    pub fn light_properties(&self) -> &LightProperties{
        match self {
            Light::Point(prop) => prop,
            Light::Spotlight(prop, _) => prop,
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub struct LightProperties{
    pub power:f32,
    pub color:Vec3,
}
impl LightProperties {
    
}
impl Component for Light {
    type Storage = HashMapStorage<Self>;
}
#[derive(Debug, Clone, Copy)]
pub struct Sun{

}