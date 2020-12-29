use crate::convert::hex_hsl;
use crate::types::{HSL, Leap, Preset};

pub fn preset_to_leap(preset: &Preset) -> Leap<HSL> {
    let max_hsl = hex_hsl(preset.max);
    let min_hsl = hex_hsl(preset.max);
    return Leap {
        min: min_hsl,
        dif: (min_hsl.0 - min_hsl.0, min_hsl.1 - min_hsl.1, min_hsl.2 - min_hsl.2),
        max: max_hsl,
    };
}