use std::ops::{Deref, DerefMut};
use std::sync::LazyLock;

use specs::*;
use transform::Transform;

use crate::define_vertex;
use crate::ecs::render_target::RenderTarget;
use crate::ecs::{MainCamera, Sun};
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
impl RenderSystem {
    pub fn new(
        lit_shading: impl LitShading + 'static,
        viewport: Viewport,
        postprocessing: impl PostProcessing + 'static,
    ) -> Self {
        let lit_shading = Box::new(lit_shading);
        let postprocessing = Box::new(postprocessing);
        Framebuffer::unbind();
        Self {
            lit_shading,
            viewport,
            postprocessing,
        }
    }
    pub fn render_with_main_camera<'a>(
        &mut self,
        transforms: &ReadStorage<'_, Transform>,
        models: &mut WriteStorage<'_, MeshRenderer>,
        camera_storage: &'a WriteStorage<'a, Camera>,
        main_camera: &'a Write<'a, MainCamera>,
        materials: &ReadStorage<'_, Material>,
        light_collection: &Vec<(&Light, &Transform)>,
        sun:&Read<'a,Sun>
    ) {
        let main_camera_ent = main_camera.get(camera_storage).unwrap();
        self.lit_shading.render_scene(
            transforms,
            models,
            &main_camera_ent,
            materials,
            light_collection,
            sun
        );
        main_camera.framebuffer().draw_bind();
        main_camera.viewport().set_gl_viewport();
        self.postprocessing.apply_to(
            &main_camera.framebuffer(),
            self.lit_shading
                .out_framebuffer()
                .attachment_texture(FramebufferAttachment::Color(0))
                .unwrap(),
        );
    }
}
impl<'a> System<'a> for RenderSystem {
    type SystemData = (
        WriteStorage<'a, MeshRenderer>,
        ReadStorage<'a, Light>,
        ReadStorage<'a, Material>,
        ReadStorage<'a, Transform>,
        WriteStorage<'a, Camera>,
        Write<'a, ResizeEvent>,
        Write<'a, MainCamera>,
        Read<'a, Sun>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut model,
            light,
            material,
            transform_storage,
            mut camera_storage,
            resize_event,
            main_camera,
            sun,
        ) = data;
        
        if resize_event.occured() {
            self.lit_shading.resize(resize_event.viewport);
            self.viewport = resize_event.viewport;
        }
        let light_collection = (&light, &transform_storage)
            .join()
            .collect::<Vec<(&Light, &Transform)>>();

        for camera in (&mut camera_storage).join() {
            if let RenderTarget::Texture(texture) = &camera.render_target {
                texture.draw_bind();
                texture.viewport().set_gl_viewport();
                self.lit_shading.render_scene(
                    &transform_storage,
                    &mut model,
                    camera,
                    &material,
                    &light_collection,
                    &sun
                );
                self.postprocessing.apply_to(
                    &texture,
                    texture
                        .attachment_texture(FramebufferAttachment::Color(0))
                        .unwrap(),
                );
            }
        }
        self.render_with_main_camera(
            &transform_storage,
            &mut model,
            &camera_storage,
            &main_camera,
            &material,
            &light_collection,
            &sun
        );
    }
}
#[derive(Debug, Clone, Copy, Default)]
pub struct ResizeEvent {
    dirty: bool,
    viewport: Viewport,
}
impl ResizeEvent {
    pub fn occured(&self) -> bool {
        self.dirty
    }
    pub fn viewport(&self) -> Viewport {
        self.viewport
    }
    ///Ends resize event
    pub fn end(&mut self){
        self.dirty = false;
    }
    pub fn send(&mut self, viewport: &Viewport) {
        self.viewport = *viewport;
        self.dirty = true;
    }
}
pub static FULL_SCREEN: LazyLock<InstancedModel> =
    LazyLock::new(|| InstancedModel::new_without_vertex(3));
pub static FULLSCREENPASS_VERTEX_SHADER: LazyLock<SubShader> =
    LazyLock::new(|| SubShader::new(include_str!("./shaders/opt_vert.glsl"), ShaderType::Vertex));
use vertex::{IntoGLenum, Vertex};

use super::postprocessing::PostProcessing;
use super::LitShading;
define_vertex!(SimpleV, pos, f32, 2, tex_coord, f32, 2);
