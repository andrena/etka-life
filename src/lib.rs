#![feature(stmt_expr_attributes)]
use std::ops::{Index, IndexMut};

use wasm_bindgen::prelude::wasm_bindgen;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl Cell {
    fn is_alive(&self) -> bool {
        *self as u8 == 1
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Universe {
    fn idx(&self, (row, column): (u32, u32)) -> usize {
        (row * self.width + column) as usize
    }

    fn neighbourhood(&self, row: u32, col: u32) -> (Cell, [Cell; 8]) {
        #![allow(warnings)]
        let last_row = self.height - 1;
        let last_col = self.width - 1;
        let N = if row == 0 { last_row } else { row - 1 };
        let S = if row == last_row { 0 } else { row + 1 };
        let W = if col == 0 { last_col } else { col - 1 };
        let E = if col == last_col { 0 } else { col + 1 };

        let cell = self[(row, col)];
        let neighbour_cells = #[rustfmt::skip] {
      [
      self[(N, W)], self[(N, col)], self[(N, E)],
      self[(row, W)],                self[(row, E)],
      self[(S, W)], self[(S, col)], self[(S, E)],
      ]
    };
        (cell, neighbour_cells)
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        use Cell::*;
        let width = 128;
        let height = 128;

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Alive
                } else {
                    Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn tick(&mut self) {
        use Cell::*;
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let (cell, neighbours) = self.neighbourhood(row, col);
                let live_neighbors = neighbours
                    .into_iter()
                    .filter(|cell| cell.is_alive())
                    .count();

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours dies, as if caused by underpopulation.
                    (Alive, x) if x < 2 => Dead,
                    // Rule 2: Any live cell with two or three live neighbours lives on to the next generation.
                    (Alive, 2) | (Alive, 3) => Alive,
                    // Rule 3: Any live cell with more than three live neighbours dies, as if by overpopulation.
                    (Alive, x) if x > 3 => Dead,
                    // Rule 4: Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
                    (Dead, 3) => Alive,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                let idx = self.idx((row, col));
                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }
}

impl Index<(u32, u32)> for Universe {
    type Output = Cell;

    fn index(&self, index: (u32, u32)) -> &Cell {
        let index = self.idx(index);
        &self.cells[index]
    }
}

impl IndexMut<(u32, u32)> for Universe {
    fn index_mut(&mut self, index: (u32, u32)) -> &mut Cell {
        let index = self.idx(index);
        &mut self.cells[index]
    }
}
