// use num_traits::Num;
//
// struct Point<T> {
//     x: T,
//     y: T,
// }
//
// impl<T: Num> Point<T> {
//     fn distance(&self) -> &T {
//         &self.x ^ 2 + &self.y ^ 2
//     }
// }
//
// #[test]
// fn generic_struct_test() {
//     let p = Point { x: 5, y: 10 };
//
//     println!("p.x = {}", p.distance());
// }