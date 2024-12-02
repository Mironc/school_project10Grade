use std::time::Instant;
#[derive(Debug, Clone, Copy)]
pub struct Time {
    startup: Instant,
    last_frame: Instant,
    time:f32,
    delta_time:f32,
    frame_count: u64,
}
impl Time {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn delta_time(&self) -> f32 {
        self.delta_time
    }
    pub fn time(&self) -> f32 {
        self.time
    }
    pub fn update(&mut self) {
        self.delta_time = self.last_frame.elapsed().as_secs_f32();
        self.time = self.startup.elapsed().as_secs_f32();
        self.last_frame = Instant::now();
        self.frame_count += 1;
    }
}
impl Default for Time {
    fn default() -> Self {
        Self {
            startup: Instant::now(),
            last_frame: Instant::now(),
            frame_count: 0,
            delta_time: 0.0,
            time: 0.0,
        }
    }
}
