use num::Num;
use num::traits::AsPrimitive;

use crate::dye::DyeFactory;
use crate::enums::Effect;
use crate::projector::utils::{preset_to_leap, scale};
use crate::types::{div, HSL, Leap, Preset};

pub struct ProjectorFactory {
    pub floor: f32,
    pub lever: HSL,
    pub base: HSL,
    pub factory: DyeFactory<HSL>,
}

impl ProjectorFactory {
    pub fn build<T>(leap: &Leap<T>, preset: &Preset, effects: &[Effect]) -> ProjectorFactory
        where T: Num + AsPrimitive<f32> + Copy
    {
        let color_leap = preset_to_leap(preset);
        // println!("max = {:?}, min = {:?}, dif = {:?}", color_leap.max, color_leap.min, color_leap.dif);
        return ProjectorFactory {
            floor: leap.min.as_(),
            lever: div(&color_leap.dif, leap.dif.as_()),
            base: color_leap.min,
            factory: DyeFactory::hsl(effects),
        };
    }

    pub fn make<T>(&self, value: T) -> impl Fn(&str) -> String + '_
        where T: Num + AsPrimitive<f32> + Copy
    {
        let floor = self.floor;
        let (lever_h, lever_s, lever_l) = self.lever;
        let (base_h, base_s, base_l) = self.base;
        let hsl = (
            scale(value.as_(), floor, lever_h, base_h, 360.0),
            scale(value.as_(), floor, lever_s, base_s, 100.0),
            scale(value.as_(), floor, lever_l, base_l, 100.0),
        );
        // println!("lever = {:?}, base = {:?}", self.lever, self.base);
        return self.factory.make(&hsl);
    }
}


#[cfg(test)]
mod projector_tests {
    use crate::presets::FRESH;
    use crate::projector::projector::ProjectorFactory;
    use crate::types::Leap;

    #[test]
    fn test() {
        let leap = Leap { max: 5, min: 0, dif: 5 };
        let preset = FRESH;
        let effects = [];
        let factory = ProjectorFactory::build(&leap, &preset, &effects);
        for x in 0..5 {
            let dye = factory.make(x);
            println!("[{}]: {}", x, dye("some"));
        }
        // let preset=ATLAS;
    }
}



