use crate::screen::Screen;
use crossterm::style;

const MAX_SNOW_FLAKES: usize = 500;
const SNOW_FLAKE_SPAWN_PROBABILITY: f64 = 0.7;
const SNOW_FLAKE_SPRITES: [char; 3] = ['*', '·', '•'];

pub struct Snowflake {
    x: f64,
    y: f64,
    speed: f64,
    sprite: char,
}

pub fn create(width: u16, height: u16) -> Vec<Snowflake> {
    let mut snow_flakes: Vec<Snowflake> = Vec::new();
    for _ in 0..100 {
        snow_flakes.push(Snowflake {
            x: (width as f64 * rand::random::<f64>()).floor(),
            y: ((height - 1) as f64 * rand::random::<f64>()).floor(),
            speed: (rand::random::<f64>() * 1.0) + 0.5,
            sprite: SNOW_FLAKE_SPRITES
                [(rand::random::<u16>() % SNOW_FLAKE_SPRITES.len() as u16) as usize],
        });
    }

    snow_flakes
}

pub fn update(snow_flakes: &mut Vec<Snowflake>, width: u16, height: u16, phase: f64, dt: f64) {
    prune_snow_flakes(snow_flakes);

    for snow_flake in snow_flakes.iter_mut() {
        snow_flake.y += snow_flake.speed * dt;
        snow_flake.x += (phase + snow_flake.x / 8.0).sin() * 1.0 * dt;

        let ground_level = height - 1;

        if snow_flake.y as u16 >= ground_level {
            snow_flake.y = 0.0;
            snow_flake.x = (width - 1) as f64 * rand::random::<f64>();
        }

        snow_flake.x = snow_flake.x.clamp(0.0, (width - 1) as f64);
    }
}

fn prune_snow_flakes(snow_flakes: &mut Vec<Snowflake>) {
    while snow_flakes.len() > MAX_SNOW_FLAKES {
        snow_flakes.remove(0);
    }
}

pub fn spawn_mouse_snow_flakes(snow_flakes: &mut Vec<Snowflake>, mouse_position: (u16, u16)) {
    if rand::random::<f64>() < SNOW_FLAKE_SPAWN_PROBABILITY {
        const SPAWN_SPREAD: f64 = 6.0;
        snow_flakes.push(Snowflake {
            x: mouse_position.0 as f64 + (rand::random::<f64>() * SPAWN_SPREAD),
            y: mouse_position.1 as f64 + (rand::random::<f64>() * SPAWN_SPREAD) - 1.0,
            speed: (rand::random::<f64>() * 0.5) + 2.8,
            sprite: SNOW_FLAKE_SPRITES
                [(rand::random::<u16>() % SNOW_FLAKE_SPRITES.len() as u16) as usize],
        });
    }
}

pub fn draw(screen: &mut Screen, snow_flakes: &Vec<Snowflake>) {
    for snow_flake in snow_flakes {
        let x = snow_flake.x as u16;
        let y = snow_flake.y as u16;

        screen.set_cell(x, y, snow_flake.sprite, style::Color::White);
    }
}
