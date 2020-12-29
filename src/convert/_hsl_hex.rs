use crate::convert::_hsl_rgb::hsl_rgb;
use crate::convert::_rgb_hex::rgb_hex;
use crate::types::HSL;

pub fn hsl_hex(hsl: &HSL) -> String {
    rgb_hex(&hsl_rgb(&hsl))
}