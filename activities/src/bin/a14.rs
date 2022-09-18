// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    name: String,
    age: i32,
    favorite_color: String,
}

impl Person {
    fn print(&self) {
        println!("{}'s favorite color is {}", self.name, self.favorite_color);
    }
}

fn main() {
    let people = vec![
        Person {
            name: String::from("Alice"),
            age: 9,
            favorite_color: String::from("red"),
        },
        Person {
            name: String::from("Panos"),
            age: 28,
            favorite_color: String::from("purple"),
        },
        Person {
            name: String::from("Antonis"),
            age: 27,
            favorite_color: String::from("blue"),
        },
    ];

    for person in &people {
        if person.age <= 10 {
            person.print()
        }
    }
}
