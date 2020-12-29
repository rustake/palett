use crate::cards::{AMBER, DEEP_PURPLE, PINK};
use crate::types::Preset;

pub const VIOLA: Preset = Preset {
    max: PINK.lighten_4,
    min: DEEP_PURPLE.accent_2,
    na: AMBER.darken_2,
};
