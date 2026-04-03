use macroquad::prelude::*;

use crate::{
    game::{Game, Square},
    piece::MyColor,
};

pub struct GUI {
    game: Game,
    tile_size: f32,
    selected_square: Option<usize>,
}

impl GUI {
    pub fn new(game: Game) -> Self {
        let tile_size = screen_width().min(screen_height()) / game.get_size() as f32;
        Self {
            game: game,
            selected_square: None,
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

        let idx = row * &self.game.get_size() + col;

        if row < self.game.get_size() && col < self.game.get_size() {
            self.selected_square = Some(idx);
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

            // Drawing selected square
            if self.selected_square.is_some() && idx == self.selected_square.unwrap() {
                self.color_square(x, y, PINK);

                if let Square::Occupied(piece) = square {
                    for pos in piece.get_piece_moves(self.game.board()) {
                        self.color_square(pos.col as f32, pos.row as f32, GRAY);
                    }
                }
            }

            //self.debug_square_drawing();
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

        for square in self.game.squares().iter() {
            if let Square::Occupied(piece) = square {
                if piece.color() == MyColor::White {
                    continue;
                }

                for pos in piece.get_piece_moves(self.game.board()) {
                    self.color_square(pos.col as f32, pos.row as f32, GRAY);
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
