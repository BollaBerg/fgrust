use crate::input::Input;
use crate::screen::Screen;
use crate::state_machine::State;
use crate::transition::Transition;

pub struct TransitionState {
    next_state: Option<Box<dyn State>>,
}

impl TransitionState {
    pub fn new(next_state: Box<dyn State>) -> Self {
        TransitionState {
            next_state : Some(next_state),
        }
    }
}

impl State for TransitionState {
    fn enter(&mut self, _screen: &mut Screen, _input: &mut Input) {
    }

    fn update(&mut self, _screen: &mut Screen, _input: &mut Input, _transition: &mut Transition, _dt: f64) -> Option<Box<dyn State>> {
        // Calculate the progress of the transition
        // let progress = self.start_time.elapsed().as_secs_f64() / self.duration.as_secs_f64();

        // If the transition is complete, return the next state
        // if progress >= 1.0 {
        //     Some(self.next_state)
        // } else {
        //     None
        // }
        // Some(self.next_state)
        self.next_state.take()
    }

    fn exit(&mut self, _screen: &mut Screen, _input: &mut Input) {
    }
}