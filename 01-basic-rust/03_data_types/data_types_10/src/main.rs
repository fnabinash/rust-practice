// Create a program that calculates the area of a circle using a floating-point number.

use std::f64::consts::PI;

fn main() {
    let redius: f64 = 3.4;
    let area: f64 = PI * redius.powf(2.0);

    println!("Area of circle with {} is {}", redius, area);
}
