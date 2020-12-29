use crate::convert::hex_hsl;
use crate::types::{HSL, Leap, Preset};

pub fn preset_to_leap(preset: &Preset) -> Leap<HSL> {
    let max_hsl = hex_hsl(preset.max);
    let min_hsl = hex_hsl(preset.min);
    return Leap {
        min: min_hsl,
        dif: (max_hsl.0 - min_hsl.0, max_hsl.1 - min_hsl.1, max_hsl.2 - min_hsl.2),
        max: max_hsl,
    };
}