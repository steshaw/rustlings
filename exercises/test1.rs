// test1.rs
// This is a test for the following sections:
// - Variables
// - Functions

// Mary is buying apples. One apple usually costs 2 Rustbucks, but if you buy
// more than 40 at once, each apple only costs 1! Write a function that calculates
// the price of an order of apples given the order amount. No hints this time!

// Put your function here!
fn calculate_apple_price(quantity: i32) -> i32 {
    let price_per_item = if (quantity > 40) { 1 } else { 2 };
    price_per_item * quantity
}

// Don't modify this function!
#[test]
fn verify_test() {
    assert_eq!(0, calculate_apple_price(0));
    assert_eq!(2, calculate_apple_price(1));
    assert_eq!(4, calculate_apple_price(2));

    let price1 = calculate_apple_price(35);
    assert_eq!(70, price1);

    let price2 = calculate_apple_price(65);
    assert_eq!(65, price2);
}
