use graphics::{ecs::ResizeEvent, objects::viewport::Viewport};
use specs::WorldExt;
use winit::{event::WindowEvent, event_loop::ActiveEventLoop};

use super::ApplicationState;

pub struct EventHandler {}
impl EventHandler {
    pub fn handle(app: &mut ApplicationState, event_loop: &ActiveEventLoop, event: WindowEvent) {
        match event {
            WindowEvent::Resized(physical_size)  if physical_size.height != 0 && physical_size.width != 0 =>{
                if let Some(world) = &app.world{
                    let mut resize_event = world.write_resource::<ResizeEvent>();
                    resize_event.send(&Viewport::new(0, 0, physical_size.width as i32 , physical_size.height as i32));
                }
                app.window
                    .update_viewport(physical_size.width, physical_size.height)
            }
            //WindowEvent::ScaleFactorChanged { scale_factor, inner_size_writer } => todo!(),
            //WindowEvent::Moved(physical_position) => todo!(),
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Destroyed => (),
            WindowEvent::RedrawRequested => app.update(),

            //WindowEvent::CursorEntered { device_id } => todo!(),
            WindowEvent::Focused(s) => {
                app.set_focus(s);
            }
            //WindowEvent::CursorLeft { device_id } => todo!(),
            
            //WindowEvent::MouseWheel { device_id, delta, phase } => todo!(),
            WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } => {
                app.input_handler._button(event.logical_key, event.state,event.repeat);
            }
            WindowEvent::CursorMoved {
                device_id: _,
                position,
            } => app.input_handler._cursor_pos(position.x, position.y),
            WindowEvent::MouseInput {
                device_id: _,
                state,
                button,
            } => {
                app.input_handler._mouse_button(button, state);
            }
            _ => (),
        }
    }
}
