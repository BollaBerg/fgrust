use std::io::{Stdout, Write};
use crossterm::{cursor, queue, style, terminal};
use crossterm::cursor::MoveTo;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::style::{SetForegroundColor, Stylize};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};

#[derive(Clone)]
pub struct Cell {
    pub rune: char,
    pub color: style::Color,
}

impl Cell {
    fn new(c: char, color: style::Color) -> Cell {
        Cell {rune: c, color }
    }

    fn clear(&mut self) {
        self.set(' ', style::Color::White);
    }

    fn set(&mut self, c: char, color: style::Color) {
        self.rune = c;
        self.color = color;
    }
}

pub struct Screen {
    stdout: Stdout,
    width: u16,
    height: u16,
    real_width: u16,
    real_height: u16,
    buffer: Vec<Cell>,
}

impl Screen {
    pub fn new(stdout: Stdout, size: (u16, u16)) -> Screen {
        let (width, height) = Screen::clamp_screen(size);
        let mut screen = Screen {
            stdout,
            width,
            height,
            real_width: size.0,
            real_height: size.1,
            buffer: Vec::new(),
        };

        screen.resize(size);
        screen
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }
    
    pub fn clone_buffer(&self) -> Vec<Cell> {
        self.buffer.clone()
    }

    pub fn init(&mut self) -> Result<(), std::io::Error> {
        enable_raw_mode()?;

        queue!(
            self.stdout,
            terminal::EnterAlternateScreen,
            cursor::Hide,
            EnableMouseCapture,
            Clear(ClearType::All)
        )?;
        Ok(())
    }

    pub fn cleanup(&mut self) -> Result<(), std::io::Error> {
        disable_raw_mode()?;

        queue!(
            self.stdout,
            cursor::Show,
            terminal::LeaveAlternateScreen,
            DisableMouseCapture
        )?;

        self.stdout.flush()?;
        Ok(())
    }

    pub fn clear(&mut self) {
        for i in 0..self.buffer.len() {
            self.buffer[i].clear();
        }
    }

    pub fn resize(&mut self, size: (u16, u16)) {
        let (width, height) = Screen::clamp_screen(size);
        self.width = width;
        self.height = height;
        self.real_width = size.0;
        self.real_height = size.1;

        if self.buffer.len() == (width * height) as usize {
            return;
        }

        self.buffer.clear();
        for _ in 0..(width * height) {
            self.buffer.push(Cell::new(' ', style::Color::White));
        }
        self.clear();
    }

    fn clamp_screen((width, height) : (u16, u16)) -> (u16, u16) {
        let new_width = (width - 1).clamp(80, 200);
        let new_height = height.clamp(40, 70);
        (new_width, new_height)
    }

    pub fn set_cell(&mut self, x: u16, y: u16, c: char, color: style::Color) {
        let index = self.xy_to_index(x, y);

        if x >= self.width || y >= self.height {
            return;
        }

        if x >= self.real_width || y >= self.real_height {
            return;
        }

        self.buffer[index].set(c, color);
    }

    pub fn xy_to_index(&self, x: u16, y: u16) -> usize {
        (y * self.width + x) as usize
    }

    pub fn index_to_xy(&self, index: usize) -> (u16, u16) {
        let x = index as u16 % self.width;
        let y = index as u16 / self.width;
        (x, y)
    }

    pub fn render(&mut self) {
        for (i, cell) in self.buffer.iter().enumerate() {
            let (x, y) = self.index_to_xy(i);
            queue!(
                self.stdout,
                MoveTo(x, y),
                SetForegroundColor(cell.color),
                style::PrintStyledContent(cell.rune.stylize())
            ).unwrap();
        }
        self.stdout.flush().unwrap();
    }
}