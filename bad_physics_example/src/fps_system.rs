use engine_3d::{
    rendering::camera::{Camera, MainCamera},
    input_handler::SharedInputState,
    math::{vec3, Vec3},
    specs::*,
    time::Time,
    transform::Transform,
};

use crate::physics::{collision::Collision3D, rigidbody::Rigidbody};
pub struct FpsSystem {
    pub sensetivity: f32,
    pub move_speed: f32,
    pub rotation_x: f32,
    pub rotation_y: f32,
    pub camera_height: f32,
}
impl<'a> System<'a> for FpsSystem {
    type SystemData = (
        WriteStorage<'a, Camera>,
        WriteStorage<'a, Rigidbody>,
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Collision3D>,
        Write<'a, SharedInputState>,
        Write<'a, MainCamera>,
        Read<'a, Time>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut camera_storage,
            mut rigidbody,
            mut transform,
            collision,
            mut shared_input_state,
            main_c,
            time,
        ) = data;
        let input_state = shared_input_state.input_state();
        let movement_x = (input_state.button_repeat("s") as i32
            - input_state.button_repeat("w") as i32) as f32;
        let movement_z = (input_state.button_repeat("a") as i32
            - input_state.button_repeat("d") as i32) as f32;
        let movement = vec3(movement_x, 0.0, movement_z).normalize_or_zero() * self.move_speed;
        let (mouse_move_x, mouse_move_y) = input_state.mouse_move();

        for (camera, rigidbody, transform, collider) in (
            &mut camera_storage,
            &mut rigidbody,
            &mut transform,
            &collision,
        )
            .join()
        {
            //println!("{:?}", collider.collisions());
            let x_diff = mouse_move_y * time.delta_time() * self.sensetivity;
            let y_diff = mouse_move_x * time.delta_time() * self.sensetivity;
            self.rotation_y += y_diff;
            self.rotation_x += x_diff;
            transform.set_rotation(vec3(0.0, -self.rotation_y, 0.0));
            camera
                .transform
                .set_rotation(vec3(self.rotation_x, self.rotation_y, 0.0));
            camera.transform.position = transform.position + Vec3::Y * self.camera_height;
                println!("{}",rigidbody.velocity());
            if rigidbody.velocity().length_squared() <= 0.1{
                rigidbody.set_velocity(Vec3::ZERO);
            }
            rigidbody.add_force(
                movement.z * transform.forward() + movement.x * transform.right()
            );
        }
    }
}
