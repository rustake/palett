use aryth::Bound;
use num::Num;
use num::traits::AsPrimitive;

use crate::convert::hex_hsl;
use crate::dye::DyeFactory;
use crate::enums::Effect;
use crate::projector::utils::{preset_to_leap, scale};
use crate::types::{div, HSL, Preset};

pub struct ProjectorFactory {
    pub floor: f32,
    pub lever: HSL,
    pub base: HSL,
    pub factory: DyeFactory<HSL>,
    pub default: HSL,
}

impl ProjectorFactory {
    pub fn optional_build<T>(
        bound: &Option<Bound<T>>,
        preset: &Preset,
        effects: &[Effect],
    ) -> Option<ProjectorFactory>
        where T: Num + AsPrimitive<f32> + Copy
    {
        match bound {
            None => { None }
            Some(b) => { Some(ProjectorFactory::build(b, preset, effects)) }
        }
    }
    pub fn build<T>(bound: &Bound<T>, preset: &Preset, effects: &[Effect]) -> ProjectorFactory
        where T: Num + AsPrimitive<f32> + Copy
    {
        let color_leap = preset_to_leap(preset);
        // println!("max = {:?}, min = {:?}, dif = {:?}", color_leap.max, color_leap.min, color_leap.dif);
        return ProjectorFactory {
            floor: bound.min.as_(),
            lever: div(&color_leap.dif, bound.dif().as_()),
            base: color_leap.min,
            factory: DyeFactory::hsl(effects),
            default: hex_hsl(preset.na),
        };
    }

    pub fn project<T>(&self, value: T) -> HSL where
        T: Num + AsPrimitive<f32> + Copy
    {
        let val = value.as_();
        if f32::is_nan(val) { return self.default; }
        let floor = self.floor;
        let (lever_h, lever_s, lever_l) = self.lever;
        let (base_h, base_s, base_l) = self.base;
        (
            scale(val, floor, lever_h, base_h, 360.0),
            scale(val, floor, lever_s, base_s, 100.0),
            scale(val, floor, lever_l, base_l, 100.0),
        )
    }

    pub fn render<T>(&self, value: T, text: &str) -> String
        where T: Num + AsPrimitive<f32> + Copy
    { self.factory.render(&self.project(value), text) }

    pub fn make<T>(&self, value: T) -> impl Fn(&str) -> String
        where T: Num + AsPrimitive<f32> + Copy
    {
        self.factory.make(&self.project(value))
    }

    pub fn make_box<'a, T>(&self, value: T) -> Box<dyn Fn(&str) -> String>
        where T: Num + AsPrimitive<f32> + Copy
    { Box::new(self.factory.make(&self.project(value))) }

    pub fn make_default(&self) -> impl Fn(&str) -> String
    { self.factory.make(&self.default) }
}


#[cfg(test)]
mod projector_tests {
    use aryth::Bound;

    use crate::presets::FRESH;
    use crate::projector::projector::ProjectorFactory;

    #[test]
    fn test_make() {
        let bound = Bound { max: 5, min: 0 };
        let preset = FRESH;
        let effects = [];
        let factory = ProjectorFactory::build(&bound, &preset, &effects);
        for number in 0..5 {
            let dye = factory.make(number);
            println!("[{}]: {}", number, dye("some"));
        }
        // let preset=ATLAS;
    }

    #[test]
    fn test_render() {
        let bound = Bound::new(0, 5);
        let preset = FRESH;
        let effects = [];
        let factory = ProjectorFactory::build(&bound, &preset, &effects);
        for number in 0..5 {
            let dyed = factory.render(number, "some");
            println!("[{}]: {}", number, dyed);
        }
        // let preset=ATLAS;
    }
}



