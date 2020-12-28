// #![feature(unboxed_closures)]
// #![feature(fn_traits)]

pub mod convert;
pub mod dye;
pub mod enums;
pub mod utils;
pub mod types;
pub mod cards;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
