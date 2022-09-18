// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavor {
    Orange,
    Apple,
    Grape,
}

struct Drink {
    flavor: Flavor,
    ounces: f64,
}

fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavor::Orange => println!("flavor: orange"),
        Flavor::Apple => println!("flavor: apple"),
        Flavor::Grape => println!("flavor: grape"),
    }
    println!("ounces: {:?}", drink.ounces);
}

fn main() {
    let my_drink = Drink {
        flavor: Flavor::Orange,
        ounces: 12.5,
    };

    print_drink(my_drink);
}
