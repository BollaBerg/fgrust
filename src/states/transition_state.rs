use std::time::Duration;
use crate::input::Input;
use crate::screen::{Cell, Screen};
use crate::state_machine::State;
use crate::transition::Transition;

pub struct TransitionState {
    next_state: Option<Box<dyn State>>,
    transition: Transition,
    prev_cells: Vec<Cell>,
}

impl TransitionState {
    pub fn new(next_state: Box<dyn State>) -> Self {
        TransitionState {
            next_state : Some(next_state),
            transition: Transition::new(Duration::from_secs(2)),
            prev_cells: vec![],
        }
    }
}

impl State for TransitionState {
    fn enter(&mut self, screen: &mut Screen, _input: &mut Input) {
        self.transition.resize(screen.width(), screen.height());
        self.transition.change_state(crate::transition::TransitionState::In);
        self.prev_cells = screen.clone_buffer();
    }

    fn update(&mut self, screen: &mut Screen, _input: &mut Input, dt: f64) -> Option<Box<dyn State>> {
        match self.transition.state() {
            Some(crate::transition::TransitionState::In) => {
                let done = self.transition.update(screen, dt);

                for (i, cell) in self.prev_cells.iter().enumerate() {
                    let pos = screen.index_to_xy(i);
                    screen.set_cell(pos.0, pos.1, cell.rune, cell.color);
                }

                if done {
                    self.transition.change_state(crate::transition::TransitionState::Out);
                }
            }
            Some(crate::transition::TransitionState::Out) => {
                let done = self.transition.update(screen, dt);
                let next_state = self.next_state.as_mut().unwrap();
                // update it
                next_state.update(screen, _input, dt);

                if done {
                    return self.next_state.take();
                }
            }
            None => {}
        }

        self.transition.draw(screen);
        None
    }

    fn exit(&mut self, _screen: &mut Screen, _input: &mut Input) {
    }
}