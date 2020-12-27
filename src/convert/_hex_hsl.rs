use crate::convert::_rgb_hsl::rgb_hsl;
use crate::convert::_hex_rgb::hex_rgb;
use crate::types::{HSL, HEX};

pub fn hex_hsl(hex: &HEX) -> HSL {
    return rgb_hsl(&hex_rgb(hex));
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::convert::_hex_hsl::hex_hsl;

    #[test]
    fn test_convert() {
        // let mut book_reviews = HashMap::new();
        let mut map = HashMap::new();
        map.insert("black", "#000000");
        map.insert("red", "#FF0000");
        map.insert("green", "#00FF00");
        map.insert("blue", "#0000FF");
        map.insert("yellow", "#FFFF00");
        map.insert("magenta", "#FF00FF");
        map.insert("cyan", "#00FFFF");
        map.insert("white", "#FFFFFF");
        // println!("{:?}", map);
        for (key, value) in map {
            println!("{} : {}, {:?}", key, value, hex_hsl(&value));
        }
    }
}