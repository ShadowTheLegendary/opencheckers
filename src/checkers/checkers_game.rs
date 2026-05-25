use crate::checkers::checkers_move::BLACK_CORONATION_Y;
use crate::checkers::checkers_move::RED_CORONATION_Y;

use crate::checkers::checkers_move::CheckersMove;
use crate::checkers::checkers_move::Square;

pub const BOARD_HEIGHT: usize = 8;
pub const BOARD_WIDTH: usize = 8;

pub const BLANK_CHECKER: Checker = Checker{occupied: false, color: CheckerColor::Black, rank: CheckerRank::Soldier};

#[derive(Clone, Copy, PartialEq)]
pub enum CheckerColor {
    Black,
    Red
}

#[derive(Clone, Copy, PartialEq)]
pub enum CheckerRank {
    Soldier,
    King
}

#[derive(Clone, Copy)]
pub struct Checker {
    pub occupied: bool,
    pub color: CheckerColor,
    pub rank: CheckerRank
}

impl Checker {
    fn make_checker(color: CheckerColor, rank: CheckerRank) -> Self {
        Checker{ occupied: true, color, rank }
    }

    fn check_move(mut potential_move: CheckersMove, game: &CheckersGame) -> (CheckersMove, bool) {
        let start_sq: Square = potential_move.start;
        let start: Checker = game.get_checker(start_sq);

        let mut end_sq: Square = potential_move.end;
        let end: Checker = game.get_checker(end_sq);

        let mut valid: bool;

        if !end.occupied {
            valid = true;
        } else if start.color != end.color { // potential capture
            let next_x: usize = end_sq.x + (end_sq.x - start_sq.x);
            let next_y: usize = end_sq.y + (end_sq.y - start_sq.y);
            let new_sq: Square = Square{ x: next_x, y: next_y };

            if !CheckersGame::in_bounds(new_sq) || game.get_checker(new_sq).occupied {
                valid = false;
            } else {
                potential_move.capture = true;
                potential_move.end = new_sq;
                
                end_sq = new_sq;

                valid = true;
            }
        } else { // end is occupied and it's one of our own
            valid = false;
        }

        if (start.color == CheckerColor::Red && end_sq.y == RED_CORONATION_Y) || (start.color == CheckerColor::Black && end_sq.y == BLACK_CORONATION_Y) {
            potential_move.coronation = true;
        }

        (potential_move, valid)
    }

    fn get_moves(&self, game: &CheckersGame) -> Vec<CheckersMove> {
        
    }
}

pub struct CheckersGame {
    data: [Checker; 64],
    forced_move: Option<CheckersMove>
}

impl CheckersGame {
    pub fn new() -> Self {
        CheckersGame{ 
            data: [BLANK_CHECKER; 64],
            forced_move: None
        }
    }

    pub fn setup(&mut self) {
        let mut tick: bool = false;
        let mut count: i32 = 1;

        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                if !tick {
                    self.set_checker(Square{ x, y }, BLANK_CHECKER);
                    tick = true;
                    continue;
                }

                if count >= 1 && count <= 12 {
                    self.set_checker(Square{ x, y }, Checker::make_checker(CheckerColor::Black, CheckerRank::Soldier));
                } else if count >= 21 && count <= 32 {
                    self.set_checker(Square{ x, y }, Checker::make_checker(CheckerColor::Red, CheckerRank::Soldier));
                } else {
                    self.set_checker(Square{ x, y }, BLANK_CHECKER);
                }

                count += 1;
                tick = false;
            }

            tick = !tick;
        }
    }

    pub fn to_str(&self) -> String {
        let mut output: String = String::new();
        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                let checker: Checker = self.get_checker(Square{ x, y });
                if !checker.occupied {
                    output += "- ";
                } else {
                    output += match (checker.color, checker.rank) {
                        (CheckerColor::Black, CheckerRank::Soldier) => "b ",
                        (CheckerColor::Black, CheckerRank::King) => "B ",
                        (CheckerColor::Red, CheckerRank::Soldier) => "r ",
                        (CheckerColor::Red, CheckerRank::King) => "R "
                    };
                }
            }
            output += "\n";
        }

        output
    }

    pub fn get_checker(&self, square: Square) -> Checker {
        self.data[square.y * BOARD_HEIGHT + square.x]
    }

    fn set_checker(&mut self, square: Square, checker: Checker) {
        self.data[square.y * BOARD_HEIGHT + square.x] = checker;
    }

    fn in_bounds(sq: Square) -> bool {
        (sq.x < BOARD_WIDTH) && (sq.x >= 0) && (sq.y < BOARD_HEIGHT) && (sq.y >= 0)
    }
}