use std::sync::LazyLock;

use specs::*;
use transform::Transform;

use graphics::define_vertex;
use crate::{{camera::Camera,}, light::Light, material::Material, mesh_renderer::MeshRenderer};
use crate::{camera::MainCamera, light::Sun};
use graphics::objects::model::InstancedModel;
use graphics::objects::shader::{ShaderType, SubShader};
use graphics::objects::viewport::Viewport;
use graphics::objects::vertex::{Vertex,IntoGLenum};
use graphics::utils::{end_debug_marker, start_debug_marker};
pub struct RenderSystem {
}
impl RenderSystem {
    pub fn new() -> Self {
        Self {  }
    }
    pub fn render_with_main_camera<'b>(
        &mut self,
        transforms: &ReadStorage<'_, Transform>,
        models: &mut WriteStorage<'_, MeshRenderer>,
        camera_storage: &mut WriteStorage<'b, Camera>,
        main_camera: &Write<'_, MainCamera>,
        materials: &ReadStorage<'_, Material>,
        light_collection: &Vec<(&Light, &Transform)>,
        sun: &Read<'_, Sun>,
    ) {
        let main_camera_ent = main_camera.get_mut(camera_storage).unwrap();
        start_debug_marker("Main camera");
        main_camera_ent.render_path.render(
            transforms,
            models,
            materials,
            light_collection,
            sun,
            main_camera_ent.frustum(),
            main_camera_ent.get_view(),
            main_camera_ent.projection().to_owned(),
            main_camera_ent.transform,
        );
        let _ = main_camera_ent
            .render_path
            .framebuffer()
            .copy_depth_to(&main_camera_ent.render_image);
        end_debug_marker();
    }
}
impl<'a> System<'a> for RenderSystem {
    type SystemData = (
        WriteStorage<'a, MeshRenderer>,
        ReadStorage<'a, Light>,
        ReadStorage<'a, Material>,
        ReadStorage<'a, Transform>,
        WriteStorage<'a, Camera>,
        Write<'a, MainCamera>,
        Read<'a, Sun>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut model,
            light,
            material,
            transform_storage,
            mut camera_storage,
            main_camera,
            sun,
            entities,
        ) = data;

        let light_collection = (&light, &transform_storage)
            .join()
            .collect::<Vec<(&Light, &Transform)>>();
        for (camera, entity) in
            (&mut camera_storage, &entities).join()
        {
            if main_camera.id() != Some(entity) {
                camera.render_path.render(
                    &transform_storage,
                    &mut model,
                    &material,
                    &light_collection,
                    &sun,
                    camera.frustum(),
                    camera.get_view(),
                    *camera.projection(),
                    camera.transform,
                );
                let _ = camera
                    .render_path
                    .framebuffer()
                    .copy_depth_to(&camera.render_image);
            }
        }
        self.render_with_main_camera(
            &transform_storage,
            &mut model,
            &mut camera_storage,
            &main_camera,
            &material,
            &light_collection,
            &sun,
        );
    }
}
define_vertex!(SimpleV, pos, f32, 2, tex_coord, f32, 2);
