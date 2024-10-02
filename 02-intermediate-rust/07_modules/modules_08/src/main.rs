// Re-export items from a submodule to the parent module using pub use.

mod submodule {
    pub fn function_one() {
        println!("Function one in submodule");
    }

    pub fn function_two() {
        println!("Function two in submodule");
    }
}

pub use submodule::{function_one, function_two};

fn main() {
    function_one();
    function_two();
}
