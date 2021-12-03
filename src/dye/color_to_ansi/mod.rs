use std::fmt::Write;

use crate::convert::{hex_int, hsl_rgb};
use crate::types::{HSL, RGB};
use crate::utils::ansi::{FORE, SC};

pub fn rgb_ansi(rgb: &RGB) -> String {
    let (r, g, b) = rgb;
    let mut text = FORE.to_owned(); // String::from(FORE);
    write!(&mut text, "{}{}{}{}{}{}", SC, r, SC, g, SC, b).unwrap();
    return text;
}


pub fn hex_ansi(hex_color: &str) -> String {
    let n = hex_int(hex_color);
    let mut text = FORE.to_owned();
    write!(&mut text, "{}{}{}{}{}{}", SC, n >> 16 & 0xFF, SC, n >> 8 & 0xFF, SC, n & 0xFF).unwrap();
    return text;
}

pub fn hsl_ansi(hsl: &HSL) -> String {
    let (r, g, b) = hsl_rgb(hsl);
    let mut text = FORE.to_owned(); // String::from(FORE);
    write!(&mut text, "{}{}{}{}{}{}", SC, r, SC, g, SC, b).unwrap();
    return text;
}

#[cfg(test)]
mod color_to_ansi_tests {
    use crate::dye::color_to_ansi::rgb_ansi;

    #[test]
    fn rgb_ansi_test() {
        let rgb = (255, 0, 0);
        let ansi = rgb_ansi(&rgb);
        println!("ansi = {}", ansi);
    }
}