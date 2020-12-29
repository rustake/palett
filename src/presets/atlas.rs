use crate::cards::{CYAN, ORANGE, PINK};
use crate::types::Preset;

pub const ATLAS: Preset = Preset {
    max: CYAN.lighten_3,
    min: ORANGE.lighten_2,
    na: PINK.lighten_4,
};
