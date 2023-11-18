use anyhow::Result;

use sorting::{check_sorted, get_count, make_one};

fn main() -> Result<()> {
    let count = get_count("How many to sort?")?;
    let max = get_count("Max value: ")?;
    let mut v = make_one(count, max);

    quicksort(&mut v);

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

// Divides the array into two particions using the
// [Luomo Partition Scheme](https://en.wikipedia.org/wiki/Quicksort#Lomuto_partition_scheme).
fn partition(s: &mut [i32]) -> usize {
    let lo = 0;
    // The last element is the pivot.
    let pvt = s.len() - 1;

    // Temporary pivot index
    let mut i = lo;

    for j in 0..pvt {
        // If the current element is less than or equal to the pivot
        if s[j] <= s[pvt] {
            // swap it
            s.swap(j, i);
            // next indext to compare.
            i = i + 1;
        }
    }

    s.swap(i, pvt);

    i
}

// Sorts a (portion of an) array, divides it into partitions,
// then sorts those.
fn quicksort(s: &mut [i32]) {
    // Ensure valid indices.
    if s.len() > 1 {
        // Partition array and get the pivot index
        let p = partition(s);

        // Sort two partitions
        quicksort(&mut s[0..p]); // Left side of pivot
        quicksort(&mut s[(p + 1)..]); // Right side of pivot
    }
}

#[cfg(test)]
mod unit {
    use super::*;

    #[test]
    fn test_quicksort() {
        for i in 50..1000 {
            let mut v = make_one(i, i * 2);
            quicksort(&mut v[..]);
            assert!(check_sorted(&v));
        }
    }
}
