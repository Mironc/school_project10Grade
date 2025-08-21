use engine_3d::{
    assets::{image_importer::ImageImporter, model_importer::Modelmporter}, graphics::{
        self as graphics,
        objects::{
            buffers::FramebufferAttachment,
            texture::{Filter, Texture2D, Texture2DBuilder},
            viewport::Viewport,
        },
    }, math::Vec3, post_processing::{DistanceFog, FogSystem, PostProcessing, PostProcessingContainer, PostProcessingSystem, SimpleColorProcessing, Tonemapping}, rendering::{camera::{projection::{Perspective, Projection}, Camera, CameraTransform, MainCamera, OnResizeEvent}, composition::CompositionSystem, light::{Light, LightProperties, Sun}, lit_shading::DeferredPath, material::Material, mesh_renderer::MeshRenderer, render_system::RenderSystem}
};
pub mod white;
pub mod free_cam;
pub extern crate self as s;
pub mod physics;
pub mod fps_system;

use engine_3d::assets::Assets;
use engine_3d::math::vec3;
use engine_3d::specs::*;
use engine_3d::transform::Transform;
use engine_3d::{window::WindowConfig, Application};
use fps_system::FpsSystem;
use free_cam::FreeCameraSystem;
use physics::{physics_system::PhysicsSystem, rigidbody::Rigidbody};
use physics::{
    collision::{Collision3D, MeshShape, Shape},
    collision_draw::DebugCollisions,
    collision_system::CollisionSystem,
    gravity::{GravitySystem, Static},
};
use rand::{Rng, SeedableRng};

pub const WINDOW_CONFIG: WindowConfig = WindowConfig {
    title: "Game",
    fullscreen: false,
    icon: None,
    width: 960,
    height: 540,
};
fn main() {
    for arg in std::env::args() {
        println!("{}", arg);
    }
    let mut win_conf = WINDOW_CONFIG;
    win_conf.icon = None;
    let app = Application::new(win_conf);
    let mut assets = Assets::from_data_file("assets.data").unwrap();
    let mut world = World::new();
    engine_3d::init(&mut world);
    world.register::<Rigidbody>();
    world.register::<Collision3D>();
    world.register::<Static>();
    let snow_texture = Texture2DBuilder::new()
        .image(assets.import_image("snow.jpg").unwrap())
        .filter(Filter::NearestLinearMipMap)
        .build()
        .unwrap();
    snow_texture.gen_mipmaps();
    //bridge
    let model = assets.import_model("bridge.obj").unwrap();
    let transform = Transform::new(
        vec3(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
        1.0,
    );
    world
        .create_entity()
        .with(MeshRenderer::new(model.instantiate(), None))
        .with(transform)
        .with(Static {})
        //.with(Collision3D::new(Shape::Box(BoxShape::new(Vec3::ZERO, vec3(1.0, 1.0, 1.0)))))
        .with(Collision3D::new(Shape::Mesh(MeshShape::new(model.clone()))))
        .with(Material {
            shininess: 1024.0,
            specular: 0.3,
            main_texture: Texture2D::white(),
            color:vec3(0.5, 0.382, 0.201),
            ..Default::default()
        })
        .build();
    //bridge fence
    let model = assets.import_model("bridge_fence.obj").unwrap();
    let transform = Transform::new(
        vec3(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
        1.0,
    );
    world
        .create_entity()
        .with(MeshRenderer::new(model.instantiate(), None))
        .with(transform)
        .with(Static {})
        //.with(Collision3D::new(Shape::Box(BoxShape::new(Vec3::ZERO, vec3(1.0, 1.0, 1.0)))))
        .with(Collision3D::new(Shape::Mesh(MeshShape::new(model.clone()))))
        .with(Material {
            shininess: 1024.0,
            specular: 0.3,
            main_texture: Texture2D::white(),
            color:vec3(0.5, 0.382, 0.201),
            ..Default::default()
        })
        .build();
    //lake terrain
    let model = assets.import_model("lake_terrain.obj").unwrap();
    let transform = Transform::new(
        vec3(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
        1.0,
    );

    world
        .create_entity()
        .with(MeshRenderer::new(model.instantiate(), None))
        .with(transform)
        .with(Static {})
        //.with(Collision3D::new(Shape::Box(BoxShape::new(Vec3::ZERO, vec3(1.0, 1.0, 1.0)))))
        .with(Collision3D::new(Shape::Mesh(MeshShape::new(model.clone()))))
        .with(Material {
            shininess: 1024.0,
            specular: 0.3,
            main_texture: snow_texture.clone(),
            ..Default::default()
        })
        .build();
    //Lake
    let model = assets.import_model("lake_plane.obj").unwrap();
    let transform = Transform::new(
        vec3(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
        1.0,
    );
    world
        .create_entity()
        .with(MeshRenderer::new(model.instantiate(), None))
        .with(transform)
        .with(Static {})
        //.with(Collision3D::new(Shape::Box(BoxShape::new(Vec3::ZERO, vec3(1.0, 1.0, 1.0)))))
        .with(Collision3D::new(Shape::Mesh(MeshShape::new(model))))
        .with(Material {
            shininess: 1024.0,
            specular: 1.0,
            main_texture: Texture2D::white(),
            color:vec3(0.099, 0.405, 0.801),
            ..Default::default()
        })
        .build();
    let miku_texture = Texture2DBuilder::new()
        .image(assets.import_image("models/Miku_body.png").unwrap())
        .build()
        .unwrap();
    miku_texture.gen_mipmaps();
    let miku = assets.import_model("models/Miku.obj").unwrap();
    let transform =Transform::new(
        vec3(0.0, 10.0, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
        1.0,
    );
    /* world
        .create_entity()
        .with(MeshRenderer::new(miku.instantiate(), None))
        .with(transform)
        .with(Material {
            shininess: 16.0,
            main_texture: miku_texture.clone(),
            ..Default::default()
        })
        .with(Rigidbody::new(&transform, 1.0, 0.2,1.0))
        .with(Collision3D::new(Shape::Mesh(MeshShape::new(miku))))
        //.with(Collision3D::new(Shape::Box(BoxShape::new(Vec3::ZERO, Vec3::ONE))))
        //.with(Collision3D::new(Shape::Sphere(SphereShape::new(Vec3::ZERO, 1.0))))
        .build(); */
    let mut postprocessing_container = PostProcessingContainer::new();

    postprocessing_container.insert(SimpleColorProcessing::new(1.0, 1.0, 1.0, 0.0, 0.6, 1.));
    postprocessing_container.insert(DistanceFog::new(0.1,10.0,vec3(1.0, 1.0, 1.0)));
    postprocessing_container.insert(Tonemapping::new());

    postprocessing_container.set_f(|p, f, t| {
        let tonemapping = p.get_mut::<Tonemapping>().unwrap();
        tonemapping.apply_to(f, t.clone());
        f.attachment_texture(FramebufferAttachment::Color(0))
            .unwrap()
            .copy_to(t);
        let simple = p.get_mut::<SimpleColorProcessing>().unwrap();
        simple.apply_to(f, t.clone());
        f.attachment_texture(FramebufferAttachment::Color(0)).unwrap().copy_to(t);
        let fog = p.get_mut::<DistanceFog>().unwrap();
        fog.apply_to(f,t.clone());
    });
    let cylinder = assets.import_model("models/cylinder.obj").unwrap();
    let main_camera = world
        .create_entity()
        .with({
            let mut cam = Camera::new(
                Projection::Perspective(Perspective::new(
                    0.1,
                    1000.0,
                    45.0,
                    app.app_state.window.viewport(),
                )),
                CameraTransform::new(vec3(0.0, 2.0, 0.0), vec3(0.0, 0.7, 0.0)),
                Viewport::new(0, 0, 1, 1),
                DeferredPath::new(app.app_state.window.viewport(),Vec3::splat(0.3)),
                //ForwardPath::new(app.app_state.window.viewport(), true),
            );
            cam.set_scale_factor(1.0);
            cam
        })
        .with(postprocessing_container.clone())
        .with(Rigidbody::new(&Transform::default(), 4.0, 0.2,0.0))
        .with(Collision3D::new(Shape::Mesh(MeshShape::new(cylinder))))
        .with(Transform::new(vec3(0.0, 2.0, 0.0),Vec3::ZERO,0.2))
        .build();
    world.fetch_mut::<MainCamera>().set(main_camera);

    world.insert(Sun::new(vec3(-1.0, -1.0, 0.0), vec3(0.9, 0.84, 0.19)));
    
    world.register::<Rigidbody>();

    let dispatcher = DispatcherBuilder::new()
        .with(
            /* FreeCameraSystem {
                sensetivity: 20.0,
                move_speed: 5.0,
                rotation_x: 0.0,
                rotation_y: 90.0,
            } */
           FpsSystem{ sensetivity: 20.0, move_speed: 4.0, rotation_x: 0.0, rotation_y: 90.0, camera_height: 1.5 },
            "FreeCameraSystem",
            &[],
        )
        .with(PhysicsSystem::new(1, 0.0), "phys", &[])
        .with_thread_local(OnResizeEvent {})
        .with_thread_local(RenderSystem::new())
        .with_thread_local(FogSystem{})
        .with_thread_local(PostProcessingSystem {})
        /* .with_thread_local(DebugCollisions::new(
            1.0,
            vec3(0.0, 1.0, 0.0),
            vec3(1.0, 0.0, 0.0),
        )) */
        .with_thread_local(CompositionSystem::new())
        .build();
    app.run(world, dispatcher)
}
