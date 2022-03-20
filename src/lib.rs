mod utils;

use fixedbitset::FixedBitSet;
use js_sys;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
pub struct Universe {
    width: u32, 
    height: u32, 
    cells: FixedBitSet, 
}


impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row*self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        
        // allows looping from edge to edge; no if condition needed to check edges; 
        // no underflow for unsigned int.
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue; // itself
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);

                count += self.cells[idx] as u8;
            }
        }
        count
    }
}


#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for column in 0..self.width {
                let idx = self.get_index(row, column);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, column);

                next.set(idx, match (cell, live_neighbors) {
                    (true, x) if x < 2 => false, 
                    (true, 2) | (true, 3) => true, 
                    (true, x) if x > 3 => false, 
                    (false, 3) => true, 
                    (same_state, _) => same_state,
                });
            }
        }

        self.cells = next;
    }
}


#[wasm_bindgen]
impl Universe { // constructor
    pub fn new() -> Universe {
        let width = 64;
        let height = 64;
        let size = (width*height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);

        // initial state
        for i in 0..size {
            cells.set(i, i%2==0 || i%7==0);
        }

        Universe { 
            width, 
            height, 
            cells, 
        }
    }

    pub fn random() -> Universe {
        let width = 64;
        let height = 64;
        let size = (width*height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);

        // initial state
        for i in 0..size {
            cells.set(i, js_sys::Math::random() < 0.5);
        }

        Universe { 
            width, 
            height, 
            cells, 
        }
    }

    pub fn glider() -> Universe {
        let width = 64;
        let height = 64;
        let size = (width*height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);

        // initial state
        let glider_idx = (js_sys::Math::random() * ((size) as f64)).floor() as u32;
        for i in 0..size {
            cells.set(i, match i {
                x if x == ((glider_idx+1) as usize % size) => true, 
                x if x == (((glider_idx+2) + width) as usize % size) => true, 
                x if x == (((glider_idx+0) + 2*width) as usize % size) => true, 
                x if x == (((glider_idx+1) + 2*width) as usize % size) => true, 
                x if x == (((glider_idx+2) + 2*width) as usize % size) => true, 
                _ => false,
            });
        }

        Universe { 
            width, 
            height, 
            cells, 
        }
    }

    pub fn middleweight_spaceship() -> Universe {
        let width = 64;
        let height = 64;
        let size = (width*height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);

        // initial state
        let glider_idx = (js_sys::Math::random() * (size as f64)).floor() as u32;
        for i in 0..size {
            cells.set(i, match i {
                x if x == ((glider_idx+1) as usize % size) => true, 
                x if x == ((glider_idx+2) as usize % size) => true, 
                x if x == ((glider_idx+3) as usize % size) => true, 
                x if x == ((glider_idx+4) as usize % size) => true, 
                x if x == ((glider_idx+5) as usize % size) => true, 
                x if x == (((glider_idx) + width) as usize % size) => true, 
                x if x == (((glider_idx+5) + width) as usize % size) => true, 
                x if x == (((glider_idx+5) + 2*width) as usize % size) => true, 
                x if x == (((glider_idx) + 3*width) as usize % size) => true, 
                x if x == (((glider_idx+4) + 3*width) as usize % size) => true, 
                x if x == (((glider_idx+2) + 4*width) as usize % size) => true, 
                _ => false,
            });
        }

        Universe { 
            width, 
            height, 
            cells, 
        }
    }
}


#[wasm_bindgen]
impl Universe {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }
}


#[wasm_bindgen]
impl Universe {
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        for i in 0..self.width*self.height {
            self.cells.set(i as usize, false);
        }
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        for i in 0..self.width*self.height {
            self.cells.set(i as usize, false);
        }
    }
}


impl Universe {
    pub fn get_cells(&self) -> &[u32] {
        &self.cells.as_slice()
    }

    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells.set(idx, true);
        }
    }
}
