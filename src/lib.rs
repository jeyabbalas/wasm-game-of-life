mod utils;

use fixedbitset::FixedBitSet;
use js_sys;
use wasm_bindgen::prelude::*;
use web_sys;


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


pub struct Timer<'a> {
    name: &'a str,
}


impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer<'a> {
        web_sys::console::time_with_label(name);
        Timer { name }
    }
}


impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        web_sys::console::time_end_with_label(self.name);
    }
}


macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    };
}


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

        let north = if row == 0 {
            self.height - 1
        } else {
            row - 1
        };

        let south = if row == self.height - 1 {
            0
        } else {
            row + 1
        };

        let east = if column == self.width - 1 {
            0
        } else {
            column + 1
        };

        let west = if column == 0 {
            self.width - 1
        } else {
            column - 1
        };

        let nw = self.get_index(north, west);
        count += self.cells[nw] as u8;

        let n = self.get_index(north, column);
        count += self.cells[n] as u8;

        let ne = self.get_index(north, east);
        count += self.cells[ne] as u8;

        let w = self.get_index(row, west);
        count += self.cells[w] as u8;

        let e = self.get_index(row, east);
        count += self.cells[e] as u8;

        let sw = self.get_index(south, west);
        count += self.cells[sw] as u8;

        let s = self.get_index(south, column);
        count += self.cells[s] as u8;

        let se = self.get_index(south, east);
        count += self.cells[se] as u8;

        count
    }
}


#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        let _timer = Timer::new("Universe::tick");

        let mut next = self.cells.clone();

        for row in 0..self.height {
            for column in 0..self.width {
                let idx = self.get_index(row, column);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, column);

                let next_cell = match (cell, live_neighbors) {
                    (true, x) if x < 2 => false, 
                    (true, 2) | (true, 3) => true, 
                    (true, x) if x > 3 => false, 
                    (false, 3) => true, 
                    (same_state, _) => same_state,
                };

                // if cell != next_cell {
                //     let was_state = if cell {"alive"} else {"dead"};
                //     let is_state = if next_cell {"alive"} else {"dead"};
                //     log!("Cell[{row},{column}]: with {live_neighbors} neighbors transitioned state {was_state} -> {is_state}.");
                // }

                next.set(idx, next_cell);
            }
        }

        self.cells = next;
    }
}


#[wasm_bindgen]
impl Universe { // constructor
    pub fn new() -> Universe {
        utils::set_panic_hook();

        let width = 128;
        let height = 128;
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


#[wasm_bindgen]
impl Universe {
    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells.toggle(idx);
    }
}
