use rand::seq::SliceRandom;
use crate::drawing::draw_text_box;
use crate::input::Input;
use crate::screen::Screen;
use crate::state_machine::State;
use crate::states::main_state::MainState;

struct Piece {
    x: u16,
    y: u16,
    sprite: char,
}

pub struct Day2State {
    pieces: Vec<Piece>,
    selected: Vec<usize>,
    moves: u32,
}

impl Day2State {
    pub fn new() -> Self {
        Day2State {
            pieces: create_pieces(),
            selected: vec![],
            moves: 0,
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

impl State for Day2State {
    fn enter(&mut self, screen: &mut Screen, input: &mut Input) {
    }

    fn update(&mut self, screen: &mut Screen, input: &mut Input, dt: f64) -> Option<Box<dyn State>> {
        let num_pieces = 16;
        let box_size = 9;
        let box_height = 4;
        let row_size = 4;
        let x_offset = -((num_pieces / row_size) * 6 / 2);
        let y_offset = -((num_pieces / row_size) * box_height / 2);

        draw_text_box(
            screen,
            screen.width(),
            screen.height(),
            &format!("Forsøk: {}", self.moves),
            0,
            -12,
            (0, 0),
            false,
        );

        for (i, piece) in self.pieces.iter_mut().enumerate() {
            let x = piece.x as i16 * box_size + x_offset;
            let y = piece.y as i16 * box_height + y_offset;

            let str = if self.selected.contains(&i) {
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
                input.is_mouse_down(),
            );

            if hovered && input.is_mouse_up() {
                if self.selected.len() == 1 && self.selected[0] == i {
                    continue;
                }
                self.selected.push(i);
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
            }

            self.selected.clear();
            self.moves += 1;
        }

        if self.pieces.len() == 0 {
            draw_text_box(
                screen,
                screen.width(),
                screen.height(),
                "Gratulerer!",
                0,
                0,
                (0, 0),
                false,
            );
        }

        let exit = draw_text_box(
            screen,
            screen.width(),
            screen.height(),
            "Tilbake",
            0,
            12,
            input.mouse_position(),
            input.is_mouse_down(),
        );
        if exit && input.is_mouse_up() {
            return Some(Box::new(MainState::new()));
        }

        None
    }

    fn exit(&mut self, screen: &mut Screen, input: &mut Input) {
    }
}
