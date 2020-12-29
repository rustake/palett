use crate::cards::{GREEN, RED, YELLOW};
use crate::types::Preset;

pub const POME: Preset = Preset {
    max: RED.lighten_2,
    min: YELLOW.darken_1,
    na: GREEN.lighten_2,
};
