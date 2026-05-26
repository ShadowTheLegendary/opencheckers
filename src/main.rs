use crate::checkers::checkers_game::CheckersGame;
use crate::checkers::checkers_move::{CheckersMove, Square};
pub mod checkers;

fn main() {
    let mut game: CheckersGame = CheckersGame::new();
    game.setup();
    let mut str = game.to_str();
    let mut moves = game.get_legal_moves();
    println!("{str}");
    println!("{moves:#?}");
    game.make_move(&CheckersMove{start: Square{x: 1, y: 2}, end: Square{x: 0, y: 3}, capture: false, coronation: false});
    moves = game.get_legal_moves();
    str = game.to_str();
    println!("{str}");
    println!("{moves:#?}");
    game.make_move(&CheckersMove{start: Square{x: 2, y: 5}, end: Square{x: 1, y: 4}, capture: false, coronation: false});
    moves = game.get_legal_moves();
    str = game.to_str();
    println!("{str}");
    println!("{moves:#?}");
}