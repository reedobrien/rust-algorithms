use std::io::Write;
use std::{io, str::FromStr};

use anyhow::{anyhow, Result};

mod prng;
pub use prng::Prng;

// Traditionally this takes an in put array and a max element.
// counting_sort(input: Vec<i32>, k: usize)
// We find the max as a first step in the fuction.
pub fn counting_sort(input: Vec<i32>) -> Vec<i32> {
    // Get the max value
    let max = input.iter().fold(0, |a, b| a.max(*b)) as usize;

    let mut counts = vec![0i32; max + 1];
    let mut output = vec![0i32; input.len()];

    // Build the counts vec first.
    input.iter().for_each(|elem| counts[*elem as usize] += 1);

    // Accumulate positions in the vec for populating the output vec.
    // We start at one and add any zeros in the zero place.
    // Spent an embarassing amount of time after setting 0 to 0 and starting at one.
    // 🤦‍
    for idx in 1..counts.len() {
        counts[idx as usize] += counts[idx as usize - 1]
    }

    // Place the elements in order in the output array.
    input.into_iter().rev().for_each(|k| {
        counts[k as usize] -= 1;
        output[counts[k as usize] as usize] = k;
    });

    output
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
pub fn quicksort(s: &mut [i32]) {
    // Ensure valid indices.
    if s.len() > 1 {
        // Partition array and get the pivot index
        let p = partition(s);

        // Sort two partitions
        quicksort(&mut s[0..p]); // Left side of pivot
        quicksort(&mut s[(p + 1)..]); // Right side of pivot
    }
}

/// Get's a number of elements to sort from the user.
pub fn get_count<T>(prompt: &str) -> Result<T>
where
    T: FromStr,
{
    println!("{prompt}");
    io::stdout().flush()?;

    let mut val = String::new();
    io::stdin().read_line(&mut val)?;

    val.trim()
        .parse::<T>()
        .map_err(|_| anyhow!(format!("failed to parse input: {prompt}")))
}

/// Makes a vec of len `num_items` wiith a max value of max.
pub fn make_one(num_items: usize, max: usize) -> Vec<i32> {
    let mut prng = Prng::new();

    let mut v = Vec::with_capacity(num_items);
    for _ in 0..num_items {
        v.push(prng.next_i32(0, max as i32));
    }

    v
}

/// Check that a vec is sorted.
pub fn check_sorted(v: &Vec<i32>) -> bool {
    if v.len() == 0 {
        return true;
    }
    let prev = v[0];

    for i in 1..v.len() {
        if v[i] < prev {
            return false;
        }
    }

    true
}

/// Print at most `num` items.
// pub fn print_vec(v: &Vec<i32>, num: usize) {
//     println!("{:#?}", v.iter().take(num).collect::<Vec<&i32>>());
// }
// This can just be:
// println!("{:#?}", v.truncate(num));

#[cfg(test)]
mod common {
    use super::*;

    #[test]
    fn test_make_one() {
        let got = make_one(3, 100);
        assert!(got.len() == 3);
        assert!(got.iter().max() < Some(&100));
        assert!(got.iter().min() > Some(&0));
    }

    #[test]
    fn test_check_sorted() {
        let table = vec![
            (vec![1, 2, 3, 4], true),
            (vec![4, 2, 3, 4], false),
            (vec![4, 2, 3, 4], false),
            (vec![0, 2, 3, 4], true),
            (vec![-4, 2, 3, 4], true),
            (vec![1, 2, 3, -4], false),
            (vec![10, 23, -3, 40], false),
            (vec![10, 12, 32, 40], true),
            (vec![9, 27, 27, 30], true),
            (vec![1, 2, 2, 4, 4, 5], true),
            (vec![1, 2, 2, 4, 4, 3], true),
        ];

        for tc in table {
            assert_eq!(check_sorted(&tc.0), tc.1);
        }
    }
    #[test]
    fn test_counting_sort() {
        for i in 5..1000 {
            let v = make_one(i, i * 2);
            let got = counting_sort(v);
            assert!(check_sorted(&got));
        }
    }
    #[test]
    fn test_quicksort() {
        for i in 50..1000 {
            let mut v = make_one(i, i * 2);
            quicksort(&mut v[..]);
            assert!(check_sorted(&v));
        }
    }
}
