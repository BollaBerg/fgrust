use std::collections::HashMap;
use std::io::Error;
use std::time::Duration;
use crossterm::event;
use crossterm::event::{read, Event};

pub struct Input {
    keymap: HashMap<event::KeyCode, Box<dyn FnMut()>>,
    resize_callback: Option<Box<dyn FnMut(u16, u16)>>,

    mouse_position: (u16, u16),
    mouse_down: bool,
    mouse_up: bool,
}
impl Input {
    pub fn new() -> Input {
        Input {
            keymap: HashMap::new(),
            resize_callback: None,
            mouse_position: (0, 0),
            mouse_down: false,
            mouse_up: false,
        }
    }

    pub fn mouse_position(&self) -> (u16, u16) {
        self.mouse_position
    }

    pub fn is_mouse_down(&self) -> bool {
        self.mouse_down
    }

    pub fn is_mouse_up(&self) -> bool {
        self.mouse_up
    }

    pub fn bind_key<F>(&mut self, key: char, callback: F) where F: FnMut() + 'static, {
        self.keymap.insert(event::KeyCode::Char(key), Box::new(callback));
    }

    pub fn bind_resize<F>(&mut self, callback: F) where F: FnMut(u16, u16) + 'static, {
        self.resize_callback = Some(Box::new(callback));
    }

    pub fn update(&mut self) -> Result<(), Error> {
        self.mouse_up = false;
        
        if event::poll(Duration::from_millis(0))? {
            let raw = read();

            if raw.is_err() {
                return Err(raw.err().unwrap());
            }

            let event = raw?;

            if let Event::Key(event) = event {
                if let Some(callback) = self.keymap.get_mut(&event.code) {
                    callback();
                }
            }
            else if let Event::Mouse(event) = event {
                if let event::MouseEventKind::Moved = event.kind {
                    self.mouse_position = (event.column, event.row);
                }
                if let event::MouseEventKind::Down(event::MouseButton::Left) = event.kind {
                    self.mouse_down = true;
                }
                if let event::MouseEventKind::Up(event::MouseButton::Left) = event.kind {
                    self.mouse_down = false;
                    self.mouse_up = true;
                }
            }
            else if let Event::Resize(width, height) = event {
                if let Some(ref mut callback) = self.resize_callback {
                    callback(width, height);
                }
            }
        }

        Ok(())
    }
}