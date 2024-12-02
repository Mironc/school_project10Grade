use glam::Mat4;
use specs::{Component, HashMapStorage};
use super::super::super::objects::viewport::Viewport;

#[derive(Debug, Clone, Copy)]
pub enum Projection{
    Perspective(Perspective),
    Orthogonal(Orthogonal)
}
impl Projection{
    pub fn get_projection(&self) -> Mat4{
        match self {
            Projection::Perspective(p) => p.projection(),
            Projection::Orthogonal(o) => o.projection(),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Orthogonal{
    z_far:f32,
    z_near:f32,
    width:f32,
    height:f32
}
impl Orthogonal{
    pub fn new(z_near:f32,z_far:f32,viewport:&Viewport) -> Self{
        Self{
            width: viewport.width() as f32,
            height: viewport.height() as f32,
            z_far,
            z_near,
        }
    }
    pub fn viewport_update(&mut self,viewport:&Viewport){
        self.width = viewport.width() as f32;
        self.height = viewport.height() as f32;
    }
    pub fn projection(&self) -> Mat4{
        Mat4::orthographic_rh_gl(0.0, self.width, 0.0, self.height, self.z_near, self.z_far)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Perspective{
    z_near:f32,
    z_far:f32,
    fov:f32,
    aspect:f32,
}
impl Perspective {
    pub fn new(z_near:f32,z_far:f32,fov:f32,viewport:&Viewport) -> Self{
        Self { z_near, z_far, fov, aspect:viewport.width() as f32 / viewport.height() as f32 }
    }
    pub fn viewport_update(&mut self,viewport:Viewport){
        self.aspect = viewport.width() as f32 / viewport.height() as f32;
    }
    pub fn z_far(&self) -> f32{
        self.z_far
    }
    pub fn z_near(&self) -> f32{
        self.z_near
    }
    pub fn fov(&self) -> f32{
        self.fov
    }
    pub fn aspect(&self) -> f32{
        self.aspect
    }
    pub fn projection(&self) -> Mat4{
        Mat4::perspective_rh_gl(self.fov.to_radians(), self.aspect, self.z_near, self.z_far)
    }
}
impl Default for Perspective {
    fn default() -> Self {
        Self { z_near: 0.01, z_far: 1024.0, fov: 45.0, aspect:1.0 }
    }
}
impl Component for Perspective {
    type Storage = HashMapStorage<Self>;
}