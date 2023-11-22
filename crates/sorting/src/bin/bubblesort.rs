use anyhow::Result;

use sorting::{check_sorted, get_count, make_one};

fn main() -> Result<()> {
    let count = get_count("How many to sort?")?;
    let max = get_count("Max value: ")?;
    let mut v = make_one(count, max);

    bubble_sort(&mut v);

    println!(
        "{:#?} {} sorted",
        v.iter().take(20).collect::<Vec<&i32>>(),
        {
            if check_sorted(&v) {
                "is"
            } else {
                "is not"
            }
        }
    );

    Ok(())
}

// Bubble sort a vector
fn bubble_sort(v: &mut Vec<i32>) {
    if v.len() < 2 {
        return;
    }

    let mut sorted = false;

    while !sorted {
        sorted = true;

        for i in 0..v.len() - 1 {
            if v[i] > v[i + 1] {
                v.swap(i, i + 1);
                sorted = false;
            }
        }
    }
}

#[cfg(test)]
mod unit {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        for i in 5..100 {
            let mut tut = make_one(i, i);
            bubble_sort(&mut tut);
            assert!(check_sorted(&tut));
        }
    }
}
