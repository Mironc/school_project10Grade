use engine_3d::{
    gl,
    graphics::{
        compare_opt::CompareOption,
        draw_options::{
            depth,
            face_culling::{self, CullFace},
        },
        objects::{
            model::{InstancedModel, Model},
            shader::{Shader, ShaderType, SubShader},
            vertex::ModelVertex,
        },
    },
    math::{vec3, Vec3},
    rendering::camera::{Camera, MainCamera},
    specs::{Join, Read, ReadStorage, System, WriteStorage},
    transform::Transform,
};

use super::collision::Collision3D;

pub struct DebugCollisions {
    sphere: InstancedModel,
    _box: InstancedModel,
    ///Shader used to draw colliders
    shader: Shader, //maybe smth other
    /*
    color: Vec3,
    non_front_face_color: Vec3,
    */
    line_width: f32,
}
impl DebugCollisions {
    pub fn new(line_width: f32, color: Vec3, non_normal_color: Vec3) -> Self {
        let _box = Model::<ModelVertex>::from_str(include_str!("box.obj"))
            .unwrap()
            .instantiate();
        let sphere = Model::<ModelVertex>::from_str(include_str!("icosahedron.obj"))
            .unwrap()
            .instantiate();
        let mut shader = Shader::new([
            SubShader::new(include_str!("collider_vs.glsl"), ShaderType::Vertex),
            SubShader::new(include_str!("collider_fs.glsl"), ShaderType::Fragment),
        ]);
        shader.set_vec3("color", &color);
        shader
            .set_vec3("non_normal_color", &non_normal_color);
        Self {
            sphere,
            _box,
            line_width,
            shader, /*
                    color,
                    non_front_face_color: non_normal_color, */
        }
    }
}
impl<'a> System<'a> for DebugCollisions {
    type SystemData = (
        ReadStorage<'a, Collision3D>,
        ReadStorage<'a, Transform>,
        WriteStorage<'a, Camera>,
        Read<'a, MainCamera>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (collisions, transforms, mut cameras, main_camera) = data;
        let main_camera = if let Some(main_camera) = main_camera.get_mut(&mut cameras) {
            main_camera
        } else {
            return;
        };
        let mut shader = &self.shader;
        shader.bind();
        depth::enable();
        depth::set_cmp_func(CompareOption::Less);
        depth::set_write(true);
        face_culling::enable();
        face_culling::set_cullface(CullFace::Front);
        unsafe {
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
        }
        main_camera.render_image.bind();
        for (collision, transform) in (&collisions, &transforms).join() {
            shader.set_matrix4("mv", &(main_camera.proj_mat() * main_camera.get_view()));
            match collision.shape() {
                super::collision::Shape::Mesh(mesh_shape) => {
                    shader.set_matrix4("transform", &transform.get_matrix());
                    mesh_shape.debug().draw();
                }
                super::collision::Shape::Sphere(sphere_shape) => {
                    shader.set_matrix4(
                        "transform",
                        &Transform::new(
                            transform.position + sphere_shape.centre(),
                            Vec3::ZERO,
                            sphere_shape.radius(),
                        )
                        .get_matrix(),
                    );
                    self.sphere.draw();
                }
                super::collision::Shape::Box(box_shape) => {
                    shader.set_matrix4(
                        "transform",
                        &Transform::new_nonuniform_scale(
                            transform.position + box_shape.centre(),
                            Vec3::ZERO,
                            box_shape.size(),
                        )
                        .get_matrix(),
                    );
                    self._box.draw();
                }
            };
        }
        unsafe {
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
        }
        depth::disable();
    }
}
