use crate::checkers::checkers_bot::CheckersBot;
use crate::checkers::checkers_game::{CheckerColor, CheckersGame};
use crate::checkers::checkers_gui::CheckersGUI;
use crate::checkers::checkers_move::{CheckersMove, Square};
use crate::checkers::checkers_game::Checker;

pub mod checkers;

use macroquad::prelude::*;

#[macroquad::main("Checkers")]
async fn main() {
    let mut game: CheckersGame = CheckersGame::new();
    game.setup();

    let gui: CheckersGUI = CheckersGUI::new();

    let mut bot: CheckersBot = CheckersBot::new();

    let mut chosen_move: CheckersMove = CheckersMove::new();
    let mut has_start: bool = false;
    let mut moves_from_start: Vec<CheckersMove> = Vec::new();
    let mut has_end: bool = false;

    loop {
        clear_background(BLACK);

        gui.draw(&game, &Vec::new(), Square::new());

        if game.turn == CheckerColor::Red {
            let mouse_pos = mouse_position();

            if is_mouse_button_pressed(MouseButton::Left) {
                let square: Square = Square { x: ((mouse_pos.0 - 32f32) as i32 / 64) as u64, y: ((mouse_pos.1 - 32f32) as i32 / 64) as u64 };

                if CheckersGame::in_bounds(square) {
                    let checker: Checker = game.get_checker(square);

                    if !has_start {
                        if checker.occupied {
                            chosen_move.start = square;
                            moves_from_start = game.get_checker_legal_moves(square);
                            has_start = true;
                        }
                    } 
                    else if has_start && !has_end {
                        for r#move in &moves_from_start {
                            if r#move.end == square {
                                chosen_move.end = r#move.end;
                                chosen_move.capture = r#move.capture;
                                chosen_move.coronation = r#move.coronation;
                                has_end = true;
                                break;
                            }
                        }
                        if !has_end {
                            has_start = false;
                            moves_from_start = Vec::new();
                            chosen_move = CheckersMove::new();
                        }
                    }
                }
                
                
            }

            if (has_start && has_end) || is_mouse_button_pressed(MouseButton::Right) {
                if has_start && has_end {
                    game.make_move(&chosen_move);
                }

                has_start = false;
                has_end = false;
                moves_from_start = Vec::new();
                chosen_move = CheckersMove::new();
            }
            
            gui.draw(&game, &moves_from_start, chosen_move.start);
        } else {
            let bot_move: CheckersMove = bot.get_best_move(&game, true);
            game.make_move(&bot_move);
            gui.draw(&game, &Vec::new(), Square::new());
        }

        next_frame().await
    }
}