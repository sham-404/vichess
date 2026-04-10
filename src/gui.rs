use macroquad::prelude::*;

use crate::{
    game::{Game, Square},
    piece::PieceColor,
};

pub struct GUI {
    game: Game,
    tile_size: f32,
    selected_pos: Option<usize>,
    color: BoardColor,
}

impl GUI {
    pub fn new(game: Game) -> Self {
        let tile_size = screen_width().min(screen_height()) / game.get_size() as f32;
        Self {
            game: game,
            selected_pos: None,
            tile_size,
            color: BoardColor::dark(),
        }
    }

    pub async fn run(&mut self) {
        loop {
            clear_background(self.color.background);
            self.draw_board();
            self.draw_pieces();
            self.handle_clicks();
            next_frame().await;
        }
    }

    pub fn handle_clicks(&mut self) {
        if !is_mouse_button_pressed(MouseButton::Left) {
            if is_mouse_button_pressed(MouseButton::Right) {
                self.game.undo_move();
            }
            return;
        }

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
                if !matches!(self.game.board().peek(new_pos), Square::_NotExists) {
                    if self.game.make_move(pos, new_pos) {
                        self.selected_pos = None;
                    } else {
                        self.selected_pos = Some(new_pos);
                    }
                }
            }
            None => self.selected_pos = Some(new_pos),
        }
    }

    pub fn draw_board(&mut self) {
        let width = screen_width();
        let height = screen_height();

        self.tile_size = width.min(height) / self.game.get_size() as f32;

        // Draw Board states
        for (idx, square) in self.game.squares().iter().enumerate() {
            let (x, y) = self.game.board().get_xy(idx);

            let color = {
                if (x + y) as usize % 2 == 0 {
                    self.color.dark_square
                } else {
                    self.color.light_square
                }
            };

            if let Square::_NotExists = square {
                continue;
            }

            self.color_square(x, y, color);

            //self.debug_square_drawing();
        }

        // Drawing selected square
        if self.selected_pos.is_some() {
            let pos = self.selected_pos.unwrap();
            let (x, y) = self.game.board().get_xy(pos);
            self.color_square(x as f32, y as f32, self.color.selected_piece);

            // Drawing possible movements for selected piece if any
            if let Square::Occupied(piece) = &self.game.board().peek(pos) {
                let moves = &self.game.get_moves(piece, pos);
                for pos in moves.iter() {
                    self.color_square(
                        self.game.col(pos.to) as f32,
                        self.game.row(pos.to) as f32,
                        self.color.possible_moves,
                    );
                }
            }
        }
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
        // possible piece movements

        for (idx, square) in self.game.squares().iter().enumerate() {
            if let Square::Occupied(piece) = square {
                if piece.color() == &PieceColor::White {
                    continue;
                }

                let moves = self.game.get_moves(piece, idx);
                for pos in moves.iter() {
                    self.color_square(
                        self.game.col(pos.to) as f32,
                        self.game.row(pos.to) as f32,
                        GRAY,
                    );
                }
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
}

#[allow(dead_code)]
impl BoardColor {
    pub fn matte() -> Self {
        Self {
            light_square: hex("#DCE1C5"),
            dark_square: hex("#5A6B3C"),
            selected_piece: hex("#D18B47"),
            possible_moves: hex("#8FBF8F80"),
            background: hex("#1F2A1F"),
        }
    }

    pub fn dark() -> Self {
        Self {
            light_square: hex("#3C3F41"),
            dark_square: hex("#2B2B2B"),
            selected_piece: hex("#F39C12"),
            possible_moves: hex("#3498DB80"),
            background: hex("#1E1E1E"),
        }
    }

    pub fn classic() -> Self {
        Self {
            light_square: hex("#EEEED2"),
            dark_square: hex("#769656"),
            selected_piece: hex("#E8C547"),
            possible_moves: hex("#A8D5BABB"),
            background: hex("#1B1B1B"),
        }
    }
}
