use crossterm::style;
use crate::drawing::{draw_ascii, draw_text_box};
use crate::input::{Input, MouseButton};
use crate::screen::Screen;
use crate::state_machine::State;
use crate::states::main_state::MainState;
use crate::states::transition_state::TransitionState;

pub struct Day14State {
    found_it: bool,
}

impl Day14State {
    pub fn new() -> Self {
        Day14State {
            found_it: false,
        }
    }
}

impl State for Day14State {
    fn enter(&mut self, screen: &mut Screen, input: &mut Input) {
    }

    fn update(&mut self, screen: &mut Screen, input: &mut Input, dt: f64) -> Option<Box<dyn State>> {

        let center_x = screen.width() / 2;
        let center_y = screen.height() / 2;
        draw_ascii(screen, NORWAY, center_x - NORWAY_WIDTH / 2, center_y - NORWAY_HEIGHT / 2);

        if !self.found_it {
            let question = "  Finn Hamar på kartet  ";
            draw_text_box(screen, screen.width(), screen.height(), &question, -20, -15, (0, 0), false);

            if input.is_mouse_down(MouseButton::Left) {
                let (mouse_x, mouse_y) = input.mouse_position();
                let hit = screen.get_cell(mouse_x, mouse_y);
                if hit.rune == HIT_CHAR {
                    self.found_it = true;
                }
            }
        } else {
            let question = " Du fant det! ";
            draw_text_box(screen, screen.width(), screen.height(), &question, -20, -15, (0, 0), false);
        }

        let exit = draw_text_box(
            screen,
            screen.width(),
            screen.height(),
            "Tilbake",
            -20,
            -12,
            input.mouse_position(),
            input.is_mouse_up(MouseButton::Left),
        );
        if exit && input.is_mouse_up(MouseButton::Left) {
            return Some(Box::new(TransitionState::new(Box::new(MainState::new()), None)));
        }

        None
    }

    fn exit(&mut self, screen: &mut Screen, input: &mut Input) {
    }
}

const NORWAY_WIDTH: u16 = 75;
const NORWAY_HEIGHT: u16 = 36;

const HIT_CHAR: char = '⠀';

const NORWAY: &str = r#"
                                                         ._,----._
                                                       Hammerfest   ~-_
                                                     _/                >
                                                 __--             ___-~
                                                /            _,-_ `---_,
                                          _--\ /            /    `--\ /
                                         /    *            |        ,'
                                     /'\ \  TROMS0        (        (
             Atlantic ocean        /'   \/      _/\___    /        /
                                 /'_,-Narvik___/ \_   `\/         |
             Norwegian Sea     /'/'  _/  . /       ~~\             \_
                              /`'   /    _/           |              \
                                   /.  _/              \              |
                                 _/ Bodoe               )            (
                               _/    /                  \             |
                              /     /                   /             |
                            _/     /                    \             \
                           /      |                 ,----+-.           (
                          /      /                .'        )           \_
                        _/      |                 |         \            (
                       /       /                  |        /~
                     _/       |                   >       / FINLAND
                  __/         _)                 /       /
              ___/    .    /~~  SWEDEN          /      /
           __/  Trondheim/                    _/      /
         ,/             <                    /      .'
        /                |                  /       |
       |      NORWAY     |                 /        |
       \____             |                /         |
       ,----'        .   /               (          |               ___---
       |       Lillehammer                \          \        __,--~~
       |.             ⠀  \                 \      _   ~-_  _*~Helsinki
      Bergen        Oslo /                  \    <_>     ~---~~
       |             *  |                    >                      ______
       `\           | | |         Stockholm*/                _-*~~~~
        `.         /   \|                 /~              <><
      Stavanger ,-'     \               _/     __          <__>\   ESTONIA
          `\___/         .Gothenburg   /      < /               ~-,_______"#;