pub use card::Card;
pub use preset::Preset;

mod card;
mod preset;

pub type RGB = (u8, u8, u8);
pub type HSL = (u16, u8, u8);
pub type HEX = str;
pub type INT = u32;
