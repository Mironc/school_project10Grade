use std::{cell::Cell, ops::BitOr};

use math::Vec4;

use crate::{
    utils::EMPTY,
    objects::{
        shader::Shader,
        texture::{Filter, Texture2D, Texture2DBuilder},
        viewport::Viewport,
    },
    utils::{end_debug_marker, start_debug_marker},
};

#[derive(Debug, Clone)]
pub enum FramebufferError {
    AttachmentNotFound(FramebufferAttachment),
    NotAppropriateUseOfDefaultFramebuffer,
    WrongAttachment,
}
#[derive(Debug)]
pub struct Framebuffer {
    id: u32,
    viewport: Viewport,
    draw_buffers: Vec<u32>,
    draw_buffer_updated: bool,
    attachments: Vec<(FramebufferAttachment, Option<Texture2D>)>,
}
static mut BINDED_READ_FRAMEBUFFER: u32 = 0;
static mut BINDED_DRAW_FRAMEBUFFER: u32 = 0;
impl Framebuffer {
    pub fn new(viewport: Viewport) -> Self {
        unsafe {
            let mut id = 0;
            gl::CreateFramebuffers(1, &mut id);
            Self {
                id,
                viewport,
                attachments: Vec::new(),
                draw_buffers: Vec::new(),
                draw_buffer_updated: false,
            }
        }
    }
    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn blit_with(&mut self, shader: &Shader) {
        self.draw_bind();
        shader.bind();
        EMPTY.draw();
    }
    fn set_viewport(&self) {
        if let Some(attachment) = self
            .attachments
            .first()
        {
            let viewport = attachment.1.as_ref().unwrap();
            Viewport::new(0, 0, viewport.width(), viewport.height()).set_gl_viewport();
        }
    }
    pub fn draw_bind(&mut self) {
        unsafe {
            if !self.current_draw() {
                self.set_viewport();
                gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, self.id);
                gl::DrawBuffers(self.draw_buffers.len() as i32, self.draw_buffers.as_ptr());
                BINDED_DRAW_FRAMEBUFFER = self.id;
                self.draw_buffer_updated = false;
            }
            if self.draw_buffer_updated {
                gl::DrawBuffers(self.draw_buffers.len() as i32, self.draw_buffers.as_ptr());
                self.draw_buffer_updated = false;
            }
        }
    }
    pub fn bind(&mut self) {
        unsafe {
            if !self.current_draw() || !self.current_read() {
                self.set_viewport();
                gl::BindFramebuffer(gl::FRAMEBUFFER, self.id);
                gl::DrawBuffers(self.draw_buffers.len() as i32, self.draw_buffers.as_ptr());
                BINDED_DRAW_FRAMEBUFFER = self.id;
                BINDED_READ_FRAMEBUFFER = self.id;
                self.draw_buffer_updated = false;
            }
            if self.draw_buffer_updated {
                gl::DrawBuffers(self.draw_buffers.len() as i32, self.draw_buffers.as_ptr());
                self.draw_buffer_updated = false;
            }
        }
    }
    pub fn read_bind(&self) {
        if !self.current_read() {
            unsafe {
                gl::BindFramebuffer(gl::READ_FRAMEBUFFER, self.id);
                BINDED_READ_FRAMEBUFFER = self.id;
            }
        }
    }
    pub fn clear_color(&mut self, color: Vec4) {
        self.draw_bind();
        unsafe {
            gl::ClearColor(color.x, color.y, color.z, color.w);
        }
    }
    pub fn clear(&mut self, clear_flags: ClearFlags) {
        self.draw_bind();
        unsafe {
            gl::Clear(clear_flags.bits());
        }
    }
    pub fn current_draw(&self) -> bool {
        self.id() == unsafe { BINDED_DRAW_FRAMEBUFFER }
    }
    pub fn current_read(&self) -> bool {
        self.id() == unsafe { BINDED_READ_FRAMEBUFFER }
    }
    pub fn unbind() {
        Self::default().bind();
    }
    pub fn viewport(&self) -> &Viewport {
        &self.viewport
    }
    pub fn add_attachment(&mut self, attachment: FramebufferAttachment, texture: Texture2D) {
        if let FramebufferAttachment::Color(_) = attachment {
            if !self.draw_buffers.contains(&attachment.into()) {
                self.draw_buffers.push(attachment.into());
            }
            self.draw_buffer_updated = true;
        }
        self.attachments.push((attachment, Some(texture.clone())));
        unsafe {
            gl::NamedFramebufferTexture(self.id, attachment.into(), texture.id(), 0);
        }
    }
    pub fn remove_attachment(&mut self, attachment: FramebufferAttachment) {
        unsafe {
            gl::NamedFramebufferTexture(self.id, attachment.into(), 0, 0);
        }
        let pos = self.attachments.iter().position(|x| x.0 == attachment);
        if let Some(pos) = pos {
            self.attachments.remove(pos);
            self.draw_buffer_updated=true;
        }
        if let FramebufferAttachment::Color(_) = attachment {
            let pos = self
                .draw_buffers
                .iter()
                .position(|x| x.eq(&attachment.into()));
            if let Some(pos) = pos {
                self.draw_buffers.remove(pos);
            }
        }
    }
    pub fn create_attachment(
        &mut self,
        attachment: FramebufferAttachment,
        texture_builder: Texture2DBuilder,
    ) -> Result<(), FramebufferError> {
        if self.id == 0 {
            return Err(FramebufferError::NotAppropriateUseOfDefaultFramebuffer);
        }
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
        Ok(())
    }
    pub fn copy_color_attachment_to_texture(
        &mut self,
        attachment: FramebufferAttachment,
        destination: &Texture2D,
        filter: Filter,
    ) -> Result<(), FramebufferError> {
        if let FramebufferAttachment::Color(_) = attachment {
            if !self.contains(attachment) {
                return Err(FramebufferError::AttachmentNotFound(attachment));
            }
        } else {
            return Err(FramebufferError::WrongAttachment);
        }
        let max_attachment = self
            .attachments
            .iter()
            .max_by(|a, b| a.0.cmp(&b.0))
            .map(|x| x.0)
            .unwrap_or(FramebufferAttachment::Color(0));
        let temp_attachment = match max_attachment {
            FramebufferAttachment::Color(n) => FramebufferAttachment::Color(n + 1),
            _ => FramebufferAttachment::Color(0),
        };
        start_debug_marker("copy");
        unsafe {
            gl::NamedFramebufferTexture(self.id, temp_attachment.into(), destination.id(), 0);
            gl::NamedFramebufferDrawBuffer(self.id, temp_attachment.into());
            gl::BlitNamedFramebuffer(
                self.id,
                self.id,
                0,
                0,
                self.viewport.width(),
                self.viewport.height(),
                0,
                0,
                destination.width(),
                destination.height(),
                gl::COLOR_BUFFER_BIT,
                filter.to_param(),
            );
            self.draw_buffer_updated = true;
            gl::NamedFramebufferTexture(self.id, temp_attachment.into(), 0, 0);
        }
        end_debug_marker();
        Ok(())
    }
    ///idk how use with front buffer, since it dont hold any viewport info
    /// but you can just inline it and provide it by yourself with resize event viewport as I did
    pub fn copy_depth_to(&self, other: &Framebuffer) -> Result<(), FramebufferError> {
        if self.id == 0 {
            return Err(FramebufferError::NotAppropriateUseOfDefaultFramebuffer);
        }
        unsafe {
            gl::BlitNamedFramebuffer(
                self.id,
                other.id,
                0,
                0,
                self.viewport.width(),
                self.viewport.height(),
                0,
                0,
                other.viewport.width(),
                other.viewport.height(),
                gl::DEPTH_BUFFER_BIT,
                Filter::Nearest.to_param(),
            );
            Ok(())
        }
    }
    /// idk how use with front buffer, since it dont hold any viewport info
    /// but you can just inline it and provide it by yourself with resize event viewport as I did
    pub fn copy_color_to(
        &self,
        other: &Framebuffer,
        filter: Filter,
    ) -> Result<(), FramebufferError> {
        if self.id == 0 {
            return Err(FramebufferError::NotAppropriateUseOfDefaultFramebuffer);
        }
        unsafe {
            gl::BlitNamedFramebuffer(
                self.id,
                other.id,
                0,
                0,
                self.viewport.width(),
                self.viewport.height(),
                0,
                0,
                other.viewport.width(),
                other.viewport.height(),
                gl::COLOR_BUFFER_BIT,
                filter.to_param(),
            );
            Ok(())
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

            if texture.width() != viewport.width() || texture.height() != viewport.height() {
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
    ///Checks if it ready to draw
    pub fn complete(&mut self) -> bool {
        self.bind();
        let b = unsafe { gl::CheckFramebufferStatus(gl::FRAMEBUFFER) == gl::FRAMEBUFFER_COMPLETE };
        Self::unbind();
        b
    }

    fn contains(&self, attachment: FramebufferAttachment) -> bool {
        self.attachments
            .iter()
            .find(|x| x.0 == attachment)
            .is_some()
    }
}
impl Default for Framebuffer {
    fn default() -> Self {
        Self {
            id: 0,
            viewport: Viewport::new(0, 0, 0, 0),
            attachments: vec![],
            draw_buffers: vec![gl::FRONT_LEFT],
            draw_buffer_updated: false,
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FramebufferAttachment {
    Depth,
    Stencil,
    DepthStencil,
    Color(u32),
}

impl Into<u32> for FramebufferAttachment {
    fn into(self) -> u32 {
        match self {
            FramebufferAttachment::Color(n) => gl::COLOR_ATTACHMENT0 + n,
            FramebufferAttachment::Depth => gl::DEPTH_ATTACHMENT,
            FramebufferAttachment::Stencil => gl::STENCIL_ATTACHMENT,
            FramebufferAttachment::DepthStencil => gl::DEPTH_STENCIL_ATTACHMENT,
        }
    }
}
