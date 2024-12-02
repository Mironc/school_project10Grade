use glam::*;
use projection::Projection;
use render_target::RenderTarget;
use specs::{Component, HashMapStorage};
mod main_camera;
pub use main_camera::MainCamera;
mod main_camera_system;
pub use main_camera_system::MainCameraSystem;
pub mod projection;
pub mod render_target;
pub struct CameraTransform {
    pub position: Vec3,
    rotation: Vec3,
    forward:Vec3,
    right:Vec3,
    up:Vec3,
}
impl CameraTransform {
    pub fn new(position: Vec3, rotation: Vec3) -> Self {
        Self {
            rotation,
            position,
            ..Default::default()
        }
    }
    pub fn from_position(position: Vec3) -> Self {
        Self {
            position,
            ..Default::default()
        }
    }
    pub fn from_rotation(rotation: Vec3) -> Self {
        Self {
            rotation,
            ..Default::default()
        }
    }
    pub fn rotation(&self) -> Vec3 {
        self.rotation
    }
    pub fn set_rotation(&mut self, rotation: Vec3) {
        self.rotation = rotation;
        self._update_vectors();
    }
    pub fn rotate_x(&mut self, rotation: f32) {
        self.rotation.x += rotation;
        self._update_vectors();
    }
    pub fn rotate_y(&mut self, rotation: f32) {
        self.rotation.y += rotation;
        self._update_vectors();
    }
    pub fn rotate_z(&mut self, rotation: f32) {
        self.rotation.z += rotation;
        self._update_vectors();
    }
    pub fn rotate(&mut self, rotation: Vec3) {
        self.rotation += rotation;
        self._update_vectors();
    }
    fn _update_vectors(&mut self) {
        self.forward = Vec3 {
            x: self.rotation.y.to_radians().cos() * self.rotation.x.to_radians().cos(),
            y: self.rotation.x.to_radians().sin(),
            z: self.rotation.y.to_radians().sin() * self.rotation.x.to_radians().cos(),
        }.normalize();
        self.right = (self.forward).cross(Vec3::Y).normalize();
        self.up = self.right.cross(self.forward).normalize();
    }
    pub fn right(&self) -> Vec3 {
        self.right
    }
    pub fn up(&self) -> Vec3 {
        self.up
    }
    pub fn forward(&self) -> Vec3 {
        self.forward
    }
}
impl Default for CameraTransform {
    fn default() -> Self {
        Self {
            rotation: Vec3::ZERO,
            position: Vec3::ZERO,
            forward: Vec3::Z,
            right: Vec3::X,
            up: Vec3::Y,
        }
    }
}
pub struct Camera {
    pub transform:CameraTransform,
    projection: Projection,
    pub render_target: RenderTarget,
}
impl Camera {
    pub fn new(projection: Projection,transform:CameraTransform) -> Self {
        Self {
            projection,
            render_target: RenderTarget::Texture(None),
            transform,
        }
    }
    pub fn projection(&self) -> &Projection {
        &self.projection
    }
    pub fn get_view(&self) -> Mat4 {
        /*println!(
            "camera vectors transform:{:?} forward:{:?} up:{:?}",
            transform.position,
            -transform.forward(),
            transform.up()
        );*/
        Mat4::look_to_rh(self.transform.position, -self.transform.forward(), self.transform.up())
    }
    pub fn get_projection_mat(&self) -> Mat4 {
        self.projection.get_projection()
    }
}
impl Component for Camera {
    type Storage = HashMapStorage<Self>;
}
