mod ai;

pub mod random;

use super::Game;

pub struct Ai {
    pub move_getter: fn(&Game) -> usize,
}

impl Ai {
    fn is_legal(game: &Game, i: usize) -> bool {
        !(game.board.get_empty_y_coord(i) == None)
    }
}
