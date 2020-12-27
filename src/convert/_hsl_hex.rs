use crate::convert::_rgb_hex::rgb_hex;
use crate::convert::_hsl_rgb::hsl_rgb;
use crate::types::HSL;

pub fn hsl_hex(hsl: &HSL) -> String {
    return rgb_hex(&hsl_rgb(&hsl));
}