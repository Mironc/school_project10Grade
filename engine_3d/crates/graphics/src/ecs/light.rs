use glam::Vec3;
use specs::{Component, HashMapStorage};

#[derive(Debug, Clone, Copy)]
pub enum Light {
    Point(LightProperties),
    Spotlight(LightProperties, Vec3),
}
impl Light {
    pub fn light_properties(&self) -> &LightProperties {
        match self {
            Light::Point(prop) => prop,
            Light::Spotlight(prop, _) => prop,
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub struct LightProperties {
    pub power: f32,
    pub color: Vec3,
}
impl LightProperties {}
impl Component for Light {
    type Storage = HashMapStorage<Self>;
}
#[derive(Debug, Clone, Copy)]
pub struct Sun {
    direction: Option<Vec3>,
    color:Vec3,
}
impl Sun {
    pub fn new(direction: Vec3,color:Vec3) -> Self {
        Self {
            direction: Some(direction.normalize()),
            color,
        }
    }
    pub fn direction(&self) -> Option<Vec3> {
        self.direction
    }
    pub fn color(&self) -> Vec3{
        self.color
    }
    pub fn active(&self) -> bool {
        self.direction.is_some()
    }
    pub fn set_direction(&mut self, direction: Vec3) {
        self.direction = Some(direction.normalize())
    }
    pub fn set_color(&mut self, color: Vec3) {
        self.color = color
    }
    pub fn remove(&mut self) {
        self.direction = None
    }
    pub fn shadowmap() {
        todo!()
    }
}
impl Default for Sun {
    fn default() -> Self {
        Self {
            direction: None,
            color: Vec3::ZERO,
        }
    }
}
