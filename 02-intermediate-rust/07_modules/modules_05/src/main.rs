// Experiment with pub to control visibility of functions and structs.

#[allow(dead_code)]
mod my_mod {
    fn private_function() {
        println!("Private function");
    }

    pub fn public_function() {
        println!("Public function");
    }

    pub mod nested {
        pub(in crate::my_mod) fn public_function_in_my_mod() {
            println!("Function visible within my_mod");
        }

        pub(super) fn public_function_in_super_mod() {
            println!("Function visible within the parent module");
        }
    }

    pub fn call_public_function_in_nested() {
        nested::public_function_in_my_mod();
        nested::public_function_in_super_mod();
    }

    pub(crate) fn public_function_in_crate() {
        println!("Function visible within the current crate");
    }

    mod private_nested {
        fn private_function() {
            println!("Private function in a private module.");
        }
    }
}

fn function() {
    println!("Function in the root module");
}

fn main() {
    function();
    my_mod::public_function();
    my_mod::call_public_function_in_nested();
    my_mod::public_function_in_crate();
}
