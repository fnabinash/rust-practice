// Implement a trait called Comparable with a method compare and implement it for different types (i32, f64, String).

use std::{cmp::Ordering, process};

trait Comparable {
    fn compare(&self, other: &Self) -> Ordering;
}

impl Comparable for i32 {
    fn compare(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}

impl Comparable for f64 {
    fn compare(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or_else(|| {
            eprintln!("Do not use NAN. Use valid numbers.");
            process::exit(1);
        })
    }
}

impl Comparable for String {
    fn compare(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}

fn main() {
    println!("Compare: 5, -8 => {:?}", 5.compare(&-8));
    println!("Compare: 4.5, 2.3 => {:?}", 4.5.compare(&2.3));
    println!("Compare: \"hello\", \"world\" => {:?}", "hello".to_string().compare(&"world".to_string()));
}
