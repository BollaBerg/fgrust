use std::time::Duration;
use crossterm::style;
use crate::screen::Screen;

struct Cell {
    x: u16,
    y: u16,
    alive: bool,
}
pub enum TransitionDirection {
    In,
    Out,
}

pub struct Transition {
    cells: Vec<Cell>,
    duration: Duration,
    timer: f64,
    state: Option<TransitionDirection>,
}

impl Transition {
    pub fn new(duration: Duration, state: Option<TransitionDirection>) -> Transition {
        Transition {
            cells: Vec::new(),
            duration,
            timer: 0.0,
            state,
        }
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        self.cells = (0..width)
            .flat_map(|x| (0..height).map(move |y| Cell { x, y, alive: false }))
            .collect();
    }

    pub fn state(&self) -> Option<TransitionDirection> {
        match self.state {
            Some(TransitionDirection::In) => Some(TransitionDirection::In),
            Some(TransitionDirection::Out) => Some(TransitionDirection::Out),
            None => None,
        }
    }

    pub fn change_state(&mut self, state: TransitionDirection) {
        self.state = Some(state);
        self.timer = 0.0;

        for cell in &mut self.cells {
            cell.alive = match self.state {
                Some(TransitionDirection::In) => false,
                Some(TransitionDirection::Out) => true,
                None => false,
            };
        }
    }

    pub fn update(&mut self, screen: &mut Screen, dt: f64) -> bool {
        self.timer += dt;

        let normalized_timer = self.timer / self.duration.as_secs_f64();
        let center_x = screen.width() as f64 / 2.0;
        let center_y = screen.height() as f64 / 2.0;

        for cell in &mut self.cells {
            let dx = (cell.x as f64 - center_x) / center_x;
            let dy = (cell.y as f64 - center_y) / center_y;
            let distance = (dx * dx + dy * dy).sqrt();

            let distance = distance / 1.5;

            cell.alive = match self.state {
                Some(TransitionDirection::In) => distance < normalized_timer,
                Some(TransitionDirection::Out) => distance > normalized_timer,
                None => false,
            };
        }

        self.timer >= self.duration.as_secs_f64()
    }

    pub fn draw(&self, screen: &mut Screen) {
        const PATTERN: [[char; 8]; 4] = [
            [' ', '_', '|', '_', ' ', ' ', ' ', ' '],
            [' ', ' ', '|', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', '_', '|', '_', ' '],
            [' ', ' ', ' ', ' ', ' ', '|', ' ', ' ']
        ];

        for cell in &self.cells {
            if cell.alive {
                let index = (cell.y % PATTERN.len() as u16) as usize;
                let rune = PATTERN[index][cell.x as usize % PATTERN[index].len()];
                screen.set_cell(cell.x, cell.y, rune, style::Color::White);
            }
        }
    }
}