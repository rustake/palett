use crate::cards::{BROWN, LIGHT_GREEN, TEAL};
use crate::types::Preset;

pub const MOSS: Preset = Preset {
    max: LIGHT_GREEN.accent_3,
    min: TEAL.lighten_3,
    na: BROWN.accent_1,
};
