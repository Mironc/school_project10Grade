pub use super::main_camera::MainCamera;
use math::*;
pub use super::on_resize::OnResizeEvent;
use super::projection::Projection;
use super::render_target::RenderTarget;
use specs::{Component, HashMapStorage};

use graphics::objects::{
    buffers::{Framebuffer, FramebufferAttachment},
    texture::{Filter, Texture2DBuilder, TextureDataType, TextureFormat, TextureWrap},
    viewport::Viewport,
};

use crate::render_path::RenderPath;
#[derive(Debug, Clone, Copy)]
pub struct CameraTransform {
    pub position: Vec3,
    rotation: Vec3,
    forward: Vec3,
    right: Vec3,
    up: Vec3,
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
        }
        .normalize();
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
#[derive(Debug, Clone, Copy)]
pub struct Plane {
    normal: Vec3,
    distance: f32,
}
impl Plane {
    pub fn new(numbers: Vec4) -> Self {
        Self {
            normal: numbers.xyz(),
            distance: numbers.w,
        }
    }
    pub fn signed_distance(&self, point: Vec3) -> f32 {
        self.normal.dot(point) + self.distance
    }
    pub fn normal(&self) -> Vec3 {
        self.normal
    }
    pub fn distance(&self) -> f32 {
        self.distance
    }
}
pub struct ViewFrustum {
    planes: [Plane; 6],
}
impl ViewFrustum {
    pub fn new(camera: &Camera) -> Self {
        let vp = camera.proj_mat() * camera.get_view();
        let planes = [
            Plane::new(vp.row(3) + vp.row(0)),
            Plane::new(vp.row(3) - vp.row(0)),
            Plane::new(vp.row(3) + vp.row(1)),
            Plane::new(vp.row(3) - vp.row(1)),
            Plane::new(vp.row(3) + vp.row(2)),
            Plane::new(vp.row(3) - vp.row(2)),
        ];
        Self { planes }
    }
    ///using custom function for frustum check
    pub fn in_frustum(&self, test_fn: &mut impl FnMut(&Plane) -> bool) -> bool {
        self.planes.iter().all(|p| test_fn(p))
    }
}
pub struct Camera {
    pub transform: CameraTransform,
    projection: Projection,
    scale_factor: f32,
    pub render_image: Framebuffer,
    pub render_path: Box<dyn RenderPath>,
    //pub render_target: RenderTarget,
}
impl Camera {
    pub fn new(
        projection: Projection,
        transform: CameraTransform,
        viewport: Viewport,
        render_path: impl RenderPath + 'static,
    ) -> Self {
        let mut framebuffer = Framebuffer::new(viewport);
        let _ = framebuffer.create_attachment(
            FramebufferAttachment::Color(0),
            Texture2DBuilder::new()
                .filter(Filter::Nearest)
                .wrap(TextureWrap::Repeat)
                .internal_format(TextureFormat::RGBA8),
        );
        let _ = framebuffer.create_attachment(
            FramebufferAttachment::DepthStencil,
            Texture2DBuilder::new()
                .filter(Filter::Nearest)
                .wrap(TextureWrap::Repeat)
                .internal_format(TextureFormat::Depth32FStencil8)
                .texture_format(TextureFormat::DepthStencilComponent)
                .texture_type(TextureDataType::Float32UnsignedInt8),
        );
        Self {
            projection,
            transform,
            render_image: framebuffer,
            render_path: Box::new(render_path),
            scale_factor: 1.0,
        }
    }
    pub fn projection(&self) -> &Projection {
        &self.projection
    }
    pub fn set_scale_factor(&mut self, factor: f32) {
        let mut viewport = *self.render_path.framebuffer().viewport();
        viewport.div_scale(self.scale_factor);
        self.render_path.resize(viewport);
        self.render_image = self.render_image.resize(viewport).unwrap();
        self.scale_factor = factor;
    }
    pub fn set_viewport(&mut self, mut viewport: Viewport) {
        viewport.mul_scale(self.scale_factor);
        println!("{} {}", viewport.width(), viewport.height());
        match self.projection {
            Projection::Perspective(mut perspective) => perspective.set_viewport(viewport),
            Projection::Orthogonal(mut orthogonal) => orthogonal.set_viewport(viewport),
        }
        self.render_path.resize(viewport);
        self.render_image = self.render_image.resize(viewport).unwrap();
    }
    pub fn get_view(&self) -> Mat4 {
        Mat4::look_to_rh(
            self.transform.position,
            -self.transform.forward(),
            self.transform.up(),
        )
    }
    ///returns camera frustum planes as Vec4 where xyz is normal of plane and w is distance
    pub fn frustum(&self) -> ViewFrustum {
        ViewFrustum::new(&self)
    }
    pub fn proj_mat(&self) -> Mat4 {
        self.projection.get_projection()
    }

    pub fn render_image(&self) -> &Framebuffer {
        &self.render_image
    }
}
impl Component for Camera {
    type Storage = HashMapStorage<Self>;
}
