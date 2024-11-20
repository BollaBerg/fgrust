use crate::input::Input;
use crate::screen::Screen;
use crate::state_machine::State;
use rand::{thread_rng, Rng};
use crate::ascii::{LAZY_CAT, PRESENT};
use crate::drawing::{draw_ascii, draw_question};
use crate::states::main_state::MainState;

struct Particle {
    x: f64,
    y: f64,
    sprite: char,
}

pub struct Day1State {
    question: String,
    correct_answer: String,
    wrong_answers: [&'static str; 2],
    correct_answer_position: usize,

    phase: f64,
    particles: Vec<Particle>,
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

            phase: 0.0,
            particles: vec![],
        }
    }
}

impl State for Day1State {
    fn enter(&mut self, screen: &mut Screen, input: &mut Input) {
        self.particles = create_particles(screen);
    }

    fn update(&mut self, screen: &mut Screen, input: &mut Input, dt: f64) -> Option<Box<dyn State>> {

        self.phase += dt;

        draw_ascii(screen, PRESENT, 12, screen.height() - 24);

        {
            let cat_x = screen.width() - 50;
            let cat_y = screen.height() - 12;

            draw_ascii(screen, LAZY_CAT, cat_x, cat_y);
            draw_particles(screen, &mut self.particles, cat_x, cat_y, self.phase, dt);
        }

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

fn create_particles(screen: &mut Screen) -> Vec<Particle> {
    let mut particles = Vec::new();
    for i in 0..3 {
        particles.push(Particle {
            x: i as f64 * 2.0,
            y: i as f64 * 2.0,
            sprite: 'Z',
        });
    }
    particles
}

fn draw_particles(screen: &mut Screen, particles: &mut Vec<Particle>, x:u16, y:u16, phase: f64, dt: f64) {
    let particle_amplitude = 4.0;
    for (i, particle) in particles.iter_mut().enumerate() {
        particle.x += (phase * 2.0 + particle.y).sin() * particle_amplitude * dt;
        screen.set_cell(x + particle.x as u16, y + particle.y as u16, particle.sprite, crossterm::style::Color::White);
    }
}