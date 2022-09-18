enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
enum Mouse {
    LeftClick,
    RightClick,
    MiddleClick,
    Scroll(i32),
    Move(i32, i32),
}

fn main() {
    let go = Direction::Left;
    match go {
        Direction::Up => println!("up"),
        Direction::Down => println!("down"),
        Direction::Left => println!("left"),
        Direction::Right => println!("right"),
    }

    let mouse = Mouse::Scroll(10);
    println!("{:?}", mouse);
}
