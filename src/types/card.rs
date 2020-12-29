use std::collections::HashMap;

pub struct Card {
    pub base: &'static str,
    pub lighten_5: &'static str,
    pub lighten_4: &'static str,
    pub lighten_3: &'static str,
    pub lighten_2: &'static str,
    pub lighten_1: &'static str,
    pub darken_1: &'static str,
    pub darken_2: &'static str,
    pub darken_3: &'static str,
    pub darken_4: &'static str,
    pub accent_1: &'static str,
    pub accent_2: &'static str,
    pub accent_3: &'static str,
    pub accent_4: &'static str,
}

impl Card {
    pub fn to_hashmap(&self) -> HashMap<&str, &str> {
        let mut map = HashMap::new();
        map.insert("base", self.base);
        map.insert("lighten_5", self.lighten_5);
        map.insert("lighten_4", self.lighten_4);
        map.insert("lighten_3", self.lighten_3);
        map.insert("lighten_2", self.lighten_2);
        map.insert("lighten_1", self.lighten_1);
        map.insert("darken_1", self.darken_1);
        map.insert("darken_2", self.darken_2);
        map.insert("darken_3", self.darken_3);
        map.insert("darken_4", self.darken_4);
        map.insert("accent_1", self.accent_1);
        map.insert("accent_2", self.accent_2);
        map.insert("accent_3", self.accent_3);
        map.insert("accent_4", self.accent_4);
        map
    }
}