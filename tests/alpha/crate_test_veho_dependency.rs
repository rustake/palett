use veho::matrix::init;

#[test]
fn test() {
    let matrix = init(4, 3, |i, j| i + j);
    println!("{:?}", matrix)
}