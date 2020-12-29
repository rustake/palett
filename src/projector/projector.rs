// use num::Num;
// use num::traits::AsPrimitive;
//
// use crate::dye::DyeFactory;
// use crate::enums::Effect;
// use crate::projector::utils::{leverage, preset_to_leap};
// use crate::types::{HSL, Leap, Preset};
//
// pub struct ProjectorFactory {
//     pub floor: f32,
//     pub lever: HSL,
//     pub base: HSL,
//     pub factory: DyeFactory<HSL>,
// }
//
// impl ProjectorFactory {
//     pub fn new<T: Num + AsPrimitive<f32> + Copy>(leap: Leap<T>, preset: &Preset, effects: &[Effect]) -> ProjectorFactory {
//         let color_leap = preset_to_leap(preset);
//         let delta: f32 = leap.dif.as_();
//         return ProjectorFactory {
//             floor: delta,
//             lever: leverage(&color_leap.dif, delta),
//             base: color_leap.min,
//             factory: DyeFactory::hsl(effects),
//         };
//     }
//
//     pub fn make(&self, value: f32) -> impl Fn(&str) -> String + '_ {
//         let floor = self.floor;
//         let (lever_h, lever_s, lever_l) = self.lever;
//         let (base_h, base_s, base_l) = self.base;
//         let hsl = (
//             scale(value, floor, lever_h, base_h, 360.0),
//             scale(value, floor, lever_s, base_s, 100.0),
//             scale(value, floor, lever_l, base_l, 100.0),
//         );
//         return self.factory.make(&hsl);
//     }
// }
//
// fn scale(x: f32, floor: f32, lever: f32, base: f32, ceil: f32) -> f32 {
//     f32::min((f32::max(x, floor) - floor) * lever + base, ceil)
// }
//
// #[cfg(test)]
// mod projector_tests {
//     use crate::types::Leap;
//
//     #[test]
//     fn test() {
//         let leap = Leap { max: 5, min: 0, dif: 5 };
//         // let preset=ATLAS;
//     }
// }
//
//
//
