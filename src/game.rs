use enum_map::{enum_map, Enum, EnumMap};
use wasm_bindgen::prelude::*;

#[allow(unused_imports)]
use crate::{game::ai::random::get_random_ai, game::ai::alphabeta::get_alpha_beta, log};

use self::{board::Square, player::Player};

mod ai;
mod board;
mod player;

pub const WIN_LENGTH: usize = 4;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Enum)]
pub enum Turn {
    P1 = 1,
    P2 = 2,
}

impl Turn {
    fn next(self) -> Turn {
        match self {
            Turn::P1 => Turn::P2,
            Turn::P2 => Turn::P1,
        }
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Enum)]
pub enum GameState {
    Ongoing = 0,
    Player1Won = 1,
    Player2Won = 2,
    Draw = 3,
}

impl GameState {
    pub fn is_game_over(&self) -> bool {
        self != &GameState::Ongoing
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Game {
    board: board::Board,
    curr_turn: Turn,
    player_map: EnumMap<Turn, Player>,
    game_state: GameState,
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Game {
        Game {
            board: board::Board::new(),
            game_state: GameState::Ongoing,
            curr_turn: Turn::P1,
            player_map: enum_map! {
                // Turn::P1 => Player::Computer(get_random_ai()),
                Turn::P2 => Player::Computer(get_alpha_beta()),
                Turn::P1 => Player::Human,
                // Turn::P2 => Player::Human,
            },
        }
    }

    pub fn make_move(&mut self, i: usize) {
        if self.game_state.is_game_over() {
            return;
        }

        let square_state = Game::turn_to_square(self.curr_turn);
        self.make_move_from_x_coord(i, square_state);
    }

    pub fn make_computer_move(&mut self) {
        if let Player::Computer(ai) = &self.player_map[self.curr_turn] {
            if self.game_state.is_game_over() {
                return;
            }

            log!("{:?}", self.clone().board);

            let x = self.clone().get_computer_move(ai);
            log!("{}", x);
            let y = self.board.get_empty_y_coord(x).unwrap();
            log!("test2");
            self.place_piece(x, y, Game::turn_to_square(self.curr_turn));
            log!("test3");
        } else {
            log!("Tried making a computer move but it wasn't a computer player");
            panic!("Illegal state");
        }
    }

    pub fn is_human_move(&self) -> bool {
        self.player_map[self.curr_turn].is_human()
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

    pub fn game_state(&self) -> GameState {
        self.game_state
    }

    pub fn current_player(&self) -> Turn {
        self.curr_turn
    }

    fn place_piece(&mut self, i: usize, j: usize, square_state: Square) {
        self.board.set(i, j, square_state);

        let game_state = self.check_game_over(i, j);
        if game_state.is_game_over() {
            log!("Game over");
            self.game_state = game_state;
            return;
        }

        self.curr_turn = self.curr_turn.next();
    }

    fn get_computer_move(&mut self, ai: &ai::Ai) -> usize {
        (ai.move_getter)(self)
    }
    
    fn check_game_over(&self, last_move_x: usize, last_move_y: usize) -> GameState {
        let last_square = self.board.get(last_move_x, last_move_y);

        if self.is_row_finished(last_move_x, last_move_y, last_square)
            || self.is_column_finished(last_move_x, last_move_y, last_square)
            || self.is_up_right_diagonal_finished(last_move_x, last_move_y, last_square)
            || self.is_down_right_diagonal_finished(last_move_x, last_move_y, last_square)
        {
            match last_square {
                Square::P1 => return GameState::Player1Won,
                Square::P2 => return GameState::Player2Won,
                _ => {
                    log!("got {:?} square when game ended", last_square);
                    panic!("Illegal state")
                }
            }
        }

        if self.board.is_full() {
            return GameState::Draw;
        }

        GameState::Ongoing
    }

    fn is_row_finished(
        &self,
        last_move_x: usize,
        last_move_y: usize,
        last_square: board::Square,
    ) -> bool {
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

    fn is_column_finished(
        &self,
        last_move_x: usize,
        last_move_y: usize,
        last_square: board::Square,
    ) -> bool {
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

    fn is_up_right_diagonal_finished(
        &self,
        last_move_x: usize,
        last_move_y: usize,
        last_square: board::Square,
    ) -> bool {
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

    fn is_down_right_diagonal_finished(
        &self,
        last_move_x: usize,
        last_move_y: usize,
        last_square: board::Square,
    ) -> bool {
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
}

impl Game {
    pub fn turn_to_square(turn: Turn) -> board::Square {
        match turn {
            Turn::P1 => board::Square::P1,
            Turn::P2 => board::Square::P2,
        }
    }

    pub fn make_move_from_x_coord(&mut self, i: usize, square_state: Square) {
        let y = self.board.get_empty_y_coord(i);
        match y {
            Some(j) => {
                self.place_piece(i, j, square_state);
            }
            None => {}
        }
    }
}
