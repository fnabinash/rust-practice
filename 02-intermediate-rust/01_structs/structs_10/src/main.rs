// Create a struct to represent a circle with a field for radius, and implement methods to calculate the circumference and area.

use std::f32::consts::PI;

struct Circle(f32);

impl Circle {
    fn area(&self) -> f32 {
        PI * self.0.powf(2.0)
    }

    fn circumference(&self) -> f32 {
        2.0 * PI * self.0
    }
}

fn main() {
    let circle: Circle = Circle(2.6);
    let area: f32 = circle.area();
    let circumference: f32 = circle.circumference();

    println!("The area of circle with {} radius is {}.", circle.0, area);
    println!("The circumference of circle with {} radius is {}.", circle.0, circumference);
}
