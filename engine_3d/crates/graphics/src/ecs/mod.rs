mod mesh_renderer;
pub use mesh_renderer::*;
mod light;
pub use light::*;
mod material;
pub use material::Material;
use specs::{World, WorldExt};
use transform::Transform;
mod camera;
pub use camera::*;
mod rendering;
pub use rendering::*;
pub fn init(world:&mut World){
    world.register::<MeshRenderer>();
    world.register::<Material>();
    world.register::<Light>();
    world.register::<Camera>();
    world.register::<Transform>();
    world.insert(ResizeEvent::default());
    //world.insert(MainCamera::);
}