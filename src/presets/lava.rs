use crate::cards::{AMBER, GREY, RED};
use crate::types::Preset;

pub const LAVA: Preset = Preset {
    max: AMBER.accent_3,
    min: RED.lighten_1,
    na: GREY.accent_2,
};
