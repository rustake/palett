use crate::convert::_hex_int::hex_int;
use crate::types::RGB;

pub fn hex_rgb(hex: &str) -> RGB {
    let int = hex_int(hex);
    return ((int >> 16 & 0xFF) as u8, (int >> 8 & 0xFF) as u8, (int & 0xFF) as u8);
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::convert::_hex_rgb::hex_rgb;

    #[test]
    fn convert_hex_to_rgb() {
        // let mut book_reviews = HashMap::new();
        let mut map = HashMap::new();
        map.insert("black", "#000000");
        map.insert("RED", "#FF0000");
        map.insert("GREEN", "#00FF00");
        map.insert("BLUE", "#0000FF");
        map.insert("YELLOW", "#FFFF00");
        map.insert("magenta", "#FF00FF");
        map.insert("CYAN", "#00FFFF");
        map.insert("white", "#FFFFFF");
        // println!("{:?}", map);
        for (key, value) in map {
            println!("{} : {}, {:?}", key, value, hex_rgb(value));
        }
    }
}