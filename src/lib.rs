// #![feature(unboxed_closures)]
// #![feature(fn_traits)]
// #![feature(trait_alias)]
// #![feature(type_alias_impl_trait)]

pub mod convert;
pub mod dye;
pub mod enums;
pub mod utils;
pub mod types;
pub mod cards;
pub mod projector;
pub mod presets;
pub mod fluo;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
