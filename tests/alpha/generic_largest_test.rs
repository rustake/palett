fn max<T: PartialOrd +Copy>(list: &[T]) -> T {
    let mut max = list[0];
    for &item in list { if item > max { max = item; } }
    max
}

#[test]
fn largest_test() {
    let list = vec![34, 50, 25, 100, 65];

    let max_value = max(&list);
    println!("The largest number is {}", max_value);

    let list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let max_value = max(&list);
    println!("The largest number is {}", max_value);
}