// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

enum Color {
    Red,
    Green,
    Blue,
    Yellow,
    Orange,
    Purple,
    Black,
    White,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Red => println!("Color: Red"),
            Color::Green => println!("Color: Green"),
            Color::Blue => println!("Color: Blue"),
            Color::Yellow => println!("Color: Yellow"),
            Color::Orange => println!("Color: Orange"),
            Color::Purple => println!("Color: Purple"),
            Color::Black => println!("Color: Black"),
            Color::White => println!("Color: White"),
        }
    }
}

struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

impl Dimensions {
    fn print(&self) {
        println!("Width: {:?}", self.width);
        println!("Height: {:?}", self.height);
        println!("Depth: {:?}", self.depth);
    }
}

struct Box {
    weight: f64,
    dimensions: Dimensions,
    color: Color,
}

impl Box {
    fn new(weight: f64, dimensions: Dimensions, color: Color) -> Self {
        // order of the fields is not important as long as the types match
        // the original names
        Self {
            dimensions,
            weight,
            color,
        }
    }

    fn print(&self) {
        println!("weight: {:?}", self.weight);

        self.dimensions.print();
        self.color.print();
    }
}

fn main() {
    let my_box = Box::new(
        10.0,
        Dimensions {
            width: 10.0,
            height: 10.0,
            depth: 10.0,
        },
        Color::Red,
    );

    my_box.print();
}
