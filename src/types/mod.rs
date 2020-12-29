pub use bound::Bound;
pub use card::Card;
pub use color_bound::ColorBound;
pub use extensions::div;
pub use leap::Leap;
pub use preset::Preset;

mod card;
mod preset;
mod leap;
mod bound;
mod color_bound;
mod extensions;

pub type RGB = (u8, u8, u8);
pub type HSL = (f32, f32, f32);
pub type HEX = str;
pub type INT = u32;
pub type TDV<T> = (T, T, T);
// impl<T: Num + Copy> Add<T> for TRIO<T> {
//     type Output = Self;
//     fn add(self, other: T) -> Self::Output {
//         let (x, y, z) = self;
//         return (x + other, y + other, z + other);
//     }
// }
//
// #[cfg(test)]
// mod trio_ops_tests {
//     #[test]
//     fn test() {
//         let xyz = (2, 4, 8);
//         let result = xyz + 2;
//         println!("{:?}", result);
//     }
// }