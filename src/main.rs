use ansi_term::{Colour::Fixed, Style};
use game_engine::Board;
use zellij_tile::prelude::*;

mod game_engine;

#[derive(Default)]
struct Dimension(usize, usize);

#[derive(Default)]
struct State {
    run_game: bool,
    board: Board,
    dimension: Dimension,
    init_probability: f64,
    time_duration: f64,
}

register_plugin!(State);

impl State {
    fn init(&mut self) {
        self.init_probability = 0.2;
        self.time_duration = 0.1;
    }

    fn clear(&mut self) {
        self.board.clear();
        self.dimension = Dimension(0, 0);
    }
}

impl ZellijPlugin for State {
    fn load(&mut self) {
        subscribe(&[EventType::Key, EventType::Timer]);
        self.init();
    }

    fn update(&mut self, event: Event) -> bool {
        let mut should_render = false;
        match event {
            Event::Key(Key::Char('s')) => {
                if !self.run_game {
                    self.run_game = true;
                    set_timeout(self.time_duration);
                } else {
                    self.run_game = false;
                }
            }

            Event::Key(Key::Char('r')) => {
                self.clear();
                should_render = true;
            }

            Event::Key(Key::Up | Key::Char('k')) => {
                self.init_probability += 0.1;
                if self.init_probability > 1.0 {
                    self.init_probability = 1.0;
                }
                self.clear();
                should_render = true;
            }

            Event::Key(Key::Down | Key::Char('j')) => {
                self.init_probability -= 0.1;
                if self.init_probability < 0.0 {
                    self.init_probability = 0.0;
                }
                self.clear();
                should_render = true;
            }

            Event::Key(Key::Left | Key::Char('h')) => {
                self.time_duration *= 1.0 + 0.1;
            }

            Event::Key(Key::Right | Key::Char('l')) => {
                self.time_duration *= 1.0 - 0.1;
            }

            Event::Timer(_) => {
                if self.run_game {
                    set_timeout(self.time_duration);
                    self.board = self.board.update();
                    should_render = true;
                }
            }
            _ => (),
        };
        should_render
    }

    fn render(&mut self, rows: usize, cols: usize) {
        if self.dimension.0 != rows || self.dimension.1 != cols {
            self.dimension = Dimension(rows, cols);
            self.board = Board::rand_init(rows, cols, self.init_probability);
        }

        let board = color_bold(CYAN, &self.board.to_string());
        let instructions = color_bold(
            ORANGE,
            "s: Toggle start/stop | r: Reset | ← slower | → faster",
        );
        let init_instructions = color_bold(
            ORANGE,
            "← slower | → faster | ↑ Higher density | ↓ Lower density",
        );
        let init_probability = color_bold(RED, &format!("p={:.1}", self.init_probability));

        println!(
            "{}\n{}\n{} | {}",
            board, instructions, init_instructions, init_probability
        );
    }
}

pub const CYAN: u8 = 51;
pub const GRAY_LIGHT: u8 = 238;
pub const GRAY_DARK: u8 = 245;
pub const WHITE: u8 = 15;
pub const BLACK: u8 = 16;
pub const RED: u8 = 124;
pub const GREEN: u8 = 154;
pub const ORANGE: u8 = 166;

fn color_bold(color: u8, text: &str) -> String {
    format!("{}", Style::new().fg(Fixed(color)).bold().paint(text))
}
