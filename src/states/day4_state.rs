use crate::input::Input;
use crate::screen::Screen;
use crate::state_machine::State;
use crate::states::main_state::MainState;
use crate::states::transition_state::TransitionState;
use crossterm::style;
use rand::Rng;
use std::time::{Duration, Instant};

pub struct Day4State {
    snowflakes: Vec<Snowflake>,
    basket_x: u16,
    score: u32,
    time_left: f64,
    last_update: Instant,
}

struct Snowflake {
    x: u16,
    y: u16,
}

impl Day4State {
    pub fn new(screen_width: u16) -> Self {
        Day4State {
            snowflakes: Vec::new(),
            basket_x: screen_width / 2,
            score: 0,
            time_left: 30.0,
            last_update: Instant::now(),
        }
    }

    fn draw_basket(&mut self, screen: &mut Screen) {
        screen.draw_text(
            self.basket_x,
            screen.height() - 1,
            &"=",
            style::Color::Green,
        );
        screen.draw_text(
            self.basket_x - 1,
            screen.height() - 1,
            "[",
            style::Color::Green,
        );
        screen.draw_text(
            self.basket_x + 1,
            screen.height() - 1,
            "]",
            style::Color::Green,
        );
    }
}

impl State for Day4State {
    fn enter(&mut self, screen: &mut Screen, _input: &mut Input) {
        screen.clear();
        self.last_update = Instant::now();
    }

    fn update(
        &mut self,
        screen: &mut Screen,
        input: &mut Input,
        dt: f64,
    ) -> Option<Box<dyn State>> {
        self.time_left -= dt;
        if self.time_left <= 0.0 {
            screen.clear();

            screen.draw_text(
                screen.height() / 2 - 10,
                screen.width() / 2,
                &format!("Time's up! Your final score: {}", self.score),
                style::Color::White,
            );
            std::thread::sleep(Duration::from_secs(5));
            return Some(Box::new(TransitionState::new(
                Box::new(MainState::new()),
                None,
            )));
        }

        if input.is_key_down('a') {
            if self.basket_x > 1 {
                self.basket_x -= 1;
            }
        } else if input.is_key_down('d') {
            if self.basket_x < screen.width() - 2 {
                self.basket_x += 1;
            }
        }

        if self.last_update.elapsed() >= Duration::from_millis(100) {
            self.last_update = Instant::now();

            if rand::thread_rng().gen_bool(0.3) {
                let x = rand::thread_rng().gen_range(0..screen.width());
                self.snowflakes.push(Snowflake { x, y: 0 });
            }

            for snowflake in &mut self.snowflakes {
                snowflake.y += 1;
            }

            self.snowflakes.retain(|snowflake| {
                if snowflake.y >= screen.height() {
                    false
                } else if snowflake.y == screen.height() - 1
                    && (snowflake.x == self.basket_x
                        || snowflake.x == self.basket_x + 1
                        || snowflake.x == self.basket_x - 1)
                {
                    self.score += 1;
                    false
                } else {
                    true
                }
            });
        }

        screen.clear();

        for snowflake in &self.snowflakes {
            screen.set_cell(snowflake.x, snowflake.y, '*', style::Color::White);
        }

        self.draw_basket(screen);

        screen.draw_text(
            screen.width() / 2 - 10,
            10,
            &format!("Score: {}  Time Left: {:.1}s", self.score, self.time_left),
            style::Color::White,
        );

        screen.draw_text(
            screen.width() / 2 - 10,
            12,
            &"Move with 'A' and 'D'",
            style::Color::White,
        );

        None
    }

    fn exit(&mut self, _screen: &mut Screen, _input: &mut Input) {}
}
