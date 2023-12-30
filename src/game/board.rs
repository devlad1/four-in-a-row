use wasm_bindgen::prelude::*;

pub const BOARD_WIDTH: usize = 7;
pub const BOARD_HEIGHT: usize = 6;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SquareState {
    Empty = 0,
    P1 = 1,
    P2 = 2,
}

#[wasm_bindgen]
pub struct Board {
    width: usize,
    height: usize,
    squares: Vec<SquareState>,
}

#[wasm_bindgen]
pub struct Square {
    x: usize,
    y: usize,
}

#[wasm_bindgen]
impl Board {
    pub fn new() -> Board {
        let width = BOARD_WIDTH;
        let height = BOARD_HEIGHT;
        let squares: Vec<SquareState> = vec![SquareState::Empty; width * height];

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

    pub fn squares(&self) -> *const SquareState {
        self.squares.as_ptr()
    }

    pub fn set(&mut self, i: usize, square: SquareState) -> Option<Square> {
        if i > self.width || i <= 0  {
            return None;
        }

        let mut j = 1;
        while self.squares[self.get_flat_index(i, j)] != SquareState::Empty {
            j += 1;

            if j > self.height {
                return None
            }
        }

        let index = self.get_flat_index(i, j);
        self.squares[index] = square;
        
        Some(Square { x: i, y: j })
    }

    pub fn get(&self, i: usize, j: usize) -> SquareState {
        let index = self.get_flat_index(i, j);
        self.squares.get(index).unwrap().clone()
    }

    fn get_flat_index(&self, i: usize, j: usize) -> usize {
        (self.height - j as usize) * self.width + (i as usize - 1)
    }
}
