use math::{Mat4, Vec3, Vec4, Vec4Swizzles};
use specs::{Join, Read, ReadStorage, WriteStorage};
use transform::Transform;

use crate::{
    camera::{CameraTransform, Plane, ViewFrustum, projection::Projection},
    light::{Light, Sun},
    material::Material,
    mesh_renderer::MeshRenderer,
    render_path::RenderPath,
};
use graphics::{
    draw_options::{
        color_mask::{self, Color},
        depth::{self},
        face_culling::{self, CullFace, FrontFaceOrder},
        stencil::{self, Action, StencilFunction, StencilOptions},
    },
    compare_opt::CompareOption,
    impl_data_type,
    objects::{
        buffers::{Buffer, ClearFlags, Data, Framebuffer, FramebufferAttachment, ShaderStorage},
        model::{InstancedModel, Model, from_str, primitives::SimpleVertex},
        shader::{Shader, ShaderType, SubShader},
        texture::{Filter, Texture2DBuilder, TextureDataType, TextureFormat},
        vertex::ModelVertex,
        viewport::Viewport,
    },
};
pub struct ForwardPath {
    out_framebuffer: Framebuffer,
    shader: Shader,
    depth_prepass: bool,
    light_props: Buffer<ShaderStorage>,
}
impl ForwardPath {
    pub fn new(viewport: Viewport, depth_prepass: bool) -> Self {
        let mut out_framebuffer = Framebuffer::new(viewport);
        let _ = out_framebuffer.create_attachment(
            FramebufferAttachment::Color(0),
            Texture2DBuilder::new()
                .internal_format(TextureFormat::RGBA16F)
                .filter(Filter::Nearest)
                .texture_type(TextureDataType::Float),
        );
        let depth_texture = Texture2DBuilder::new()
            .internal_format(TextureFormat::Depth32FStencil8)
            .texture_format(TextureFormat::DepthStencilComponent)
            .texture_type(TextureDataType::Float32UnsignedInt8)
            .filter(Filter::Nearest);
        let _ =
            out_framebuffer.create_attachment(FramebufferAttachment::Depth, depth_texture.clone());
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
        let light_props = Buffer::create();
        Self {
            depth_prepass,
            out_framebuffer,
            shader,
            light_props,
        }
    }
}
impl RenderPath for ForwardPath {
    fn render(
        &mut self,
        transforms: &ReadStorage<'_, Transform>,
        models: &mut WriteStorage<'_, MeshRenderer>,
        materials: &ReadStorage<'_, Material>,
        light_collection: &Vec<(&Light, &Transform)>,
        _sun: &Read<'_, Sun>,
        _view_frustum: ViewFrustum,
        view_mat: Mat4,
        projection: Projection,
        camera_transform: CameraTransform,
    ) {
        self.out_framebuffer.draw_bind();
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
        let mut shader_h = &self.shader;
        shader_h.bind();
        shader_h.set_shader_storage_block("lights", &self.light_props, 1);
        //depth pass
        depth::set_cmp_func(CompareOption::LessEqual);
        if self.depth_prepass {
            color_mask::set_write(false, false, false, false);
            for (mesh_renderer, transform, _) in (&mut *models, transforms, materials).join() {
                shader_h.set_matrix4("projection", &projection.get_projection());
                shader_h.set_matrix4("view", &view_mat);
                shader_h.set_matrix4("transformation", &transform.get_matrix());
                mesh_renderer.model.draw();
            }
            depth::set_cmp_func(CompareOption::Equal);
            depth::set_write(false);
            color_mask::set_write(true, true, true, true);
        }
        let mut shader = &self.shader;
        for (mesh_renderer, transform, material) in (&mut *models, transforms, materials).join() {
            //TOOD:finish
            shader_h.set_matrix4("projection", &projection.get_projection());
            shader.set_matrix4("view", &view_mat);
            shader_h.set_matrix4("transformation", &transform.get_matrix());
            shader.set_texture2d("main_texture", &material.main_texture, 1);
            shader.set_vec3("color", &material.color);
            shader.set_f32("specular", material.specular);
            shader.set_vec3("camera_position", &camera_transform.position);
            shader.set_f32("shininess", material.shininess);
            shader.set_int("light_count", light_collection.len() as i32);
            mesh_renderer.model.draw();
        }
    }

    fn resize(&mut self, viewport: Viewport) {
        self.out_framebuffer = self.out_framebuffer.resize(viewport).unwrap();
    }

    fn framebuffer(&self) -> &Framebuffer {
        &self.out_framebuffer
    }
}
pub struct DeferredPath {
    out_framebuffer: Framebuffer,
    g_buffer: Framebuffer,
    geometry_pass: Shader,

    point_light_pass: Shader,
    point_light_props: Buffer<ShaderStorage>,
    point_light_volume: InstancedModel,

    sun_light_pass: Shader,
    ambient_light_pass: Shader,
    ambient_color: Vec3,
}
impl DeferredPath {
    pub fn new(viewport: Viewport, ambient_color: Vec3) -> Self {
        let mut out_framebuffer = Framebuffer::new(viewport);
        let _ = out_framebuffer.create_attachment(
            FramebufferAttachment::Color(0),
            Texture2DBuilder::new()
                .internal_format(TextureFormat::RGBA16F)
                .filter(Filter::Nearest)
                .texture_type(TextureDataType::Float),
        );
        let depth = Texture2DBuilder::new()
            .size((1, 1))
            .internal_format(TextureFormat::Depth32FStencil8)
            .texture_format(TextureFormat::DepthStencilComponent)
            .texture_type(TextureDataType::Float32UnsignedInt8)
            .filter(Filter::Nearest)
            .build()
            .unwrap();
        out_framebuffer.add_attachment(FramebufferAttachment::DepthStencil, depth.clone());

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
        let _ = g_buffer.create_attachment(
            FramebufferAttachment::Color(0),
            Texture2DBuilder::new()
                .internal_format(TextureFormat::RGBA8SNorm)
                .texture_format(TextureFormat::RGBA)
                .filter(Filter::Nearest),
        );
        //color attachment + specular
        let _ = g_buffer.create_attachment(
            FramebufferAttachment::Color(1),
            Texture2DBuilder::new()
                .internal_format(TextureFormat::RGBA8)
                .filter(Filter::Nearest),
        );
        g_buffer.add_attachment(FramebufferAttachment::DepthStencil, depth);

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
        let sun_light_pass = Shader::new([
            *FULLSCREENPASS_VERTEX_SHADER,
            SubShader::new(
                include_str!("./shaders/deferred_shading_sun_frag.glsl"),
                ShaderType::Fragment,
            ),
        ]);
        let ambient_light_pass = Shader::new([
            *FULLSCREENPASS_VERTEX_SHADER,
            SubShader::new(
                include_str!("./shaders/deferred_shading_ambient_frag.glsl"),
                ShaderType::Fragment,
            ),
        ]);
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

        let point_light_volume: Model<ModelVertex> =
            from_str(include_str!("./icosahedron.obj")).unwrap();
        let mut verticies = Vec::new();
        for vert in point_light_volume.verticies.iter() {
            verticies.push(SimpleVertex::new(vert.position));
        }
        let point_light_volume =
            Model::new(verticies, point_light_volume.indicies.clone()).instantiate();
        let point_light_props = Buffer::<ShaderStorage>::create();
        Self {
            out_framebuffer,
            point_light_pass,
            g_buffer,
            geometry_pass,
            point_light_props,
            point_light_volume,
            sun_light_pass,
            ambient_light_pass,
            ambient_color,
        }
    }
}
impl RenderPath for DeferredPath {
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
        let mut geometry_pass = &self.geometry_pass;
        geometry_pass.bind();
        for (mesh_renderer, transform, material) in (&mut *models, transforms, materials).join() {
            geometry_pass.set_matrix4("projection", &projection.get_projection());
            geometry_pass.set_matrix4("view", &view_mat);
            geometry_pass.set_matrix4("transformation", &transform.get_matrix());
            geometry_pass.set_texture2d("main_texture", &material.main_texture, 1);
            geometry_pass.set_vec3("color", &material.color);
            geometry_pass.set_f32("specular", material.specular);
            geometry_pass.set_f32("shininess", material.shininess);
            mesh_renderer.model.draw();
        }
        //Lightning pass
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
            .attachment_texture(FramebufferAttachment::DepthStencil)
            .unwrap();
        let normal = self
            .g_buffer
            .attachment_texture(FramebufferAttachment::Color(0))
            .unwrap();
        let color_spec = self
            .g_buffer
            .attachment_texture(FramebufferAttachment::Color(1))
            .unwrap();
        let mut point_light_pass = &self.point_light_pass;
        point_light_pass.bind();
        point_light_pass.set_texture2d("position", &position, 0);
        point_light_pass.set_texture2d("normal", &normal, 1);
        point_light_pass.set_texture2d("color_spec", &color_spec, 2);

        //filling light sources data
        let mut lights = Vec::new();
        for (light, light_transform) in light_collection.iter() {
            let mut sphere_test = move |p: &Plane| {
                let res = p.signed_distance(light_transform.position)
                    >= -10.0 * (light.light_properties().power).sqrt();
                return res;
            };
            if view_frustum.in_frustum(&mut sphere_test) {
                let light_prop = light.light_properties();
                let transf = Mat4::from_translation(light_transform.position)
                    * light_transform.get_rotation_matrix()
                    * Mat4::from_scale(
                        math::Vec3::ONE * 10.0 * (light.light_properties().power).sqrt(),
                    );
                lights.push(LightProps {
                    transf,
                    light_color: light_prop.color.extend(1.0),
                    light_position: light_transform.position.extend(1.0),
                    light_power: light_prop.power,
                });
            }
        }
        lights.sort_by(|p, p1| {
            camera_transform
                .position
                .distance_squared(p1.light_position.xyz())
                .total_cmp(
                    &camera_transform
                        .position
                        .distance_squared(p.light_position.xyz()),
                )
        });
        self.point_light_props.bind();
        self.point_light_props.set_data(&lights);
        point_light_pass.set_shader_storage_block("lights", &self.point_light_props, 1);
        point_light_pass.set_vec3("camera_position", &camera_transform.position);
        point_light_pass.set_matrix4("vp", &(projection.get_projection() * view_mat));
        point_light_pass.set_matrix4("inv_proj", &projection.get_projection().inverse());
        point_light_pass.set_matrix4("inv_view", &view_mat.inverse());

        depth::set_cmp_func(CompareOption::GreaterEqual);
        stencil::enable();
        let mut i = 0;
        while i < lights.len() {
            let light = &lights[i];
            if camera_transform
                .position
                .distance(light.light_position.xyz())
                > light.light_power.sqrt() * 10.0
            {
                point_light_pass.set_int("instance", i as i32);
                face_culling::set_cullface(CullFace::Front);
                stencil::set_stencil_function(StencilFunction::with_no_mask(
                    CompareOption::Always,
                    1,
                ));
                color_mask::set_write(false, false, false, false);
                stencil::set_stencil_options(StencilOptions::new(
                    Action::Keep,
                    Action::Keep,
                    Action::Replace,
                ));
                self.point_light_volume.draw();

                color_mask::set_write(true, true, true, true);
                face_culling::set_cullface(CullFace::Back);
                stencil::set_stencil_options(StencilOptions::new(
                    Action::Keep,
                    Action::Zero,
                    Action::Zero,
                ));
                stencil::set_stencil_function(StencilFunction::with_no_mask(
                    CompareOption::GreaterEqual,
                    1,
                ));
                self.point_light_volume.draw();
            } else {
                break;
            }
            i += 1;
        }
        color_mask::set_write(true, true, true, true);
        face_culling::set_cullface(CullFace::Back);
        depth::set_cmp_func(CompareOption::GreaterEqual);
        stencil::set_stencil_options(StencilOptions::new(
            Action::Keep,
            Action::Keep,
            Action::Keep,
        ));
        stencil::set_stencil_function(StencilFunction::with_no_mask(CompareOption::Always, 0));
        while i < lights.len() {
            point_light_pass.set_int("instance", i as i32);
            self.point_light_volume.draw();
            i += 1;
        }

        face_culling::disable();
        stencil::disable();
        depth::disable();
        if let Some(direction) = sun.direction() {
            let mut sun_light_pass = &self.sun_light_pass;
            sun_light_pass.bind();
            sun_light_pass.set_texture2d("position", &position, 0);
            sun_light_pass.set_texture2d("normal", &normal, 1);
            sun_light_pass.set_texture2d("color_spec", &color_spec, 2);
            sun_light_pass.set_vec3("light_direction", &direction);
            sun_light_pass.set_vec3("light_color", &sun.color());
            sun_light_pass.set_vec3("camera_position", &camera_transform.position);
            sun_light_pass.set_matrix4(
                "inv_vp",
                &(projection.get_projection() * view_mat).inverse(),
            );
            EMPTY.draw();
        }
        let mut ambient_light_pass = &self.ambient_light_pass;
        ambient_light_pass.bind();
        ambient_light_pass.set_vec3("ambient", &self.ambient_color);
        ambient_light_pass.set_texture2d("color_spec", &color_spec, 2);
        EMPTY.draw();
        unsafe {
            gl::Disable(gl::BLEND);
        }
    }

    fn resize(&mut self, viewport: Viewport) {
        self.out_framebuffer = self.out_framebuffer.resize(viewport).unwrap();
        self.g_buffer = self.g_buffer.resize(viewport).unwrap();
    }
    fn framebuffer(&self) -> &Framebuffer {
        &self.out_framebuffer
    }
}
use graphics::objects::buffers::DataType;

use graphics::utils::{EMPTY, FULLSCREENPASS_VERTEX_SHADER};
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
