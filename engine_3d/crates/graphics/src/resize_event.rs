use std::sync::{Arc, Mutex};

use crate::objects::viewport::Viewport;

#[derive(Default)]
pub struct ResizeEvent {
    subscribers: Vec<Arc<Mutex<dyn ResizeSubscriber>>>,
    viewport: Viewport,
}
impl ResizeEvent {
    pub fn viewport(&self) -> Viewport {
        self.viewport
    }
    pub fn send(&mut self, viewport: Viewport) {
        self.viewport = viewport;
        for sub in self.subscribers.iter() {
            sub.lock().unwrap().on_resize(viewport);
        }
    }
}
pub trait ResizeSubscriber {
    fn on_resize(&mut self, viewport: Viewport);
}
