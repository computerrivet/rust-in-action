fn main() {
    let fruits = vec!["banana", "apple", "pear"];
    println!("{:?}", fruits);
    let buff_overflow = fruits[4];
    assert_eq!(buff_overflow, "pear");
}
