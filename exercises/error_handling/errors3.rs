// errors3.rs
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
// Execute `rustlings hint errors3` for hints!

// I AM NOT DONE

use std::num::ParseIntError;

fn main() {
    let mut tokens = 100;
    let pretend_user_input = "a8";

    total_cost(pretend_user_input).iter().for_each(|&cost|
        if cost > tokens {
            println!("You can't afford that many!");
        } else {
            tokens -= cost;
            println!("You now have {} tokens.", tokens);
        }
    );
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}
