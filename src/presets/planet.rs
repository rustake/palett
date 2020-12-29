use crate::cards::{BLUE, CYAN, TEAL};
use crate::types::Preset;

pub const PLANET: Preset = Preset {
    max: TEAL.accent_2,
    min: BLUE.darken_3,
    na: CYAN.lighten_4,
};
