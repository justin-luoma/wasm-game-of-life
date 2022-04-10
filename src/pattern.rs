use strum::{Display, IntoEnumIterator};
use strum_macros::EnumIter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Display, EnumIter)]
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
    Glider,
    LightSpaceship,
    MiddleSpaceship,
    GliderGun,
}

#[wasm_bindgen]
pub fn get_patterns_as_string() -> String {
    let patterns: Vec<String> = Pattern::iter().map(|pattern| pattern.to_string()).collect();
    patterns.join(",")
}
