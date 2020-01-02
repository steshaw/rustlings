// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)

#[allow(clippy::print_literal)]
fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(&vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0); // DO NOT CHANGE THIS LINE!

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

#[allow(clippy::ptr_arg)]
fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
    let mut vec = vec.clone();

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
