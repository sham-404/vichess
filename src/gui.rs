use macroquad::prelude::*;

use crate::{
    game::{Game, GameState, Square},
    piece::PieceColor,
};

pub struct GUI {
    game: Game,
    tile_size: f32,
    selected_pos: Option<usize>,
    theme: Theme,
    color: BoardColor,
    state: GameState,
}

impl GUI {
    pub fn new(game: Game) -> Self {
        let tile_size = screen_width().min(screen_height()) / game.get_size() as f32;
        Self {
            game: game,
            selected_pos: None,
            tile_size,
            theme: Theme::Dark,
            color: BoardColor::dark(),
            state: GameState::Playing,
        }
    }

    pub async fn run(&mut self) {
        loop {
            clear_background(self.color.background);

            match self.state {
                GameState::Playing => {
                    self.draw_board();
                    self.draw_pieces();
                    self.handle_clicks();
                }
                GameState::CheckMate(_) | GameState::Draw => {
                    self.draw_board();
                    self.draw_pieces();
                    self.draw_game_over();
                }
            }
            next_frame().await;
        }
    }

    fn change_theme(&mut self) {
        self.theme = self.theme.next();
        self.color = self.theme.get_colors();
    }

    fn draw_game_over(&mut self) {
        let text = match self.state {
            GameState::CheckMate(winner) => match winner {
                PieceColor::White => "White Wins!",
                PieceColor::Black => "Black Wins!",
            },
            GameState::Draw => "Draw!",
            _ => panic!("impossible, check draw_game_over()"),
        };

        let width = screen_width();
        let height = screen_height();

        let pos_to_display = width.min(height) / 3.75 as f32;
        let size_to_display = pos_to_display * 2 as f32;

        draw_rectangle(
            pos_to_display,
            pos_to_display,
            size_to_display,
            size_to_display,
            self.color.background,
        );

        draw_text(
            text,
            pos_to_display + 55.0,
            pos_to_display + 100.0,
            50.0,
            RED,
        );
        draw_text(
            "Press R to restart",
            pos_to_display + 50.0,
            pos_to_display + 200.0,
            30.0,
            WHITE,
        );
        if is_key_pressed(KeyCode::R) {
            self.game.restart();
            self.state = GameState::Playing;
        }
    }

    pub fn handle_clicks(&mut self) {
        // Undo move
        if is_mouse_button_pressed(MouseButton::Right) {
            self.game.undo_move();
        }

        // Selection and handling clicks and moves
        if is_mouse_button_pressed(MouseButton::Left) {
            let (x, y) = mouse_position();
            let col = (x / self.tile_size) as usize;
            let row = (y / self.tile_size) as usize;

            if col > self.game.get_size() - 1 || row > self.game.get_size() - 1 {
                return;
            }

            let new_pos = self.game.board().idx(row as i32, col as i32);

            if !self.game.board().within_bounds(new_pos) {
                return;
            }

            match self.selected_pos {
                Some(pos) => {
                    if self.game.make_move(pos, new_pos) {
                        self.selected_pos = None;

                        // update game state after moving
                        self.state = self.game.get_game_state();
                    } else {
                        self.selected_pos = Some(new_pos);
                    }
                }
                None => self.selected_pos = Some(new_pos),
            }
        }

        // changing themes
        if alt_down() && is_key_pressed(KeyCode::C) {
            self.change_theme();
        }

    }

    pub fn draw_board(&mut self) {
        let width = screen_width();
        let height = screen_height();

        self.tile_size = width.min(height) / self.game.get_size() as f32;

        // Draw Board states
        for (idx, _) in self.game.squares().iter().enumerate() {
            let (x, y) = self.game.board().get_xy(idx);

            let color = {
                if (x + y) as usize % 2 == 0 {
                    self.color.light_square
                } else {
                    self.color.dark_square
                }
            };

            self.color_square(x, y, color);

            // drawing index on each squares
            draw_text(
                idx.to_string().as_str(),
                x as f32 * self.tile_size,
                y as f32 * self.tile_size + 12.0,
                12.0,
                BLACK,
            );
        }

        // Drawing the last move on board
        if let Some(mov) = self.game.get_last_move() {
            self.color_square(
                self.game.col(mov.from) as f32,
                self.game.row(mov.from) as f32,
                self.color.last_move,
            );

            self.color_square(
                self.game.col(mov.to) as f32,
                self.game.row(mov.to) as f32,
                self.color.last_move,
            );
        }

        // Drawing selected square
        if self.selected_pos.is_some() {
            let pos = self.selected_pos.unwrap();
            let (x, y) = self.game.board().get_xy(pos);
            self.color_square(x as f32, y as f32, self.color.selected_piece);

            // Drawing possible movements for selected piece if any
            if let Square::Occupied(_) = &self.game.board().peek(pos) {
                let moves = &self.game.moves(pos);
                for mov in moves.iter() {
                    let color = match mov.capture {
                        Some(_) => self.color.attacked,
                        None => self.color.possible_moves,
                    };

                    self.color_square(
                        self.game.col(mov.to) as f32,
                        self.game.row(mov.to) as f32,
                        color,
                    );
                }
            }
        }
        // self.debug_square_drawing();
    }

    fn draw_pieces(&self) {
        for (idx, square) in self.game.squares().iter().enumerate() {
            let (x, y) = self.game.board().get_xy(idx);

            // Drawing pieces
            if let Square::Occupied(piece) = square {
                let color = match piece.color() {
                    PieceColor::White => LIGHTGRAY,
                    PieceColor::Black => BLACK,
                };

                draw_text(
                    &piece.get_name(),
                    x as f32 * self.tile_size,
                    y as f32 * self.tile_size + self.tile_size,
                    64.0,
                    color,
                );
            }
        }
    }

    #[allow(dead_code)]
    fn debug_square_drawing(&self) {
        // attack squares

        for (idx, attack) in self.game.get_attack_map().iter().enumerate() {
            if *attack {
                self.color_square(
                    self.game.col(idx) as f32,
                    self.game.row(idx) as f32,
                    self.color.attacked,
                );
            }
        }
    }

    fn color_square(&self, x: f32, y: f32, color: Color) {
        draw_rectangle(
            x as f32 * self.tile_size,
            y as f32 * self.tile_size,
            self.tile_size,
            self.tile_size,
            color,
        );
    }

}

fn alt_down() -> bool {
    is_key_down(KeyCode::LeftAlt) || is_key_down(KeyCode::RightAlt)
}

fn hex(hex: &str) -> Color {
    let hex = hex.trim_start_matches('#');

    let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap();

    let a = if hex.len() == 8 {
        u8::from_str_radix(&hex[6..8], 16).unwrap()
    } else {
        255
    };

    Color::from_rgba(r, g, b, a)
}

pub struct BoardColor {
    light_square: Color,
    dark_square: Color,
    selected_piece: Color,
    possible_moves: Color,
    background: Color,
    attacked: Color,
    last_move: Color,
}

#[allow(dead_code)]
impl BoardColor {
    pub fn matte() -> Self {
        Self {
            light_square: hex("#DCE1C5"),
            dark_square: hex("#5A6B3C"),
            selected_piece: hex("#D18B47C0"),
            possible_moves: hex("#8FBF8F80"),
            background: hex("#1F2A1F"),
            attacked: hex("#f2000080"),
            last_move: hex("#D18B4760"),
        }
    }

    pub fn dark() -> Self {
        Self {
            light_square: hex("#3C3F41"),
            dark_square: hex("#2B2B2B"),
            selected_piece: hex("#F39C1280"),
            possible_moves: hex("#3498DB80"),
            background: hex("#1E1E1E"),
            attacked: hex("#ff1a0080"),
            last_move: hex("#F39C1220"),
        }
    }

    pub fn classic() -> Self {
        Self {
            light_square: hex("#EEEED2"),
            dark_square: hex("#769656"),
            selected_piece: hex("#E8C547C0"),
            possible_moves: hex("#A8D5BABB"),
            background: hex("#1B1B1B"),
            attacked: hex("#b3000080"),
            last_move: hex("#E8C54760"),
        }
    }

    pub fn blue() -> Self {
        Self {
            light_square: hex("#C9D6DF"),
            dark_square: hex("#7A97A8"),
            selected_piece: hex("#5FA8D3C0"),
            possible_moves: hex("#A6C8E080"),
            background: hex("#1E2A32"),
            attacked: hex("#E0525280"),
            last_move: hex("#5FA8D360"),
        }
    }
}

pub enum Theme {
    Matte,
    Dark,
    Classic,
    Blue,
}

impl Theme {
    pub fn get_colors(&self) -> BoardColor {
        match self {
            Theme::Matte => BoardColor::matte(),
            Theme::Dark => BoardColor::dark(),
            Theme::Classic => BoardColor::classic(),
            Theme::Blue => BoardColor::blue(),
        }
    }

    pub fn next(&self) -> Self {
        match self {
            Theme::Matte => Theme::Dark,
            Theme::Dark => Theme::Classic,
            Theme::Classic => Theme::Blue,
            Theme::Blue => Theme::Matte,
        }
    }
}
