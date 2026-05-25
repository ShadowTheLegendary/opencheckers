use crate::checkers::checkers_game::CheckersGame;
pub mod checkers;

fn main() {
    let mut game: CheckersGame = CheckersGame::new();
    game.setup();
    let str = game.to_str();
    println!("{str}");
}