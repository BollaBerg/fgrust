extern crate crossterm;
mod ascii;
mod screen;
mod snowflakes;
mod drawing;
mod cannon_game;
mod state_machine;
mod input;

mod states {
    pub mod main_state;
    pub mod day1_state;
    pub mod day2_state;
}

use crate::screen::Screen;
use crossterm::terminal;
use std::io::{stdout, Error};
use std::sync::{Arc, Mutex};
use std::time::Instant;

use drawing::draw_debug_info;

fn delta_time(previous_time: &mut Instant) -> f64 {
    let new_time = Instant::now();
    let dt = new_time.duration_since(*previous_time).as_nanos() as f64 / 1_000_000_000.0;
    *previous_time = new_time;
    dt
}

fn main() -> Result<(), Error> {
    let exit = Arc::new(Mutex::new(false));
    let resize = Arc::new(Mutex::new(true));

    let exit_clone = Arc::clone(&exit);
    let resize_clone = Arc::clone(&resize);

    let mut input = input::Input::new();
    input.bind_key('q', move || {
        let mut exit = exit_clone.lock().unwrap();
        *exit = true;
    });

    input.bind_resize(move |_width: u16, _height:u16| {
        let mut resize = resize_clone.lock().unwrap();
        *resize = true;
    });

    let mut screen = Screen::new(stdout(), terminal::size()?);
    screen.init()?;

    let initial_state = states::main_state::MainState::new();
    let mut state_machine = state_machine::StateMachine::new();
    state_machine.change(&mut screen, &mut input, Some(Box::new(initial_state)));

    let mut dt;
    let mut previous_time = Instant::now();

    loop {
        if *exit.lock().unwrap() {
            break;
        }
        if *resize.lock().unwrap() {
            screen.resize(terminal::size()?);
            *resize.lock().unwrap() = false;
        }

        screen.clear();

        dt = delta_time(&mut previous_time);
        
        state_machine.update(&mut screen, &mut input, dt);

        draw_debug_info(&mut screen, input.mouse_position(), input.is_mouse_down(), dt);

        screen.render();

        input.update()?;
    }

    screen.cleanup()?;
    Ok(())
}