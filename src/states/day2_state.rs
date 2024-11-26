use rand::prelude::IndexedRandom;
use rand::Rng;
use rand::seq::SliceRandom;
use crate::drawing::draw_text_box;
use crate::input::{Input, MouseButton};
use crate::screen::Screen;
use crate::state_machine::State;
use crate::states::main_state::MainState;
use crate::transition::Transition;

struct Piece {
    x: u16,
    y: u16,
    sprite: char,
}

struct Particle {
    x: f64,
    y: f64,
    speed: f64,
    angle: f64,
    sprite: char,
}

pub struct Day2State {
    pieces: Vec<Piece>,
    selected: Vec<usize>,
    moves: u32,
    confetti: Vec<Particle>,
}

impl Day2State {
    pub fn new() -> Self {
        Day2State {
            pieces: create_pieces(),
            selected: vec![],
            moves: 0,
            confetti: vec![],
        }
    }
}

fn create_pieces() -> Vec<Piece> {
    let mut rng = rand::thread_rng();
    let mut pieces = vec![];

    let mut sprites = vec!['α', 'β', 'γ', 'δ', 'ε', 'ζ', 'η', 'θ', 'α', 'β', 'γ', 'δ', 'ε', 'ζ', 'η', 'θ'];
    sprites.shuffle(&mut rng);

    for x in 0..4 {
        for y in 0..4 {
            let sprite = sprites.pop().unwrap();
            pieces.push(Piece { x, y, sprite });
        }
    }

    pieces
}

fn create_confetti(width: u16, height: u16) -> Vec<Particle> {
    let mut rng = rand::thread_rng();
    let mut confetti = vec![];

    for _ in 0..100 {
        let x = width as f64 / 2.0;
        let y = height as f64 / 2.0;
        let speed = rng.gen_range(1.0..30.0);
        let angle = rng.gen_range(0.0..std::f64::consts::PI * 2.0);
        let sprite = ['.', ',', '\'', '`', '^', '"', '*', 'o', 'O', '@']
            .choose(&mut rng)
            .unwrap()
            .clone();
        confetti.push(Particle { x, y, speed, angle, sprite });
    }

    confetti
}

impl State for Day2State {
    fn enter(&mut self, screen: &mut Screen, input: &mut Input) {
        self.confetti = create_confetti(screen.width(), screen.height());
    }

    fn update(&mut self, screen: &mut Screen, input: &mut Input, transition: &mut Transition, dt: f64) -> Option<Box<dyn State>> {

        if let Some((width, height)) = input.resized() {
            self.confetti = create_confetti(width, height);
        }

        let new_selected = draw_boxes(screen, input, &self.pieces, &self.selected);
        if new_selected.len() > 0 && input.is_mouse_up(MouseButton::Left) {
            let i = new_selected.first().unwrap_or(&0).clone();

            if !self.selected.contains(&i) {
                if self.selected.len() == 2 {
                    self.selected.clear();
                }

                self.selected.push(i);

                if self.selected.len() == 2 {
                    self.moves += 1;
                }
            }
        }

        if self.selected.len() == 2 {
            let first = self.selected[0];
            let second = self.selected[1];

            if self.pieces[first].sprite == self.pieces[second].sprite {
                if first < second {
                    self.pieces.remove(first);
                    self.pieces.remove(second - 1);
                } else {
                    self.pieces.remove(first);
                    self.pieces.remove(second);
                }
                self.selected.clear();
            }
        }

        if self.pieces.len() == 0 {
            draw_win(screen, dt, &mut self.confetti, self.moves);
        }

        let exit = draw_text_box(
            screen,
            screen.width(),
            screen.height(),
            "Tilbake",
            0,
            12,
            input.mouse_position(),
            input.is_mouse_up(MouseButton::Left),
        );
        if exit && input.is_mouse_up(MouseButton::Left) {
            return Some(Box::new(MainState::new()));
        }

        None
    }

    fn exit(&mut self, screen: &mut Screen, input: &mut Input) {
    }
}

// draw boxes and return new selected
fn draw_boxes(screen: &mut Screen, input: &mut Input, pieces: &Vec<Piece>, selected: &Vec<usize>) -> Vec<usize> {
    let num_pieces = 16;
    let box_size = 9;
    let box_height = 4;
    let row_size = 4;
    let x_offset = -((num_pieces / row_size) * 6 / 2);
    let y_offset = -((num_pieces / row_size) * box_height / 2);

    let mut new_selected = vec![];

    for (i, piece) in pieces.iter().enumerate() {
        let x = piece.x as i16 * box_size + x_offset;
        let y = piece.y as i16 * box_height + y_offset;

        let str = if selected.contains(&i) {
            piece.sprite.to_string()
        } else {
            "  ".to_string()
        };

        let hovered = draw_text_box(
            screen,
            screen.width(),
            screen.height(),
            &str,
            x,
            y,
            input.mouse_position(),
            input.is_mouse_up(MouseButton::Left),
        );

        if hovered {
            new_selected.push(i);
        }
    }

    new_selected
}

fn draw_win(screen: &mut Screen, dt: f64, confetti: &mut Vec<Particle>, moves: u32) {
    for particle in confetti.iter_mut() {
        particle.x += particle.speed * particle.angle.cos() * dt;
        particle.y += particle.speed * particle.angle.sin() * dt;

        if particle.x < 0.0 || particle.x >= screen.width() as f64 || particle.y < 0.0 || particle.y >= screen.height() as f64 {
            particle.x = screen.width() as f64 / 2.0;
            particle.y = screen.height() as f64 / 2.0;
        }

        screen.set_cell(particle.x as u16, particle.y as u16, particle.sprite, crossterm::style::Color::White);
    }

    let str = format!("Gratulerer! Du klarte det på {} trekk!", moves);
    draw_text_box(
        screen,
        screen.width(),
        screen.height(),
        &str,
        0,
        0,
        (0, 0),
        false,
    );
}