use std::fmt::Display;

use num::Num;


fn decipher_tuple<A: Display, B: Display, C: Display>(xyz: (A, B, C)) {
    let (x, y, z) = xyz;
    println!("decipher: (x = {}, y = {}, z = {})", x, y, z);
}

fn cumulative_addition<T: Num + PartialOrd + Copy>(vec: &Vec<T>) -> Vec<T> {
    // let one: T = (1i32).as_();
    let one = T::one();
    return vec.into_iter().map(|x| *x + one).collect::<Vec<_>>();
}

// fn is_prime<N: Num + PartialOrd + Copy>(n: N) -> bool {
//     let _0 = N::zero();
//     let _1 = N::one();
//     let _2 = _1 + _1;
//     let _3 = _2 + _1;
//     let _5 = _2 + _3;
//     let _6 = _3 + _3;
//     if n == _2 || n == _3 {
//         return true;
//     } else if n % _2 == _0 || n % _3 == _0 {
//         return false;
//     }
//
//     let mut i = _5;
//     let mut w = _2;
//     while i * i <= n {
//         if n % i == _0 {
//             return false;
//         }
//         i = i + w;
//         w = _6 - w;
//     }
//     true
// }


#[test]
fn tuple_test() {
    let tup = (500, 6.4, 1);
    decipher_tuple(tup);
    let (x, y, _z) = tup;

    println!("The value of y is: {}", y);

    let vec = vec![x, x + 1];
    let x = 5;
    let vec_b = cumulative_addition(&vec);
    println!("x = {}", x);
    println!("vec = {:?}", y);
    let vec_c = cumulative_addition(&vec_b);
    println!("vec_b = {:?}", vec_b);
    println!("vec_b = {:?}", vec_c);


    let s = String::from("hello");
    // s 被声明有效

    takes_ownership(s);
    // s 的值被当作参数传入函数
    // 所以可以当作 s 已经被移动，从这里开始已经无效

    let x = 5;
    // x 被声明有效

    makes_copy(x);
    // x 的值被当作参数传入函数
    // 但 x 是基本类型，依然有效
    // 在这里依然可以使用 x 却不能使用 s
}


fn takes_ownership(some_string: String) {
    // 一个 String 参数 some_string 传入，有效
    println!("{}", some_string);
} // 函数结束, 参数 some_string 在这里释放

fn makes_copy(some_integer: i32) {
    // 一个 i32 参数 some_integer 传入，有效
    println!("{}", some_integer);
} // 函数结束, 参数 some_integer 是基本类型, 无需释放