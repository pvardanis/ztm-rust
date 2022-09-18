// Topic: Result
//
// Requirements:
// * Create an structure named `Adult` that represents a person aged 21 or older:
//   * The structure must contain the person's name and age
//   * Implement Debug print functionality using `derive`
// * Implement a `new` function for the `Adult` structure that returns a Result:
//   * The Ok variant should contain the initialized structure, but only
//     if the person is aged 21 or older
//   * The Err variant should contain a String (or &str) that explains why
//     the structure could not be created
// * Instantiate two `Adult` structures:
//   * One should be aged under 21
//   * One should be 21 or over
// * Use `match` to print out a message for each `Adult`:
//   * For the Ok variant, print any message you want
//   * For the Err variant, print out the error message

#[derive(Debug)]
struct Adult {
    name: String,
    age: i32,
}

impl Adult {
    fn new(name: String, age: i32) -> Result<Self, String> {
        match age >= 21 {
            true => Ok(Self { name, age }),
            false => Err("Age must be at least 21".to_string()),
        }
    }
}

fn print_adult(adult: &Adult) {
    println!("Adult: {:?}", adult);
}

fn main() {
    let adults = vec![
        Adult::new("John".to_owned(), 21),
        Adult::new("Jane".to_owned(), 20),
    ];

    for adult in adults {
        match adult {
            Ok(adult) => print_adult(&adult),
            Err(error) => println!("Error: {:?}", error),
        }
    }
}
