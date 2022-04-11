use std::fmt::{Display, Formatter};

use rand::prelude::*;
use wasm_bindgen::prelude::*;

use crate::cell::Cell;
use crate::cell_state::CellState;
use crate::pattern::Pattern;
use crate::random_bool;

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

    pub fn get_cells(&self) -> *const bool {
        let mut flat: Vec<bool> = vec![];
        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                let cell = self.cells[x][y];
                match cell.state {
                    CellState::Alive => flat.push(true),
                    CellState::Dead => flat.push(false),
                }
            }
        }

        flat.as_ptr()
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn init(&mut self) {

        // self.randomize();
        // self.spawn_blinker(25, 15);
        // self.spawn_blinker(35, 5);
        // self.spawn_pentadecanthlon(45, 10);
        // self.spawn_pulsar(60, 35);
    }

    pub fn spawn_pattern(&mut self, pattern: Pattern, x: usize, y: usize) {
        if x >= 0 && y >= 0 && x < self.size.0 && y < self.size.1 {
            match pattern {
                Pattern::Blinker => self.spawn_blinker(x, y),
                Pattern::Toad => self.spawn_toad(x, y),
                Pattern::Beacon => self.spawn_beacon(x, y),
                Pattern::Pulsar => self.spawn_pulsar(x, y),
                Pattern::Pentadecathlon => self.spawn_pentadecanthlon(x, y),
                Pattern::Glider => self.spawn_glider(x, y),
                Pattern::GliderGun => self.spawn_glider_gun(x, y),
                Pattern::Block => self.spawn_block(x, y),
                Pattern::BeeHive => self.spawn_beehive(x, y),
                Pattern::Loaf => self.spawn_loaf(x, y),
                Pattern::Boat => self.spawn_boat(x, y),
                Pattern::LightSpaceship => self.spawn_light_spaceship(x, y),
                Pattern::MiddleSpaceship => self.spawn_middle_spaceship(x, y),
                Pattern::HeavySpaceship => self.spawn_heavy_spaceship(x, y),
                Pattern::RPentomino => self.spawn_r_pentomino(x, y),
                Pattern::DieHard => self.spawn_die_hard(x, y),
                Pattern::Acorn => self.spawn_acorn(x, y),
                Pattern::InfiniteGrowth1 => self.spawn_infinite_growth_1(x, y),
                Pattern::InfiniteGrowth2 => self.spawn_infinite_growth_2(x, y),
                Pattern::Copperhead => self.spawn_copperhead(x, y),
                Pattern::Almosymmetric => self.spawn_almosymmetric(x, y),
                Pattern::GliderLoop => self.spawn_glider_loop(x, y),
                Pattern::FastForwardForceField => self.spawn_fast_forward_force_field(x, y),
            }
        }
    }

    pub fn randomize(&mut self) {
        let mut rng = rand::thread_rng();

        for x in 0..self.size.0 {
            for y in 0..self.size.1 {
                let alive = random_bool(&mut rng);
                match alive {
                    true => self.update_cell(x, y, CellState::Alive),
                    false => self.update_cell(x, y, CellState::Dead),
                }
            }
        }
    }

    pub fn randomize_center(&mut self) {
        let mut rng = rand::thread_rng();

        for x in 0..self.size.0 {
            for y in 0..self.size.1 {
                if y > (self.size.1 / 2 - (self.size.1 / 4))
                    && x < (self.size.0 / 2 + (self.size.0 / 4))
                    && x > (self.size.0 / 2 - (self.size.0 / 4))
                    && y < (self.size.1 / 2 + (self.size.1 / 4))
                {
                    let alive = random_bool(&mut rng);
                    match alive {
                        true => self.update_cell(x, y, CellState::Alive),
                        false => self.update_cell(x, y, CellState::Dead),
                    }
                }
            }
        }
    }

    pub fn reset(&mut self) {
        for x in 0..self.size.0 {
            for y in 0..self.size.1 {
                self.update_cell(x, y, CellState::Dead);
            }
        }
    }

    pub fn spawn_glider(&mut self, x: usize, y: usize) {
        if x - 1 >= 0 && x + 1 < self.size.0 && y - 1 >= 0 && y + 1 < self.size.1 {
            self.set_cells_alive(vec![
                (x, y - 1),
                (x + 1, y),
                (x, y + 1),
                (x - 1, y + 1),
                (x + 1, y + 1),
            ]);
        }
    }

    pub fn spawn_blinker(&mut self, x: usize, y: usize) {
        if x - 1 >= 0 && x + 1 < self.size.0 && y - 1 >= 0 && y + 1 < self.size.1 {
            self.set_cells_alive(vec![(x, y - 1), (x, y), (x, y + 1)]);
        }
    }

    pub fn spawn_block(&mut self, x: usize, y: usize) {
        if x >= 0 && y >= 0 && x + 1 < self.size.0 && y + 1 < self.size.1 {
            self.set_cells_alive(vec![(x, y), (x, y + 1), (x + 1, y), (x + 1, y + 1)]);
        }
    }

    pub fn spawn_beehive(&mut self, x: usize, y: usize) {
        if x + 2 < self.size.0 && y + 3 < self.size.1 {
            self.set_cells_alive(vec![
                (x + 1, y),
                (x + 2, y),
                (x, y + 1),
                (x + 3, y + 1),
                (x + 1, y + 2),
                (x + 2, y + 2),
            ]);
        }
    }

    pub fn spawn_loaf(&mut self, x: usize, y: usize) {
        if x + 3 < self.size.0 && y + 3 < self.size.1 {
            self.set_cells_alive(vec![
                (x + 1, y),
                (x + 2, y),
                (x, y + 1),
                (x + 3, y + 1),
                (x + 1, y + 2),
                (x + 3, y + 2),
                (x + 2, y + 3),
            ]);
        }
    }

    pub fn spawn_boat(&mut self, x: usize, y: usize) {
        if x + 2 < self.size.0 && y + 2 < self.size.1 {
            self.set_cells_alive(vec![
                (x, y),
                (x + 1, y),
                (x, y + 1),
                (x + 2, y + 1),
                (x + 1, y + 2),
            ]);
        }
    }

    pub fn spawn_toad(&mut self, x: usize, y: usize) {
        if x - 1 >= 0 && y - 2 >= 0 && x + 2 < self.size.0 && y + 1 < self.size.1 {
            self.set_cells_alive(vec![
                (x, y),
                (x, y - 1),
                (x - 1, y),
                (x + 1, y),
                (x + 1, y - 1),
                (x + 2, y - 1),
            ]);
        }
    }

    pub fn spawn_beacon(&mut self, x: usize, y: usize) {
        if x - 1 >= 0 && y - 1 >= 0 && x + 2 < self.size.0 && y + 2 < self.size.1 {
            self.set_cells_alive(vec![
                (x, y),
                (x, y - 1),
                (x - 1, y),
                (x - 1, y - 1),
                (x + 1, y + 1),
                (x + 2, y + 1),
                (x + 1, y + 2),
                (x + 2, y + 2),
            ]);
        }
    }

    pub fn spawn_pulsar(&mut self, x: usize, y: usize) {
        if x - 6 >= 0 && x + 6 < self.size.0 && y - 6 >= 0 && y + 6 < self.size.1 {
            self.set_cells_alive(vec![
                //upper left
                (x - 2, y - 1),
                (x - 3, y - 1),
                (x - 4, y - 1),
                (x - 1, y - 2),
                (x - 1, y - 3),
                (x - 1, y - 4),
                (x - 2, y - 6),
                (x - 3, y - 6),
                (x - 4, y - 6),
                (x - 6, y - 2),
                (x - 6, y - 3),
                (x - 6, y - 4),
                //upper right
                (x - 2, y + 1),
                (x - 3, y + 1),
                (x - 4, y + 1),
                (x - 1, y + 2),
                (x - 1, y + 3),
                (x - 1, y + 4),
                (x - 2, y + 6),
                (x - 3, y + 6),
                (x - 4, y + 6),
                (x - 6, y + 2),
                (x - 6, y + 3),
                (x - 6, y + 4),
                //lower left
                (x + 2, y - 1),
                (x + 3, y - 1),
                (x + 4, y - 1),
                (x + 1, y - 2),
                (x + 1, y - 3),
                (x + 1, y - 4),
                (x + 2, y - 6),
                (x + 3, y - 6),
                (x + 4, y - 6),
                (x + 6, y - 2),
                (x + 6, y - 3),
                (x + 6, y - 4),
                //lower right
                (x + 2, y + 1),
                (x + 3, y + 1),
                (x + 4, y + 1),
                (x + 1, y + 2),
                (x + 1, y + 3),
                (x + 1, y + 4),
                (x + 2, y + 6),
                (x + 3, y + 6),
                (x + 4, y + 6),
                (x + 6, y + 2),
                (x + 6, y + 3),
                (x + 6, y + 4),
            ]);
        }
    }

    pub fn spawn_pentadecanthlon(&mut self, x: usize, y: usize) {
        if x - 4 >= 0 && y - 7 >= 0 && x + 4 < self.size.0 && y + 8 < self.size.1 {
            self.set_cells_alive(vec![
                (x, y),
                (x, y - 1),
                (x - 1, y - 2),
                (x + 1, y - 2),
                (x, y - 3),
                (x, y - 4),
                (x, y + 1),
                (x, y + 2),
                (x - 1, y + 3),
                (x + 1, y + 3),
                (x, y + 4),
                (x, y + 5),
            ]);
        }
    }

    pub fn spawn_light_spaceship(&mut self, x: usize, y: usize) {
        if x + 4 < self.size.0 && y + 3 < self.size.1 {
            self.set_cells_alive(vec![
                (x, y),
                (x + 3, y),
                (x + 4, y + 1),
                (x, y + 2),
                (x + 4, y + 2),
                (x + 1, y + 3),
                (x + 2, y + 3),
                (x + 3, y + 3),
                (x + 4, y + 3),
            ]);
        }
    }

    pub fn spawn_middle_spaceship(&mut self, x: usize, y: usize) {
        if x + 5 < self.size.0 && y + 4 < self.size.1 {
            self.set_cells_alive(vec![
                (x + 2, y),
                (x, y + 1),
                (x + 4, y + 1),
                (x + 5, y + 2),
                (x, y + 3),
                (x + 5, y + 3),
                (x + 1, y + 4),
                (x + 2, y + 4),
                (x + 3, y + 4),
                (x + 4, y + 4),
                (x + 5, y + 4),
            ]);
        }
    }

    pub fn spawn_heavy_spaceship(&mut self, x: usize, y: usize) {
        if x + 6 < self.size.0 && y + 4 < self.size.1 {
            self.set_cells_alive(vec![
                (x + 2, y),
                (x + 3, y),
                (x, y + 1),
                (x + 5, y + 1),
                (x + 6, y + 2),
                (x, y + 3),
                (x + 6, y + 3),
                (x + 1, y + 4),
                (x + 2, y + 4),
                (x + 3, y + 4),
                (x + 4, y + 4),
                (x + 5, y + 4),
                (x + 6, y + 4),
            ]);
        }
    }

    pub fn spawn_glider_gun(&mut self, x: usize, y: usize) {
        if x - 18 >= 0 && y - 4 >= 0 && x + 17 < self.size.0 && y + 4 < self.size.1 {
            self.set_cells_alive(vec![
                //left
                (x - 1, y + 1),
                (x - 2, y),
                (x - 2, y + 1),
                (x - 2, y + 2),
                (x - 3, y - 1),
                (x - 3, y + 3),
                (x - 4, y + 1),
                (x - 5, y - 2),
                (x - 5, y + 4),
                (x - 6, y - 2),
                (x - 6, y + 4),
                (x - 7, y - 1),
                (x - 7, y + 3),
                (x - 8, y),
                (x - 8, y + 2),
                (x - 8, y + 1),
                (x - 17, y),
                (x - 18, y),
                (x - 17, y + 1),
                (x - 18, y + 1),
                //right
                (x + 2, y),
                (x + 2, y - 1),
                (x + 2, y - 2),
                (x + 3, y),
                (x + 3, y - 1),
                (x + 3, y - 2),
                (x + 4, y + 1),
                (x + 4, y - 3),
                (x + 6, y + 1),
                (x + 6, y + 2),
                (x + 6, y - 3),
                (x + 6, y - 4),
                (x + 16, y - 1),
                (x + 16, y - 2),
                (x + 17, y - 1),
                (x + 17, y - 2),
            ]);
        }
    }

    pub fn spawn_copperhead(&mut self, x: usize, y: usize) {
        if x + 11 < self.size.0 && y + 7 < self.size.1 {
            self.set_cells_alive(vec![
                (x + 5, y),
                (x + 7, y),
                (x + 8, y),
                (x + 4, y + 1),
                (x + 11, y + 1),
                (x + 3, y + 2),
                (x + 4, y + 2),
                (x + 8, y + 2),
                (x + 11, y + 2),
                (x, y + 3),
                (x + 1, y + 3),
                (x + 3, y + 3),
                (x + 9, y + 3),
                (x + 10, y + 3),
                (x, y + 4),
                (x + 1, y + 4),
                (x + 3, y + 4),
                (x + 9, y + 4),
                (x + 10, y + 4),
                (x + 3, y + 5),
                (x + 4, y + 5),
                (x + 8, y + 5),
                (x + 11, y + 5),
                (x + 4, y + 6),
                (x + 11, y + 6),
                (x + 5, y + 7),
                (x + 7, y + 7),
                (x + 8, y + 7),
            ])
        }
    }

    pub fn spawn_r_pentomino(&mut self, x: usize, y: usize) {
        if x + 2 < self.size.0 && y + 2 < self.size.1 {
            self.set_cells_alive(vec![
                (x + 1, y),
                (x + 2, y),
                (x, y + 1),
                (x + 1, y + 1),
                (x + 1, y + 2),
            ])
        }
    }

    pub fn spawn_die_hard(&mut self, x: usize, y: usize) {
        if x + 7 < self.size.0 && y + 2 < self.size.1 {
            self.set_cells_alive(vec![
                (x + 6, y),
                (x, y + 1),
                (x + 1, y + 1),
                (x + 1, y + 2),
                (x + 5, y + 2),
                (x + 6, y + 2),
            ])
        }
    }

    pub fn spawn_acorn(&mut self, x: usize, y: usize) {
        if x + 6 < self.size.0 && y + 2 < self.size.1 {
            self.set_cells_alive(vec![
                (x + 1, y),
                (x + 3, y + 1),
                (x, y + 2),
                (x + 1, y + 2),
                (x + 4, y + 2),
                (x + 5, y + 2),
                (x + 6, y + 2),
            ])
        }
    }

    pub fn spawn_almosymmetric(&mut self, x: usize, y: usize) {
        if x + 7 < self.size.0 && y + 8 < self.size.1 {
            self.set_cells_alive(vec![
                (x + 3, y),
                (x + 3, y + 1),
                (x + 5, y + 1),
                (x + 1, y + 2),
                (x + 6, y + 3),
                (x + 7, y + 3),
                (x, y + 4),
                (x + 1, y + 4),
                (x + 6, y + 5),
                (x + 2, y + 6),
                (x + 1, y + 7),
                (x + 4, y + 7),
                (x + 6, y + 7),
                (x + 1, y + 8),
                (x + 2, y + 8),
                (x + 5, y + 8),
                (x + 6, y + 8),
            ]);
        }
    }

    pub fn spawn_infinite_growth_1(&mut self, x: usize, y: usize) {
        if x + 7 < self.size.0 && y + 5 < self.size.1 {
            self.set_cells_alive(vec![
                (x + 6, y),
                (x + 4, y + 1),
                (x + 6, y + 1),
                (x + 7, y + 1),
                (x + 4, y + 2),
                (x + 6, y + 2),
                (x + 4, y + 3),
                (x + 2, y + 4),
                (x, y + 5),
                (x + 2, y + 5),
            ])
        }
    }

    pub fn spawn_infinite_growth_2(&mut self, x: usize, y: usize) {
        if x + 4 < self.size.0 && y + 4 < self.size.1 {
            self.set_cells_alive(vec![
                (x, y),
                (x + 1, y),
                (x + 2, y),
                (x + 4, y),
                (x, y + 1),
                (x + 3, y + 2),
                (x + 4, y + 2),
                (x + 1, y + 3),
                (x + 2, y + 3),
                (x + 4, y + 3),
                (x, y + 4),
                (x + 2, y + 4),
                (x + 4, y + 4),
            ])
        }
    }

    pub fn spawn_fast_forward_force_field(&mut self, x: usize, y: usize) {
        if x + 16 < self.size.0 && y + 6 < self.size.1 {
            self.set_cells_alive(vec![
                (x + 7, y),
                (x + 14, y),
                //
                (x + 8, y + 1),
                (x + 15, y + 1),
                (x + 16, y + 1),
                //
                (x + 2, y + 2),
                (x + 3, y + 2),
                (x + 6, y + 2),
                (x + 7, y + 2),
                (x + 8, y + 2),
                (x + 14, y + 2),
                (x + 15, y + 2),
                //
                (x, y + 3),
                (x + 1, y + 3),
                (x + 3, y + 3),
                (x + 4, y + 3),
                //
                (x, y + 4),
                (x + 1, y + 4),
                (x + 2, y + 4),
                (x + 3, y + 4),
                (x + 13, y + 4),
                //
                (x + 1, y + 5),
                (x + 2, y + 5),
                (x + 12, y + 5),
                (x + 13, y + 5),
                //
                (x + 12, y + 6),
                (x + 14, y + 6),
            ]);
        }
    }

    pub fn spawn_glider_loop(&mut self, x: usize, y: usize) {
        if x - 32 >= 0 && y - 32 >= 0 && x + 32 < self.size.0 && y + 32 < self.size.1 {
            self.set_cells_alive(vec![
                (x - 5, y - 32),
                (x - 4, y - 32),
                //
                (x - 5, y - 31),
                (x - 3, y - 31),
                //
                (x - 3, y - 30),
                (x + 2, y - 30),
                (x + 3, y - 30),
                //
                (x - 7, y - 29),
                (x - 6, y - 29),
                (x - 5, y - 29),
                (x - 4, y - 29),
                (x - 2, y - 29),
                (x - 1, y - 29),
                (x + 2, y - 29),
                (x + 5, y - 29),
                //
                (x - 7, y - 28),
                (x - 4, y - 28),
                (x, y - 28),
                (x + 2, y - 28),
                (x + 4, y - 28),
                (x + 5, y - 28),
                //
                (x - 4, y - 27),
                (x - 2, y - 27),
                (x, y - 27),
                (x + 2, y - 27),
                //
                (x - 3, y - 26),
                (x - 2, y - 26),
                (x, y - 26),
                (x + 2, y - 26),
                //
                (x + 1, y - 25),
                //
                (x - 13, y - 23),
                (x - 12, y - 23),
                //
                (x - 12, y - 22),
                (x - 3, y - 22),
                //
                (x - 12, y - 21),
                (x - 10, y - 21),
                (x - 4, y - 21),
                (x - 3, y - 21),
                //
                (x - 11, y - 20),
                (x - 10, y - 20),
                //
                (x + 3, y - 19),
                //
                (x + 4, y - 18),
                //
                (x + 2, y - 17),
                (x + 3, y - 17),
                (x + 4, y - 17),
                //
                (x - 7, y - 15),
                //
                (x - 7, y - 14),
                (x - 6, y - 14),
                //
                (x - 8, y - 13),
                (x - 6, y - 13),
                (x - 1, y - 13),
                (x, y - 13),
                (x + 23, y - 13),
                //
                (x - 1, y - 12),
                (x + 21, y - 12),
                (x + 22, y - 12),
                (x + 23, y - 12),
                //
                (x, y - 11),
                (x + 1, y - 11),
                (x + 2, y - 11),
                (x + 20, y - 11),
                //
                (x + 2, y - 10),
                (x + 20, y - 10),
                (x + 21, y - 10),
                //
                (x + 13, y - 8),
                //
                (x + 14, y - 7),
                (x + 15, y - 7),
                (x + 28, y - 7),
                (x + 29, y - 7),
                //
                (x + 13, y - 6),
                (x + 14, y - 6),
                (x + 29, y - 6),
                //
                (x - 29, y - 5),
                (x - 28, y - 5),
                (x + 29, y - 5),
                (x + 31, y - 5),
                (x + 32, y - 5),
                //
                (x - 28, y - 4),
                (x - 18, y - 4),
                (x - 17, y - 4),
                (x + 21, y - 4),
                (x + 27, y - 4),
                (x + 28, y - 4),
                (x + 29, y - 4),
                (x + 32, y - 4),
                //
                (x - 30, y - 3),
                (x - 19, y - 3),
                (x - 17, y - 3),
                (x + 21, y - 3),
                (x + 22, y - 3),
                (x + 26, y - 3),
                (x + 30, y - 3),
                (x + 31, y - 3),
                //
                (x - 30, y - 2),
                (x - 29, y - 2),
                (x - 28, y - 2),
                (x - 27, y - 2),
                (x - 26, y - 2),
                (x - 17, y - 2),
                (x - 11, y - 2),
                (x - 10, y - 2),
                (x + 26, y - 2),
                (x + 27, y - 2),
                (x + 29, y - 2),
                //
                (x - 25, y - 1),
                (x - 11, y - 1),
                (x + 12, y - 1),
                (x + 13, y - 1),
                (x + 29, y - 1),
                //
                (x - 28, y),
                (x - 27, y),
                (x - 26, y),
                (x - 13, y),
                (x - 11, y),
                (x + 11, y),
                (x + 13, y),
                (x + 26, y),
                (x + 27, y),
                (x + 28, y),
                //
                (x - 29, y + 1),
                (x - 13, y + 1),
                (x - 12, y + 1),
                (x + 11, y + 1),
                (x + 25, y + 1),
                //
                (x - 29, y + 2),
                (x - 27, y + 2),
                (x - 26, y + 2),
                (x + 10, y + 2),
                (x + 11, y + 2),
                (x + 17, y + 2),
                (x + 26, y + 2),
                (x + 27, y + 2),
                (x + 28, y + 2),
                (x + 29, y + 2),
                (x + 30, y + 2),
                //
                (x - 31, y + 3),
                (x - 30, y + 3),
                (x - 26, y + 3),
                (x - 22, y + 3),
                (x - 21, y + 3),
                (x + 17, y + 3),
                (x + 19, y + 3),
                (x + 30, y + 3),
                //
                (x - 32, y + 4),
                (x - 29, y + 4),
                (x - 28, y + 4),
                (x - 27, y + 4),
                (x - 21, y + 4),
                (x + 17, y + 4),
                (x + 18, y + 4),
                (x + 28, y + 4),
                //
                (x - 32, y + 5),
                (x - 31, y + 5),
                (x - 29, y + 5),
                (x + 28, y + 5),
                (x + 29, y + 5),
                //
                (x - 29, y + 6),
                (x - 14, y + 6),
                (x - 13, y + 6),
                //
                (x - 29, y + 7),
                (x - 28, y + 7),
                (x - 15, y + 7),
                (x - 14, y + 7),
                //
                (x - 13, y + 8),
                //
                (x - 21, y + 10),
                (x - 20, y + 10),
                (x - 2, y + 10),
                //
                (x - 20, y + 11),
                (x - 2, y + 11),
                (x - 1, y + 11),
                (x, y + 11),
                //
                (x - 23, y + 12),
                (x - 22, y + 12),
                (x - 21, y + 12),
                (x + 1, y + 12),
                //
                (x - 23, y + 13),
                (x, y + 13),
                (x + 1, y + 13),
                (x + 6, y + 13),
                (x + 8, y + 13),
                //
                (x + 6, y + 14),
                (x + 7, y + 14),
                //
                (x + 7, y + 15),
                //
                (x - 4, y + 17),
                (x - 3, y + 17),
                (x - 2, y + 17),
                //
                (x - 4, y + 18),
                //
                (x - 3, y + 19),
                //
                (x + 10, y + 20),
                (x + 11, y + 20),
                //
                (x + 3, y + 21),
                (x + 4, y + 21),
                (x + 10, y + 21),
                (x + 12, y + 21),
                //
                (x + 3, y + 22),
                (x + 12, y + 22),
                //
                (x + 12, y + 23),
                (x + 13, y + 23),
                //
                (x - 1, y + 25),
                //
                (x - 2, y + 26),
                (x, y + 26),
                (x + 2, y + 26),
                (x + 3, y + 26),
                //
                (x - 2, y + 27),
                (x, y + 27),
                (x + 2, y + 27),
                (x + 4, y + 27),
                //
                (x - 5, y + 28),
                (x - 4, y + 28),
                (x - 2, y + 28),
                (x, y + 28),
                (x + 4, y + 28),
                (x + 7, y + 28),
                //
                (x - 5, y + 29),
                (x - 2, y + 29),
                (x + 1, y + 29),
                (x + 2, y + 29),
                (x + 4, y + 29),
                (x + 5, y + 29),
                (x + 6, y + 29),
                (x + 7, y + 29),
                //
                (x - 3, y + 30),
                (x - 2, y + 30),
                (x + 3, y + 30),
                //
                (x + 3, y + 31),
                (x + 5, y + 31),
                //
                (x + 4, y + 32),
                (x + 5, y + 32),
            ])
        }
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

    pub fn revive_cell(&mut self, x: usize, y: usize) {
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
