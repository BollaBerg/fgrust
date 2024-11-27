use crate::input::Input;
use crate::screen::Screen;
use crate::state_machine::State;

pub struct Day14State {
}

impl Day14State {
    pub fn new() -> Self {
        Day14State {
        }
    }
}

impl State for Day14State {
    fn enter(&mut self, screen: &mut Screen, input: &mut Input) {
    }

    fn update(&mut self, screen: &mut Screen, input: &mut Input, dt: f64) -> Option<Box<dyn State>> {
        None
    }

    fn exit(&mut self, screen: &mut Screen, input: &mut Input) {
    }
}