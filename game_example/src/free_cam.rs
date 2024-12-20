use engine_3d::{
    graphics::ecs::{Camera, MainCamera},
    input_handler::SharedInputState,
    math::vec3,
    specs::*,
    time::Time,
};
pub struct FreeCameraSystem {
    pub sensetivity: f32,
    pub move_speed: f32,
    pub rotation_x: f32,
    pub rotation_y: f32,
}
impl<'a> System<'a> for FreeCameraSystem {
    type SystemData = (
        WriteStorage<'a, Camera>,
        Write<'a, SharedInputState>,
        Write<'a, MainCamera>,
        Read<'a, Time>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut camera_storage, mut shared_input_state, main_c, time) = data;

        let input_state = shared_input_state.input_state();
        let movement_z = (input_state.button_repeat("s") as i32
            - input_state.button_repeat("w") as i32) as f32
            * self.move_speed;
        let movement_x = (input_state.button_repeat("a") as i32
            - input_state.button_repeat("d") as i32) as f32
            * self.move_speed;
        let (mouse_move_x, mouse_move_y) = input_state.mouse_move();
        let camera = camera_storage.get_mut(main_c.id().unwrap()).unwrap();

        let x_diff = mouse_move_y * time.delta_time() * self.sensetivity;
        let y_diff = mouse_move_x * time.delta_time() * self.sensetivity;
        self.rotation_y += y_diff;
        self.rotation_x += x_diff;
        camera.transform.set_rotation(vec3(self.rotation_x, self.rotation_y, 0.0));
        camera.transform.position +=
            (movement_z * camera.transform.forward() + movement_x * camera.transform.right()) * time.delta_time();
    }
}
