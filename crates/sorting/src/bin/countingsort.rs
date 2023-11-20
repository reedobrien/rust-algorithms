use anyhow::Result;

use sorting::{check_sorted, get_count, make_one};

fn main() -> Result<()> {
    let count = get_count("How many elements?")?;
    let max = get_count("With what max value?")?;

    let mut v = make_one(count, max);

    v = counting_sort(v);

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

// Traditionally this takes an in put array and a max element.
// counting_sort(input: Vec<i32>, k: usize)
// We find the max as a first step in the fuction.
fn counting_sort(input: Vec<i32>) -> Vec<i32> {
    // Get the max value
    let max = input.iter().fold(std::i32::MIN, |a, b| a.max(*b)) as usize;

    let mut counts = vec![0i32; max + 1];
    let mut output = vec![0i32; input.len()];

    // Build the counts vec first.
    input.iter().for_each(|elem| counts[*elem as usize] += 1);

    // Accumulate positions in the vec for populating the output vec.
    // We start at one and add any zeros in the zero place.
    // Spent an embarassing amount of time after setting 0 to 0 and starting at one.
    // 🤦‍
    for idx in 1..counts.len() {
        counts[idx as usize] += counts[(idx as usize) - 1]
    }

    // Place the elements in order in the output array.
    input.into_iter().rev().for_each(|k| {
        counts[k as usize] -= 1;
        output[counts[k as usize] as usize] = k;
    });

    output
}

#[cfg(test)]
mod unit {
    use super::*;

    #[test]
    fn test_counting_sort() {
        for i in 5..1000 {
            let v = make_one(i, i * 2);
            let got = counting_sort(v);
            assert!(check_sorted(&got));
        }
    }
}
