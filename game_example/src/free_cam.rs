use engine_3d::{
    graphics::ecs::{MainCamera, Camera},
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
        let (mut cameras, mut shared_input_state, mut main_c, time) = data;

        let input_state = shared_input_state.input_state();
        let movement_z = (input_state.button_repeat("s") as i32
            - input_state.button_repeat("w") as i32) as f32
            * self.move_speed;
        let movement_x = (input_state.button_repeat("a") as i32
            - input_state.button_repeat("d") as i32) as f32
            * self.move_speed;
        //println!("{} {} {:?} {:?} {:?} {:?}",movement_x,movement_z,input_state.button("w"),input_state.button("s"),input_state.button("a"),input_state.button("d"));
        let (mouse_move_x, mouse_move_y) = input_state.mouse_move();
        for camera in (&mut cameras).join() {
            let transform = &mut camera.transform;
            let x_diff = mouse_move_y * time.delta_time() * self.sensetivity;
            let y_diff = mouse_move_x * time.delta_time() * self.sensetivity;
            self.rotation_y += y_diff;
            self.rotation_x += x_diff;
            transform.set_rotation(vec3(self.rotation_x, self.rotation_y, 0.0));

            transform.position += (movement_z * transform.forward()
                + movement_x * transform.right())
                * time.delta_time();
        }
    }
}
