use std::io::Write;
use std::{io, str::FromStr};

use anyhow::{anyhow, Result};

mod prng;
pub use prng::Prng;

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
}
