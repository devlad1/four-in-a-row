use web_sys::js_sys::Math::floor;

use web_sys::js_sys::Math::random;

use crate::game::Game;

use super::Ai;

#[allow(unused)]
pub fn get_random_ai() -> Ai {
    Ai {
        move_getter: |game: &mut Game| {
            let mut i = floor((random() * (game.board_width() as f64)) + 1.0) as usize;
            while !Ai::is_legal(game, i) {
                i = floor((random() * (game.board_width() as f64)) + 1.0) as usize;
            }
            i
        },
    }
}
