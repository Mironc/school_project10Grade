use specs::{Join, Read, System, Write, WriteStorage};

use crate::ecs::ResizeEvent;

use super::{projection::Projection, Camera, MainCamera};

pub struct OnResizeEvent{

}
impl<'a> System<'a> for OnResizeEvent {
    type SystemData = (Read<'a,ResizeEvent>,Write<'a,MainCamera>,WriteStorage<'a,Camera>);

    fn run(&mut self, data: Self::SystemData) {
        let (resize_event,mut main_camera,mut camera_storage) = data;
        if resize_event.occured(){
            println!("resized");
            main_camera.set_viewport(resize_event.viewport());
            for camera in (&mut camera_storage).join() {
                match &mut camera.projection {
                    Projection::Perspective(perspective) => perspective.viewport_update(resize_event.viewport()),
                    Projection::Orthogonal(orthogonal) => orthogonal.viewport_update(resize_event.viewport()),
                } 
            }
        }
    }
}