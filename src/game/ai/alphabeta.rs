use crate::{game::{self, board::Square, Game}, log};

use super::Ai;

use minimax_alpha_beta::strategy::{game_strategy::GameStrategy, alpha_beta_minimax::AlphaBetaMiniMaxStrategy};

type ConnectFourMove = usize;

impl GameStrategy for Game {
    type Player = game::Turn;
    type Move = ConnectFourMove;
    type Board = game::board::Board;

    fn evaluate(&self) -> f64 {
        log!("test evaluate");

        if self.is_game_tied() {
            return 0.
        } else if let Some(player) = self.get_winner() {
            if player == self.curr_turn {
                return f64::INFINITY
            } else {
                return f64::NEG_INFINITY
            }
        }

        let player_square: Square = Game::turn_to_square(self.curr_turn);
        let mut score: f64 = 0.0;

        for i in 0..self.board_width() {
            for j in 0..self.board_height() {
                for k in -1..=1 {
                    for n in -1..=1 {
                        let x_index = i as i32 + k;
                        let y_index = j as i32 + n;

                        if x_index < 0
                            || x_index >= (self.board_width() as i32)
                            || y_index < 0
                            || y_index >= (self.board_height() as i32)
                        {
                            continue;
                        }

                        if self.board.get(x_index as usize, y_index as usize) == player_square {
                            score += 1.0
                        }
                    }
                }
            }
        }

        log!("test evaluate");

        score
    }

    fn get_winner(&self) -> Option<Self::Player> {
        match self.game_state {
            game::GameState::Ongoing => None,
            game::GameState::Player1Won => Some(game::Turn::P1),
            game::GameState::Player2Won => Some(game::Turn::P2),
            game::GameState::Draw => None,
        }
    }

    fn is_game_tied(&self) -> bool {
        self.game_state == game::GameState::Draw
    }

    fn is_game_complete(&self) -> bool {
        log!("Checking if game is complete, game state is {:?}", self.game_state);
        log!("Returning {:?}", self.game_state != game::GameState::Ongoing);
        self.game_state != game::GameState::Ongoing
    }

    fn get_available_moves(&self) -> Vec<Self::Move> {
        log!("test get_available_moves");

        let mut available_moves: Vec<Self::Move> = vec![];

        for i in 1..=self.board_width() {
            if self.is_a_valid_move(&i) {
                available_moves.push(i)
            }
        }

        log!("test get_available_moves {:?}", available_moves);

        available_moves
    }

    fn play(&mut self, mv: &Self::Move, _maximizer: bool) {
        log!("test play");
        self.make_move_from_x_coord(*mv, Game::turn_to_square(self.curr_turn));
        log!("test play");
    }

    fn clear(&mut self, mv: &Self::Move) {
        for y in 2..=self.board_height() {
            if self.board.get(*mv, y).is_empty() {
                self.board.set(*mv, y, Square::Empty);
                return;
            }
        }

        self.board.set(*mv, self.board_height(), Square::Empty);
    }

    fn get_board(&self) -> &Self::Board {
        &self.board
    }

    fn is_a_valid_move(&self, mv: &Self::Move) -> bool {
        log!("test is_a_valid_name {} {}", *mv, self.board_height());
        let a = self.board.get(*mv, self.board_height()) == Square::Empty;
        log!("test is_a_valid_name, {:?}", a);

        self.board.get(*mv, self.board_height()) == Square::Empty
    }

    fn get_a_sentinel_move(&self) -> Self::Move {
        self.board_width() + 1
    }
}

pub fn get_alpha_beta() -> Ai {
    Ai {
        move_getter: |game: &mut Game| game.get_best_move(30, true),
    }
}
