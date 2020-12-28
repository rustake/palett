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

fn build_dye_factory<T>(color_to_ansi: fn(T) -> String, effects: &[Effect]) -> DyeFactory<T> {
    let mut dye = DyeFactory { head: "".to_owned(), tail: "".to_owned(), ansi: color_to_ansi };
    (&mut dye).assign_effects(effects);
    return dye;
}

impl DyeFactory<&RGB> {
    pub fn rgb(effects: &[Effect]) -> Self {
        build_dye_factory(rgb_ansi, effects)
    }
}

impl DyeFactory<&HSL> {
    pub fn hsl(effects: &[Effect]) -> Self {
        build_dye_factory(hsl_ansi, effects)
    }
}

impl DyeFactory<&HEX> {
    pub fn hex(effects: &[Effect]) -> Self {
        build_dye_factory(hex_ansi, effects)
    }
}

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

    pub fn make(&self, color: T) -> impl Fn(&str) -> String + '_ {
        let ansi = (self.ansi)(color);
        let head = format!("{}{}{}{}{}", L, &self.head, SC, &ansi, R);
        let tail = format!("{}{}{}", L, &self.tail, R);
        move |text| format!("{}{}{}", head, text, tail)
    }

    // pub fn fission(&self, rgb: &RGB) -> Box<dyn Fn(&str) -> String + '_> {
    //     let ansi = rgb_ansi(rgb);
    //     Box::new(move |text| format!("{}{}{}{}{}{}{}{}{}", L, &self.head, SC, &ansi, R, text, L, &self.tail, R))
    // }
}

// impl<T> Fn<(T, )> for DyeFactory<T> {
//     extern "rust-call" fn call(&self, args: (T, )) -> fn(&str) -> String {
//         let ansi = (self.ansi)(args.0);
//         move |text| format!("{}{}{}{}{}{}{}{}{}", L, &self.head, SC, &ansi, R, text, L, &self.tail, R)
//     }
// }
//
// impl<T> FnMut<(T, )> for DyeFactory<T> {
//     extern "rust-call" fn call_mut(&mut self, args: (T, )) -> fn(&str) -> String {
//         self.call(args)
//     }
// }
//
// impl<T> FnOnce<(T, )> for DyeFactory<T> {
//     type Output = String;
//     extern "rust-call" fn call_once(self, args: (T, )) -> fn(&str) -> String {
//         self.call(args)
//     }
//     // impl Fn(&str) -> String + '_
// }

#[cfg(test)]
mod tests {
    use crate::enums::{Effect};
    use crate::dye::DyeFactory;
    // use crate::dye::dye_factory::DyeFactoryInitializer;

    #[test]
    fn test_rgb() {
        let dye_factory = DyeFactory::rgb(&[Effect::Bold]);
        let dye = dye_factory.make(&(255, 0, 0));
        let text = dye("shakes - rgb");
        println!("{}", text);
    }

    #[test]
    fn test_hex() {
        let dye_factory = DyeFactory::hex(&[Effect::Bold]);
        let dye = dye_factory.make("#00FF00");
        let text = dye("shakes - hex");
        println!("{}", text);
    }

    #[test]
    fn test_hsl() {
        let dye_factory = DyeFactory::hsl(&[Effect::Bold]);
        let dye = dye_factory.make(&(240, 120, 84));
        let text = dye("shakes - hex");
        println!("{}", text);
    }
}


// fn enclose(text: &str) -> String {
//     return format!("{}{}{}", L, text, R);
// }