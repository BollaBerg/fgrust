use crate::input::{Input, MouseButton};
use crate::screen::Screen;
use crate::state_machine::State;
use rand::{thread_rng, Rng};
use crate::drawing::{draw_ascii, draw_question};
use crate::states::main_state::MainState;
use crate::states::transition_state::TransitionState;

struct Particle {
    x: f64,
    y: f64,
    sprite: char,
}

pub struct Day5State {
    question: String,
    correct_answer: String,
    wrong_answers: [&'static str; 2],
    correct_answer_position: usize,

    phase: f64,
    particles: Vec<Particle>,
}

impl Day5State {
    pub fn new() -> Self {
        // spørsmål om jul
        let question= "Hvilket land er kjent for å ha oppfunnet tradisjonen med juletreet?".to_string(); 
        let correct_answer = "Tyskland".to_string();
        let wrong_answers = ["Norge", "Sverige"];
        let number_of_answers = wrong_answers.len() + 1;
        let correct_answer_position = thread_rng().gen_range(0..number_of_answers);
        Day5State {
            question,
            correct_answer,
            wrong_answers,
            correct_answer_position,

            phase: 0.0,
            particles: vec![],
        }
    }
}

impl State for Day5State {
    fn enter(&mut self, screen: &mut Screen, input: &mut Input) {
        self.particles = create_particles();
    }

    fn update(&mut self, screen: &mut Screen, input: &mut Input, dt: f64) -> Option<Box<dyn State>> {

        self.phase += dt;

        draw_ascii(screen, TREE_FIREPLACE, screen.width() - 43, (screen.height() as i16 - 40).clamp(0, screen.height() as i16) as u16);
        draw_ascii(screen, PRESENT, 12, screen.height() - 26);

        {
            let cat_x = screen.width() - 50;
            let cat_y = screen.height() - 12;

            draw_ascii(screen, LAZY_CAT, cat_x, cat_y);
            draw_particles(screen, &mut self.particles, cat_x, cat_y, self.phase, dt);
        }

        let mut correct = false;
        draw_question(
            screen,
            input.mouse_position(),
            input.is_mouse_up(MouseButton::Left),
            &self.question,
            &self.correct_answer,
            &self.wrong_answers,
            self.correct_answer_position,
            &mut || correct = true,
        );

        if correct {
            return Some(Box::new(TransitionState::new(Box::new(MainState::new()), None)));
        }

        None
    }

    fn exit(&mut self, screen: &mut Screen, input: &mut Input) {
    }
}

fn create_particles() -> Vec<Particle> {
    let mut particles = Vec::new();
    for i in 0..3 {
        particles.push(Particle {
            x: i as f64 * 2.0,
            y: i as f64 * 2.0,
            sprite: 'Z',
        });
    }
    particles
}

fn draw_particles(screen: &mut Screen, particles: &mut Vec<Particle>, x:u16, y:u16, phase: f64, dt: f64) {
    let particle_amplitude = 4.0;
    for (_i, particle) in particles.iter_mut().enumerate() {
        particle.x += (phase * 2.0 + particle.y).sin() * particle_amplitude * dt;
        screen.set_cell(x + particle.x as u16, y + particle.y as u16, particle.sprite, crossterm::style::Color::White);
    }
}

pub const LAZY_CAT: &str = r#"
                      ⢀⡀
    ⢀⡴⣆     ⣠⡀       ⣼⣿⡗
   ⣠⠟⠀⠘⠷⠶⠶⠶⠾⠉⢳⡄     ⣧⣿
  ⣰⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢻⣤⣤⣤⣤⣤⣿⢿⣄
  ⡇⠀⢀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣧⠀⠀⠀⠀⠀⠀⠙⣷⡴⠶⣦
  ⢱⡀⠀⠉⠉⠀⠀⠀⠀⠛⠃⠀⢠⡟⠂⠀⠀⢀⣀⣠⣤⠿⠞⠛⠋
⣠⠾⠋⠙⣶⣤⣤⣤⣤⣤⣀⣠⣤⣾⣿⠴⠶⠚⠋⠉⠁
⠛⠒⠛⠉⠉   ⣴⠟⣣⡴⠛⠋
        ⠛⠛⠉"#;

pub const PRESENT: &str = r#"
            ⢀⣀⣀           ⣀⡠⢄⣀
          ⢀⠞⠉⠀⠀⠙⢦⠀⠀⠀⠀⠀⠀⠀⢠⠎⠁⠀⠀⠈⠱⡄
         ⡞⠂⠀⠀⠀⠀⠈⣧⠖⠚⠉⠓⠲⢤⠇⠐⠀⠀⠀⠀⠀⢹
         ⣇⠀⣠⡀⠀⠀⠀⠸⡄⢀⣄⣠⢀⡼⠀⠀⠀⣀⣤⠿⡄⣼
   ⣀⣠⠤⠖⠚⠙⠹⣼⡏⡙⠳⢤⣀⠀⢹⠏⠉⠉⣾⠀⣀⣠⡶⠟⠉⣰⡿⠛⠒⠲⢤⣀⣀
⡶⢾⣉⡁⠀⡀⠠⠀⢂⠀⠈⠙⢳⣶⠦⠭⠽⠿⣦⣀⡠⠿⠿⠿⢶⣶⡞⠛⠉⠀⡀⠄⠠⠀⢀⢈⣩⣶⡆
⣇⠠⡈⠉⠳⠦⣤⣂⡤⠼⠚⠋⠉⠀⣀⡤⠴⠞⠋⠉⠙⠲⠤⣄⣀⠀⠉⠙⠲⠦⣤⣐⣤⠾⡛⠫⢑⢸⡇
⠙⡦⢥⣐⠀⡀⠀⣽⠐⠀⢠⣶⡞⠉⠡⠐⠀⠄⠂⠁⡐⠀⡀⢀⠈⢉⣒⣦⣄⠀⠠⣟⢈⡐⣡⣸⡴⣾⠃
⠈⡇⠀⠈⠙⠳⠦⣾⠀⠀⢸⢬⠉⠛⠶⣤⣈⡀⠄⠁⡀⣐⣠⡴⠾⡛⠍⢃⡇⡈⢔⣯⢶⠻⠍⢃⠱⣻
⠈⡇⠀⠁⠂⡀⠄⢻⠩⠓⢾⢦⣀⡄⠀⡀⠈⠙⠲⣶⠻⠍⢃⡉⠔⣀⣣⡾⡷⡞⠯⣏⠐⡌⠰⢁⠪⣽
⠈⡇⠀⠁⠄⠀⠄⣻⠀⠀⢸⠀⠈⠙⠓⠦⣌⣀⠄⡿⢐⣨⣴⠶⡛⢋⠱⢈⡇⡐⠠⡗⢠⠂⢅⠢⢑⣿
⠈⡇⠀⡈⠀⠌⠀⢾⠀⠀⢸⠀⢈⠠⠐⠀⡀⠉⠛⣿⠛⠱⠈⠤⠑⡨⠐⠌⡇⠄⠡⡟⠠⠌⢂⠔⡡⢾
⠈⡇⠀⡀⠌⠀⠄⣻⠀⠀⢸⠀⠠⠀⡐⠀⠄⠂⠀⣿⠈⠔⡉⠄⣃⠐⡉⢌⡗⡈⠐⣯⠐⣁⠊⡐⢌⣿
⠐⡇⠀⠀⠄⠈⠀⢾⠀⠀⢸⠀⠄⠁⡀⠐⢀⠈⠄⣿⠈⡰⠈⠔⡀⠎⡐⢂⡧⢀⠡⡗⢠⠂⡘⡀⢎⣾
⠐⡇⠀⢁⠠⠁⠈⢾⠀⠀⢸⠀⡀⠂⠀⠌⠀⠠⠀⣿⠐⢠⠉⡰⠈⠔⡠⢃⡇⢂⠐⣯⠀⠆⢡⠐⢢⢿
⠠⡇⠀⠠⠀⠂⠁⢾⠀⠀⢸⠀⠀⠄⠁⠠⠈⠀⠄⣿⠠⢁⠢⢁⠜⠠⣁⠢⡏⠠⢈⡧⠘⡈⢄⠊⡔⣻
⠠⡇⠀⡁⠐⠈⠀⣻⠀⠀⢸⠀⠁⠠⠈⡀⠄⠁⡀⣿⢀⠊⡐⠌⡠⠃⢄⠒⡏⡐⠠⡟⢠⠁⠆⢌⠰⣻
⠐⡇⢁⠀⠄⠁⠠⣹⠀⠀⢸⠀⠈⠄⠐⠀⡀⠂⠀⣿⠀⠜⢠⠘⠠⠑⡂⡘⡇⠄⠡⣟⠠⠌⡈⢄⢣⣿
 ⠉⠚⠣⢤⣈⠀⣽⠀⠀⢸⠀⠁⠠⠈⠀⠄⠠⠁⣿⠈⡐⢂⠡⢃⡁⠆⢡⡏⠠⢁⡷⢀⣣⡼⠖⠋⠁⠀
      ⠈⠉⢻⠀⠀⢸⠀⠈⡄⢠⠁⠀⠂⡄⣿⠀⡁⠊⢰⠀⡆⠘⢠⡇⠁⣦⡟⠉⠁
         ⠉⠒⠾⠲⢤⣀⠄⠀⡁⠠⠀⣿⠐⠤⢁⠢⢁⣔⡥⠾⠷⠛⠉
              ⠈⠉⠓⠦⣤⣀⡿⢠⣼⠴⠛⠉⠁
                   ⠈⠙⠉"#;

const TREE_FIREPLACE: &str = r#"
                 .!,            .!,
                ~ 6 ~          ~ 6 ~
           .    ' i `  .-^-.   ' i `
         _.|,_   | |  / .-. \   | |
          '|`   .|_|.| (-` ) | .|_|.
          /⠀\ ___)_(_|__`-'__|__)_(______
         /`,o\)_______________________o_(
        /_*⠀~_\[___]___[___]___[___[_[\`-.
        /⠀o⠀.'\[_]___[___]___[___]_[___)`-)
       /_,~'⠀*_\_]                 [_[(  (
       /`. *⠀⠀*\_]                 [___\ _\
      /⠀⠀⠀`~. o⠀\]      ;( ( ;     [_[_]`-'
     /_ *⠀⠀⠀⠀`~,_\    (( )( ;(;    [___]
     /⠀⠀⠀o⠀⠀*⠀⠀~'\   /\ /\ /\ /\   [_[_]
    /⠀*⠀⠀⠀⠀.~~'⠀⠀o\  ||_||_||_||   [___]
   /_,.~~'`    *  _\_||_||_||_||___[_[_]_
   /`~..  o        \:::::::::::::::::::::\
  / *   `'~..   *   \:::::::::::::::::::::\
 /_     o    ``~~.,,_\=========\_/========='
 /  *      *     ..~'\         _|_ .-_--.
/*    o   _..~~`'*   o\           ( (_)  )
`-.__.~'`'   *   ___.-'            `----'
      ":-------:"
        \_____/  "#;