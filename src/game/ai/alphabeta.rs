use std::{
    collections::HashMap,
    f64::{INFINITY, NEG_INFINITY},
};

use crate::game::{board::Square, Game, GameState, Turn};

use self::alphabeta_algorithm::GameNode;

use super::Ai;

mod alphabeta_algorithm;

impl GameNode<Game, usize> for Game {
    fn evaluate(&self) -> f64 {
        match self.game_state {
            GameState::Ongoing => (),
            GameState::Draw => return 0.,
            GameState::Player1Won => {
                if self.curr_turn == Turn::P1 {
                    return INFINITY;
                } else {
                    return NEG_INFINITY;
                }
            }
            GameState::Player2Won => {
                if self.curr_turn == Turn::P2 {
                    return INFINITY;
                } else {
                    return NEG_INFINITY;
                }
            }
        }
        
        let player_square: Square = Game::turn_to_square(self.curr_turn);
        let mut score: f64 = 0.;

        for i in 0..self.board_width() {
            for j in 0..self.board_height() {
                for k in -1..=1 {
                    for n in -1..=1 {
                        let x_index = i as i32 + k;
                        let y_index = j as i32 + n;

                        if x_index < 1
                            || x_index > (self.board_width() as i32)
                            || y_index < 1
                            || y_index > (self.board_height() as i32)
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
        score
    }

    fn get_children_nodes(&self) -> HashMap<usize, Box<Game>> {
        let mut cloned_games_map: HashMap<usize, Box<Game>> = self
            .get_available_moves()
            .iter()
            .map(|mv| (*mv, Box::new(self.clone())))
            .collect();

        cloned_games_map
            .iter_mut()
            .for_each(|(mv, game)| game.make_move(*mv));

        cloned_games_map
    }
}

impl Game {
    fn is_a_valid_move(&self, mv: usize) -> bool {
        self.board.get(mv, self.board_height()) == Square::Empty
    }

    fn get_available_moves(&self) -> Vec<usize> {
        let mut available_moves: Vec<usize> = vec![];

        for i in 1..=self.board_width() {
            if self.is_a_valid_move(i) {
                available_moves.push(i)
            }
        }

        available_moves
    }
}

#[allow(unused)]
pub fn get_alphabeta_ai() -> Ai {
    Ai {
        move_getter: |game: &Game| alphabeta_algorithm::get_best_move(game, 20).unwrap(),
    }
}
