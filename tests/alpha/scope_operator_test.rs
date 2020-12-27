// use num_traits::Num;
//
// struct Point<T> {
//     x: T,
//     y: T,
// }
//
// fn addition<T>(pair: (T, T)) -> T {
//     let (a, b) = pair;
//     return a + b;
// }
//
// #[test]
// fn operator_test() {
//     let a = 12;
//     let b = 6;
//     let c = addition((a, b));
//     println!("{} plus {} equals to {}", a, b, c)
// }