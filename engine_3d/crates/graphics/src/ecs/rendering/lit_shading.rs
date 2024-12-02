use glam::{Mat4, Vec3, Vec4};
use specs::{Join, ReadStorage, WriteStorage};
use transform::Transform;

use crate::{
    color::{self, Color},
    compare_opt::CompareOption,
    depth::{self},
    ecs::{Camera, Light, Material, MeshRenderer},
    face_culling::{self, CullFace, FrontFaceOrder},
    impl_data_type,
    objects::{
        buffers::{Buffer, ClearFlags, Framebuffer, FramebufferAttachment, ShaderStorage},
        model::{from_str, primitives::SimpleVertex, InstancedModel, Model},
        shader::{Shader, ShaderType, SubShader},
        texture::{Filter, Texture2DBuilder, TextureFormat, TextureType},
        vertex::ModelVertex,
        viewport::Viewport,
    },
    stencil::{self, Action, StencilFunction, StencilOptions},
    utils::{end_debug_marker, start_debug_marker},
};
pub trait LitShading {
    fn render_scene(
        &mut self,
        transforms: &ReadStorage<'_, Transform>,
        models: &mut WriteStorage<'_, MeshRenderer>,
        cameras: &mut WriteStorage<'_, Camera>,
        materials: &ReadStorage<'_, Material>,
        light_collection: &Vec<(&Light, &Transform)>,
    );
    fn out_framebuffer(&self) -> &Framebuffer;
    fn resize(&mut self, viewport: Viewport);
}
pub struct DeferredRendering {
    out_framebuffer: Framebuffer,
    stencil_pass: Shader,
    light_pass: Shader,
    g_buffer: Framebuffer,
    geometry_pass: Shader,
    light_props: Buffer<ShaderStorage>,
    light_volume: InstancedModel,
}
impl DeferredRendering {
    pub fn new(viewport: Viewport) -> Self {
        let mut out_framebuffer = Framebuffer::new(viewport);
        out_framebuffer.create_attachment(
            FramebufferAttachment::Color(0),
            Texture2DBuilder::new()
                .internal_format(TextureFormat::RGBA16F)
                .filter(Filter::Nearest)
                .texture_type(TextureType::Float),
        );
        let depth = Texture2DBuilder::new()
            .size((1, 1))
            .internal_format(TextureFormat::Depth24Stencil8)
            .texture_format(TextureFormat::DepthStencilComponent)
            .texture_type(TextureType::UnsignedInt24_8)
            .filter(Filter::Nearest)
            .build()
            .unwrap();
        out_framebuffer.add_attachment(FramebufferAttachment::DepthStencil, depth);

        let mut g_buffer = Framebuffer::new(viewport);
        //position attachment
        /*framebuffer.create_attachment(
            FrameBufferAttachment::Color(0),
            Texture2DBuilder::new()
                .internal_format(TextureFormat::RGB16F)
                .filter(Filter::Nearest)
                .texture_type(TextureType::Float),
        );*/
        //normal attachment + shininess
        g_buffer.create_attachment(
            FramebufferAttachment::Color(0),
            Texture2DBuilder::new()
                .internal_format(TextureFormat::RGB10A2)
                .texture_format(TextureFormat::RGB)
                .filter(Filter::Nearest),
        );
        //color attachment + specular
        g_buffer.create_attachment(
            FramebufferAttachment::Color(1),
            Texture2DBuilder::new()
                .internal_format(TextureFormat::RGBA8)
                .filter(Filter::Nearest),
        );
        g_buffer.create_attachment(
            FramebufferAttachment::Depth,
            Texture2DBuilder::new()
                .internal_format(TextureFormat::DepthComponent)
                .texture_format(TextureFormat::DepthComponent)
                .texture_type(TextureType::Float),
        );

        let light_pass = Shader::new([
            SubShader::new(
                include_str!("./shaders/deferred_shading_vert.glsl"),
                ShaderType::Vertex,
            ),
            SubShader::new(
                include_str!("./shaders/deferred_shading_frag.glsl"),
                ShaderType::Fragment,
            ),
        ]);
        let stencil_pass = Shader::new([SubShader::new(
            include_str!("./shaders/deferred_shading_vert.glsl"),
            ShaderType::Vertex,
        )]);
        let geometry_pass = Shader::new([
            SubShader::new(
                include_str!("./shaders/gbuffer_pass_vert.glsl"),
                ShaderType::Vertex,
            ),
            SubShader::new(
                include_str!("./shaders/gbuffer_pass_frag.glsl"),
                ShaderType::Fragment,
            ),
        ]);

        let model: Model<ModelVertex> = from_str(include_str!("./icosahedron.obj")).unwrap();
        let mut verticies = Vec::new();
        for vert in model.verticies.iter() {
            verticies.push(SimpleVertex::new(vert.position));
        }
        let light_volume = Model::new(verticies, model.indicies.clone()).instantiate();
        let light_props = Buffer::<ShaderStorage>::gen();
        Self {
            stencil_pass,
            out_framebuffer,
            light_pass,
            g_buffer,
            geometry_pass,
            light_props,
            light_volume,
        }
    }
}
impl LitShading for DeferredRendering {
    fn render_scene(
        &mut self,
        transforms: &ReadStorage<'_, Transform>,
        models: &mut WriteStorage<'_, MeshRenderer>,
        cameras: &mut WriteStorage<'_, Camera>,
        materials: &ReadStorage<'_, Material>,
        light_collection: &Vec<(&Light, &Transform)>,
    ) {
        //geometry pass
        self.g_buffer.bind();
        self.g_buffer.viewport().set_gl_viewport();
        depth::set_write(true);
        self.g_buffer.clear(ClearFlags::Color | ClearFlags::Depth);
        self.g_buffer.clear_color(Color::new(0.0, 0.0, 0.0, 1.0));
        depth::enable();
        depth::set_cmp_func(CompareOption::LessEqual);
        face_culling::enable();
        face_culling::set_cullface(CullFace::Front);
        face_culling::set_frontface_order(FrontFaceOrder::Clockwise);

        for camera in cameras.join() {
            for (mesh_renderer, transform, material) in (&mut *models, transforms, materials).join()
            {
                self.geometry_pass.bind();
                self.geometry_pass
                    .set_matrix4("projection", &camera.get_projection_mat());
                self.geometry_pass.set_matrix4("view", &camera.get_view());
                self.geometry_pass
                    .set_matrix4("transformation", &transform.get_matrix());
                self.geometry_pass
                    .set_texture2d("main_texture", &material.main_texture, 1);
                self.geometry_pass.set_vec3("color", &material.color);
                self.geometry_pass.set_f32("specular", material.specular);
                self.geometry_pass.set_f32("shininess", material.shininess);
                mesh_renderer.model.draw();
            }
        }
        //Lightning pass
        self.g_buffer
            .copy_depth_to(&self.out_framebuffer, Filter::Nearest);
        self.out_framebuffer.bind();
        stencil::enable();
        self.out_framebuffer
            .clear(ClearFlags::Color | ClearFlags::Stencil);
        self.out_framebuffer
            .clear_color(Color::new(0.0, 0.0, 0.0, 1.0));
        depth::set_write(false);
        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::ONE, gl::ONE);
        }
        let position = self
            .g_buffer
            .attachment_texture(FramebufferAttachment::Depth)
            .unwrap();
        let normal = self
            .g_buffer
            .attachment_texture(FramebufferAttachment::Color(0))
            .unwrap();
        let color_spec = self
            .g_buffer
            .attachment_texture(FramebufferAttachment::Color(1))
            .unwrap();
        self.light_pass.bind();
        self.light_pass.set_texture2d("position", &position, 0);
        self.light_pass.set_texture2d("normal", &normal, 1);
        self.light_pass.set_texture2d("color_spec", &color_spec, 2);

        //filling light sources data
        let mut lights = Vec::new();
        for light in light_collection.iter() {
            let light_prop = light.0.light_properties();
            let transform = light.1;
            let transf = Mat4::from_translation(transform.position)
                * transform.get_rotation_matrix()
                * Mat4::from_scale(
                    glam::Vec3::ONE * 10.0 * (light.0.light_properties().power).sqrt(),
                );
            lights.push(LightProps {
                transf,
                light_color: light_prop.color.extend(1.0),
                light_position: light.1.position.extend(1.0),
                light_power: light_prop.power,
            });
        }
        self.light_props.set_data(&lights);
        self.light_pass
            .set_shader_storage_block("lights", &self.light_props, 1);
        self.stencil_pass.bind();
        self.stencil_pass
            .set_shader_storage_block("lights", &self.light_props, 1);

        for camera in cameras.join() {
            start_debug_marker("stencil pass");
            face_culling::set_cullface(CullFace::Front);
            depth::set_cmp_func(CompareOption::Less);
            stencil::set_stencil_function(StencilFunction::with_no_mask(CompareOption::Always, 1));
            color::set_write(false, false, false, false);
            stencil::set_stencil_options(StencilOptions::new(
                Action::Keep,
                Action::Keep,
                Action::Replace,
            ));
            self.light_pass.bind();
            self.light_pass
                .set_vec3("camera_position", &camera.transform.position);
            self.light_pass
                .set_matrix4("vp", &(camera.get_projection_mat() * camera.get_view()));
            self.light_pass.set_matrix4(
                "inv_vp",
                &(camera.get_projection_mat() * camera.get_view()).inverse(),
            );
            for (i, (light, light_transform)) in light_collection.iter().enumerate() {
                if camera.transform.position.distance(light_transform.position)
                    > 10.0 * (light.light_properties().power).sqrt()
                {
                    self.light_pass.set_int("instance", i as i32);
                    self.light_volume.draw();
                }
            }
            end_debug_marker();

            start_debug_marker("light pass");
            color::set_write(true, true, true, true);
            face_culling::set_cullface(CullFace::Back);
            depth::set_cmp_func(CompareOption::GreaterEqual);
            stencil::set_stencil_function(StencilFunction::with_no_mask(
                CompareOption::GreaterEqual,
                1,
            ));
            stencil::set_stencil_options(StencilOptions::new(
                Action::Keep,
                Action::Keep,
                Action::Keep,
            ));
            color::set_write(true, true, true, true);
            for (i, (light, light_transform)) in light_collection.iter().enumerate() {
                self.light_pass.set_int("instance", i as i32);
                self.light_volume.draw();
                //self.light_volume.draw_instanced(light_collection.len() as i32);
            }
            end_debug_marker();
        }
        stencil::disable();
        unsafe {
            gl::Disable(gl::BLEND);
        }
    }

    fn out_framebuffer(&self) -> &Framebuffer {
        &self.out_framebuffer
    }

    fn resize(&mut self, viewport: Viewport) {
        self.out_framebuffer = self.out_framebuffer.resize(viewport).unwrap();
        self.g_buffer = self.g_buffer.resize(viewport).unwrap();
    }
}
pub struct ForwardRendering {
    pub out_framebuffer: Framebuffer,
    shader: Shader,
    depth_prepass: bool,
    light_props: Buffer<ShaderStorage>,
}
impl ForwardRendering {
    pub fn new(viewport: Viewport, depth_prepass: bool) -> Self {
        let mut out_framebuffer = Framebuffer::new(viewport);
        out_framebuffer.create_attachment(
            FramebufferAttachment::Color(0),
            Texture2DBuilder::new()
                .internal_format(TextureFormat::RGBA16F)
                .filter(Filter::Nearest)
                .texture_type(TextureType::Float),
        );
        let depth_texture = Texture2DBuilder::new()
            .size((1, 1))
            .internal_format(TextureFormat::DepthComponent)
            .texture_format(TextureFormat::DepthComponent)
            .filter(Filter::Nearest)
            .texture_type(TextureType::UnsignedByte)
            .build()
            .unwrap();
        out_framebuffer.add_attachment(FramebufferAttachment::Depth, depth_texture.clone());
        let shader = Shader::new([
            SubShader::new(
                include_str!("./shaders/forward_vert.glsl"),
                ShaderType::Vertex,
            ),
            SubShader::new(
                include_str!("./shaders/forward_frag.glsl"),
                ShaderType::Fragment,
            ),
        ]);
        let light_props = Buffer::gen();
        Self {
            depth_prepass,
            out_framebuffer,
            shader,
            light_props,
        }
    }
}
impl LitShading for ForwardRendering {
    fn render_scene(
        &mut self,
        transforms: &ReadStorage<'_, Transform>,
        models: &mut WriteStorage<'_, MeshRenderer>,
        cameras: &mut WriteStorage<'_, Camera>,
        materials: &ReadStorage<'_, Material>,
        light_collection: &Vec<(&Light, &Transform)>,
    ) {
        self.out_framebuffer.bind();
        self.out_framebuffer.viewport().set_gl_viewport();
        depth::enable();
        depth::set_write(true);
        self.out_framebuffer
            .clear(ClearFlags::Color | ClearFlags::Depth);
        self.out_framebuffer
            .clear_color(Color::new(0.0, 0.0, 0.0, 1.0));
        face_culling::enable();
        face_culling::set_cullface(CullFace::Front);
        face_culling::set_frontface_order(FrontFaceOrder::Clockwise);

        //filling in light sources data
        let mut lights = Vec::new();
        for light in light_collection.iter() {
            let light_prop = light.0.light_properties();
            let transform = light.1;
            let transf = Mat4::from_translation(transform.position)
                * transform.get_rotation_matrix()
                * Mat4::from_scale(Vec3::ONE * 20.0 * (light.0.light_properties().power).sqrt());
            lights.push(LightProps {
                transf,
                light_color: light_prop.color.extend(1.0),
                light_position: light.1.position.extend(1.0),
                light_power: light_prop.power,
            });
        }
        self.light_props.set_data(&lights);
        self.shader
            .set_shader_storage_block("lights", &self.light_props, 1);
        for camera in cameras.join() {
            //depth pass
            depth::set_cmp_func(CompareOption::LessEqual);
            if self.depth_prepass {
                color::set_write(false, false, false, false);
                for (mesh_renderer, transform, _) in (&mut *models, transforms, materials).join() {
                    self.shader.bind();
                    self.shader
                        .set_matrix4("projection", &camera.get_projection_mat());
                    self.shader.set_matrix4("view", &camera.get_view());
                    self.shader
                        .set_matrix4("transformation", &transform.get_matrix());
                    mesh_renderer.model.draw();
                }
                depth::set_cmp_func(CompareOption::Equal);
                depth::set_write(false);
                color::set_write(true, true, true, true);
            }
            for (mesh_renderer, transform, material) in (&mut *models, transforms, materials).join()
            {
                self.shader.bind();
                //TOOD:finish
                self.shader
                    .set_matrix4("projection", &camera.get_projection_mat());
                self.shader.set_matrix4("view", &camera.get_view());
                self.shader
                    .set_matrix4("transformation", &transform.get_matrix());
                self.shader
                    .set_texture2d("main_texture", &material.main_texture, 1);
                self.shader.set_vec3("color", &material.color);
                self.shader.set_f32("specular", material.specular);
                self.shader
                    .set_vec3("camera_position", &camera.transform.position);
                self.shader.set_f32("shininess", material.shininess);
                self.shader.set_f32("ambient", material.ambient);
                self.shader
                    .set_int("light_count", light_collection.len() as i32);
                mesh_renderer.model.draw();
            }
        }
    }

    fn out_framebuffer(&self) -> &Framebuffer {
        &self.out_framebuffer
    }

    fn resize(&mut self, viewport: Viewport) {
        self.out_framebuffer = self.out_framebuffer.resize(viewport).unwrap();
    }
}
use crate::objects::buffers::DataType;
impl_data_type!(
    LightProps,
    transf,
    Mat4,
    light_color,
    Vec4,
    light_position,
    Vec4,
    light_power,
    f32
);
