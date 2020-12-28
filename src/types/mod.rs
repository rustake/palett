pub type RGB = (u8, u8, u8);
pub type HSL = (u16, u8, u8);
pub type HEX = str;
pub type INT = u32;

pub struct Card {
    pub base: String,
    pub lighten_5: String,
    pub lighten_4: String,
    pub lighten_3: String,
    pub lighten_2: String,
    pub lighten_1: String,
    pub darken_1: String,
    pub darken_2: String,
    pub darken_3: String,
    pub darken_4: String,
    pub accent_1: String,
    pub accent_2: String,
    pub accent_3: String,
    pub accent_4: String,
}