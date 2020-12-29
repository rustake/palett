use crate::cards::{BLUE, DEEP_ORANGE, LIGHT_GREEN};
use crate::types::Preset;

pub const FRESH: Preset = Preset {
    max: LIGHT_GREEN.accent_3,
    min: DEEP_ORANGE.accent_3,
    na: BLUE.lighten_3,
};
