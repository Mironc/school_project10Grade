use std::sync::LazyLock;

use specs::*;
use transform::Transform;

use crate::define_vertex;
use crate::ecs::{camera::Camera, Light, Material, MeshRenderer};
use crate::objects::model::InstancedModel;
use crate::objects::shader::{ShaderType, SubShader};
use crate::objects::{buffers::Framebuffer, viewport::Viewport};
use crate::objects::{buffers::*, vertex};
pub struct RenderSystem {
    lit_shading: Box<dyn LitShading>,
    postprocessing: Box<dyn PostProcessing>,
    viewport: Viewport,
}
impl<'a> System<'a> for RenderSystem {
    type SystemData = (
        WriteStorage<'a, MeshRenderer>,
        ReadStorage<'a, Light>,
        ReadStorage<'a, Material>,
        ReadStorage<'a, Transform>,
        WriteStorage<'a, Camera>,
        Write<'a, ResizeEvent>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut model, light, material, transform_storage, mut camera, mut resize_event) = data;
        if resize_event.dirty {
            self.lit_shading.resize(resize_event.viewport);
            self.viewport = resize_event.viewport;
            resize_event.dirty = false;
        }
        let light_collection = (&light, &transform_storage)
            .join()
            .collect::<Vec<(&Light, &Transform)>>();
        self.lit_shading.render_scene(
            &transform_storage,
            &mut model,
            &mut camera,
            &material,
            &light_collection,
        );
        let out_texture = self
            .lit_shading
            .out_framebuffer()
            .attachment_texture(FramebufferAttachment::Color(0))
            .unwrap();

        Framebuffer::unbind();
        self.viewport.set_gl_viewport();
        self.postprocessing
            .apply_to(&Framebuffer::default(), out_texture);
    }
}
#[derive(Debug, Clone, Copy, Default)]
pub struct ResizeEvent {
    dirty: bool,
    viewport: Viewport,
}
impl ResizeEvent {
    pub fn occured(&self) -> bool{
        self.dirty
    }
    pub fn viewport(&self) -> Viewport{
        self.viewport
    }
    pub fn send(&mut self, viewport: &Viewport) {
        self.viewport = *viewport;
        self.dirty = true;
    }
}
impl RenderSystem {
    pub fn new(
        lit_shading: impl LitShading + 'static,
        viewport: &Viewport,
        postprocessing: impl PostProcessing + 'static,
    ) -> Self {
        let lit_shading = Box::new(lit_shading);
        let postprocessing = Box::new(postprocessing);
        Framebuffer::unbind();
        Self {
            lit_shading,
            viewport: *viewport,
            postprocessing,
        }
    }
}
pub static BLIT_MODEL: LazyLock<InstancedModel> =
    LazyLock::new(|| InstancedModel::new_without_vertex(3));
pub static BLIT_VERTEX_SHADER: LazyLock<SubShader> =
    LazyLock::new(|| SubShader::new(include_str!("./opt_vert.glsl"), ShaderType::Vertex));
use vertex::{IntoGLenum, Vertex};

use super::postprocessing::PostProcessing;
use super::LitShading;
define_vertex!(SimpleV, pos, f32, 2, tex_coord, f32, 2);