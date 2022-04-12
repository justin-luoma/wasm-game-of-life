use strum::{Display, EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Copy, Clone, Display, EnumIter, EnumCountMacro)]
pub enum Pattern {
    Blinker,
    Block,
    BeeHive,
    Loaf,
    Boat,
    Toad,
    Beacon,
    Pulsar,
    Pentadecathlon,
    Glider1,
    Glider2,
    Glider3,
    Glider4,
    LightSpaceship,
    MiddleSpaceship,
    HeavySpaceship,
    Copperhead,
    FastForwardForceField,
    GliderGun,
    RPentomino,
    DieHard,
    Acorn,
    Almosymmetric,
    GliderLoop,
    InfiniteGrowth1,
    InfiniteGrowth2,
}

impl Pattern {
    pub fn count() -> usize {
        Pattern::COUNT
    }

    pub fn get_patterns() -> Vec<Pattern> {
        Pattern::iter().collect()
    }
}

#[wasm_bindgen]
pub fn get_patterns_as_string() -> String {
    let patterns: Vec<String> = Pattern::iter().map(|pattern| pattern.to_string()).collect();
    patterns.join(",")
}
