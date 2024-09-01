// Implement a program that uses an enum to represent different shapes (Circle, Square, Rectangle) and calculate their area.

use std::f32::consts::PI;

#[derive(Debug)]
enum Shapes {
    Circle(f32),
    Square(f32),
    Rectangle{l: f32, b: f32}
}

impl Shapes {
    fn get_area(&self) -> f32 {
        match self {
            Self::Circle(r) => PI * r.powf(2.0),
            Self::Square(s) => s.powf(2.0),
            Self::Rectangle { l, b } => l * b,
        }
    }
}

fn main() {
    let circle: Shapes = Shapes::Circle(6.7);
    let area: f32 = circle.get_area();
    println!("Area of {:?} is {}.", circle, area);

    let square: Shapes = Shapes::Square(8.0);
    let area: f32 = square.get_area();
    println!("Area of {:?} is {}.", square, area);

    let rectangle: Shapes = Shapes::Rectangle { l: 9.0, b: 6.0 };
    let area: f32 = rectangle.get_area();
    println!("Area of {:?} is {}.", rectangle, area);
}
