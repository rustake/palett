// #[cfg(test)]
// mod tests {
//     fn create_head_tail() -> (String, String) { ("[".to_string(), "]".to_string()) }
//
//     type SF<'a> = impl Fn(&'a str) -> String;
//
//     fn closure_factory<'a>() -> impl Fn(&'a str) -> SF<'a>
//     // where
//     //     T: fmt::Display
//     {
//         let result = move |prefix: &'a str| {
//             let (head, tail) = create_head_tail();
//             let func = move |text: &'a str| -> String {
//                 format!("{} {} {} {}", head, prefix, text, tail)
//             };
//             return func;
//         };
//         result
//     }
//
//     // pub fn make_clo(&self, color: &T) -> (fn(&str) -> String) {
//     //     let ansi = (self.ansi)(color);
//     //     let head = format!("{}{}{}{}{}", L, &self.head, SC, &ansi, R);
//     //     let tail = format!("{}{}{}", L, &self.tail, R);
//     //     let f = |text: &str| -> String { format!("{}{}{}", head, text, tail) };
//     //     f
//     // }
//     #[test]
//     fn return_closure_test() {
//         // let expensive_closure = |num: u32| -> u32 {
//         //     println!("calculating slowly...");
//         //     thread::sleep(Duration::from_secs(1));
//         //     num
//         // };
//         let expensive_closure = closure_factory();
//         let result = expensive_closure("5");
//         println!("{}", result("a"));
//     }
// }