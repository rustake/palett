// use crate::dye::DyeFactory;
// use crate::enums::Effect;
// use crate::types::Preset;
//
// pub fn preset_to_flat(preset: &Preset, effects: &[Effect]) -> impl Fn(&str) -> String + 'static {
//     let factory = DyeFactory::hex(effects);
//     let func = factory.make(preset.na);
//     func
// }