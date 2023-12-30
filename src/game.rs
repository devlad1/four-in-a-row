use wasm_bindgen::prelude::*;

mod board;

const WIN_LENGTH: usize = 4;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Turn {
    P1 = 1,
    P2 = 2
}

#[wasm_bindgen]
pub struct Game {
    board: board::Board,
    curr_turn: Turn
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Game {        
        Game {
            board: board::Board::new(),
            curr_turn: Turn::P1,
        }
    }

    pub fn make_move(&mut self, i: usize) {
        let square_state = Game::turn_to_square(self.curr_turn);
        let set_result: Option<board::Square> = self.board.set(i, square_state);

        match set_result {
            Some(square) => {

                
                if self.curr_turn == Turn::P1 {
                    self.curr_turn = Turn::P2
                } else {
                    self.curr_turn = Turn::P1
                }
            }
            None => {}
        }
    }

    pub fn is_human_move(&self) -> bool {
        true
    }

    pub fn board_width(&self) -> usize {
        self.board.width()
    }

    pub fn board_height(&self) -> usize {
        self.board.height()
    }

    pub fn squares(&self) -> *const board::SquareState {
        self.board.squares()
    }

    fn is_game_over(square: &board::Square) -> Option<Turn> {     

        Option::None
    }

    fn turn_to_square(turn: Turn) -> board::SquareState {
        match turn {
            Turn::P1 => board::SquareState::P1,
            Turn::P2 => board::SquareState::P2,
        }
    }
}