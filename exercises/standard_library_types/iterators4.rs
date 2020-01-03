// iterators4.rs

fn main() {}

fn fact_recursive(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn fact_fold(n: u64) -> u64 {
    (1..=n).fold(1, |acc, n| acc * n)
}

fn fact_product(n: u64) -> u64 {
    (1..=n).product()
}

// Complete this function to return factorial of num
// Do not use:
// - return
// For extra fun don't use:
// - imperative style loops (for, while)
// - additional variables
// For the most fun don't use:
// - recursion
// Execute `rustlings hint iterators4` for hints.
pub fn factorial(n: u64) -> u64 {
    fact_product(n)
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
