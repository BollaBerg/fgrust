use crate::drawing::{draw_ascii, draw_calendar, draw_ground};
use crate::state_machine::State;
use crate::screen::Screen;
use crate::{ascii, snowflakes, states};
use crate::input::{Input, MouseButton};
use crate::snowflakes::Snowflake;

pub struct MainState {
    snowflakes: Vec<Snowflake>,
    phase: f64,
    prev_width: u16,
    prev_height: u16,
}

impl MainState {
    pub fn new() -> MainState {
        MainState {
            snowflakes: Vec::new(),
            phase: 0.0,
            prev_width: 0,
            prev_height: 0,
        }
    }
}

impl State for MainState {
    fn enter(&mut self, screen: &mut Screen, input: &mut Input) {
        self.prev_width = screen.width();
        self.prev_height = screen.height();
        self.snowflakes = snowflakes::create(screen.width(), screen.height());
    }

    fn update(&mut self, screen: &mut Screen, input: &mut Input, dt: f64) -> Option<Box<dyn State>> {
        let screen_height = screen.height();
        let screen_width = screen.width();

        if self.prev_width != screen_width || self.prev_height != screen_height {
            self.prev_width = screen_width;
            self.prev_height = screen_height;
            self.snowflakes = snowflakes::create(screen_width, screen_height);
        }

        self.phase += dt;

        snowflakes::update(&mut self.snowflakes, screen_width, screen_height, self.phase, dt);
        if input.is_mouse_down(MouseButton::Left) {
            snowflakes::spawn_mouse_snow_flakes(&mut self.snowflakes, input.mouse_position())
        }

        draw_ascii(screen, ascii::SANTA, 2, screen_height - 20);
        snowflakes::draw(screen, &self.snowflakes);
        draw_ascii(screen, ascii::SYSTEK, screen_width / 2 - 32, 1);
        draw_ground(screen);

        if let Some(ref day) = draw_calendar(screen, input.mouse_position(), input.is_mouse_up(MouseButton::Left)) {
            let next: Option<Box<dyn State>> =  match day {
                1 => Some(Box::new(states::day1_state::Day1State::new())),
                2 => Some(Box::new(states::day2_state::Day2State::new())),
                3 => Some(Box::new(states::day3_state::Day3State::new())),
                4 => Some(Box::new(states::day4_state::Day4State::new(screen_width))),
                // 5 => Some(Box::new(states::day5_state::Day5State::new())),
                // 6 => Some(Box::new(states::day6_state::Day6State::new())),
                // 7 => Some(Box::new(states::day7_state::Day7State::new())),
                // 8 => Some(Box::new(states::day8_state::Day8State::new())),
                // 9 => Some(Box::new(states::day9_state::Day9State::new())),
                // 10 => Some(Box::new(states::day10_state::Day10State::new())),
                // 11 => Some(Box::new(states::day11_state::Day11State::new())),
                // 12 => Some(Box::new(states::day12_state::Day12State::new())),
                // 13 => Some(Box::new(states::day13_state::Day13State::new())),
                14 => Some(Box::new(states::day14_state::Day14State::new())),
                // 15 => Some(Box::new(states::day15_state::Day15State::new())),
                // 16 => Some(Box::new(states::day16_state::Day16State::new())),
                // 17 => Some(Box::new(states::day17_state::Day17State::new())),
                // 18 => Some(Box::new(states::day18_state::Day18State::new())),
                // 19 => Some(Box::new(states::day19_state::Day19State::new())),
                // 20 => Some(Box::new(states::day20_state::Day20State::new())),
                // 21 => Some(Box::new(states::day21_state::Day21State::new())),
                // 22 => Some(Box::new(states::day22_state::Day22State::new())),
                // 23 => Some(Box::new(states::day23_state::Day23State::new())),
                24 => Some(Box::new(states::day24_state::Day24State::new())),
                _ => None,
            };

            if next.is_some() {
                return Some(Box::new(states::transition_state::TransitionState::new(next.unwrap(), None)));
            }
        }

        None
    }

    fn exit(&mut self, screen: &mut Screen, input: &mut Input) {
    }
}