use std::any::Any;

use gl::{DRAW_FRAMEBUFFER, READ_FRAMEBUFFER};
use glam::Vec4;

use crate::{
    ecs::FULL_SCREEN,
    objects::{
        shader::Shader,
        texture::{Filter, Texture2D, Texture2DBuilder},
        viewport::Viewport,
    },
};

#[derive(Debug, Clone)]
pub enum FramebufferError {
    AttachmentNotFound(FramebufferAttachment),
    NotAppropriateUseOfDefaultFramebuffer,
}
#[derive(Debug, Clone)]
pub struct Framebuffer {
    id: u32,
    viewport: Viewport,
    draw_buffers: Vec<u32>,
    attachments: Vec<(FramebufferAttachment, Option<Texture2D>)>,
}
static mut BINDED_FRAMEBUFFER: u32 = 0;
static mut BINDED_DRAW_FRAMEBUFFER: u32 = 0;
impl Framebuffer {
    pub fn new(viewport: Viewport) -> Self {
        unsafe {
            let mut id = 0;
            gl::GenFramebuffers(1, &mut id);
            Self {
                id,
                viewport,
                attachments: Vec::new(),
                draw_buffers: Vec::new(),
            }
        }
    }
    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn blit_with(&self, shader: &Shader) {
        self.bind();
        shader.bind();
        FULL_SCREEN.draw();
    }
    pub fn draw_bind(&self) {
        unsafe {
            if BINDED_DRAW_FRAMEBUFFER != self.id() {
                gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, self.id);
                gl::DrawBuffers(self.draw_buffers.len() as i32, self.draw_buffers.as_ptr());
                BINDED_DRAW_FRAMEBUFFER = self.id;
            }
        }
    }
    pub fn bind(&self) {
        unsafe {
            if !self.current() {
                gl::BindFramebuffer(gl::FRAMEBUFFER, self.id);
                gl::DrawBuffers(self.draw_buffers.len() as i32, self.draw_buffers.as_ptr());
                BINDED_FRAMEBUFFER = self.id;
            }
        }
    }
    pub fn clear_color(&self, color: Vec4) {
        self.draw_bind();
        unsafe {
            gl::ClearColor(color.x, color.y, color.z, color.w);
        }
    }
    pub fn clear(&self, clear_flags: ClearFlags) {
        self.draw_bind();
        unsafe {
            gl::Clear(clear_flags.bits());
        }
    }
    pub fn current(&self) -> bool {
        self.id() == unsafe { BINDED_FRAMEBUFFER }
    }
    pub fn unbind() {
        Self::default().bind();
    }
    pub fn viewport(&self) -> &Viewport {
        &self.viewport
    }
    pub fn add_attachment(&mut self, attachment: FramebufferAttachment, texture: Texture2D) {
        self.bind();
        unsafe {
            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                attachment.into(),
                gl::TEXTURE_2D,
                texture.id(),
                0,
            );
        }
        unsafe{BINDED_FRAMEBUFFER = 0}
        if let FramebufferAttachment::Color(_) = attachment {
            self.draw_buffers.push(attachment.into());
        }
        self.attachments.push((attachment, Some(texture)));
    }
    pub fn create_attachment(
        &mut self,
        attachment: FramebufferAttachment,
        texture_builder: Texture2DBuilder,
    ) {
        self.bind();
        self.add_attachment(attachment, {
            let build_res = texture_builder.clone().build();
            if let Err(_) = &build_res {
                texture_builder
                    .size((self.viewport.width(), self.viewport.height()))
                    .build()
                    .unwrap()
            } else {
                build_res.unwrap()
            }
        });
    }
    pub fn read_bind(&self) {
        unsafe {
            gl::BindFramebuffer(READ_FRAMEBUFFER, self.id);
        }
    }
    pub fn copy_depth_to(&self, other: &Framebuffer, filter: Filter) {
        self.read_bind();
        other.draw_bind();
        unsafe {
            gl::BlitFramebuffer(
                0,
                0,
                self.viewport.width(),
                self.viewport.height(),
                0,
                0,
                other.viewport.width(),
                other.viewport.height(),
                gl::DEPTH_BUFFER_BIT,
                filter.to_param(),
            );
        }
    }
    ///
    pub fn attachment_texture(
        &self,
        framebuffer_attachment: FramebufferAttachment,
    ) -> Result<Texture2D, FramebufferError> {
        if self.id == 0 {
            return Err(FramebufferError::NotAppropriateUseOfDefaultFramebuffer);
        }
        Ok(self
            .attachments
            .iter()
            .find(|x| x.0 == framebuffer_attachment)
            .ok_or(FramebufferError::AttachmentNotFound(framebuffer_attachment))?
            .1
            .clone()
            .unwrap())
    }
    ///safe variety of framebuffer resize
    ///cause error only when used to default framebuffer
    pub fn resize(&mut self, viewport: Viewport) -> Result<Framebuffer, FramebufferError> {
        if self.attachments.iter().any(|(_, x)| x.is_none()) {}
        let mut fbo = Framebuffer::new(viewport);
        for attachment in self.attachments.iter_mut() {
            let texture = attachment.1.as_mut().unwrap();
            texture.bind();
            //So if 2 framebuffers shares same attachment they wont resize it twice
            if texture.width() != viewport.width() && texture.height() != viewport.height() {
                texture.finalize(
                    texture.internal_format(),
                    texture.texture_format(),
                    texture.texture_type(),
                    viewport.width(),
                    viewport.height(),
                );
            }
            fbo.add_attachment(attachment.0, texture.clone());
        }
        Ok(fbo)
    }
    pub fn complete(&self) -> bool {
        self.bind();
        let b = unsafe { gl::CheckFramebufferStatus(gl::FRAMEBUFFER) == gl::FRAMEBUFFER_COMPLETE };
        Self::unbind();
        b
    }
}
impl Default for Framebuffer {
    fn default() -> Self {
        Self {
            id: 0,
            viewport: Viewport::new(0, 0, 0, 0),
            attachments: vec![],
            draw_buffers: vec![gl::FRONT_LEFT],
        }
    }
}
impl Drop for Framebuffer {
    fn drop(&mut self) {
        if self.id() != 0 {
            unsafe { gl::DeleteFramebuffers(1, &self.id) }
        }
    }
}
bitflags::bitflags! {
    pub struct ClearFlags:u32 {
        const Color = 0x4000;
        const Depth = 0x100;
        const Stencil = 0x400;
    }

}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FramebufferAttachment {
    Depth,
    Stencil,
    DepthStencil,
    Color(u32),
}
impl Into<u32> for FramebufferAttachment {
    fn into(self) -> u32 {
        match self {
            FramebufferAttachment::Depth => gl::DEPTH_ATTACHMENT,
            FramebufferAttachment::Stencil => gl::STENCIL_ATTACHMENT,
            FramebufferAttachment::DepthStencil => gl::DEPTH_STENCIL_ATTACHMENT,
            FramebufferAttachment::Color(n) => gl::COLOR_ATTACHMENT0 + n,
        }
    }
}
