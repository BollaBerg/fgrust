use std::collections::HashMap;
use std::io::Error;
use std::time::Duration;
use crossterm::event;
use crossterm::event::{read, Event};

#[derive(Clone)]
pub enum InputEvent {
    Down,
    Up,
    None,
}

pub enum MouseButton {
    Left,
    Right,
    Middle,
}

pub struct Input {
    keymap: HashMap<event::KeyCode, Option<InputEvent>>,
    resize: Option<(u16, u16)>,

    mouse_position: (u16, u16),
    mousemap: HashMap<event::MouseButton, Option<InputEvent>>,
}

impl Input {
    pub fn new() -> Input {
        Input {
            keymap: HashMap::new(),
            resize: None,
            mouse_position: (0, 0),
            mousemap: HashMap::new(),
        }
    }
    
    pub fn keymap(&self) -> &HashMap<event::KeyCode, Option<InputEvent>> {
        &self.keymap
    }
    
    pub fn mousemap(&self) -> &HashMap<event::MouseButton, Option<InputEvent>> {
        &self.mousemap
    }

    pub fn mouse_position(&self) -> (u16, u16) {
        self.mouse_position
    }
    
    pub fn is_key_down(&self, key: char) -> bool {
        let key = event::KeyCode::Char(key);
        if let Some(event) = self.keymap.get(&key) {
            if let Some(InputEvent::Down) = event {
                return true;
            }
        }

        false
    }
    
    pub fn is_key_up(&self, key: char) -> bool {
        let key = event::KeyCode::Char(key);
        if let Some(event) = self.keymap.get(&key) {
            if let Some(InputEvent::Up) = event {
                return true;
            }
        }

        false
    }

    pub fn mouse_state(&self, button: MouseButton) -> Option<InputEvent> {
        let button = match button {
            MouseButton::Left => event::MouseButton::Left,
            MouseButton::Right => event::MouseButton::Right,
            MouseButton::Middle => event::MouseButton::Middle,
        };

        self.mousemap.get(&button).cloned().unwrap_or(None)
    }

    pub fn is_mouse_down(&self, button: MouseButton) -> bool {
        let button = match button {
            MouseButton::Left => event::MouseButton::Left,
            MouseButton::Right => event::MouseButton::Right,
            MouseButton::Middle => event::MouseButton::Middle,
        };
        
        if let Some(event) = self.mousemap.get(&button) {
            if let Some(InputEvent::Down) = event {
                return true;
            }
        }
        
        false
    }

    pub fn is_mouse_up(&self, button: MouseButton) -> bool {
        let button = match button {
            MouseButton::Left => event::MouseButton::Left,
            MouseButton::Right => event::MouseButton::Right,
            MouseButton::Middle => event::MouseButton::Middle,
        };
        
        if let Some(event) = self.mousemap.get(&button) {
            if let Some(InputEvent::Up) = event {
                return true;
            }
        }
        
        false
    }

    pub fn resized(&self) -> Option<(u16, u16)> {
        self.resize
    }

    pub fn update(&mut self) -> Result<(), Error> {
        for (_, event) in self.mousemap.iter_mut() {
            if let Some(InputEvent::Up) = event {
                *event = None;
            }
        }

        for (_, event) in self.mousemap.iter_mut() {
            if let Some(InputEvent::Up) = event {
                *event = Some(InputEvent::None);
            }
        }
        
        self.keymap.retain(|_, event| {
            if let Some(InputEvent::Up) = event {
                false
            } else {
                true
            }
        });

        for (_, event) in self.keymap.iter_mut() {
            if let Some(InputEvent::Down) = event {
                *event = Some(InputEvent::Up);
            }
        }

        self.resize = None;
        
        if event::poll(Duration::from_millis(0))? {
            let raw = read();

            if raw.is_err() {
                return Err(raw.err().unwrap());
            }

            let event = raw?;

            if let Event::Key(event) = event {
                if event.kind == event::KeyEventKind::Press {
                    self.keymap.insert(event.code, Some(InputEvent::Down));
                }
                if event.kind == event::KeyEventKind::Release {
                    self.keymap.insert(event.code, Some(InputEvent::Up));
                }
            }

            if let Event::Mouse(event) = event {
                self.mouse_position = (event.column, event.row);

                if let event::MouseEventKind::Down(button) = event.kind {
                    self.mousemap.insert(button, Some(InputEvent::Down));
                }

                if let event::MouseEventKind::Up(button) = event.kind {
                    self.mousemap.insert(button, Some(InputEvent::Up));
                }
            }

            if let Event::Resize(width, height) = event {
                self.resize = Some((width, height));
            }
        }

        Ok(())
    }
}