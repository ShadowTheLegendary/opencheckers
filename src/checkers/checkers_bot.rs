use std::time::{Duration, Instant};

use crate::checkers::{
    checkers_game::{BOARD_HEIGHT, BOARD_WIDTH, CheckerColor, CheckerRank, CheckersGame},
    checkers_move::{CheckersMove, Square},
};

const WIN: i32 = 100_000;

pub struct CheckersBot;

impl CheckersBot {
    pub fn new() -> Self {
        CheckersBot
    }

    pub fn get_best_move(
        &mut self,
        position: &CheckersGame,
        maximizing_player: bool,
        time_limit: Duration,
    ) -> CheckersMove {
        let deadline: Instant = Instant::now() + time_limit;
        let mut position: CheckersGame = *position;

        let (mut legal_moves, _) = position.get_legal_moves();
        if legal_moves.is_empty() {
            return CheckersMove::new();
        }
        let mut best_move = legal_moves[0];
        let mut best_eval;
        let mut best_index: usize = 0;

        let mut depth: u64 = 1;

        loop {
            best_eval = if maximizing_player {
                i32::MIN
            } else {
                i32::MAX
            };
            for i in 0..legal_moves.len() {
                let mut position = position;
                position.make_move(&legal_moves[i]);
                let eval = self.minimax(
                    &position,
                    depth,
                    0,
                    i32::MIN,
                    i32::MAX,
                    deadline,
                );

                if Instant::now() >= deadline {
                    break;
                }

                if (maximizing_player && (eval >= best_eval))
                    || (!maximizing_player && (eval <= best_eval))
                {
                    best_move = legal_moves[i];
                    best_eval = eval;
                    best_index = i;
                }
            }

            if Instant::now() > deadline {
                break;
            }

            if let Ok(best_move @ 1..) = usize::try_from(best_index)
                && let Some(to_rotate) = legal_moves.get_mut(0..=best_move)
            {
                to_rotate.rotate_right(1);
                best_index = 0;
            }

            depth += 1;
        }

        println!("bestmove {:?} eval {} depth {}", best_move, best_eval, depth);

        best_move
    }

    fn minimax(
        &mut self,
        position: &CheckersGame,
        depth: u64,
        inverse_depth: u64,
        alpha: i32,
        beta: i32,
        deadline: Instant,
    ) -> i32 {
        let mut position: CheckersGame = *position;
        let maximizing_player = position.turn == CheckerColor::Black;
        let depth = depth;

        let mut alpha: i32 = alpha;
        let mut beta: i32 = beta;
        let mut depth = depth;

        let (legal_moves, capture) = position.get_legal_moves();

        if capture && (depth == 0) {
            depth += 1;
        }

        if legal_moves.is_empty() {
            return if maximizing_player {
                -WIN + inverse_depth as i32
            } else {
                WIN - inverse_depth as i32
            };
        }

        if depth == 0 {
            return CheckersBot::evaluate(&position);
        }

        let mut best_eval = if maximizing_player {
            i32::MIN
        } else {
            i32::MAX
        };

        for i in 0..legal_moves.len() {
            let mut position = position;
            position.make_move(&legal_moves[i]);
            let eval = self.minimax(
                &position,
                depth - 1,
                inverse_depth + 1,
                alpha,
                beta,
                deadline,
            );

            if Instant::now() >= deadline {
                return 0;
            }

            if (maximizing_player && (eval > best_eval))
                || (!maximizing_player && (eval < best_eval))
            {
                best_eval = eval;
            }
            alpha = if maximizing_player {
                i32::max(alpha, best_eval)
            } else {
                alpha
            };
            beta = if !maximizing_player {
                i32::min(beta, best_eval)
            } else {
                beta
            };

            if beta <= alpha {
                break;
            }
        }

        best_eval
    }

    fn evaluate(position: &CheckersGame) -> i32 {
        let mut eval: i32 = 0;

        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                let checker = position.get_checker(Square { x, y });
                if !checker.occupied {
                    continue;
                }

                eval += match (checker.color, checker.rank) {
                    (CheckerColor::Black, CheckerRank::Soldier) => 10,
                    (CheckerColor::Black, CheckerRank::King) => 25,
                    (CheckerColor::Red, CheckerRank::Soldier) => -10,
                    (CheckerColor::Red, CheckerRank::King) => -25,
                }
            }
        }

        eval
    }
}
