use specs::{Join, Read, System, Write, WriteStorage};

use graphics::resize_event::ResizeEvent;

use super::{Camera, MainCamera};

pub struct OnResizeEvent {}
impl<'a> System<'a> for OnResizeEvent {
    type SystemData = (
        Read<'a, ResizeEvent>,
        Write<'a, MainCamera>,
        WriteStorage<'a, Camera>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (resize_event, main_camera, mut camera_storage) = data;

        {
            println!("resized");
            for camera in (&mut camera_storage).join() {
                camera.set_viewport(resize_event.viewport());
            }
        }
    }
}
