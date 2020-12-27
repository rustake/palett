use crate::enums::Effect;

pub const BOLD: &str = "1";
pub const ITALIC: &str = "3";
pub const UNDERLINE: &str = "4";
pub const INVERSE: &str = "7";
pub const CLR_BOLD: &str = "22";
pub const CLR_ITALIC: &str = "23";
pub const CLR_UNDERLINE: &str = "24";
pub const CLR_INVERSE: &str = "27";


pub fn effect_to_ansi<'a>(effect: &Effect) -> (&'a str, &'a str) {
    match effect {
        Effect::Bold => { (BOLD, CLR_BOLD) }
        Effect::Italic => { (ITALIC, CLR_ITALIC) }
        Effect::Underline => { (UNDERLINE, CLR_UNDERLINE) }
        Effect::Inverse => { (INVERSE, CLR_INVERSE) }
    }
}

#[test]
fn test() {
    let bold = Effect::Bold;
    let effect = effect_to_ansi(&bold);
    println!("{:?}", effect);
    // println!("{:?}", bold.type_id())
}