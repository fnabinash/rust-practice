// Write a program that creates a struct to represent a point in 2D space and calculate the distance between two points.

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32
}

impl Point {
    fn calculate_destance(&self, other: Point) -> f32 {
        ((other.x - self.x).powf(2.0) + (other.y - self.y).powf(2.0)).powf(0.5)
    }
}

fn main() {
    let point1: Point = Point { x: -5.6, y: 3.0 };
    let point2: Point = Point { x: 8.9, y: 5.0 };

    println!("Point1: {:?}", point1);
    println!("Point2: {:?}", point2);
    
    let distance: f32 = point1.calculate_destance(point2);
    println!("Distance between point1 and point2 is {:?}.", distance);
}
