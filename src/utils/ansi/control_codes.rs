pub const CLR_ALL: &str = "0";
pub const ESC: &str = "\u{001b}";
pub const L: &str = "\u{001b}[";
pub const R: &str = "m";
pub const SC: &str = ";";

#[test]
fn test() {
    println!("{}", L);
}