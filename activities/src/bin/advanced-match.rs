enum Discount {
    Percentage(i32),
    Flat(i32),
}

struct Ticket {
    event: String,
    price: i32,
}

fn main() {
    let n = 4;
    match n {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        other => println!("something else: {:?}", other),
    }

    let flat_discount = Discount::Flat(10);
    match flat_discount {
        Discount::Flat(amount) => println!("${} off", amount),
        Discount::Flat(2) => println!("flat $2"),
        _ => (),
    }

    let ticket = Ticket {
        event: String::from("Concert"),
        price: 100,
    };

    match ticket {
        Ticket { price, .. } => println!("Ticket for concert: ${}", price),
        Ticket { price: 50, event } => println!("Ticket for concert: $50"),
    }
}
