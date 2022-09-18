fn main() {
    let some_bool = true;
    match some_bool {
        true => println!("true"),
        false => println!("false"),
    }

    let some_int = 3;
    match some_int {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("something else"),
    }
}
