use crate::checkers::checkers_game::CheckersGame;
use crate::checkers::checkers_game::BOARD_WIDTH;
use crate::checkers::checkers_game::BOARD_HEIGHT;
use crate::checkers::checkers_move::CheckersMove;
use crate::checkers::checkers_move::Square;
use crate::checkers::checkers_game::CheckerColor;
use crate::checkers::checkers_game::CheckerRank;

use macroquad::prelude::*;
use macroquad::texture::Texture2D;

use std::collections::HashSet;

pub struct  CheckersGUI {
    textures: [Texture2D; 5]
}

impl CheckersGUI {
    pub fn new() -> Self {
        CheckersGUI { 
            textures: [
                Texture2D::from_file_with_format(include_bytes!("assets/black_soldier_checker.png"), Some(ImageFormat::Png)),
                Texture2D::from_file_with_format(include_bytes!("assets/black_king_checker.png"), Some(ImageFormat::Png)),
                Texture2D::from_file_with_format(include_bytes!("assets/red_soldier_checker.png"), Some(ImageFormat::Png)),
                Texture2D::from_file_with_format(include_bytes!("assets/red_king_checker.png"), Some(ImageFormat::Png)),
                Texture2D::from_file_with_format(include_bytes!("assets/white_checker_outline.png"), Some(ImageFormat::Png))
            ]
        }
    }

    pub fn draw(&self, game: &CheckersGame, moves: &Vec<CheckersMove>, selected_square: Square) {
        let mut end_squares: HashSet<Square> = HashSet::new();

        for r#move in moves {
            end_squares.insert(r#move.end);
        }

        draw_rectangle(0f32, 0f32, 576f32, 576f32, DARKBROWN);

        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                let mut square_color: Color = DARKGREEN;
                if (x + y) % 2 == 0 {
                    square_color = WHITE;
                }
                if end_squares.contains(&Square{x, y}) {
                    square_color = GREEN;
                }

                let nx: f32 = ((x * 64) as f32) + 32f32;
                let ny: f32 = ((y * 64) as f32) + 32f32;

                draw_rectangle(nx, ny, 64f32, 64f32, square_color);

                let checker = game.get_checker(Square {x, y});
                if !checker.occupied {
                    continue;
                }

                if (Square{x, y}) == selected_square {
                    draw_texture(
                        &self.textures[4],
                        nx,
                        ny,
                        WHITE
                    )
                }

                draw_texture(
                    match (checker.color, checker.rank) { (CheckerColor::Black, CheckerRank::Soldier) => &self.textures[0], (CheckerColor::Black, CheckerRank::King) => &self.textures[1], (CheckerColor::Red, CheckerRank::Soldier) => &self.textures[2], (CheckerColor::Red, CheckerRank::King) => &self.textures[3] },
                    nx,
                    ny,
                    WHITE
                )
            }
        }
    }
}