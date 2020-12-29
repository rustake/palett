use crate::cards::{BLUE_GREY, LIGHT_GREEN, LIME};
use crate::types::Preset;

pub const JUNGLE: Preset = Preset {
    max: LIME.accent_3,
    min: LIGHT_GREEN.accent_3,
    na: BLUE_GREY.accent_1,
};
