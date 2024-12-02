use glam::*;
use specs::{Component, VecStorage};
///Quaternion based
/// FIXME:independency of axes in quaternion causes some artifacts in use with Camera
/*
#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub rotation: Quat,
    pub position: Vec3,
    pub scale: Vec3,
}
impl Transform {
    pub fn new(position: Vec3, rotation: Vec3, scale: Vec3) -> Self {
        Self {
            rotation: Quat::from_euler(
                EulerRot::XYZ,
                rotation.x.to_radians(),
                rotation.y.to_radians(),
                rotation.z.to_radians(),
            ),
            position,
            scale,
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
            rotation: Quat::from_euler(
                EulerRot::XYZ,
                rotation.x.to_radians(),
                rotation.y.to_radians(),
                rotation.z.to_radians(),
            ),
            ..Default::default()
        }
    }
    pub fn translation_rotation(position: Vec3, rotation: Vec3) -> Self {
        Self {
            rotation: Quat::from_euler(
                EulerRot::XYZ,
                rotation.x.to_radians(),
                rotation.y.to_radians(),
                rotation.z.to_radians(),
            ),
            position,
            ..Default::default()
        }
    }
    pub fn rotate(&mut self, rotation: Quat) {
        self.rotation = rotation * self.rotation;
    }
    pub fn rotate_axis(&mut self,axis:Vec3,angle:f32){
        self.rotate(
            Quat::from_axis_angle(axis, angle)
        );
    }
    pub fn rotate_x(&mut self, x: f32) {
        self.rotate(Quat::from_rotation_x(x.to_radians()));
    }
    pub fn rotate_y(&mut self, y: f32) {
        self.rotate(Quat::from_rotation_y(y.to_radians()));
    }
    pub fn rotate_z(&mut self, z: f32) {
        self.rotate(Quat::from_rotation_z(z.to_radians()));
    }
    pub fn rotate_eulers(&mut self, rotation: Vec3) {
        self.rotate(Quat::from_euler(
            EulerRot::XYZ,
            rotation.x.to_radians(),
            rotation.y.to_radians(),
            rotation.z.to_radians(),
        ));
    }
    pub fn rotate_local(&mut self, rotation: Quat) {
        self.rotation *= rotation;
    }
    pub fn rotate_local_x(&mut self, x: f32) {
        self.rotate_local(Quat::from_rotation_x(x.to_radians()));
    }
    pub fn rotate_local_y(&mut self, y: f32) {
        self.rotate_local(Quat::from_rotation_y(y.to_radians()));
    }
    pub fn rotate_local_z(&mut self, z: f32) {
        self.rotate_local(Quat::from_rotation_z(z.to_radians()));
    }
    pub fn rotate_local_eulers(&mut self, rotation: Vec3) {
        self.rotate_local(Quat::from_euler(
            EulerRot::XYZ,
            rotation.x.to_radians(),
            rotation.y.to_radians(),
            rotation.z.to_radians(),
        ));
    }
    pub fn right(&self) -> Vec3 {
        self.rotation * Vec3::X
    }
    pub fn up(&self) -> Vec3 {
        self.rotation * Vec3::Y
    }
    pub fn forward(&self) -> Vec3 {
        self.rotation * Vec3::Z
    }
    pub fn get_matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.position)
    }
}
impl Default for Transform {
    fn default() -> Self {
        Self {
            rotation: Quat::IDENTITY,
            position: Vec3::ZERO,
            scale: Vec3::ONE,
        }
    }
}
impl Component for Transform {
    type Storage = VecStorage<Self>;
}
*/
pub struct Transform {
    pub position: Vec3,
    rotation: Vec3,
    pub scale: f32,
    rot_mat: Mat4,
}
impl Transform {
    pub fn new(position: Vec3, rotation: Vec3, scale: f32) -> Self {
        Self {
            rotation,
            position,
            scale,
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
        self._update_rot_mat();
    }
    pub fn translation_rotation(position: Vec3, rotation: Vec3) -> Self {
        Self {
            rotation,
            position,
            ..Default::default()
        }
    }
    pub fn rotate_x(&mut self, rotation: f32) {
        self.rotation.x += rotation;
        self._update_rot_mat();
    }
    pub fn rotate_y(&mut self, rotation: f32) {
        self.rotation.y += rotation;
        self._update_rot_mat();
    }
    pub fn rotate_z(&mut self, rotation: f32) {
        self.rotation.z += rotation;
        self._update_rot_mat();
    }
    pub fn rotate(&mut self, rotation: Vec3) {
        self.rotation += rotation;
        self._update_rot_mat();
    }
    fn _update_rot_mat(&mut self) {
        self.rot_mat = Mat4::from_rotation_x(self.rotation.x.to_radians())
            * Mat4::from_rotation_y(self.rotation.y.to_radians())
            * Mat4::from_rotation_z(self.rotation.z.to_radians())
    }
    pub fn right(&self) -> Vec3 {
        self.rot_mat.transform_vector3(Vec3::X)
    }
    pub fn up(&self) -> Vec3 {
        self.rot_mat.transform_vector3(Vec3::Y)
    }
    pub fn forward(&self) -> Vec3 {
        self.rot_mat.transform_vector3(Vec3::Z)
    }
    pub fn get_rotation_matrix(&self) -> Mat4{
        self.rot_mat
    }
    pub fn get_matrix(&self) -> Mat4 {
        Mat4::from_translation(self.position) * self.rot_mat * Mat4::from_scale(Vec3::ONE * self.scale) 
    }
}
impl Default for Transform {
    fn default() -> Self {
        Self {
            rotation: Vec3::ZERO,
            position: Vec3::ZERO,
            scale: 1.0,
            rot_mat: Mat4::IDENTITY,
        }
    }
}
impl Component for Transform {
    type Storage = VecStorage<Self>;
}
