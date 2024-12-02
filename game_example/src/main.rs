use engine_3d::{
    assets::{image_importer::ImageImporter, model_importer::Modelmporter},
    graphics::{
        self as graphics,
        ecs::{
            MainCamera, postprocessing::SimplePostProcessing, projection::*, Camera, CameraTransform, DeferredRendering, ForwardRendering, Light, LightProperties, Material, MeshRenderer, RenderSystem
        },
        objects::texture::{Filter, Texture2DBuilder, TextureFormat, TextureWrap},
    },
    math::Vec3,
};
pub mod free_cam;
use engine_3d::assets::Assets;
use engine_3d::math::vec3;
use engine_3d::specs::*;
use engine_3d::transform::Transform;
use engine_3d::{window::WindowConfig, Application};
use free_cam::FreeCameraSystem;
use rand::{Rng, SeedableRng};

pub const WINDOW_CONFIG: WindowConfig = WindowConfig {
    title: "Game",
    fullscreen: true,
    icon: None,
    width: 960,
    height: 540,
};
fn main() {
    for arg in std::env::args() {
        println!("{}", arg);
    }

    let app = Application::new(WINDOW_CONFIG);
    let mut assets = Assets::from_data_file("assets.data").unwrap();
    let mut world = World::new();
    graphics::ecs::init(&mut world);
    /* let model = assets
        .import_model(&"models/pool.obj")
        .unwrap()
        .instantiate();
    let text = assets.import_image("models/pool_text.png").unwrap();
    world
        .create_entity()
        .with(MeshRenderer::new(model, None))
        .with(Transform::default())
        .with(Material {
            main_texture: Texture2DBuilder::new()
                .internal_format(TextureFormat::RGBA)
                .filter(Filter::Linear)
                .wrap(TextureWrap::Repeat)
                .gen_mipmaps()
                .image(text)
                .build()
                .unwrap(),
            shininess: 64.0,
            ..Default::default()
        })
        .build(); */
    /*
    let model = assets
        .import_model("models/floor_wall.obj")
        .unwrap()
        .instantiate();
    world
        .create_entity()
        .with(MeshRenderer::new(model, None))
        .with(Transform::new(Vec3::ZERO, Vec3::ZERO, 1.0))
        .with(Material {
            shininess: 16.0,
            ..Default::default()
        })
        .build();
    let model = assets
        .import_model("models/obstacles.obj")
        .unwrap()
        .instantiate();
    world
        .create_entity()
        .with(MeshRenderer::new(model, None))
        .with(Transform::new(Vec3::ZERO, Vec3::ZERO, 1.0))
        .with(Material {
            shininess: 16.0,
            ..Default::default()
        })
        .build();
    */
    //Plane
     let model = assets
           .import_model("models/plane.obj")
           .unwrap()
           .instantiate();
       world
           .create_entity()
           .with(MeshRenderer::new(model, None))
           .with(Transform::new(vec3(0.0, 0.0, 6.0), Vec3::ZERO, 0.125))
           .with(Material {
               shininess: 1023.0,
               ..Default::default()
           })
           .build();

       let model = assets
           .import_model("models/obstacles.obj")
           .unwrap()
           .instantiate();
       world
           .create_entity()
           .with(MeshRenderer::new(model, None))
           .with(Transform::new(vec3(0.0, 0.0, 20.0), Vec3::ZERO, 0.25))
           .with(Material {
               shininess: 8.0,
               ..Default::default()
           })
           .build();
   
    let main_camera = world
        .create_entity()
        .with(Camera::new(
            Projection::Perspective(Perspective::new(
                1.0,
                1000.0,
                45.0,
                app.app_state.window.get_viewport(),
            )),
            CameraTransform::new(vec3(0.0, 6.1, 16.0), vec3(0.0, 0.7, 0.0)),
        ))
        .build();
    world.insert(MainCamera::new(main_camera));
    let n = 100;
    let mut rand = rand::prelude::StdRng::from_seed([192; 32]);
    for _ in 0..n {
        world
            .create_entity()
            .with(Light::Point(LightProperties {
                power: rand.gen_range(3.0..5.0),
                color: vec3(
                    rand.gen_range(0.1..1.0),
                    rand.gen_range(0.1..1.0),
                    rand.gen_range(0.1..1.0),
                ),
            }))
            .with(Transform::from_position(vec3(
                rand.gen_range(-100.0..100.0),
                rand.gen_range(0.1..5.0),
                rand.gen_range(0.1..50.0),
            )))
            .build();
    }
    world
        .create_entity()
        .with(Light::Point(LightProperties {
            power: 5.0,
            color: vec3(
                0.5,
                1.0,
                0.9,
            ),
        }))
        .with(Transform::from_position(vec3(
            1.0,
            1.0,
            25.0,
        )))
        .build();
    world
        .create_entity()
        .with(Light::Point(LightProperties {
            power: 10.0,
            color: vec3(
                1.0,
                0.0,
                0.9,
            ),
        }))
        .with(Transform::from_position(vec3(
            1.0,
            2.0,
            30.0,
        )))
        .build();

    let dispatcher = DispatcherBuilder::new()
        .with(
            FreeCameraSystem {
                sensetivity: 20.0,
                move_speed: 5.0,
                rotation_x: 0.0,
                rotation_y: -90.0,
            },
            "FreeCameraSystem",
            &[],
        )
        .with_thread_local(RenderSystem::new(
            //ForwardRendering::new(*app.app_state.window.get_viewport(), true),
            DeferredRendering::new(*app.app_state.window.get_viewport()),
            app.app_state.window.get_viewport(),
            SimplePostProcessing::new(1.0,1.2,0.0, *app.app_state.window.get_viewport())
        ))
        .build();
    app.run(world, dispatcher)
}
