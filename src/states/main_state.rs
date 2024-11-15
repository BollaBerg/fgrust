use crate::drawing::{draw_ascii, draw_ground};
use crate::state_machine::State;
use crate::screen::Screen;
use crate::{ascii, snowflakes};
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
    fn enter(&mut self, screen: &mut Screen) {
        self.prev_width = screen.width();
        self.prev_height = screen.height();
        self.snowflakes = snowflakes::create(screen.width(), screen.height());
    }

    fn update(&mut self, screen: &mut Screen, dt: f64) -> Option<Box<dyn State>> {
        let screen_height = screen.height();
        let screen_width = screen.width();

        if self.prev_width != screen_width || self.prev_height != screen_height {
            self.prev_width = screen_width;
            self.prev_height = screen_height;
            self.snowflakes = snowflakes::create(screen_width, screen_height);
        }

        self.phase += dt;

        snowflakes::update(&mut self.snowflakes, screen_width, screen_height, self.phase, dt);

        draw_ascii(screen, ascii::SANTA, 2, screen_height - 20);
        snowflakes::draw(screen, &self.snowflakes);
        draw_ascii(screen, ascii::SYSTEK, screen_width / 2 - 32, 1);
        draw_ground(screen);

        None
    }

    fn exit(&mut self, screen: &mut Screen) {
    }
}