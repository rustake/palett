use crate::types::{HEX, INT};

pub fn hex_int(hex: &HEX) -> INT {
    let hex = hex.trim_start_matches('#');
    return match u32::from_str_radix(hex, 16) {
        Ok(v) => v,
        Err(_e) => 0
    };
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::convert::_hex_int::hex_int;

    #[test]
    fn test_convert() {
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
            println!("{} : {}, {}", key, value, hex_int(value));
        }
    }
}