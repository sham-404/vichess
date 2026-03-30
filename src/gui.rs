use macroquad::prelude::*;

use crate::game::{Game, Square};

pub struct GUI {
    tile_size: f32,
    selected_square: Option<usize>,
}

impl GUI {
    pub fn new(game: &Game) -> Self {
        let tile_size = screen_width().min(screen_height()) / game.get_size() as f32;
        Self {
            selected_square: None,
            tile_size,
        }
    }

    pub async fn run(&mut self, game: &Game) {
        loop {
            clear_background(LIGHTGRAY);
            self.draw_board(&game);
            self.handle_clicks(&game);
            next_frame().await;
        }
    }

    pub fn handle_clicks(&mut self, game: &Game) {
        if !is_mouse_button_pressed(MouseButton::Left) {
            return;
        }

        let (x, y) = mouse_position();
        let col = (x / self.tile_size) as usize;
        let row = (y / self.tile_size) as usize;

        let idx = row * game.get_size() + col;

        if row < game.get_size() && col < game.get_size() {
            self.selected_square = Some(idx);
        }
    }

    pub fn draw_board(&mut self, game: &Game) {
        let width = screen_width();
        let height = screen_height();

        self.tile_size = width.min(height) / game.get_size() as f32;

        // Draw Board states
        for (idx, square) in game.squares().iter().enumerate() {
            let (x, y) = game.board().get_xy(idx);

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

            draw_rectangle(
                x as f32 * self.tile_size,
                y as f32 * self.tile_size,
                self.tile_size,
                self.tile_size,
                color,
            );

            // Drawing selected square
            if self.selected_square.is_some() && idx == self.selected_square.unwrap() {
                draw_rectangle(
                    x as f32 * self.tile_size,
                    y as f32 * self.tile_size,
                    self.tile_size,
                    self.tile_size,
                    PINK,
                );
            }
        }

        for (idx, square) in game.squares().iter().enumerate() {
            let (x, y) = game.board().get_xy(idx);
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
}
