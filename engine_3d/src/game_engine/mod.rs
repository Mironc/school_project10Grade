pub mod event_handler;
pub mod input_handler;
pub mod window;
use input_handler::InputHandler;
use specs::{Dispatcher, World, WorldExt};
use window::{Window, WindowConfig};
use winit::{application::ApplicationHandler, event::DeviceEvent, event_loop::EventLoop};
pub struct ApplicationState {
    pub window: window::Window,
    input_handler: InputHandler,
    world: Option<World>,
    dispatcher: Option<Dispatcher<'static, 'static>>,
    focus: bool,
}

impl ApplicationState {
    pub(crate) fn new(window: Window) -> Self {
        Self {
            focus: true,
            world: None,
            dispatcher: None,
            window,
            input_handler: InputHandler::new(),
        }
    }
    pub(crate) fn set_focus(&mut self,focus:bool){
        self.focus = focus
    }
    pub fn focus(&self) -> bool{
        self.focus
    }
    pub fn update(&mut self) {
        if let Some(dispatcher) = self.dispatcher.as_mut() {
            if let Some(world) = self.world.as_mut() {
                world.write_resource::<time::Time>().update();
                dispatcher.dispatch(&world);
                self.input_handler._update();
                self.window.request_redraw();
                self.window.show_frame();
            }
        }
    }
}
pub struct Application {
    event_loop: EventLoop<()>,
    pub app_state: ApplicationState,
}
impl ApplicationHandler for ApplicationState {
    fn resumed(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        println!("resumed")
    }
    fn device_event(
            &mut self,
            _event_loop: &winit::event_loop::ActiveEventLoop,
            _device_id: winit::event::DeviceId,
            event: winit::event::DeviceEvent,
        ) {
        match event {
            winit::event::DeviceEvent::MouseMotion { delta } => self.input_handler._mouse_motion(delta),
            _=>(),
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if self.window.id() == window_id {
            if self.world.is_some() {
                event_handler::EventHandler::handle(self, event_loop, event);
            }
        }
    }

    fn suspended(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        println!("suspended");
    }
    
    fn memory_warning(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        println!("memory warning")
    }
}
impl Application {
    pub fn new(window_config: WindowConfig) -> Self {
        let event_loop = EventLoop::builder().build().unwrap();
        let window = Window::new(window_config, &event_loop);
        pretty_env_logger::init();
        Self {
            event_loop,
            app_state: ApplicationState::new(window),
        }
    }
    pub fn run(mut self, mut world: World, dispatcher: Dispatcher<'static, 'static>) {
        world.insert(time::Time::new());
        world.insert(self.app_state.input_handler.input_state());
        self.app_state.dispatcher = Some(dispatcher);
        self.app_state.world = Some(world);
        self.app_state.dispatcher.as_mut().unwrap().dispatch(&self.app_state.world.as_ref().unwrap());
        let _ = EventLoop::run_app(self.event_loop, &mut self.app_state);
    }
}