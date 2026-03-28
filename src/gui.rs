use macroquad::prelude::*;

use crate::game::{Game, Square};

pub async fn run(game: Game) {
    println!("{}", game.size);
    loop {
        clear_background(LIGHTGRAY);
        draw_text("HI bros", 20.0, 40.0, 20.0, BLACK);

        draw_board(&game);
        next_frame().await;
    }
}

fn draw_board(game: &Game) {
    let width = screen_width();
    let height = screen_height();

    let tile_size = width.min(height) / game.size as f32;

    for (idx, square) in game.board.iter().enumerate() {
        let row = idx / game.size;
        let col = idx % game.size;

        let (x, y) = (col as f32, row as f32);

        let color = { if (row + col) % 2 == 0 { BLACK } else { WHITE } };

        if let Square::_NotExists = square {
            continue;
        }

        draw_rectangle(
            x as f32 * tile_size,
            y as f32 * tile_size,
            tile_size,
            tile_size,
            color,
        );

        if let Square::Occupied(piece) = square {
            draw_text(
                &piece.name,
                x as f32 * tile_size,
                y as f32 * tile_size + tile_size,
                64.0,
                GREEN,
            );
        }
    }
}
