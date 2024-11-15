use crate::screen::Screen;

pub trait State {
    fn enter(&mut self, screen: &mut Screen);
    fn update(&mut self, screen: &mut Screen, dt: f64) -> Option<Box<dyn State>>;
    fn exit(&mut self, screen: &mut Screen);
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
    
    pub fn current_state(&self) -> &Option<Box<dyn State>> {
        &self.current_state
    }

    pub fn change(&mut self, screen: &mut Screen, new_state: Option<Box<dyn State>>) {
        if let Some(ref mut state) = self.current_state {
            state.exit(screen);
        }

        self.current_state = new_state;

        if let Some(ref mut state) = self.current_state {
            state.enter(screen);
        }
    }

    pub fn update(&mut self, screen: &mut Screen, dt: f64) {
        if let Some(ref mut state) = self.current_state {
            if let Some(new_state) = state.update(screen, dt) {
                self.change(screen, Some(new_state));
            }
        }
    }
}