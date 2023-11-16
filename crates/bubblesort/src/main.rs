use std::io;
use std::io::Write;

use anyhow::{anyhow, Result};

use common::{check_sorted, get_count, make_one, Prng};

fn main() -> Result<()> {
    let count = get_count("How many to sort?")?;
    let mut v = make_one(count, 1000);

    bubble_sort(&mut v);
    println!("{:#?} {} sorted", v, {
        if check_sorted(&v) {
            "is"
        } else {
            "is not"
        }
    });

    Ok(())
}

// Bubble sort a vector
fn bubble_sort(v: &mut Vec<i32>) {
    let _v = v;
}
