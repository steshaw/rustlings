// modules2.rs
// Make me compile! Execute `rustlings hint modules2` for hints :)

mod delicious_snacks {
    pub use self::fruits::APPLE as apple;
    pub use self::fruits::PEAR as fruit;
    pub use self::veggies::CARROT as carrot;
    pub use self::veggies::CUCUMBER as veggie;

    mod fruits {
        pub const PEAR: &str = "Pear";
        pub const APPLE: &str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &str = "Cucumber";
        pub const CARROT: &str = "Carrot";
    }
}

fn main() {
    println!(
        "fruits and veg: {:?} ",
        [
            delicious_snacks::fruit,
            delicious_snacks::apple,
            delicious_snacks::veggie,
            delicious_snacks::carrot
        ]
    );
}
