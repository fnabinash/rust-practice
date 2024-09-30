// Create a module geometry with submodules shapes and operations.

mod geometry {
    pub mod shapes {
        #[derive(Debug)]
        pub struct Circle {
            pub radius: f32,
        }
        
        #[derive(Debug)]
        pub struct Square {
            pub side: u32,
        }
        
        #[derive(Debug)]
        pub struct Rectangle {
            pub length: u32,
            pub bredth: u32,
        }
    }

    pub mod operations {
        use std::f32::consts::PI;

        use super::shapes;

        impl shapes::Circle {
            pub fn area(&self) -> f32 {
                PI * self.radius * self.radius
            }

            pub fn perimeter(&self) -> f32 {
                2.0 * PI * self.radius
            }
        }

        impl shapes::Square {
            pub fn area(&self) -> u32 {
                u32::pow(self.side, 2)
            }
        
            pub fn perimeter(&self) -> u32 {
                4 * self.side
            }
        }

        impl shapes::Rectangle {
            pub fn area(&self) -> u32 {
                self.length * self.bredth
            }

            pub fn perimeter(&self) -> u32 {
                2 * (self.length + self.bredth)
            }
        }
    }
}

fn main() {
    use geometry::shapes;

    let circle: shapes::Circle = shapes::Circle { radius: 4.6 };
    println!("Area of {:?} is {}.", circle, circle.area());
    println!("Perimeter of {:?} is {}.", circle, circle.perimeter());
    
    let square: shapes::Square = shapes::Square { side: 8 };
    println!("Area of {:?} is {}.", square, square.area());
    println!("Perimeter of {:?} is {}.", square, square.perimeter());
    
    let rectangle: shapes::Rectangle = shapes::Rectangle { length: 9, bredth: 4 };
    println!("Area of {:?} is {}.", rectangle, rectangle.area());
    println!("Perimeter of {:?} is {}.", rectangle, rectangle.perimeter());
}
