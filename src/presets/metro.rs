use crate::cards::{BLUE, PINK, TEAL};
use crate::types::Preset;

pub const METRO: Preset = Preset {
    max: PINK.lighten_2,
    min: BLUE.lighten_4,
    na: TEAL.accent_3,
};
