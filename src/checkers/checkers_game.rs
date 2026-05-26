use crate::checkers::checkers_move::BLACK_CORONATION_Y;
use crate::checkers::checkers_move::RED_CORONATION_Y;

use crate::checkers::checkers_move::CheckersMove;
use crate::checkers::checkers_move::Square;

pub const BOARD_HEIGHT: u64 = 8;
pub const BOARD_WIDTH: u64 = 8;

const RED_MOVEGEN_OFFSETS: [(i32, i32); 4] = [(-1, -1), (1, -1), (0, 0), (0, 0)];
const BLACK_MOVEGEN_OFFSETS: [(i32, i32); 4] = [(1, 1), (-1, 1), (0, 0), (0, 0)];
const KING_MOVEGEN_OFFSETS: [(i32, i32); 4] = [(-1, -1), (1, -1), (1, 1), (-1, 1)];

pub const BLANK_CHECKER: Checker = Checker{occupied: false, color: CheckerColor::Black, rank: CheckerRank::Soldier};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CheckerColor {
    Black,
    Red
}

#[derive(Clone, Copy, PartialEq, Debug)]
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

        let valid: bool = 

        if !end.occupied {
            true
        } else if start.color != end.color { // potential capture
            let next_x: u64 = (end_sq.x as i64 + (end_sq.x as i64 - start_sq.x as i64)) as u64;
            let next_y: u64 = (end_sq.y as i64 + (end_sq.y as i64 - start_sq.y as i64)) as u64;
            let new_sq: Square = Square{ x: next_x, y: next_y };

            if !CheckersGame::in_bounds(new_sq) || game.get_checker(new_sq).occupied {
                false
            } else {
                potential_move.capture = true;
                potential_move.end = new_sq;
                
                end_sq = new_sq;

                true
            }
        } else { // end is occupied and it's one of our own
            false
        };

        if (start.color == CheckerColor::Red && end_sq.y == RED_CORONATION_Y) || (start.color == CheckerColor::Black && end_sq.y == BLACK_CORONATION_Y) {
            potential_move.coronation = true;
        }

        (potential_move, valid)
    }

    fn get_moves(&self, game: &CheckersGame, sq: &Square) -> (Vec<CheckersMove>, bool) {
        let mut moves: Vec<CheckersMove> = Vec::new();
        let mut found_capture: bool = false;
        let checker: Checker = game.get_checker(*sq);

        for i in 0..4 {
            let potential_end =  
            match checker.rank {
                CheckerRank::King => sq.apply(&KING_MOVEGEN_OFFSETS[i]),
                CheckerRank::Soldier => {
                    match checker.color {CheckerColor::Black => sq.apply(&BLACK_MOVEGEN_OFFSETS[i]), CheckerColor::Red => sq.apply(&RED_MOVEGEN_OFFSETS[i])}
                }
            };

            if !CheckersGame::in_bounds(potential_end) {
                continue;
            }

            let (r#move, valid) = Checker::check_move(
                CheckersMove { 
                    start: *sq, 
                    end: potential_end, 
                    capture: false, 
                    coronation: false 
                }, 
                game
            );

            if !valid {
                continue;
            }

            if r#move.capture && !found_capture {
                found_capture = true;
                moves.clear();
            } else if !r#move.capture && found_capture {
                continue;
            }

            moves.push(r#move);
        }

        (moves, found_capture)
    }
}

pub struct CheckersGame {
    data: [Checker; 64],
    pub turn: CheckerColor,
    forced_move: Option<Square>
}

impl CheckersGame {
    pub fn new() -> Self {
        CheckersGame{ 
            data: [BLANK_CHECKER; 64],
            turn: CheckerColor::Black,
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

    pub fn make_move(&mut self, r#move: &CheckersMove) {
        let mut start_checker: Checker = self.get_checker(r#move.start);
        if r#move.coronation {
            start_checker.rank = CheckerRank::King;
        }

        self.set_checker(r#move.start, BLANK_CHECKER);
        self.set_checker(r#move.end, start_checker);
        let mut forced_move: bool = false;

        if r#move.capture {
            let midpoint: Square = Square {
                x: (r#move.start.x + r#move.end.x) / 2,
                y: (r#move.start.y + r#move.end.y) / 2
            };

            self.set_checker(midpoint, BLANK_CHECKER);

            let (_checker_moves, capture) = start_checker.get_moves(self, &r#move.end);
            if capture {
                self.forced_move = Some(r#move.end);
                forced_move = true;
            }
        }

        if !forced_move {
            self.turn = match self.turn { CheckerColor::Black => CheckerColor::Red, CheckerColor::Red => CheckerColor::Black };
            self.forced_move = None;
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

    pub fn get_legal_moves(&self) -> Vec<CheckersMove> {
        let mut moves: Vec<CheckersMove> = Vec::new();
        let mut found_capture: bool = false;

        if self.forced_move == None {
            for y in 0..BOARD_HEIGHT {
                for x in 0..BOARD_WIDTH {
                    let square: Square = Square { x, y };
                    let checker: Checker = self.get_checker(square);
                    if !checker.occupied || checker.color != self.turn {
                        continue;
                    }
                    
                    let (mut checker_moves, capture) = checker.get_moves(self, &square);

                    if checker_moves.is_empty() {
                        continue;
                    }

                    if capture && !found_capture {
                        found_capture = true;
                        moves.clear();
                    } else if !capture && found_capture {
                        continue;
                    }

                    moves.append(&mut checker_moves);
                }
            }
        } else {
            let sq : Square = self.forced_move.unwrap();

            let (checker_moves, _capture) = self.get_checker(sq).get_moves(self, &sq);

            return checker_moves;
        }

        moves
    }

    pub fn get_checker_legal_moves(&self, square: Square) -> Vec<CheckersMove> {
        let moves: Vec<CheckersMove> = self.get_legal_moves();
        let mut filtered_moves: Vec<CheckersMove> = Vec::new();

        for r#move in &moves {
            if r#move.start == square {
                filtered_moves.push(*r#move);
            }
        }

        filtered_moves
    }

    pub fn get_checker(&self, square: Square) -> Checker {
        self.data[(square.y * BOARD_HEIGHT + square.x) as usize]
    }

    fn set_checker(&mut self, square: Square, checker: Checker) {
        self.data[(square.y * BOARD_HEIGHT + square.x) as usize] = checker;
    }

    pub fn in_bounds(sq: Square) -> bool {
        (sq.x < BOARD_WIDTH) && (sq.y < BOARD_HEIGHT)
    }
}