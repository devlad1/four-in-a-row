use wasm_bindgen::prelude::*;

use crate::log;

mod board;

const WIN_LENGTH: usize = 4;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Turn {
    P1 = 1,
    P2 = 2,
}

#[wasm_bindgen]
pub struct Game {
    board: board::Board,
    curr_turn: Turn,
    is_game_over: bool,
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Game {
        Game {
            board: board::Board::new(),
            curr_turn: Turn::P1,
            is_game_over: false,
        }
    }

    pub fn make_move(&mut self, i: usize) {
        if self.is_game_over {
            return
        }

        let square_state = Game::turn_to_square(self.curr_turn);
        let y = self.board.get_empty_y_coord(i);
        match y {
            Some(j) => {
                self.board.set(i, j, square_state);

                if self.check_game_over(i, j) {
                    log!("Game over");
                    self.is_game_over = true;
                }

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

    pub fn squares(&self) -> *const board::Square {
        self.board.squares()
    }

    pub fn is_game_over(&self) -> bool {
        self.is_game_over
    }

    pub fn current_player(&self) -> Turn {
        self.curr_turn
    }

    fn check_game_over(&self, last_move_x: usize, last_move_y: usize) -> bool {
        let last_square = self.board.get(last_move_x, last_move_y);

        self.is_row_finished(last_move_x, last_move_y, last_square)
            || self.is_column_finished(last_move_x, last_move_y, last_square)
            || self.is_up_right_diagonal_finished(last_move_x, last_move_y, last_square)
            || self.is_down_right_diagonal_finished(last_move_x, last_move_y, last_square)
    }

    fn is_row_finished(&self, last_move_x: usize, last_move_y: usize, last_square: board::Square) -> bool {
        let mut streak = 1;
        for i in 1..WIN_LENGTH {
            let x = last_move_x + i;

            if x > self.board_width() {
                break;
            }

            if self.board.get(x, last_move_y) == last_square {
                streak += 1;
            } else {
                break;
            }
        }

        for i in 1..WIN_LENGTH {
            let x = last_move_x - i;

            if x <= 0 {
                break;
            }

            if self.board.get(x, last_move_y) == last_square {
                streak += 1;
            } else {
                break;
            }
        }

        streak >= WIN_LENGTH
    }

    fn is_column_finished(&self, last_move_x: usize, last_move_y: usize, last_square: board::Square) -> bool {
        let mut streak = 1;
        for i in 1..WIN_LENGTH {
            let y = last_move_y + i;

            if y > self.board_height() {
                break;
            }

            if self.board.get(last_move_x, y) == last_square {
                streak += 1;
            } else {
                break;
            }
        }

        for i in 1..WIN_LENGTH {
            let y = last_move_y - i;

            if y <= 0 {
                break;
            }

            if self.board.get(last_move_x, y) == last_square {
                streak += 1;
            } else {
                break;
            }
        }

        streak >= WIN_LENGTH
    }

    fn is_up_right_diagonal_finished(&self, last_move_x: usize, last_move_y: usize, last_square: board::Square) -> bool {
        let mut streak = 1;
        for i in 1..WIN_LENGTH {
            let x = last_move_x + i;
            let y = last_move_y + i;

            if !self.is_legal_coord(x, y) {
                break;
            }

            if self.board.get(x, y) == last_square {
                streak += 1;
            } else {
                break;
            }
        }

        for i in 1..WIN_LENGTH {
            let x = last_move_x - i;
            let y = last_move_y - i;

            if !self.is_legal_coord(x, y) {
                break;
            }

            if self.board.get(x, y) == last_square {
                streak += 1;
            } else {
                break;
            }
        }

        streak >= WIN_LENGTH
    }

    fn is_down_right_diagonal_finished(&self, last_move_x: usize, last_move_y: usize, last_square: board::Square) -> bool {
        let mut streak = 1;
        for i in 1..WIN_LENGTH {
            let x = last_move_x - i;
            let y = last_move_y + i;

            if !self.is_legal_coord(x, y) {
                break;
            }

            if self.board.get(x, y) == last_square {
                streak += 1;
            } else {
                break;
            }
        }

        for i in 1..WIN_LENGTH {
            let x = last_move_x + i;
            let y = last_move_y - i;

            if !self.is_legal_coord(x, y) {
                break;
            }

            if self.board.get(x, y) == last_square {
                streak += 1;
            } else {
                break;
            }
        }

        streak >= WIN_LENGTH
    }

    fn is_legal_coord(&self, i: usize, j: usize) -> bool {
        i > 0 && j > 0 && i <= self.board_width() && j <= self.board_height()
    }

    fn turn_to_square(turn: Turn) -> board::Square {
        match turn {
            Turn::P1 => board::Square::P1,
            Turn::P2 => board::Square::P2,
        }
    }
}
