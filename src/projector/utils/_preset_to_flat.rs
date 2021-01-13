use crate::dye::DyeFactory;
use crate::enums::Effect;
use crate::types::Preset;

pub fn preset_to_flat<'a>(preset: &Preset, effects: &[Effect]) -> impl Fn(&str) -> String + 'a {
    let factory = DyeFactory::hex(effects);
    (&factory).make(preset.na)
}

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
}