// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

/// Print out the details of a student's locker assignment.
struct Student {
    /// The student's name.
    name: String,
    /// The student's locker number.
    locker: Option<i32>,
}

fn main() {
    let student = Student {
        name: "Panos".to_owned(),
        locker: Some(123),
    };

    match student.locker {
        Some(locker) => println!("{}'s locker is {}", student.name, locker),
        None => println!("{} has no locker", student.name),
    }
}
