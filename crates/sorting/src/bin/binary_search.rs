use std::cmp::Ordering;

use anyhow::Result;

use sorting::{get_count, make_one, quicksort};

fn main() -> Result<()> {
    let size = get_count("How many:")?;
    let max = get_count("Max:")?;
    let mut v = make_one(size, max);

    quicksort(&mut v[..]);

    loop {
        let query = get_count::<i32>("Target (-1 to quit):")?;
        match query {
            -1 => std::process::exit(0),
            _ => {
                let res = binary_search(&mut v, query);
                if res.0 != -1 {
                    println!("numbers[{}] = {}, {} tests", res.0, query, res.1);
                } else {
                    println!("Target {} not found, {} tests", query, res.1);
                }
            }
        }
    }
}

fn binary_search(input: &mut Vec<i32>, query: i32) -> (i32, i32) {
    let mut left = 0;
    let mut right = input.len() - 1;
    let mut tests = 0;

    while left <= right {
        tests += 1;
        let middle = (left + right) / 2;

        match input[middle].cmp(&query) {
            Ordering::Greater => left = middle + 1,
            Ordering::Less => {
                if middle > 0 {
                    right = middle - 1
                } else {
                    break;
                }
            }
            Ordering::Equal => return (middle as i32, tests),
        }
    }

    (-1, tests)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        let mut v = vec![
            22, 10, 52, 9, 78, 76, 41, 36, 99, 23, 1, 74, 38, 12, 44, 52, 67, 14, 1, 58,
        ];
        quicksort(&mut v);

        assert_eq!(binary_search(&mut v, 10), (3, 5));

        // Light test to ensure it doesn't panic.
        for i in 5..4000 {
            let mut v = make_one(i, i * 2);

            binary_search(&mut v, (i + i) as i32);
        }
    }
}
