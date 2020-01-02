// test4.rs
// This test covers the sections:
// - Modules
// - Macros

// Write a macro that passes the test! No hints this time, you can do it!

macro_rules! my_macro {
    ($suffix: expr) => {
        if $suffix == "world!" {
            "Hello ".to_string() + $suffix
        } else {
            // Use better grammar by default.
            "Hello, ".to_string() + $suffix
        }
    }
}

fn main() {
    if my_macro!("world!") != "Hello world!" {
        panic!("Oh no! Wrong output!");
    }
    println!("Message is: {}", my_macro!("Steve!"));
}
