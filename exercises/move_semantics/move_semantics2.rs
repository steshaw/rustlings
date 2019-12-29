//
// move_semantics2.rs
// Make me compile without changing line 10! Scroll down for hints :)
//

use std::fmt;

//
// Here is a comment about this o function.
//
fn o<T>(name: &str, v: &Vec<T>)
where
    T: fmt::Debug,
{
    println!("{} has length {} content `{:?}`", name, v.len(), v);
}

fn some_mapping() {
    let opt_n = Some(2_i128);
    let r = opt_n
        .iter()
        .map(|a| {
            println!("a = {}", a);
        })
        .for_each(|()| {});
    println!("opt_n = {:?}", opt_n);
    assert_eq!(opt_n, Some(2));
    assert_eq!(r, ());
}

fn main() {
    some_mapping();

    let mut vec0 = vec![];
    let mut vec1 = fill_vec(&mut vec0);

    // Do not change the following line!
    o("vec0", &vec0);

    vec1.push(88);
    o("vec1", &vec1);

    // Drain vec0.
    // XXX: Draining with map does not work as expected...
    vec0.iter().for_each(|a| {
        println!("vec0 mapping = {}", a);
    });
    let r = vec0.drain(..).for_each(|a| {
        println!("vec0 drain = {}", a);
        ()
    });
    println!("r = {:?}", r);
    o("vec0", &vec0);

    // Drain vec1.
    vec1.drain(..).for_each(|a| {
        println!("vec1 drain = {}", a);
    });
    o("vec1", &vec1);
}

// =========================================================================
// This is a comment.
// =========================================================================
fn fill_vec(vec: &mut Vec<i32>) -> Vec<i32> {
    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec.to_vec()
}

// So `vec0` is being *moved* into the function `fill_vec` when we call it on
// line 7, which means it gets dropped at the end of `fill_vec`, which means we
// can't use `vec0` again on line 10 (or anywhere else in `main` after the
// `fill_vec` call for that matter). We could fix this in a few ways, try them
// all!
// 1. Make another, separate version of the data that's in `vec0` and pass that
// to `fill_vec` instead.
// 2. Make `fill_vec` borrow its argument instead of taking ownership of it,
// and then copy the data within the function in order to return an owned
// `Vec<i32>`
// 3. Make `fill_vec` *mutably* borrow its argument (which will need to be
// mutable), modify it directly, then not return anything. Then you can get rid
// of `vec1` entirely -- note that this will change what gets printed by the
// first `println!`
