use crate::input::Input;
use crate::screen::Screen;

pub trait State {
    fn enter(&mut self, screen: &mut Screen, input: &mut Input);
    fn update(&mut self, screen: &mut Screen, input: &mut Input, dt: f64) -> Option<Box<dyn State>>;
    fn exit(&mut self, screen: &mut Screen, input: &mut Input);
}

pub struct StateMachine {
    current_state: Option<Box<dyn State>>,
}

impl StateMachine {
    pub fn new() -> StateMachine {
        StateMachine {
            current_state: None,
        }
    }
    
    pub fn change(&mut self, screen: &mut Screen, input: &mut Input, new_state: Option<Box<dyn State>>) {
        if let Some(ref mut state) = self.current_state {
            state.exit(screen, input);
        }

        self.current_state = new_state;

        if let Some(ref mut state) = self.current_state {
            state.enter(screen, input);
        }
    }

    pub fn update(&mut self, screen: &mut Screen, input: &mut Input, dt: f64) {
        if let Some(ref mut state) = self.current_state {
            if let Some(new_state) = state.update(screen, input, dt) {
                self.change(screen, input, Some(new_state));
            }
        }
    }
}