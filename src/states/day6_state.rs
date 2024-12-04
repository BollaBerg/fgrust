use crate::drawing::{draw_ascii_safe_c, draw_text_box};
use crate::input::{Input, MouseButton};
use crate::screen::Screen;
use crate::state_machine::State;
use crate::states::main_state::MainState;
use crate::states::transition_state::TransitionState;
use crossterm::style;
use rand::{thread_rng, Rng};

#[derive(Clone)]
struct Card {
    id: u8,
    x: f64,
    y: f64,
    fraction: f64,
}

pub struct Day6State {
    cards: Vec<Card>,
    picked_cards: Vec<Card>,
    timer: f64,
}

impl Day6State {
    pub fn new(screen: &mut Screen) -> Self {
        Day6State {
            cards: create_cards(screen.width(), screen.height()),
            picked_cards: vec![],
            timer: 0.0,
        }
    }
}

fn update_cards(cards: &mut Vec<Card>, x: f64, y: f64, dt: f64) {
    for card in cards {
        let dx = x - card.x;
        let dy = y - card.y;
        let distance = (dx * dx + dy * dy).sqrt();
        let speed = 100.0;
        let fraction = (speed * dt / distance).min(1.0);
        card.x = lerp(card.x, x, fraction);
        card.y = lerp(card.y, y, fraction);
    }
}

fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

impl State for Day6State {
    fn enter(&mut self, screen: &mut Screen, input: &mut Input) {
        self.timer = 0.0;
    }

    fn update(&mut self, screen: &mut Screen, input: &mut Input, dt: f64) -> Option<Box<dyn State>> {

        let mouse_pos = input.mouse_position();
        let mx = mouse_pos.0 as i16;
        let my = mouse_pos.1 as i16;

        update_cards(&mut self.picked_cards, screen.width() as f64 / 2.0 - CARD_WIDTH / 2.0, screen.height() as f64 / 2.0, dt);

        for card in &self.picked_cards {
            draw_ascii_safe_c(screen, CARD_ASCII, card.x as i16, card.y as i16, style::Color::Green);
        }

        let mut to_remove: Vec<u8> = vec![];

        for card in &self.cards {
            if mx >= card.x as i16 && mx < card.x as i16 + CARD_WIDTH as i16 && my >= card.y as i16 && my < card.y as i16 + CARD_HEIGHT as i16 {
                draw_ascii_safe_c(screen, CARD_ASCII, card.x as i16, card.y as i16, style::Color::Yellow);

                if input.is_mouse_down(MouseButton::Left) {
                    self.picked_cards.push(card.clone());
                    to_remove.push(card.id);
                }
            }
            else {
                draw_ascii_safe_c(screen, CARD_ASCII, card.x as i16, card.y as i16, style::Color::White);
            }
        }

        for id in to_remove {
            self.cards.retain(|card| card.id != id);
        }

        if self.cards.is_empty() {
            let string = format!("  Bra jobba! Du klarte det på {:.2} sekunder.", self.timer);
            draw_text_box(screen, screen.width(), screen.height(), &string, 0, -8, (0, 0), false);

            let hovered = draw_text_box(screen, screen.width(), screen.height(), "Tilbake", 0, -4, input.mouse_position(), input.is_mouse_down(MouseButton::Left));
            if hovered && input.is_mouse_up(MouseButton::Left) {
                return Some(Box::new(TransitionState::new(Box::new(MainState::new()), None)));
            }
        }
        else {
            self.timer += dt;

            draw_text_box(screen, screen.width(), screen.height(),"    Din nevø på 7 har vært på besøk.", 0, -8, (0, 0), false);
            draw_text_box(screen, screen.width(), screen.height()," Rydd opp alle kortene han kastet ut på gulvet.", 0, -4, (0, 0), false);
        }

        None
    }

    fn exit(&mut self, screen: &mut Screen, input: &mut Input) {
    }
}

fn create_cards(width: u16, height: u16) -> Vec<Card> {
    let mut rng = thread_rng();

    let mut cards = vec![];
    for i in 0..52 {
        let x = rng.gen_range(CARD_WIDTH..width as f64 - CARD_WIDTH);
        let y = rng.gen_range(CARD_HEIGHT..height as f64 - CARD_HEIGHT);
        cards.push(Card { id: i, x, y, fraction: 0.0 });
    }
    cards
}


const CARD_WIDTH: f64 = 7.0;
const CARD_HEIGHT: f64 = 4.0;

const CARD_ASCII: &str =
"┌─────┐
│⠀⠀⠀⠀⠀│
│⠀⠀⠀⠀⠀│
└─────┘";