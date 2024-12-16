use glam::{Mat4, Vec3, Vec4};
use specs::{Join, Read, ReadStorage, WriteStorage};
use transform::Transform;

use crate::{
    color::{self, Color},
    compare_opt::CompareOption,
    depth::{self},
    ecs::{Camera, Light, Material, MeshRenderer, Plane, Sun},
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
    utils::{end_debug_marker, error_check, start_debug_marker},
};
pub trait LitShading {
    fn render_scene(
        &mut self,
        transforms: &ReadStorage<'_, Transform>,
        models: &mut WriteStorage<'_, MeshRenderer>,
        camera: &Camera,
        materials: &ReadStorage<'_, Material>,
        light_collection: &Vec<(&Light, &Transform)>,
        sun:&Read<'_, Sun>,
    );
    fn out_framebuffer(&self) -> &Framebuffer;
    fn resize(&mut self, viewport: Viewport);
}
pub struct DeferredRendering {
    out_framebuffer: Framebuffer,
    stencil_pass: Shader,
    g_buffer: Framebuffer,
    geometry_pass: Shader,

    point_light_pass: Shader,
    point_light_props: Buffer<ShaderStorage>,
    point_light_volume: InstancedModel,

    sun_light_pass:Shader,
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
                .internal_format(TextureFormat::RGBA8SNorm)
                .filter(Filter::Nearest),
        );
        //color attachment + specular
        g_buffer.create_attachment(
            FramebufferAttachment::Color(1),
            Texture2DBuilder::new()
                .internal_format(TextureFormat::RGBA)
                .texture_type(TextureType::Byte)
                .filter(Filter::Linear),
        );
        g_buffer.create_attachment(
            FramebufferAttachment::Depth,
            Texture2DBuilder::new()
                .internal_format(TextureFormat::DepthComponent)
                .texture_format(TextureFormat::DepthComponent)
                .texture_type(TextureType::Float),
        );

        let point_light_pass = Shader::new([
            SubShader::new(
                include_str!("./shaders/deferred_shading_vert.glsl"),
                ShaderType::Vertex,
            ),
            SubShader::new(
                include_str!("./shaders/deferred_shading_point_frag.glsl"),
                ShaderType::Fragment,
            ),
        ]);
        let sun_light_pass = Shader::new([*FULLSCREENPASS_VERTEX_SHADER,SubShader::new(include_str!("./shaders/deferred_shading_sun_frag.glsl"), ShaderType::Fragment)]);
        let stencil_pass = Shader::new([SubShader::new(
            include_str!("./shaders/deferred_shading_vert.glsl"),
            ShaderType::Vertex,
        )]);
        let geometry_pass = Shader::new([
            SubShader::new(
                include_str!("./shaders/gbuffer_vert.glsl"),
                ShaderType::Vertex,
            ),
            SubShader::new(
                include_str!("./shaders/gbuffer_frag.glsl"),
                ShaderType::Fragment,
            ),
        ]);

        let point_light_volume: Model<ModelVertex> = from_str(include_str!("./icosahedron.obj")).unwrap();
        let mut verticies = Vec::new();
        for vert in point_light_volume.verticies.iter() {
            verticies.push(SimpleVertex::new(vert.position));
        }
        let point_light_volume = Model::new(verticies, point_light_volume.indicies.clone()).instantiate();
        let point_light_props = Buffer::<ShaderStorage>::gen();
        Self {
            stencil_pass,
            out_framebuffer,
            point_light_pass,
            g_buffer,
            geometry_pass,
            point_light_props,
            point_light_volume,
            sun_light_pass,
        }
    }
}
impl LitShading for DeferredRendering {
    fn render_scene(
        &mut self,
        transforms: &ReadStorage<'_, Transform>,
        models: &mut WriteStorage<'_, MeshRenderer>,
        camera: &Camera,
        materials: &ReadStorage<'_, Material>,
        light_collection: &Vec<(&Light, &Transform)>,
        sun:&Read<'_,Sun>
    ) {
        //geometry pass
        self.g_buffer.draw_bind();
        self.g_buffer.viewport().set_gl_viewport();
        depth::set_write(true);
        self.g_buffer.clear(ClearFlags::Color | ClearFlags::Depth);
        self.g_buffer.clear_color(Color::new(0.0, 0.0, 0.0, 1.0));
        depth::enable();
        depth::set_cmp_func(CompareOption::LessEqual);
        face_culling::enable();
        face_culling::set_cullface(CullFace::Front);
        face_culling::set_frontface_order(FrontFaceOrder::Clockwise);
        for (mesh_renderer, transform, material) in (&mut *models, transforms, materials).join() {
            self.geometry_pass.bind();
            self.geometry_pass
                .set_matrix4("projection", &camera.get_projection());
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
        //Lightning pass
        self.g_buffer
            .copy_depth_to(&self.out_framebuffer, Filter::Nearest);
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
        self.point_light_pass.bind();
        self.point_light_pass.set_texture2d("position", &position, 0);
        self.point_light_pass.set_texture2d("normal", &normal, 1);
        self.point_light_pass.set_texture2d("color_spec", &color_spec, 2);

        //filling light sources data
        let mut lights = Vec::new();
        for (light, light_transform) in light_collection.iter() {
            let mut sphere_test = move |p: &Plane| {
                let res = p.signed_distance(light_transform.position)
                >= -10.0 * (light.light_properties().power).sqrt();
                return res;
            };
            if camera.frustum().in_frustum(&mut sphere_test) {
                let light_prop = light.light_properties();
                let transf = Mat4::from_translation(light_transform.position)
                    * light_transform.get_rotation_matrix()
                    * Mat4::from_scale(
                        glam::Vec3::ONE * 10.0 * (light.light_properties().power).sqrt(),
                    );
                lights.push(LightProps {
                    transf,
                    light_color: light_prop.color.extend(1.0),
                    light_position: light_transform.position.extend(1.0),
                    light_power: light_prop.power,
                });
            }
        }
        self.point_light_props.set_data(&lights);
        self.point_light_pass
            .set_shader_storage_block("lights", &self.point_light_props, 1);
        self.stencil_pass.bind();
        self.stencil_pass
            .set_shader_storage_block("lights", &self.point_light_props, 1);
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
        self.point_light_pass.bind();
        self.point_light_pass
            .set_vec3("camera_position", &camera.transform.position);
        self.point_light_pass
            .set_matrix4("vp", &(camera.get_projection() * camera.get_view()));
        self.point_light_pass.set_matrix4(
            "inv_vp",
            &(camera.get_projection() * camera.get_view()).inverse(),
        );

        for (i, _) in lights.iter().enumerate() {
            self.point_light_pass.set_int("instance", i as i32);
            self.point_light_volume.draw();
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
        for (i, _) in lights.iter().enumerate() {
            self.point_light_pass.set_int("instance", i as i32);
            self.point_light_volume.draw();
        }

        if let Some(direction) = sun.direction(){
            self.sun_light_pass.bind();
            self.sun_light_pass.set_texture2d("position", &position, 0);
            self.sun_light_pass.set_texture2d("normal", &normal, 1);
            self.sun_light_pass.set_texture2d("color_spec", &color_spec, 2);
            self.sun_light_pass.set_vec3("light_direction", &direction);
            self.sun_light_pass.set_vec3("light_color", &sun.color());
            self.sun_light_pass.set_vec3("camera_position", &camera.transform.position);
            self.sun_light_pass.set_matrix4(
                "inv_vp",
                &(camera.get_projection() * camera.get_view()).inverse(),
            );

            face_culling::disable();
            stencil::disable();
            depth::disable();
            FULL_SCREEN.draw();
        }
        end_debug_marker();
        stencil::disable();
        unsafe {
            gl::Disable(gl::BLEND);
        }
    }

    fn out_framebuffer(&self) -> &Framebuffer {
        &self.out_framebuffer
    }

    fn resize(&mut self, mut viewport: Viewport) {
        //viewport.set_size(viewport.width()/2, viewport.height()/2);
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
            .internal_format(TextureFormat::DepthComponent)
            .texture_format(TextureFormat::DepthComponent)
            .filter(Filter::Nearest)
            .texture_type(TextureType::UnsignedByte);
        out_framebuffer.create_attachment(FramebufferAttachment::Depth, depth_texture.clone());
        println!("{:?}", error_check());
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
        camera: &Camera,
        materials: &ReadStorage<'_, Material>,
        light_collection: &Vec<(&Light, &Transform)>,
        sun:&Read<'_,Sun>
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
                * Mat4::from_scale(Vec3::ONE * 10.0 * (light.0.light_properties().power).sqrt());
            lights.push(LightProps {
                transf,
                light_color: light_prop.color.extend(1.0),
                light_position: light.1.position.extend(1.0),
                light_power: light_prop.power,
            });
        }
        self.light_props.set_data(&lights);

        self.shader.bind();
        self.shader
            .set_shader_storage_block("lights", &self.light_props, 1);
        //depth pass
        depth::set_cmp_func(CompareOption::LessEqual);
        if self.depth_prepass {
            color::set_write(false, false, false, false);
            for (mesh_renderer, transform, _) in (&mut *models, transforms, materials).join() {
                self.shader
                    .set_matrix4("projection", &camera.get_projection());
                self.shader.set_matrix4("view", &camera.get_view());
                self.shader
                    .set_matrix4("transformation", &transform.get_matrix());
                mesh_renderer.model.draw();
            }
            depth::set_cmp_func(CompareOption::Equal);
            depth::set_write(false);
            color::set_write(true, true, true, true);
        }
        for (mesh_renderer, transform, material) in (&mut *models, transforms, materials).join() {
            //TOOD:finish
            self.shader
                .set_matrix4("projection", &camera.get_projection());
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

    fn out_framebuffer(&self) -> &Framebuffer {
        &self.out_framebuffer
    }

    fn resize(&mut self, viewport: Viewport) {
        self.out_framebuffer = self.out_framebuffer.resize(viewport).unwrap();
    }
}
use crate::objects::buffers::DataType;

use super::{FULL_SCREEN, FULLSCREENPASS_VERTEX_SHADER};
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
