use glutin::config::{Config, ConfigTemplateBuilder};
use glutin::context::{ContextApi, ContextAttributesBuilder, NotCurrentContext, Version};
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin::surface::{Surface, SurfaceAttributes, SwapInterval, WindowSurface};
use graphics::objects::viewport::Viewport;
use image::DynamicImage;
use raw_window_handle::HasWindowHandle;
use std::num::NonZero;
use std::ops::{Deref, DerefMut};
use winit::dpi::LogicalSize;
use winit::event_loop::{ControlFlow, DeviceEvents, EventLoop};
use winit::platform::windows::WindowExtWindows;
use winit::window::{self, Icon, WindowAttributes};

use glutin_winit::{DisplayBuilder, GlWindow};
pub struct WindowConfig {
    pub width: u32,
    pub height: u32,
    pub fullscreen: bool,
    pub icon: Option<DynamicImage>,
    pub title: &'static str,
}
pub struct Window {
    window: window::Window,
    gl_surface: Surface<WindowSurface>,
    viewport: Viewport,
    context: glutin::context::PossiblyCurrentContext,
}
pub fn gl_config_picker(configs: Box<dyn Iterator<Item = Config> + '_>) -> Config {
    if let Some(config) = configs.reduce(|accum, config| {
        let transparency_check = config.supports_transparency().unwrap_or(false)
            & !accum.supports_transparency().unwrap_or(false);

        if transparency_check || config.num_samples() > accum.num_samples() {
            config
        } else {
            accum
        }
    }) {
        config
    } else {
        panic!("No configs or no fitting config")
    }
}
fn create_gl_context(window: &window::Window, gl_config: &Config) -> NotCurrentContext {
    let raw_window_handle = window.window_handle().ok().map(|wh| wh.as_raw());

    let context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::OpenGl(Some(Version { major: 3, minor: 2 })))
        .build(raw_window_handle);

    let fallback_context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::Gles(Some(Version::new(3, 0))))
        .build(raw_window_handle);

    let legacy_context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::OpenGl(Some(Version::new(4, 3))))
        .build(raw_window_handle);

    // Reuse the uncurrented context from a suspended() call if it exists, otherwise
    // this is the first time resumed() is called, where the context still
    // has to be created.
    let gl_display = gl_config.display();

    unsafe {
        gl_display
            .create_context(gl_config, &context_attributes)
            .unwrap_or_else(|_| {
                gl_display
                    .create_context(gl_config, &fallback_context_attributes)
                    .unwrap_or_else(|_| {
                        gl_display
                            .create_context(gl_config, &legacy_context_attributes)
                            .expect("failed to create context")
                    })
            })
    }
}
fn window_attributes(window_config: &WindowConfig) -> WindowAttributes {
    WindowAttributes::default()
        .with_inner_size(LogicalSize::new(window_config.width, window_config.height))
        .with_maximized(window_config.fullscreen)
        .with_title(window_config.title)
}
impl Window {
    pub fn new(window_config: WindowConfig, event_loop: &EventLoop<()>) -> Self {
        let template_builder = ConfigTemplateBuilder::default();

        let (window, config) = DisplayBuilder::new()
            .with_window_attributes(Some(window_attributes(&window_config)))
            .with_preference(glutin_winit::ApiPreference::PreferEgl)
            .build(event_loop, template_builder, gl_config_picker)
            .unwrap();
        let window = window.unwrap();
        if let Some(icon) = window_config.icon {
            let rgba_icon = icon.to_rgba8();
            window.set_window_icon(Some(
                Icon::from_rgba(rgba_icon.to_vec(), rgba_icon.width(), rgba_icon.height()).unwrap(),
            ));
        }
        let attrs = window
            .build_surface_attributes(Default::default())
            .expect("Failed to build surface attributes");
        let gl_surface = unsafe {
            config
                .display()
                .create_window_surface(&config, &attrs)
                .unwrap()
        };
        let context = create_gl_context(&window, &config)
            .make_current(&gl_surface)
            .unwrap();
        gl_surface
            .set_swap_interval(&context, SwapInterval::DontWait)
            .unwrap();
        //TODO:icon load
        gl::load_with(|symbol| {
            let symbol = std::ffi::CString::new(symbol).unwrap();
            config.display().get_proc_address(symbol.as_c_str()).cast()
        });

        Self {
            window,
            gl_surface,
            context,
            viewport: Viewport::new(
                0,
                0,
                window_config.width as i32,
                window_config.height as i32,
            ),
        }
    }

    pub(crate) fn update_viewport(&mut self, width: u32, height: u32) {
        unsafe {
            self.viewport.set_size(width as i32, height as i32);
            gl::Viewport(0, 0, width as i32, height as i32)
        }
        self.gl_surface.resize(
            &self.context,
            NonZero::new(width as u32).unwrap(),
            NonZero::new(height as u32).unwrap(),
        );
    }

    pub(crate) fn show_frame(&mut self) {
        self.gl_surface.swap_buffers(&self.context).unwrap();
    }
    pub fn get_viewport(&self) -> &Viewport {
        &self.viewport
    }
}
impl Deref for Window {
    type Target = window::Window;

    fn deref(&self) -> &Self::Target {
        &self.window
    }
}
impl DerefMut for Window {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.window
    }
}
