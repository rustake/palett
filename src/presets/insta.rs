use crate::cards::{GREY, ORANGE, PURPLE};
use crate::types::Preset;

pub const INSTA: Preset = Preset {
    max: ORANGE.accent_2,
    min: PURPLE.accent_1,
    na: GREY.lighten_2,
};
