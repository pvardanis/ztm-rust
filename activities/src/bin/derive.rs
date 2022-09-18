// Ownership is not happening anymore, instead it copies the data
// only apply it to structures that are small in size

#[derive(Debug, Clone, Copy)]
enum Position {
    Manager,
    Supervisor,
    Worker,
}

#[derive(Debug, Clone, Copy)]
struct Employee {
    position: Position,
    work_hours: i32,
}

fn print_employee(emp: Employee) {
    println!("{:?}", emp);
}

fn main() {
    let me = Employee {
        position: Position::Manager,
        work_hours: 40,
    };

    println!("{:?}", me);

    // this will create a copy
    print_employee(me);
    print_employee(me);
}
