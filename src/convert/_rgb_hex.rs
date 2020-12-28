use crate::convert::_rgb_int::rgb_int;
use crate::types::RGB;

pub fn rgb_hex(rgb: &RGB) -> String {
    format!("#{:0>6X}", rgb_int(rgb))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::convert::_rgb_hex::rgb_hex;

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
            println!("{} : {:?}, {:?}", key, value, rgb_hex(&value));
        }
    }
}