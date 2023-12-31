use wasm_bindgen::prelude::*;

#[allow(unused_imports)]
use crate::log;

pub const BOARD_WIDTH: usize = 7;
pub const BOARD_HEIGHT: usize = 6;
// pub const BOARD_WIDTH: usize = 2;
// pub const BOARD_HEIGHT: usize = 2;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Square {
    Empty = 0,
    P1 = 1,
    P2 = 2,
}

impl Square {
    pub fn is_empty(&self) -> bool {
        self == &Square::Empty
    }
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Board {
    width: usize,
    height: usize,
    squares: Vec<Square>,
    num_pieces: usize,
}

#[wasm_bindgen]
impl Board {
    pub fn new() -> Board {
        let width = BOARD_WIDTH;
        let height = BOARD_HEIGHT;
        let squares: Vec<Square> = vec![Square::Empty; width * height];

        Board {
            width,
            height,
            squares,
            num_pieces: 0,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn squares(&self) -> *const Square {
        self.squares.as_ptr()
    }

    pub fn get_empty_y_coord(&self, x: usize) -> Option<usize> {
        if x > self.width || x <= 0  {
            return None;
        }

        let mut y = 1;
        while self.squares[self.get_flat_index(x, y)] != Square::Empty {
            y += 1;

            if y > self.height {
                return None;
            }
        }

        Some(y)
    }

    pub fn set(&mut self, i: usize, j: usize, square: Square) {
        if i > self.width || i <= 0 || j > self.height || j <= 0 {
            return;
        }

        let index = self.get_flat_index(i, j);

        if !square.is_empty() && self.squares[index].is_empty() {
            self.num_pieces += 1;
        } else if square.is_empty() && !self.squares[index].is_empty() {
            self.num_pieces -= 1;
        }

        self.squares[index] = square;
    }

    pub fn get(&self, i: usize, j: usize) -> Square {
        let index = self.get_flat_index(i, j);
        self.squares.get(index).unwrap().clone()
    }

    fn get_flat_index(&self, i: usize, j: usize) -> usize {
        (self.height - j as usize) * self.width + (i as usize - 1)
    }
}

impl Board{
    pub fn is_full(&self) -> bool {
        self.num_pieces == self.width * self.height
    }
}
