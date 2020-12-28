use crate::types::{HSL, RGB};

fn hf(n: f32, h: f32, a: f32, l: f32) -> f32 {
    let k = (n + h / 30.0) % 12.0;
    return l - a * f32::max(f32::min(f32::min(k - 3.0, 9.0 - k), 1.0), -1.0);
}

pub fn hsl_rgb(hsl: &HSL) -> RGB {
    let (h, s, l) = hsl;
    let h = *h as f32;
    let s = (*s as f32) / 100.0;
    let l = (*l as f32) / 100.0;
    // println!("h = {}, s = {}, l = {}", h, s, l);
    let a = s * f32::min(l, 1.0 - l);
    let r = hf(0.0, h, a, l);
    let g = hf(8.0, h, a, l);
    let b = hf(4.0, h, a, l);
    // println!("r = {}, g = {}, b = {}", r, g, b);
    return ((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8);
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::convert::_hsl_rgb::hsl_rgb;

    #[test]
    fn test_hsl_to_rgb() {
        // let mut book_reviews = HashMap::new();
        let mut map = HashMap::new();
        map.insert("black", (0u16, 0u8, 0u8));
        map.insert("red", (0u16, 100u8, 50u8));
        map.insert("green", (120u16, 90u8, 50u8));
        map.insert("blue", (240u16, 100u8, 50u8));
        map.insert("yellow", (60u16, 100u8, 50u8));
        map.insert("magenta", (300u16, 100u8, 50u8));
        map.insert("cyan", (180u16, 100u8, 50u8));
        map.insert("white", (0u16, 0u8, 100u8));

        // println!("{:?}", map);
        for (key, value) in map {
            println!("{} : {:?}, {:?}", key, value, hsl_rgb(&value));
        }
    }
}