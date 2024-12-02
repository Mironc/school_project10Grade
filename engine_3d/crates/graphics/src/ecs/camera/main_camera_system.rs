use specs::{Read, System, Write};

use crate::ecs::ResizeEvent;

use super::MainCamera;

pub struct MainCameraSystem{

}
impl<'a> System<'a> for MainCameraSystem {
    type SystemData = (Read<'a,ResizeEvent>,Write<'a,MainCamera>);

    fn run(&mut self, data: Self::SystemData) {
        let (resize_event,mut main_camera) = data;
        if resize_event.occured(){
            main_camera.resize(resize_event.viewport());
        }
    }
}