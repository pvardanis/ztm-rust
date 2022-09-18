#[derive(Debug)]
enum MenuChoice {
    MainMenu,
    Start,
    Quit,
}

fn get_choice(input: &str) -> Result<MenuChoice, String> {
    match input {
        "mainmenu" => Ok(MenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err("Invalid choice".to_owned()),
    }
}

fn print_choice(choice: &MenuChoice) {
    println!("choice = {:?}", choice);
}

// a better solution
fn pick_choice(input: &str) -> Result<(), String> {
    let choice: MenuChoice = get_choice(input)?; // instead of returning a Result, it will automatically perform a match operation
    print_choice(&choice);
    Ok(())
}

fn main() {
    // let choice: Result<MenuChoice, _> = get_choice("mainmenu");
    // let choice: Result<MenuChoice, _> = get_choice("leave");
    // match choice {
    //     Ok(inner_choice) => print_choice(&inner_choice),
    //     Err(error) => println!("Error: {}", error),
    // }

    // a better solution
    pick_choice("start");
    let choice = pick_choice("leave");
    println!("choice = {:?}", choice)
}
