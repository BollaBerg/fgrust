extern crate crossterm;

use crossterm::style;
use crate::screen::Screen;

use crate::days::{CalendarDay, RunStatus};

pub fn draw_debug_info(
    screen: &mut Screen,
    mouse_position: (u16, u16),
    mouse_down: bool, dt: f64,
) {
    let fps_str = format!("FPS: {:.0}", 1.0 / dt);
    for (i, c) in fps_str.chars().enumerate() {
        screen.set_cell(i as u16, 0, c, style::Color::White);
    }

    let mouse_pos_str = format!("Mouse: ({}, {})", mouse_position.0, mouse_position.1);
    for (i, c) in mouse_pos_str.chars().enumerate() {
        screen.set_cell(i as u16, 1, c, style::Color::White);
    }

    let mouse_down_str = format!("Mouse Down: {}", mouse_down);
    for (i, c) in mouse_down_str.chars().enumerate() {
        screen.set_cell(i as u16, 2, c, style::Color::White);
    }
}

pub fn draw_ground(screen: &mut Screen) {
    for i in 0..screen.width() {
        screen.set_cell(i, screen.height() - 1, '█', style::Color::White);
    }
}

pub fn draw_ascii(screen: &mut Screen, ascii: &str, x: u16, y: u16) {
    let lines = ascii.lines();

    for (i, line) in lines.enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == ' ' {
                continue;
            }

            screen.set_cell(x + j as u16, y + i as u16, c, style::Color::White);
        }
    }
}

pub fn draw_question(screen: &mut Screen, mouse_position: (u16, u16), mouse_down: bool,
    question: &str,
    correct_answer: &str,
    wrong_answers: &[&str],
    correct_answer_position: usize,
    on_correct_answer: &mut dyn FnMut(),
) {
    let width = screen.width();
    let height = screen.height();

    draw_text_box(screen, width, height, question, 0, -5, (0, 0), false);

    let total_answers = wrong_answers.len() + 1;

    let mut incorrect_is_hovered: Vec<bool> = vec![];
    let mut correct_answer_hovered: bool = false;


    let delta_offset: i16 = 20;
    let minimum_offset: i16 =
        (((total_answers - 1) as f32 / 2.0) * -delta_offset as f32) as i16;

    let mut wrong_answer_iterator: usize = 0;

    for i in 0..total_answers {
        let x_offset = minimum_offset + delta_offset * i as i16;
        if i == correct_answer_position {
            correct_answer_hovered = draw_text_box(
                screen,
                width,
                height,
                correct_answer,
                x_offset,
                0,
                mouse_position,
                mouse_down,
            );
        } else {
            incorrect_is_hovered.push(draw_text_box(
                screen,
                width,
                height,
                wrong_answers[wrong_answer_iterator],
                x_offset,
                0,
                mouse_position,
                mouse_down,
            ));
            wrong_answer_iterator += 1;
        }
    }

    if mouse_down {
        let any_incorrect_is_hovered = incorrect_is_hovered.into_iter().any(|x| x);
        if correct_answer_hovered && !any_incorrect_is_hovered {
            draw_text_box(
                screen,
                width,
                height,
                "Correct!",
                0,
                5,
                (0, 0),
                false,
            );
            on_correct_answer();
        } else if any_incorrect_is_hovered {
            draw_text_box(screen, width, height, "Wrong!", 0, 5, (0, 0), false);
        }
    }
}

fn draw_text_box(screen: &mut Screen, width: u16, height: u16, q: &str, x_offset: i16, y_offset: i16, mouse_position: (u16, u16), mouse_down: bool) -> bool {
    let question = q;
    let x_origin = ((width as i16 - question.len() as i16) / 2 + x_offset) as u16;
    let y_origin = (height as i16 / 2 + y_offset) as u16;

    let fancy_top_border = "╭".to_string() + &"─".repeat(question.len() + 4) + "╮";
    let fancy_bottom_border = "╰".to_string() + &"─".repeat(question.len() + 4) + "╯";

    let mut color = style::Color::White;

    let mut is_hovered = false;
    if mouse_position.0 >= x_origin - 3 &&
        mouse_position.0 <= x_origin + question.len() as u16 + 2 &&
        mouse_position.1 >= y_origin - 1 &&
        mouse_position.1 <= y_origin + 1 {

        if mouse_down {
            color = style::Color::Rgb {
                r: 0,
                g: 255,
                b: 0,
            };
        } else {
            color = style::Color::Rgb {
                r: 255,
                g: 255,
                b: 0,
            };
        }
        is_hovered = true;
    }

    for i in 0..3 {
        let line = " ".repeat(question.len() + 4);
        for (j, c) in line.chars().enumerate() {
            screen.set_cell(x_origin - 2 + j as u16, y_origin + i as u16, c, style::Color::White);
        }
    }

    if y_origin > 0 {
        let line = &fancy_top_border;
        for (j, c) in line.chars().enumerate() {
            screen.set_cell(x_origin - 3 + j as u16, y_origin - 1, c, color);
        }
    }

    if y_origin < height - 1
    {
        let line = &fancy_bottom_border;
        for (j, c) in line.chars().enumerate() {
            screen.set_cell(x_origin - 3 + j as u16, y_origin + 1, c, color);
        }
    }

    if x_origin > 3 {
        screen.set_cell(x_origin - 3, y_origin, '│', color);
    }

    if x_origin + question.len() as u16 + 2 < width {
        screen.set_cell(x_origin + question.len() as u16 + 2, y_origin, '│', color);
    }

    for (i, c) in question.chars().enumerate() {
        screen.set_cell(x_origin + i as u16, y_origin, c, color);
    }

    is_hovered
}

pub fn draw_calendar(
    screen: &mut Screen,
    mouse_position: (u16, u16),
    mouse_down: bool,
) -> Option<usize> {
    let total_days:usize = 24;
    let columns:i16 = 6;
    let rows:i16 = (total_days as f32 / columns as f32).ceil() as i16;

    let box_width = 6;

    let padding = 4;
    let x_start:i16 = -(columns * box_width) / 2 - box_width;
    let y_start:i16 = -((rows - 1) * padding) / 2;

    let x_step = box_width + padding;
    let y_step = padding;

    let mut day_is_hovered = vec![false; total_days];

    for i in 0..total_days {
        let x_offset = x_start + (i as i16 % columns) * x_step;
        let y_offset = y_start + (i as i16 / columns) * y_step;

        let day_text = if i < 9 {
            format!("0{}", i + 1)
        } else {
            (i + 1).to_string()
        };

        day_is_hovered[i] = draw_text_box(
            screen,
            screen.width(),
            screen.height(),
            &day_text,
            x_offset,
            y_offset,
            mouse_position,
            mouse_down,
        );
    }

    if mouse_down {
        for (i, is_hovered) in day_is_hovered.iter().enumerate() {
            if *is_hovered {
                return Some(i+1);
            }
        }
    }
    None
}
