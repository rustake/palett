use std::fmt::Write;
use crate::utils::ansi::{effect_to_ansi, SC, L, R};
use crate::enums::{Effect};
use crate::dye::color_to_ansi::{rgb_ansi, hex_ansi, hsl_ansi};
use crate::types::{RGB, HSL, HEX};

pub struct DyeFactory<T>
{
    head: String,
    tail: String,
    ansi: fn(T) -> String,
}

// fn create_dye_factory(color_space: &ColorSpace, effects: &[Effect]) -> DyeFactory {
//     let ansi_fn = match color_space {
//         ColorSpace::RGB => { rgb_ansi }
//         ColorSpace::HSL => { hsl_ansi }
//         ColorSpace::HEX => { hex_ansi }
//     };
//     let mut dye = DyeFactory {
//         head: "".to_owned(),
//         tail: "".to_owned(),
//         ansi: ansi_fn,
//     };
//     (&mut dye).assign_effects(effects);
//     return dye;
// }

impl DyeFactory<&RGB> {
    pub fn rgb(effects: &[Effect]) -> DyeFactory<&RGB> {
        let mut dye_factory = DyeFactory { head: "".to_owned(), tail: "".to_owned(), ansi: rgb_ansi };
        (&mut dye_factory).assign_effects(effects);
        dye_factory
    }
}

impl DyeFactory<&HSL> {
    pub fn hsl(effects: &[Effect]) -> DyeFactory<&HSL> {
        let mut dye_factory = DyeFactory { head: "".to_owned(), tail: "".to_owned(), ansi: hsl_ansi };
        (&mut dye_factory).assign_effects(effects);
        dye_factory
    }
}

impl DyeFactory<&HEX> {
    pub fn hex(effects: &[Effect]) -> DyeFactory<&HEX> {
        let mut dye_factory = DyeFactory { head: "".to_owned(), tail: "".to_owned(), ansi: hex_ansi };
        (&mut dye_factory).assign_effects(effects);
        dye_factory
    }
}

// trait DyeFactoryInitializer {
//     type ColorType;
//     fn new(color_type: ColorSpace) -> DyeFactory<Self::ColorType>;
// }
//
// impl<T> DyeFactoryInitializer for DyeFactory<T> {
//     type ColorType = T;
//     fn new(color_type: ColorSpace) -> DyeFactory<Self::ColorType> {
//         let ansi = match color_type {
//             ColorSpace::RGB => { rgb_ansi }
//             ColorSpace::HSL => { hsl_ansi }
//             ColorSpace::HEX => { hex_ansi }
//         };
//         return DyeFactory { head: "".to_owned(), tail: "".to_owned(), ansi };
//     }
// }

impl<T> DyeFactory<T>
{
    pub fn assign_effects(&mut self, effects: &[Effect]) -> &Self {
        for effect in effects {
            let (h, t) = effect_to_ansi(&effect);
            write!(&mut self.head, "{}{}", SC, h).unwrap(); // &mut self.head.push_str(&*(SC.to_owned() + h));
            write!(&mut self.tail, "{}{}", SC, t).unwrap(); // &mut self.tail.push_str(&*(SC.to_owned() + t));
        }
        self
    }

    // pub fn fission(&self, rgb: &RGB) -> Box<dyn Fn(&str) -> String + '_> {
    //     let ansi = rgb_ansi(rgb);
    //     Box::new(move |text| format!("{}{}{}{}{}{}{}{}{}", L, &self.head, SC, &ansi, R, text, L, &self.tail, R))
    // }

    pub fn project(&self, color: T) -> impl Fn(&str) -> String + '_ {
        let ansi = (self.ansi)(color);
        move |text| format!("{}{}{}{}{}{}{}{}{}", L, &self.head, SC, &ansi, R, text, L, &self.tail, R)
    }
}

// impl Fn<(&RGB, )> for DyeFactory {
//     extern "rust-call" fn call(&self, args: (&RGB, )) -> impl Fn(&str) -> String + '_ {
//         let ansi = rgb_ansi(args.0);
//         move |text| format!("{}{}{}{}{}{}{}{}{}", L, &self.head, SC, &ansi, R, text, L, &self.tail, R)
//     }
// }
//
// impl FnMut<(&RGB, )> for DyeFactory {
//     extern "rust-call" fn call_mut(&mut self, args: (&RGB, )) -> impl Fn(&str) -> String + '_ {
//         self.call(args)
//     }
// }
//
// impl FnOnce<(&RGB, )> for DyeFactory {
//     type Output = String;
//     extern "rust-call" fn call_once(self, args: (&RGB, )) -> impl Fn(&str) -> String + '_ {
//         self.call(args)
//     }
// }

#[cfg(test)]
mod tests {
    use crate::enums::{Effect};
    use crate::dye::DyeFactory;
    // use crate::dye::dye_factory::DyeFactoryInitializer;

    #[test]
    fn test_rgb() {
        let dye_factory = DyeFactory::rgb(&[Effect::Bold]);
        let dye = dye_factory.project(&(255, 0, 0));
        let text = dye("abc");
        println!("{}", text);
    }

    #[test]
    fn test_hex() {
        let dye_factory = DyeFactory::hex(&[Effect::Bold]);
        let dye = dye_factory.project("#00FF00");
        let text = dye("abc");
        println!("{}", text);
    }

    // #[test]
    // fn test_beta() {
    //     let dye_factory = DyeFactory::new(ColorSpace::RGB);
    //     let func = dye_factory.init(&(255, 0, 0));
    //     let text = func("abc");
    //     println!("{}", text);
    // }
}


// fn enclose(text: &str) -> String {
//     return format!("{}{}{}", L, text, R);
// }