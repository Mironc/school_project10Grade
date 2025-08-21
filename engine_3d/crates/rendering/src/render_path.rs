use graphics::objects::{buffers::Framebuffer, viewport::Viewport};
use math::Mat4;
use specs::{Read, ReadStorage, WriteStorage};
use transform::Transform;

use crate::{camera::{projection::Projection, CameraTransform, ViewFrustum}, light::{Light, Sun}, material::Material, mesh_renderer::MeshRenderer};


pub trait RenderPath {
    fn render(
        &mut self,
        transforms: &ReadStorage<'_, Transform>,
        models: &mut WriteStorage<'_, MeshRenderer>,
        materials: &ReadStorage<'_, Material>,
        light_collection: &Vec<(&Light, &Transform)>,
        sun: &Read<'_, Sun>,
        view_frustum: ViewFrustum,
        view_mat: Mat4,
        projection: Projection,
        camera_transform: CameraTransform,
    );
    fn resize(&mut self, viewport: Viewport);
    fn framebuffer(&self) -> &Framebuffer;
}