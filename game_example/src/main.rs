
pub mod free_cam;
use engine_3d::{assets::{image_importer::ImageImporter, model_importer::Modelmporter, Assets}, graphics::objects::texture::{Filter, Texture2DBuilder}, math::{vec3, Vec3}, post_processing::{PostProcessing, PostProcessingContainer, PostProcessingSystem, SimpleColorProcessing}, rendering::{camera::{projection::{Perspective, Projection}, Camera, CameraTransform, MainCamera, OnResizeEvent}, composition::CompositionSystem, light::{Light, LightProperties}, lit_shading::DeferredPath, material::Material, mesh_renderer::MeshRenderer, render_system::RenderSystem}, specs::{Builder, DispatcherBuilder, World, WorldExt}, transform::Transform, window::WindowConfig, Application};
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
    let mut win_conf = WINDOW_CONFIG;
    win_conf.icon = Some(
        engine_3d::image::load_from_memory(include_bytes!("../ico.jpg")).expect("error with ico"),
    );
    let app = Application::new(win_conf);
    let mut assets = Assets::from_data_file("assets.data").unwrap();
    let mut world = World::new();
    engine_3d::init(&mut world);
    /* let model = assets
    .import_model(&"models/pool.obj")
    .unwrap()
    .instantiate();
    let text = assets.import_image("models/pool_text.png").unwrap();world
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

    let checked_texture = Texture2DBuilder::new()
        .image(assets.import_image("models/checked_texture.png").unwrap())
        .filter(Filter::NearestLinearMipMap)
        .build()
        .unwrap();
    checked_texture.gen_mipmaps();
    //Plane
    let plane = assets
        .import_model("models/plane.obj")
        .unwrap()
        .instantiate();

    world
        .create_entity()
        .with(MeshRenderer::new(plane, None))
        .with(Transform::new(
            vec3(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
            0.125,
        ))
        .with(Material {
            shininess: 1024.0,
            specular: 0.3,
            main_texture: checked_texture.clone(),
            ..Default::default()
        })
        .build();

    let obstacles = assets
        .import_model("models/obstacles.obj")
        .unwrap()
        .instantiate();
    world
        .create_entity()
        .with(MeshRenderer::new(obstacles, None))
        .with(Transform::new(
            vec3(0.0, 0.0, 0.0),
            Vec3::new(0.0, 90.0, 0.0),
            0.125,
        ))
        .with(Material {
            shininess: 16.0,
            main_texture: checked_texture.clone(),
            ..Default::default()
        })
        .build();
    let mut postprocessing_container = PostProcessingContainer::new();
    postprocessing_container.set_f(|p,mut f, t| {
        let mut simple = p.get_mut::<SimpleColorProcessing>().unwrap();
        simple.apply_to(&mut f, t.clone());
    });

    postprocessing_container.insert(SimpleColorProcessing::new(1.0, 1.0, 1.0, 0.0, 0.6, 1.));
    let main_camera = world
        .create_entity()
        .with(Camera::new(
            Projection::Perspective(Perspective::new(
                1.0,
                1000.0,
                45.0,
                app.app_state.window.viewport(),
            )),
            CameraTransform::new(vec3(0.0, 2.0, 6.0), vec3(0.0, 0.7, 0.0)),
            app.app_state.window.viewport(),
            DeferredPath::new(app.app_state.window.viewport(), vec3(0.0, 0.0, 0.0)),
        ))
        .with(postprocessing_container.clone())
        .build();
    world.fetch_mut::<MainCamera>().set(main_camera);

    // world.insert(Sun::new(vec3(-1.0, -1.0, 0.0), vec3(0.9, 0.84, 0.19)));
    let n = 100;
    let mut rand = rand::prelude::StdRng::from_seed([102; 32]);
    for _ in 0..n {
        world
            .create_entity()
            .with(Light::Point(LightProperties {
                power: rand.gen_range(3.0..10.0),
                color: vec3(
                    rand.gen_range(0.1..1.0),
                    rand.gen_range(0.1..1.0),
                    rand.gen_range(0.0..1.0),
                ),
            }))
            .with(Transform::from_position(vec3(
                rand.gen_range(-100.0..100.0),
                rand.gen_range(0.1..5.0),
                rand.gen_range(-100.0..100.0),
            )))
            .build();
    }
    world
        .create_entity()
        .with(Light::Point(LightProperties {
            power: 5.0,
            color: vec3(0.5, 1.0, 0.9),
        }))
        .with(Transform::from_position(vec3(1.0, 1.0, 25.0)))
        .build();
    world
        .create_entity()
        .with(Light::Point(LightProperties {
            power: 5.0,
            color: vec3(1.0, 0.0, 0.9),
        }))
        .with(Transform::from_position(vec3(1.0, 2.0, 30.0)))
        .build();

    let dispatcher = DispatcherBuilder::new()
        .with(
            FreeCameraSystem {
                sensetivity: 20.0,
                move_speed: 5.0,
                rotation_x: 0.0,
                rotation_y: 90.0,
            },
            "FreeCameraSystem",
            &[],
        )
        .with_thread_local(OnResizeEvent {})
        .with_thread_local(RenderSystem::new())
        .with_thread_local(PostProcessingSystem {})
        .with_thread_local(CompositionSystem::new())
        .build();
    app.run(world, dispatcher)
}
