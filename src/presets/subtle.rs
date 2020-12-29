use crate::cards::{GREY, INDIGO};
use crate::types::Preset;

pub const SUBTLE: Preset = Preset {
    max: GREY.lighten_5,
    min: GREY.darken_1,
    na: INDIGO.lighten_3,
};
