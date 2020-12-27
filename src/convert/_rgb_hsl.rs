use num_traits::{Num};
use crate::types::{RGB, HSL};

const THOUSAND: f32 = 1000.0;

pub fn rgb_hsl(rgb: &RGB) -> HSL {
    let (r, g, b) = rgb;
    let (r, g, b) = (
        f32::from(*r) / 255.0,
        f32::from(*g) / 255.0,
        f32::from(*b) / 255.0
    );
    let Bound { max, sum, dif } = bound((r, g, b));
    let h = hue(r, g, b, max, dif) * 60.0;
    let s = if dif == 0.0 { 0.0 } else { dif / (2.0 - sum) };
    let l = sum / 2.0;
    let s = s * THOUSAND / 10.0;
    let l = l * THOUSAND / 10.0;
    return (h as u16, s as u8, l as u8);
}

pub struct Bound<T> {
    max: T,
    sum: T,
    dif: T,
}

pub fn bound<T: Num + PartialOrd + Copy>(rgb: (T, T, T)) -> Bound<T> {
    let (r, g, b) = rgb;
    let mut va = r;
    let mut vi = r;
    if g > r { va = g } else { vi = g }
    if b > va { va = b }
    if b < vi { vi = b }
    let sum = va + vi;
    let dif = va - vi;
    return Bound { max: va, sum, dif };
}

pub fn hue(r: f32, g: f32, b: f32, max: f32, dif: f32) -> f32 {
    if dif == 0.0 { return 0.0; }
    return match max {
        _ if max == r => ((g - b) / dif + if g < b { 6.0 } else { 0.0 }) % 6.0,
        _ if max == g => (b - r) / dif + 2.0,
        _ if max == b => (r - g) / dif + 4.0,
        _ => 0.0
    };
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::convert::_rgb_hsl::rgb_hsl;

    #[test]
    fn test_rgb_to_hsl() {
        // let mut book_reviews = HashMap::new();
        let mut map = HashMap::new();
        map.insert("black", (0u8, 0u8, 0u8));
        map.insert("red", (255u8, 0u8, 0u8));
        map.insert("green", (0u8, 255u8, 0u8));
        map.insert("blue", (0u8, 0u8, 255u8));
        map.insert("yellow", (255u8, 255u8, 0u8));
        map.insert("magenta", (255u8, 0u8, 255u8));
        map.insert("cyan", (0u8, 255u8, 255u8));
        map.insert("white", (255u8, 255u8, 255u8));
        // println!("{:?}", map);
        for (key, value) in map {
            println!("{} : {:?}, {:?}", key, value, rgb_hsl(&value));
        }
    }
}