use std::time::{Duration, Instant};

use crate::checkers::{checkers_bot::transposition_table::TTFlag, checkers_game::{BOARD_HEIGHT, BOARD_WIDTH, CheckerColor, CheckerRank, CheckersGame}, checkers_move::{CheckersMove, Square}};
use crate::checkers::checkers_bot::transposition_table::TranspositionTable;
use crate::checkers::checkers_bot::transposition_table::ZobristHashing;

mod transposition_table;

pub struct CheckersBot {
    tt: TranspositionTable,
    zobrist: ZobristHashing
}

impl CheckersBot {
    pub fn new() -> Self {
        CheckersBot {tt: TranspositionTable::new(64), zobrist: ZobristHashing::register() }
    }

    pub fn get_best_move(&mut self, position: &CheckersGame, maximizing_player: bool, time_limit: Duration) -> CheckersMove {
        let deadline: Instant = Instant::now() + time_limit;
        let mut position: CheckersGame = *position;

        let mut legal_moves = position.get_legal_moves();
        if legal_moves.is_empty() {
            return CheckersMove::new();
        }
        let mut best_move = legal_moves[0];
        let mut best_eval = if maximizing_player { i32::MIN } else { i32::MAX };
        let mut best_index: usize = 0;

        let piece_count = position.get_piece_count();
        let mut depth: u64 = if piece_count > 8 { 5 } else { 10 };

        loop {
            for i in 0..legal_moves.len() {
                let mut position = position;
                position.make_move(&legal_moves[i]);
                let eval = self.minimax(&position, depth, i32::MIN, i32::MAX, !maximizing_player, deadline);

                if Instant::now() > deadline {
                    break;
                }

                if (maximizing_player && (eval > best_eval)) || (!maximizing_player && (eval < best_eval)) {
                    best_move = legal_moves[i];
                    best_eval = eval;
                    best_index = i;
                }
            }

            if Instant::now() > deadline {
                break;
            }

            if let Ok(best_move @ 1..) = usize::try_from(best_index) && 
            let Some(to_rotate) = legal_moves.get_mut(0..best_move) {
                to_rotate.rotate_right(1);
                best_index = 0;
            }

            depth += if piece_count > 8 { 2 } else { 4 };
        }

        best_move
    }

    fn minimax(&mut self, position: &CheckersGame, depth: u64, alpha: i32, beta: i32 , maximizing_player: bool, deadline: Instant) -> i32 {
        let mut position: CheckersGame = *position;
        let depth = depth;
        let original_alpha = alpha;
        let hash = self.zobrist.hash(&position.data());

        let mut alpha: i32 = alpha;
        let mut beta: i32 = beta;

        let option_tt_entry = self.tt.probe(hash);

        if let Some(tt_entry) = option_tt_entry {
            if tt_entry.depth as u64 >= depth {
                match tt_entry.flag {
                    TTFlag::Exact => {
                        return tt_entry.score;
                    }
                    TTFlag::LowerBound => {
                        alpha = i32::max(alpha, tt_entry.score);
                    }
                    TTFlag::UpperBound => {
                        beta  = i32::min(beta,  tt_entry.score);
                    }
                }

                if alpha >= beta {
                    return tt_entry.score;
                }
            }
        }

        let mut legal_moves = position.get_legal_moves();

        if (depth == 0) || legal_moves.is_empty() {
            return CheckersBot::evaluate(&position);
        }

        if let Some(tt_entry) = option_tt_entry
        // required to be >= 1 for rotate_right
        && let Ok(best_move @ 1..) = usize::try_from(tt_entry.best_move) 
        && let Some(to_rotate) = legal_moves.get_mut(0..best_move) {
            to_rotate.rotate_right(1);
        }

        let mut best_eval = if maximizing_player { i32::MIN } else { i32::MAX };
        let mut best_move: u8 = 0;

        for i in 0..legal_moves.len() {
            let mut position = position;
            position.make_move(&legal_moves[i]);
            let eval = self.minimax(&position, depth - 1, alpha, beta, !maximizing_player, deadline);

            if Instant::now() >= deadline {
                return 0;
            }

            if (maximizing_player && (eval > best_eval)) || (!maximizing_player && (eval < best_eval)) {
                best_eval = eval;
                best_move = i as u8;
            }
            alpha = if maximizing_player { i32::max(alpha, best_eval) } else { alpha };
            beta = if !maximizing_player { i32::min(beta, best_eval) } else { beta };

            if beta <= alpha {
                break;
            }
        }

        let flag: TTFlag = if best_eval <= original_alpha { TTFlag::UpperBound } else if best_eval >= beta { TTFlag::LowerBound } else { TTFlag::Exact };

        self.tt.store(hash, best_eval, depth as u8, flag, best_move);

        best_eval
    }

    fn evaluate(position: &CheckersGame) -> i32 {
        if let Some(winner) = position.winner {
            return match winner {
                CheckerColor::Black => i32::MAX,
                CheckerColor::Red => i32::MIN,
            }
        }

        let mut eval: i32 = 0;

        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                let checker = position.get_checker(Square{x, y});
                if !checker.occupied {
                    continue;
                }

                eval += match (checker.color, checker.rank) {
                    (CheckerColor::Black, CheckerRank::Soldier) => 10,
                    (CheckerColor::Black, CheckerRank::King) => 25,
                    (CheckerColor::Red, CheckerRank::Soldier) => -10,
                    (CheckerColor::Red, CheckerRank::King) => -25
                }
            }
        }

        eval
    }
}