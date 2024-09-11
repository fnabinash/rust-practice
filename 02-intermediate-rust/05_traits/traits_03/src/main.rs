// Implement a trait called Drawable with a method draw and implement it for different shapes (Circle, Square, Rectangle).

trait Drawable {
    fn draw(&self);
}

enum Shapes {
    Circle(f32),
    Square(u32),
    Rectangle(u32, u32)
}

impl Drawable for Shapes {
    fn draw(&self) {
        match self {
            Self::Circle(r) => println!("Draw a circle of radius {}.", r),
            Self::Square(s) => println!("Draw a square of side {}.", s),
            Self::Rectangle(l, b) => println!("Draw a rectangle of length {} and bredth {}.", l, b),
        }
    }
}

fn main() {
    let circle: Shapes = Shapes::Circle(42.5);
    circle.draw();

    let square: Shapes = Shapes::Square(8);
    square.draw();

    let rectangle: Shapes = Shapes::Rectangle(7, 4);
    rectangle.draw();
}
