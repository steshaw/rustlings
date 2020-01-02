// errors3.rs
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
// Execute `rustlings hint errors3` for hints!

// I AM NOT DONE

use std::num::ParseIntError;

fn asdf(tokens: &mut i32, pretend_user_input: &str) {
    println!("You have {} tokens.", tokens);
    println!("pretend_user_input = {:?}", pretend_user_input);
    let cost = total_cost(pretend_user_input);
    match cost {
        Ok(cost) => {
            println!("cost = {}", cost);
            if cost > *tokens {
                println!("You can't afford that many!");
            } else {
                *tokens -= cost;
                println!("You now have {} tokens.", tokens);
            }
        }
        Err(parse_err) => println!("Error: {}", parse_err),
    }
}

fn main() {
    let mut tokens = 100;
    let inputs = ["", "1", " 2", "3 ", " 4 ", "5x", "x6", "0xFF"];
    for (i, pretend_user_input) in inputs.iter().enumerate() {
        if i > 0 {
            println!();
        }
        asdf(&mut tokens, pretend_user_input);
    }
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}
