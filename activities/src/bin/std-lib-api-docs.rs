fn main() {
    let numbers = vec![1, 2, 3];

    match numbers.is_empty() {
        true => println!("The vector is empty"),
        false => println!("The vector is not empty"),
    }
}
