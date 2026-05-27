use crate::checkers::{checkers_game::{BOARD_HEIGHT, BOARD_WIDTH, CheckerColor, CheckerRank, CheckersGame}, checkers_move::{CheckersMove, Square}};

pub struct CheckersBot;

impl CheckersBot {
    pub fn get_best_move(position: &CheckersGame, maximizing_player: bool) -> CheckersMove {
        let legal_moves = position.get_legal_moves();
        let mut best_move = legal_moves[0];
        let mut best_eval = if maximizing_player { i64::MIN } else { i64::MAX };

        for r#move in &legal_moves {
            let mut position = *position;
            position.make_move(r#move);
            let eval = CheckersBot::minimax(&position, 5, !maximizing_player);

            if (maximizing_player && (eval > best_eval)) || (!maximizing_player && (eval < best_eval)) {
                best_move = *r#move;
                best_eval = eval;
            }
        }

        best_move
    }

    fn minimax(position: &CheckersGame, depth: u64, maximizing_player: bool) -> i64 {
        let legal_moves = position.get_legal_moves();

        if (depth == 0) || legal_moves.is_empty() {
            return CheckersBot::evaluate(position);
        }

        let mut best_eval = if maximizing_player { i64::MIN } else { i64::MAX };
        // let mut best_move: CheckersMove = CheckersMove::new();

        for r#move in &legal_moves {
            let mut position = *position;
            position.make_move(r#move);
            let eval = CheckersBot::minimax(&position, depth - 1, !maximizing_player);

            if (maximizing_player && (eval > best_eval)) || (!maximizing_player && (eval < best_eval)) {
                best_eval = eval;
            }
        }

        best_eval
    }

    fn evaluate(position: &CheckersGame) -> i64 {
        let mut eval: i64 = 0;

        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                let checker = position.get_checker(Square{x, y});
                if !checker.occupied {
                    continue;
                }

                eval += match (checker.color, checker.rank) {
                    (CheckerColor::Black, CheckerRank::Soldier) => 1,
                    (CheckerColor::Black, CheckerRank::King) => 3,
                    (CheckerColor::Red, CheckerRank::Soldier) => -1,
                    (CheckerColor::Red, CheckerRank::King) => -3 
                }
            }
        }

        eval
    }
}