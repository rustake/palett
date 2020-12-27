use palett::utils::ansi::{effect_to_ansi, SC, L, R};
use std::fmt::Write;
use palett::enums::Effect;

pub struct Dye {
    head: String,
    tail: String,
}


impl Dye {
    fn build() -> Dye {
        Dye { head: "".to_string(), tail: "".to_string() }
    }
    fn assign_effects<'a, I>(&mut self, effects: I) -> &Dye where
        I: IntoIterator<Item=&'a Effect>
    {
        // let iter = effects.into_iter();
        for effect in effects.into_iter() {
            let (h, t) = effect_to_ansi(effect);
            let h = h.to_owned();
            let t = t.to_owned();
            write!(&mut self.head, "{}{}", SC, h).unwrap(); // &mut self.head.push_str(&*(SC.to_owned() + h));
            write!(&mut self.tail, "{}{}", SC, t).unwrap(); // &mut self.tail.push_str(&*(SC.to_owned() + t));
        }
        self
    }
}

// fn enclose(text: &str) -> String {
//     return format!("{}{}{}", L, text, R);
// }

impl Fn<(&str, )> for Dye {
    extern "rust-call" fn call(&self, args: (&str, )) -> String {
        let (text, ) = args;
        return format!("{}{}{}{}{}{}{}", L, &self.head, R, text, L, &self.tail, R);
    }
}

impl FnMut<(&str, )> for Dye {
    extern "rust-call" fn call_mut(&mut self, args: (&str, )) -> String {
        self.call(args)
        // let (text, ) = args;
        // return format!("{}{}{}", self.head, text, self.tail);
    }
}

impl FnOnce<(&str, )> for Dye {
    type Output = String;
    extern "rust-call" fn call_once(self, args: (&str, )) -> String {
        self.call(args)
        // let (text, ) = args;
        // return format!("{}{}{}", self.head, text, self.tail);
    }
}

#[cfg(test)]
mod dye_tests {
    use palett::dye::DyeFactory;
    use palett::enums::Effect;

    // #[test]
    // fn test() {
    //     // let mut dye = Dye { head: "".to_string(), tail: "".to_string() };
    //     let mut dye = Dye::build();
    //     let effects = vec![&Effects::Bold];
    //     (&mut dye).assign_effects(&mut effects.into_iter());
    //     println!("{}", dye("some"));
    // }

    #[test]
    fn test() {
        // let mut dye = Dye { head: "".to_string(), tail: "".to_string() };
        let mut dye = DyeFactory::build();
        let effects = vec![&Effect::Bold];
        let some = (&mut dye).assign_effects(&effects);
        println!("{}", dye("some"));
    }
}