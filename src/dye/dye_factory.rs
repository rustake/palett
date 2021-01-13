use std::fmt::Write;

use crate::dye::color_to_ansi::{hex_ansi, hsl_ansi, rgb_ansi};
use crate::enums::Effect;
use crate::types::{HEX, HSL, RGB};
use crate::utils::ansi::{effect_to_ansi, L, R, SC};

pub struct DyeFactory<T> where
    T: ?Sized
{
    head: String,
    tail: String,
    ansi: fn(&T) -> String,
}

impl<T> DyeFactory<T> where T: ?Sized {
    pub fn build(color_to_ansi: fn(&T) -> String, effects: &[Effect]) -> DyeFactory<T> {
        let mut dye = DyeFactory { head: "".to_owned(), tail: "".to_owned(), ansi: color_to_ansi };
        (&mut dye).assign_effects(effects);
        return dye;
    }
}

impl DyeFactory<RGB> {
    pub fn rgb(effects: &[Effect]) -> Self
    { DyeFactory::build(rgb_ansi, effects) }
}

impl DyeFactory<HSL> {
    pub fn hsl(effects: &[Effect]) -> Self
    { DyeFactory::build(hsl_ansi, effects) }
}

impl DyeFactory<HEX> {
    pub fn hex(effects: &[Effect]) -> Self
    { DyeFactory::build(hex_ansi, effects) }
}

impl<T: ?Sized> DyeFactory<T>
{
    pub fn assign_effects(&mut self, effects: &[Effect]) -> &Self {
        for effect in effects {
            let (h, t) = effect_to_ansi(&effect);
            write!(&mut self.head, "{}{}", SC, h).unwrap(); // &mut self.head.push_str(&*(SC.to_owned() + h));
            write!(&mut self.tail, "{}{}", SC, t).unwrap(); // &mut self.tail.push_str(&*(SC.to_owned() + t));
        }
        self
    }

    pub fn make(&self, color: &T) -> impl Fn(&str) -> String + 'static {
        let ansi = (self.ansi)(color);
        let head = format!("{}{}{}{}{}", L, &self.head, SC, &ansi, R);
        let tail = format!("{}{}{}", L, &self.tail, R);
        move |text| format!("{}{}{}", head, text, tail)
    }

    pub fn render(&self, color: &T, text: &str) -> String {
        let ansi = (self.ansi)(color);
        format!("{}{}{}{}{}{}{}{}{}", L, &self.head, SC, &ansi, R, text, L, &self.tail, R)
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
    use crate::dye::DyeFactory;
    use crate::enums::Effect;

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
        let dye = dye_factory.make(&(240.0, 120.0, 84.0));
        let text = dye("shakes - hex");
        println!("{}", text);
    }
}


// fn enclose(text: &str) -> String {
//     return format!("{}{}{}", L, text, R);
// }