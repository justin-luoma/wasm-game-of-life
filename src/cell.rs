use crate::cell_state::CellState;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Cell {
    pub id: u32,
    pub state: CellState,
}
