use macroquad::prelude::*;

use crate::{
    game::{Game, Square},
    piece::MyColor,
};

pub struct GUI {
    game: Game,
    tile_size: f32,
    selected_pos: Option<usize>,
}

impl GUI {
    pub fn new(game: Game) -> Self {
        let tile_size = screen_width().min(screen_height()) / game.get_size() as f32;
        Self {
            game: game,
            selected_pos: None,
            tile_size,
        }
    }

    pub async fn run(&mut self) {
        loop {
            clear_background(LIGHTGRAY);
            self.draw_board();
            self.handle_clicks();
            next_frame().await;
        }
    }

    pub fn handle_clicks(&mut self) {
        if !is_mouse_button_pressed(MouseButton::Left) {
            return;
        }

        let (x, y) = mouse_position();
        let col = (x / self.tile_size) as usize;
        let row = (y / self.tile_size) as usize;

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
                    BLACK
                } else {
                    WHITE
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
            self.color_square(x as f32, y as f32, PINK);

            // Drawing possible movements for selected piece if any
            if let Square::Occupied(piece) = &self.game.board().peek(pos) {
                let moves = &self.game.get_piece_moves(piece, pos);
                for &pos in moves.iter() {
                    self.color_square(self.game.col(pos) as f32, self.game.row(pos) as f32, GRAY);
                }
            }
        }

        for (idx, square) in self.game.squares().iter().enumerate() {
            let (x, y) = self.game.board().get_xy(idx);
            // Drawing pieces
            if let Square::Occupied(piece) = square {
                draw_text(
                    &piece.name(),
                    x as f32 * self.tile_size,
                    y as f32 * self.tile_size + self.tile_size,
                    64.0,
                    GREEN,
                );
            }
        }
    }

    #[allow(dead_code)]
    fn debug_square_drawing(&self) {
        // possible piece movements

        for (idx, square) in self.game.squares().iter().enumerate() {
            if let Square::Occupied(piece) = square {
                if piece.color() == MyColor::White {
                    continue;
                }

                let moves = self.game.get_piece_moves(piece, idx);
                for &pos in moves.iter() {
                    self.color_square(self.game.col(pos) as f32, self.game.row(pos) as f32, GRAY);
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
