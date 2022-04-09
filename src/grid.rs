use std::fmt::{Display, Formatter};

use wasm_bindgen::prelude::*;

use crate::cell::Cell;
use crate::cell_state::CellState;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Grid {
    cells: Vec<Vec<Cell>>,
    size: (usize, usize),
}

#[wasm_bindgen]
impl Grid {
    pub fn new(size_x: usize, size_y: usize) -> Self {
        let mut cells: Vec<Vec<Cell>> = vec![];
        let mut count = 1;
        for _ in 0..size_x {
            let mut tmp: Vec<Cell> = vec![];
            for _ in 0..size_y {
                let cell = Cell {
                    id: count,
                    state: CellState::Dead,
                };
                tmp.push(cell);
                count += 1;
            }
            cells.push(tmp);
        }

        Self {
            cells,
            size: (size_x, size_y),
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn init(&mut self) {
        self.set_cells_alive(vec![(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)]);
    }

    pub fn spawn_new(&mut self) {
        self.set_cells_alive(vec![(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)]);
    }

    pub fn step_forward(&mut self) {
        let state = self.clone();
        for x in 0..self.size.0 {
            for y in 0..self.size.1 {
                let alive_neighbors = state.alive_neighbors(x, y);
                let mut cell = self.get_mut_cell(x, y);
                if cell.state == CellState::Alive {
                    if alive_neighbors < 2 {
                        cell.state = CellState::Dead;
                    }
                    if alive_neighbors > 3 {
                        cell.state = CellState::Dead;
                    }
                }
                if alive_neighbors == 3 {
                    cell.state = CellState::Alive;
                }
            }
        }
    }

    fn kill_cell(&mut self, x: usize, y: usize) {
        self.update_cell(x, y, CellState::Dead);
    }

    fn revive_cell(&mut self, x: usize, y: usize) {
        self.update_cell(x, y, CellState::Alive);
    }

    fn get_cell(&self, x: usize, y: usize) -> Cell {
        self.cells[x][y]
    }

    fn update_cell(&mut self, x: usize, y: usize, state: CellState) {
        if x < self.size.0 && y < self.size.1 {
            self.cells[x][y].state = state;
        }
    }

    fn get_neighbors(&self, x: usize, y: usize) -> Vec<Cell> {
        let mut neighbors: Vec<Cell> = vec![];
        if x < self.size.0 && y < self.size.1 {
            //left
            if x > 0 {
                neighbors.push(self.cells[x - 1][y]);
            }
            //right
            if x + 1 < self.size.0 {
                neighbors.push(self.cells[x + 1][y]);
            }
            //up
            if y > 0 {
                neighbors.push(self.cells[x][y - 1]);
            }
            //down
            if y + 1 < self.size.1 {
                neighbors.push(self.cells[x][y + 1]);
            }
            //upper left
            if x > 0 && y > 0 {
                neighbors.push(self.cells[x - 1][y - 1]);
            }
            //upper right
            if x + 1 < self.size.0 && y > 0 {
                neighbors.push(self.cells[x + 1][y - 1]);
            }
            //lower left
            if x > 0 && y + 1 < self.size.1 {
                neighbors.push(self.cells[x - 1][y + 1]);
            }
            //lower right
            if x + 1 < self.size.0 && y + 1 < self.size.1 {
                neighbors.push(self.cells[x + 1][y + 1]);
            }
        }

        neighbors
    }

    fn alive_neighbors(&self, x: usize, y: usize) -> usize {
        let neighbors = self.get_neighbors(x, y);
        neighbors
            .iter()
            .filter(|cell| cell.state == CellState::Alive)
            .count()
    }

    fn get_mut_cell(&mut self, x: usize, y: usize) -> &mut Cell {
        let cell = &mut self.cells[x][y];
        cell
    }

    fn set_cells_alive(&mut self, coords: Vec<(usize, usize)>) {
        coords
            .iter()
            .for_each(|coord| self.update_cell(coord.0, coord.1, CellState::Alive));
    }

    fn print(&self) {
        let size = self.size;
        for y in 0..size.1 {
            for x in 0..size.0 {
                if self.get_cell(x, y).state == CellState::Alive {
                    print!("*");
                } else {
                    print!("0");
                }
            }
            println!();
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let size = self.size;
        for y in 0..size.1 {
            for x in 0..size.0 {
                let symbol = if self.get_cell(x, y).state == CellState::Alive {
                    '◻'
                } else {
                    '◼'
                };
                write!(f, "{}", symbol)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step_forward_blinker_test() {
        let mut grid = Grid::new(11, 10);
        grid.set_cells_alive(vec![(1, 1), (2, 1), (3, 1)]);
        grid.step_forward();
        assert_eq!(grid.alive_neighbors(1, 1), 3);
        assert_eq!(grid.alive_neighbors(3, 1), 3);
        assert_eq!(grid.alive_neighbors(2, 3), 1);
    }

    #[test]
    fn alive_neighbors_test() {
        let mut grid = Grid::new(11, 10);
        let coords: Vec<(usize, usize)> = vec![
            (0, 0),
            (0, 1),
            (0, 2),
            (1, 0),
            (1, 1),
            (1, 2),
            (2, 0),
            (2, 1),
            (2, 2),
        ];
        grid.set_cells_alive(coords);
        assert_eq!(grid.alive_neighbors(1, 1), 8);
        grid.update_cell(0, 0, CellState::Dead);
        assert_eq!(grid.alive_neighbors(1, 1), 7);
    }

    #[test]
    fn alive_neighbors_lower_corner_test() {
        let mut grid = Grid::new(10, 10);
        let coords: Vec<(usize, usize)> = vec![(8, 8), (9, 8), (8, 9)];
        grid.set_cells_alive(coords);
        assert_eq!(grid.alive_neighbors(9, 9), 3);
        grid.update_cell(9, 8, CellState::Dead);
        assert_eq!(grid.alive_neighbors(9, 9), 2);
    }

    #[test]
    fn get_neighbors_left_test() {
        let mut grid = Grid::new(10, 10);
        grid.update_cell(1, 0, CellState::Alive);
        let neighbors = grid.get_neighbors(0, 0);
        let cell = neighbors[0];
        assert_eq!(cell.state, CellState::Alive);
    }

    #[test]
    fn get_neighbors_right_test() {
        let mut grid = Grid::new(10, 10);
        grid.update_cell(9, 0, CellState::Alive);
        let neighbors = grid.get_neighbors(8, 0);
        let cell = neighbors[1];
        assert_eq!(cell.state, CellState::Alive);
    }

    #[test]
    fn get_neighbors_up_test() {
        let mut grid = Grid::new(10, 10);
        grid.update_cell(9, 0, CellState::Alive);
        let neighbors = grid.get_neighbors(9, 1);
        let cell = neighbors[1];
        assert_eq!(cell.state, CellState::Alive);
    }

    #[test]
    fn get_neighbors_down_test() {
        let mut grid = Grid::new(10, 10);
        grid.update_cell(1, 1, CellState::Alive);
        let neighbors = grid.get_neighbors(1, 0);
        let cell = neighbors[2];
        assert_eq!(cell.state, CellState::Alive);
    }

    #[test]
    fn get_neighbors_upper_left_test() {
        let mut grid = Grid::new(10, 10);
        grid.update_cell(0, 0, CellState::Alive);
        let neighbors = grid.get_neighbors(1, 1);
        let cell = neighbors[4];
        assert_eq!(cell.state, CellState::Alive);
    }

    #[test]
    fn get_neighbors_upper_right_test() {
        let mut grid = Grid::new(10, 10);
        grid.update_cell(0, 1, CellState::Alive);
        let neighbors = grid.get_neighbors(1, 0);
        let cell = neighbors[3];
        assert_eq!(cell.state, CellState::Alive);
    }

    #[test]
    fn get_neighbors_lower_left_test() {
        let mut grid = Grid::new(10, 10);
        grid.update_cell(0, 1, CellState::Alive);
        let neighbors = grid.get_neighbors(1, 0);
        let cell = neighbors[3];
        assert_eq!(cell.state, CellState::Alive);
    }

    #[test]
    fn get_neighbors_lower_right_test() {
        let mut grid = Grid::new(10, 10);
        grid.update_cell(2, 1, CellState::Alive);
        let neighbors = grid.get_neighbors(1, 0);
        let cell = neighbors[4];
        assert_eq!(cell.state, CellState::Alive);
    }
}
