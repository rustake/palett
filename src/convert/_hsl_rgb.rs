use crate::types::{HSL, RGB};

fn hf(n: f32, h: f32, a: f32, l: f32) -> f32 {
    let k = (n + h / 30.0) % 12.0;
    return l - a * f32::max(f32::min(f32::min(k - 3.0, 9.0 - k), 1.0), -1.0);
}

pub fn hsl_rgb(hsl: &HSL) -> RGB {
    let (h, s, l) = hsl;
    let h = *h;
    let s = *s / 100.0;
    let l = *l / 100.0;
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
        map.insert("BLACK", (0.0, 0.0, 0.0));
        map.insert("RED", (0.0, 100.0, 50.0));
        map.insert("GREEN", (120.0, 90.0, 50.0));
        map.insert("BLUE", (240.0, 100.0, 50.0));
        map.insert("YELLOW", (60.0, 100.0, 50.0));
        map.insert("MAGENTA", (300.0, 100.0, 50.0));
        map.insert("CYAN", (180.0, 100.0, 50.0));
        map.insert("WHITE", (0.0, 0.0, 100.0));

        // println!("{:?}", map);
        for (key, value) in map {
            println!("{} : {:?}, {:?}", key, value, hsl_rgb(&value));
        }
    }
}