use crate::cards::{CYAN, GREEN, GREY};
use crate::types::Preset;

pub const AQUA: Preset = Preset {
    max: CYAN.accent_2,
    min: GREEN.darken_1,
    na: GREY.lighten_4,
};
