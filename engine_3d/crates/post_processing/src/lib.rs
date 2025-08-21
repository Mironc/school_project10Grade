
use std::{any::TypeId, collections::{hash_map::Keys, HashMap}};

use graphics::{
    draw_options::{depth,
    face_culling,},
    objects::{
        buffers::{Framebuffer, FramebufferAttachment},
        shader::{Shader, ShaderType, SubShader},
        texture::Texture2D,
        viewport::Viewport,
    },
};
use anymap::{CloneAny, Map, RawMap};
use math::Vec3;
use rendering::camera::{projection::Projection, Camera};
use specs::{Component, HashMapStorage, Join, LendJoin, ReadStorage, System, WriteStorage};
pub struct PostProcessingSystem {}
impl<'a> System<'a> for PostProcessingSystem {
    type SystemData = (
        WriteStorage<'a, Camera>,
        WriteStorage<'a, PostProcessingContainer>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut camera_storage, mut postprocesing) = data;
        for (camera, post_processing) in (&mut camera_storage, (&mut postprocesing).maybe()).join() {
            let rendering_image = camera
                .render_path
                .framebuffer()
                .attachment_texture(FramebufferAttachment::Color(0))
                .unwrap();
            let render_image = camera
                .render_image
                .attachment_texture(FramebufferAttachment::Color(0))
                .unwrap();
            camera.render_image.bind();
            if let Some(post_processing) = post_processing {
                post_processing.apply_to(&mut camera.render_image, rendering_image);
            } else {
                rendering_image.copy_to(&render_image);
            }
        }
    }
}

use graphics::utils::FULLSCREENPASS_VERTEX_SHADER;
#[derive(Debug, Clone)]
pub struct PostProcessingContainer {
    anymap: anymap::Map<dyn CloneAny>,
    f: fn(&mut Self, &mut Framebuffer, &Texture2D) -> (),
}
impl Component for PostProcessingContainer {
    type Storage = HashMapStorage<Self>;
}
impl PostProcessingContainer {
    pub fn new() -> Self {
        Self {
            anymap: Map::new(),
            f: |_, _, _| (),
        }
    }
    pub fn set_f(&mut self, f: fn(&mut Self, &mut Framebuffer, &Texture2D) -> ()) {
        self.f = f;
    }
    pub fn get_all_raw(&self) -> Keys<TypeId,Box<dyn CloneAny>> {
        self.anymap.as_raw().keys()
    }
    pub fn insert<T: PostProcessing + 'static + Clone>(&mut self, item: T) {
        self.anymap.insert(item);
    }
    pub fn get<T: PostProcessing + 'static + Clone>(&self) -> Option<&T> {
        self.anymap.get::<T>()
    }
    pub fn get_mut<T: PostProcessing + 'static + Clone>(&mut self) -> Option<&mut T> {
        self.anymap.get_mut::<T>()
    }
    pub fn contains<T: PostProcessing + 'static + Clone>(&mut self) -> bool {
        self.anymap.contains::<T>()
    }
    pub fn remove<T: PostProcessing + 'static + Clone>(&mut self) -> Option<()> {
        self.anymap.remove::<T>();
        Some(())
    }

    pub fn apply_to(&mut self, dest_framebuffer: &mut Framebuffer, source_texture: Texture2D) {
        dest_framebuffer.bind();
        dest_framebuffer.viewport().set_gl_viewport();
        depth::disable();
        face_culling::disable();
        (self.f)(self, dest_framebuffer, &source_texture)
    }
}
pub trait PostProcessing {
    fn apply_to(&mut self, framebuffer: &mut Framebuffer, texture: Texture2D);
    fn resize(&mut self, viewport: Viewport);
}
#[derive(Debug, Clone)]
pub struct SimpleColorProcessing {
    gamma: f32,
    brightness: f32,
    contrast: f32,
    midpoint: f32,
    exposure: f32,
    saturation: f32,
    simple_color_correction: Shader,
}
impl SimpleColorProcessing {
    pub fn new(
        gamma: f32,
        exposure: f32,
        contrast: f32,
        brightness: f32,
        midpoint: f32,
        saturation: f32,
    ) -> Self {
        let simple_color_correction = Shader::new([
            *FULLSCREENPASS_VERTEX_SHADER,
            SubShader::new(
                include_str!("./shaders/color_correction.glsl"),
                ShaderType::Fragment,
            ),
        ]);

        Self {
            gamma,
            simple_color_correction,
            brightness,
            contrast,
            midpoint,
            saturation,
            exposure,
            ..Default::default()
        }
    }
}
impl Default for SimpleColorProcessing {
    fn default() -> Self {
        Self {
            gamma: 1.0,
            brightness: 0.0,
            contrast: 1.0,
            midpoint: 0.5,
            exposure: 1.0,
            saturation: 1.0,
            simple_color_correction: Shader::new([
                *FULLSCREENPASS_VERTEX_SHADER,
                SubShader::new(
                    include_str!("./shaders/color_correction.glsl"),
                    ShaderType::Fragment,
                ),
            ]),
        }
    }
}
impl PostProcessing for SimpleColorProcessing {
    fn apply_to(&mut self, framebuffer: &mut Framebuffer, texture: Texture2D) {
        let mut simple_color_correction = &self.simple_color_correction;
        simple_color_correction.bind();
        simple_color_correction.set_f32("gamma", self.gamma);
        simple_color_correction
            .set_f32("contrast", self.contrast);
        simple_color_correction
            .set_f32("brightness", self.brightness);
        simple_color_correction
            .set_f32("midpoint", self.midpoint);
        simple_color_correction
            .set_texture2d("color", &texture, 0);
        simple_color_correction
            .set_f32("saturation", self.saturation);
        simple_color_correction
            .set_f32("exposure", self.exposure);

        framebuffer.blit_with(&self.simple_color_correction);
    }

    fn resize(&mut self, _viewport: Viewport) {}
}

#[derive(Debug, Clone)]
pub struct Tonemapping {
    shader: Shader,
}
impl Tonemapping {
    pub fn new() -> Self {
        let shader = Shader::new([
            *FULLSCREENPASS_VERTEX_SHADER,
            SubShader::new(
                include_str!("./shaders/tonemapping_reinhard_jodie.frag"),
                ShaderType::Fragment,
            ),
        ]);

        Self {
            shader,
            ..Default::default()
        }
    }
}
pub struct FogSystem {}
impl<'a> System<'a> for FogSystem {
    type SystemData = (
        ReadStorage<'a, Camera>,
        WriteStorage<'a, PostProcessingContainer>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (camera, mut post_processing) = data;
        for (camera, post_processing) in (&camera, &mut post_processing).join() {
            if let Some(fog_effect) = post_processing.get_mut::<DistanceFog>() {
                let mut shader = &fog_effect.shader;
                match camera.projection() {
                    Projection::Perspective(perspective) => {
                        shader.set_f32("near", perspective.z_near());
                        shader.set_f32("far", perspective.z_far());
                    }
                    Projection::Orthogonal(orthogonal) => {
                        shader.set_f32("near", orthogonal.z_near());
                        shader.set_f32("far", orthogonal.z_far());
                    }
                }
            }
        }
    }
}
impl Default for Tonemapping {
    fn default() -> Self {
        let shader = Shader::new([
            *FULLSCREENPASS_VERTEX_SHADER,
            SubShader::new(
                include_str!("./shaders/color_correction.glsl"),
                ShaderType::Fragment,
            ),
        ]);
        Self { shader }
    }
}
impl PostProcessing for Tonemapping {
    fn apply_to(&mut self, framebuffer: &mut Framebuffer, texture: Texture2D) {
        self.shader.set_texture2d("color", &texture, 0);
        framebuffer.blit_with(&self.shader);
    }

    fn resize(&mut self, _viewport: Viewport) {}
}
#[derive(Debug,Clone)]
pub struct DistanceFog{
    shader:Shader,
    strength:f32,
    offset:f32,
    color:Vec3
}
impl DistanceFog {
    pub fn new(strength:f32,offset:f32,color:Vec3) -> Self{
        let shader = Shader::new([*FULLSCREENPASS_VERTEX_SHADER,SubShader::new(include_str!("./shaders/fog.glsl"), ShaderType::Fragment)]);
        Self { shader , strength, offset, color }
    }
}
impl PostProcessing for DistanceFog {
    fn apply_to(&mut self, framebuffer: &mut Framebuffer, texture: Texture2D) {
        let depth = framebuffer
            .attachment_texture(FramebufferAttachment::DepthStencil)
            .unwrap();
        let mut shader = &self.shader;
        shader.set_texture2d("color",&texture, 0);
        shader.set_texture2d("depth", &depth, 1);
        shader.set_f32("strength", self.strength);
        shader.set_f32("offset", self.offset);
        shader.set_vec3("fog_color", &self.color);
        framebuffer.blit_with(&self.shader);
    }
    
    fn resize(&mut self, viewport: Viewport) {}
}


#[cfg(test)]
pub mod test {

    use std::{any::TypeId, ops::Deref};

    use anymap::{CloneAny, RawMap};

    use crate::objects::{buffers::Framebuffer, texture::Texture2D, viewport::Viewport};

    use super::{PostProcessing, PostProcessingContainer, SimpleColorProcessing};
    #[derive(Debug, Clone, Copy)]
    struct PSTest {}
    impl PostProcessing for PSTest {
        fn apply_to(
            &mut self,
            framebuffer: &crate::objects::buffers::Framebuffer,
            texture: crate::objects::texture::Texture2D,
        ) {
            println!("applied");
        }

        fn resize(&mut self, viewport: Viewport) {
            println!("resized {:?}", viewport);
        }
    }
    #[derive(Debug, Clone, Copy)]
    struct PSTes1t {}
    impl PostProcessing for PSTes1t {
        fn apply_to(
            &mut self,
            framebuffer: &crate::objects::buffers::Framebuffer,
            texture: crate::objects::texture::Texture2D,
        ) {
            println!("applied ps21");
        }

        fn resize(&mut self, viewport: Viewport) {
            println!("resized {:?}", viewport);
        }
    }
    fn testf(m: &mut PostProcessingContainer, f: &Framebuffer, t: &Texture2D) {
        println!("{}", m.get_all().len());
        let posteffect = m.get_mut::<PSTest>().unwrap();
        posteffect.apply_to(f, t.clone());
        let posteffect = m.get_mut::<PSTes1t>().unwrap();
        posteffect.apply_to(f, t.clone());
    }
    #[test]
    fn test() {
        let mut container = PostProcessingContainer::new();
        container.insert(PSTest {});
        container.insert(PSTes1t {});
        container.set_f(testf);
        println!("{}", container.get_mut::<PSTest>().is_some());
        container
            .get_mut::<PSTest>()
            .unwrap()
            .resize(Viewport::new(0, 0, 100, 100));
        container.apply_to(&Framebuffer::default(), Texture2D::default());
    }
}
