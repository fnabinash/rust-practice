// Use Cargo feature flags to conditionally compile different modules.

#[cfg(feature = "feature_a")]
mod module_a {
    pub fn feature_a_function() {
        println!("Hello from Feature A.");
    }
}

#[cfg(feature = "feature_b")]
mod module_b {
    pub fn feature_b_function() {
        println!("Hello from Feature B.");
    }
}


fn main() {
    #[cfg(feature = "feature_a")]
    {
        module_a::feature_a_function();
    }

    #[cfg(feature = "feature_b")]
    {
        module_b::feature_b_function();
    }

    if cfg!(feature = "feature_a") {
        println!("Feature A is enabled.");
    }

    if cfg!(feature = "feature_b") {
        println!("Feature B is enabled.");
    }

    if !cfg!(feature = "feature_a") && !cfg!(feature = "feature_b") {
        println!("No feature is enabled.")
    }
}

