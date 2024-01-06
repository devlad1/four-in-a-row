mod ai;

pub mod random;
pub mod alphabeta;

use super::Game;

#[derive(Clone)]
pub struct Ai {
    pub move_getter: fn(&Game) -> usize,
}

impl Ai {
    fn is_legal(game: &Game, i: usize) -> bool {
        !(game.board.get_empty_y_coord(i) == None)
    }
}
