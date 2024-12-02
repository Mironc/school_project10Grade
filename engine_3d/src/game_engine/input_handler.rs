use std::{
    ops::Deref,
    sync::{Arc, RwLock, RwLockReadGuard},
};
use winit::{
    event::{ElementState, MouseButton, RawKeyEvent},
    keyboard::Key,
};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Press,
    Repeat,
    Release,
}
impl From<ElementState> for Action {
    fn from(value: ElementState) -> Self {
        match value {
            ElementState::Pressed => Self::Press,
            ElementState::Released => Self::Release,
        }
    }
}
///Gives thread safe acceess to InputState which gives possibility to use it with Dispatcher::dispatch_par()
///call "input_state()" to acquire InputState
#[derive(Debug, Default, Clone)]
pub struct SharedInputState {
    input_state: Arc<RwLock<InputState>>,
}
impl SharedInputState {
    pub fn new() -> Self {
        Self {
            input_state: Arc::new(RwLock::new(InputState::new())),
        }
    }

    ///causes RwLock::read()
    pub fn input_state(&mut self) -> RwLockReadGuard<'_, InputState> {
        self.input_state.read().unwrap()
    }
}
impl Deref for SharedInputState {
    type Target = Arc<RwLock<InputState>>;

    fn deref(&self) -> &Self::Target {
        &self.input_state
    }
}
#[derive(Debug, Clone)]
pub struct InputState {
    buttons: Vec<(Key, Option<Action>)>,
    cursor_pos: (f64, f64),
    mouse_move: (f64, f64),
    mouse_buttons: [Option<Action>; 64],
    scroll_delta: (f64, f64),
}
impl InputState {
    pub fn new() -> Self {
        Self {
            buttons: Vec::new(),
            cursor_pos: (0.0, 0.0),
            mouse_move: (0.0, 0.0),
            mouse_buttons: [None; 64],
            scroll_delta: (0.0, 0.0),
        }
    }
    pub fn mouse_move(&self) -> (f32, f32) {
        (self.mouse_move.0 as f32, self.mouse_move.1 as f32)
    }
    pub fn cursor_pos(&self) -> (f32, f32) {
        (self.cursor_pos.0 as f32, self.cursor_pos.1 as f32)
    }
    pub fn scroll(&self) -> f64 {
        self.scroll_delta.1
    }
    pub fn mouse_button(&self, button_n: usize) -> Option<Action> {
        *(self.mouse_buttons.get(button_n)?)
    }
    pub fn button(&self, key_literal: &str) -> Option<Action> {
        self.buttons
            .iter()
            .find(|x| {
                if let Some(key) = x.0.to_text() {
                    return key == key_literal;
                } else {
                    false
                }
            })?
            .1
    }
    pub fn button_released(&self, key_literal: &str) -> bool {
        self.buttons.iter().any(|x| {
            x.0.to_text() == Some(key_literal) && x.1.unwrap_or(Action::Press) == Action::Release
        })
    }
    pub fn button_pressed(&self, key_literal: &str) -> bool {
        self.buttons.iter().any(|x| {
            x.0.to_text() == Some(key_literal) && x.1.unwrap_or(Action::Release) == Action::Press
        })
    }
    pub fn button_repeat(&self, key_literal: &str) -> bool {
        self.buttons.iter().any(|x| {
            x.0.to_text() == Some(key_literal) && x.1.unwrap_or(Action::Release) == Action::Repeat
        })
    }
}
impl Default for InputState {
    fn default() -> Self {
        Self::new()
    }
}
//TODO:Integration with ecs
pub struct InputHandler {
    input_state: SharedInputState,
}
impl InputHandler {
    pub fn new() -> Self {
        Self {
            input_state: SharedInputState::new(),
        }
    }
    pub fn input_state(&self) -> SharedInputState {
        self.input_state.clone()
    }
    pub(crate) fn _cursor_pos(&mut self, x: f64, y: f64) {
        let mut input_state = self.input_state.write().unwrap();
        input_state.cursor_pos = (x, y);
    }
    pub(crate) fn _mouse_button(&mut self, mouse_button: MouseButton, action: ElementState) {
        let mut input_state = self.input_state.write().unwrap();
        input_state.mouse_buttons[match mouse_button {
            MouseButton::Left => 1,
            MouseButton::Right => 2,
            MouseButton::Middle => 3,
            MouseButton::Back => 4,
            MouseButton::Forward => 5,
            MouseButton::Other(n) => n as usize,
        }] = Some(Action::from(action));
    }
    pub(crate) fn _mouse_motion(&mut self,mouse_motion:(f64,f64)) {
        let mut input_state = self.input_state.write().unwrap();
        input_state.mouse_move = mouse_motion;
    }
    pub(crate) fn _button(&mut self,key:Key,state:ElementState,repeat:bool) {
        let mut input_state = self.input_state.write().unwrap();
        if repeat{
            return;
        }
        if let Some(key) = input_state.buttons.iter().position(|x| x.0 == key) {
            input_state.buttons[key].1 = Some(Action::from(state));
        } else {
            input_state
                .buttons
                .push((key, Some(Action::from(state))));
        }
    }
    pub(crate) fn _update(&mut self) {
        let mut input_state = self.input_state.write().unwrap();
        for (_, state) in input_state.buttons.iter_mut() {
            if let Some(pstate) = state.as_mut() {
                if *pstate == Action::Press {
                    *pstate = Action::Repeat;
                }
                if *pstate == Action::Release{
                    *state = None; 
                } 
            }
        }
        
        input_state.mouse_move = (0.0, 0.0);
        for state in input_state.mouse_buttons.iter_mut() {
            if let Some(pstate) = state.as_mut() {
                if *pstate == Action::Press {
                    *pstate = Action::Repeat;
                }
                if *pstate == Action::Release{
                    *state = None; 
                } 
            }
        }
    }
    //TODO: scroll handle
    pub(crate) fn _scroll(&mut self, a: f64, b: f64) {
        let mut input_state = self.input_state.write().unwrap();
        input_state.scroll_delta = (a, b);
    }
}
