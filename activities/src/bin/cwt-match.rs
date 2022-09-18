fn main() {
    let my_name = "Alice";
    match my_name {
        "Jayson" => println!("That is my name"),
        "Bob" => println!("Not my name"),
        "Alice" => println!("Nice to meet you"),
        _ => println!("Who are you?"),
    }
}
