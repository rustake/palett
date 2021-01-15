// pub type Colorant<'a> = impl Fn(&str) -> String + 'a;

// pub type Dye = impl Fn(&str) -> String;
//
// pub fn create_dye(head: &str, tail: &str) -> Dye {
//     let h = head.to_owned();
//     let t = tail.to_owned();
//     move |text| format!("{}{}{}", h, text, t)
// }

// pub fn create_colorant<'a>(head: &'a str, tail: &'a str) -> Colorant<'a> {
//     move |text| format!("{}{}{}", head.to_string(), text, tail.to_string())
// }
