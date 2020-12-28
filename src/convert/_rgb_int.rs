use crate::types::RGB;

pub fn rgb_int(rgb: &RGB) -> u32 {
    let (r, g, b) = *rgb;
    let (r, g, b) = (u32::from(r), u32::from(g), u32::from(b));
    return ((r & 0xFF) << 16) + ((g & 0xFF) << 8) + (b & 0xFF);
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::convert::_rgb_int::rgb_int;

    #[test]
    fn test_convert() {
        // let mut book_reviews = HashMap::new();
        let mut map = HashMap::new();
        map.insert("black", (0u8, 0u8, 0u8));
        map.insert("RED", (255u8, 0u8, 0u8));
        map.insert("GREEN", (0u8, 255u8, 0u8));
        map.insert("BLUE", (0u8, 0u8, 255u8));
        map.insert("YELLOW", (255u8, 255u8, 0u8));
        map.insert("magenta", (255u8, 0u8, 255u8));
        map.insert("CYAN", (0u8, 255u8, 255u8));
        map.insert("white", (255u8, 255u8, 255u8));
        // println!("{:?}", map);
        for (key, value) in map {
            println!("{} : {:?}, {}", key, value, rgb_int(&value));
        }
    }
}