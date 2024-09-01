// Create a struct to represent a rectangle and calculate its area.

#[derive(Debug)]
struct Rectangle {
    length: u32,
    breadth: u32
}

impl Rectangle {
    fn calculate_area(&self) -> u32 {
        self.length * self.breadth
    }
}

fn main() {
    let rectangle: Rectangle = Rectangle {
        length: 45,
        breadth: 32,
    };
    let area: u32 = rectangle.calculate_area();
    println!("The area of rectangle {:?} is {}.", rectangle, area);
}
