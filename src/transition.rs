use crate::screen::Screen;

struct Cell {
    x: u16,
    y: u16,
    alive: bool,
}

pub enum TransitionState {
    In,
    Out,
}

pub struct Transition {
    cells: Vec<Cell>,
    timer: f64,
    state: Option<TransitionState>,
}

impl Transition {
    pub fn new() -> Transition {
        Transition {
            cells: Vec::new(),
            timer: 0.0,
            state: None,
        }
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        self.cells = (0..width)
            .flat_map(|x| (0..height).map(move |y| Cell { x, y, alive: false }))
            .collect();
    }
    
    pub fn running(&self) -> bool {
        self.state.is_some()
    }

    pub fn change_state(&mut self, state: TransitionState) {
        self.state = Some(state);
        self.timer = 0.0;

        for cell in &mut self.cells {
            cell.alive = match self.state {
                Some(TransitionState::In) => true,
                Some(TransitionState::Out) => false,
                None => false,
            };
        }
    }

    pub fn update(&mut self, screen: &mut Screen, dt: f64) -> bool {
        self.timer += dt;

        let center_x = screen.width() as f64 / 2.0;
        let center_y = screen.height() as f64 / 2.0;

        for cell in &mut self.cells {
            let dx = (cell.x as f64 - center_x) / center_x;
            let dy = (cell.y as f64 - center_y) / center_y;
            let distance = (dx * dx + dy * dy).sqrt();

            match self.state {
                Some(TransitionState::In) => {
                    cell.alive = distance > self.timer;
                }
                Some(TransitionState::Out) => {
                    cell.alive = distance < self.timer;
                }
                None => {
                }
            }
        }

        // depending on the state, when all the cells are alive or dead, change state to None
        let all_alive = self.cells.iter().all(|cell| cell.alive);
        let all_dead = self.cells.iter().all(|cell| !cell.alive);
        if all_alive || all_dead {
            self.state = None;
        }
        
        self.state.is_none()
    }

    pub fn draw(&self, screen: &mut Screen) {
        for cell in &self.cells {
            if cell.alive {
                screen.set_cell(cell.x, cell.y, 'O', crossterm::style::Color::White);
            }
        }
    }
}