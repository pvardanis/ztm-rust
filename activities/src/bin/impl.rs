struct Temperature {
    celsius: f64,
}

impl Temperature {
    fn freezing() -> Self {
        Self { celsius: 0.0 }
    }

    fn show_temp(&self) {
        println!("Temperature: {:?}°C", self.celsius);
    }
}

fn main() {
    let temp = Temperature::freezing();
    temp.show_temp();
}
