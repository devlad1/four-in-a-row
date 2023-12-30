mod utils;

use wasm_bindgen::prelude::*;

const BOARD_WIDTH: usize = 7;
const BOARD_HEIGHT: usize = 6;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Square {
    Empty = 0,
    P1 = 1,
    P2 = 2
}

#[wasm_bindgen]
pub struct Board {
    width: usize,
    height: usize,
    squares: Vec<Square>
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

    pub fn set(&mut self, i: u16, j: u16, square: Square) {
        let index = self.get_flat_index(i, j);
        self.squares[index] = square;
    }

    pub fn get(&self, i: u16, j: u16) -> Square {
        let index = self.get_flat_index(i, j);
        self.squares.get(index).unwrap().clone()
    }

    fn get_flat_index(&self, i: u16, j: u16) -> usize {
        (self.height - j as usize) * self.width + (i  as usize - 1)
    }
}
