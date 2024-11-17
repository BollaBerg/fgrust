use crate::input::Input;
use crate::screen::Screen;
use crate::state_machine::State;
use rand::{thread_rng, Rng};
use crate::drawing::draw_question;
use crate::states::main_state::MainState;

pub struct Day1State {
    question: String,
    correct_answer: String,
    wrong_answers: [&'static str; 2],
    correct_answer_position: usize,
}

impl Day1State {
    pub fn new() -> Self {
        let question = "What is the answer to life, the universe, and everything?".to_string();
        let correct_answer = "42".to_string();
        let wrong_answers = ["24", "69"];
        let number_of_answers = wrong_answers.len() + 1;
        let correct_answer_position = thread_rng().gen_range(0..number_of_answers);
        Day1State {
            question,
            correct_answer,
            wrong_answers,
            correct_answer_position,
        }
    }
}

impl State for Day1State {
    fn enter(&mut self, screen: &mut Screen, input: &mut Input) {
    }

    fn update(&mut self, screen: &mut Screen, input: &mut Input, _dt: f64) -> Option<Box<dyn State>> {
        let mut correct = false;
        draw_question(
            screen,
            input.mouse_position(),
            input.is_mouse_up(),
            &self.question,
            &self.correct_answer,
            &self.wrong_answers,
            self.correct_answer_position,
            &mut || correct = true,
        );

        if correct {
            return Some(Box::new(MainState::new()));
        }

        None
    }

    fn exit(&mut self, screen: &mut Screen, input: &mut Input) {
    }
}