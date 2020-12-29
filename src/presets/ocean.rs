use crate::cards::{INDIGO, LIGHT_BLUE, PINK};
use crate::types::Preset;

pub const OCEAN: Preset = Preset {
    max: LIGHT_BLUE.accent_2,
    min: INDIGO.base,
    na: PINK.lighten_3,
};
