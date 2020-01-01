// iterators4.rs
//

fn main() {
}

pub fn factorial(num: u64) -> u64 {
    // Complete this function to return factorial of num
    // Do not use:
    // - return
    // For extra fun don't use:
    // - imperative style loops (for, while)
    // - additional variables
    // For the most fun don't use:
    // - recursion
    // Scroll down for hints.
    //
    let mut r : u64 = 1;
    let mut n = num;
    while n > 0 {
        r = n * r;
        n -= 1;
    }
    r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}

























// In an imperative language you might write a for loop to iterate through
// multiply the values into a mutable variable. Or you might write code more
// functionally with recursion and a match clause. But you can also use ranges
// and iterators to solve this in rust.
