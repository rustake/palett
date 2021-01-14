use crate::dye::DyeFactory;
use crate::enums::Effect;
use crate::types::Preset;

pub fn preset_to_flat(preset: &Preset, effects: &[Effect]) -> impl Fn(&str) -> String {
    let factory = DyeFactory::hex(effects);
    (&factory).make(preset.na)
}

// type Dye = impl Fn(&str) -> String;
//
// pub fn preset_to_flat_factory<'a>(preset: &'a Preset, effects: &'a [Effect]) -> impl Fn() -> Dye + 'a {
//     let factory = DyeFactory::hex(effects);
//     move || (&factory).make(preset.na)
// }

#[cfg(test)]
mod tests {
    use crate::presets::MOSS;

    use super::*;

    #[test]
    fn test_preset_to_flat() {
        let dye = preset_to_flat(&MOSS, &[Effect::Bold]);
        let tx = dye("some");
        println!("{}", tx);
    }

    // #[test]
    // fn test_preset_to_flat_factory() {
    //     let dye_factory = preset_to_flat_factory(&MOSS, &[Effect::Bold]);
    //     let tx = dye_factory()("some");
    //     println!("{}", tx);
    // }
}