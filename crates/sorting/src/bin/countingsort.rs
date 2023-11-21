use core::fmt;
use std::{slice::Iter, vec::IntoIter};

use anyhow::Result;

use sorting::{get_count, Prng};

fn main() -> Result<()> {
    let count = get_count("How many elements?")?;
    let max = get_count("With what max value?")?;

    let v = make_customers(count, max);

    let c = counting_sort(Customers(v));

    let sorted = sorted_customers(&c);
    let c = Customers(c.into_iter().take(20).collect::<Vec<Customer>>());
    println!("{} {} sorted", c, {
        if sorted {
            "is"
        } else {
            "is not"
        }
    });

    Ok(())
}

// Traditionally this takes an in put array and a max element.
// counting_sort(input: Vec<i32>, k: usize)
// We find the max as a first step in the fuction.
fn counting_sort(input: Customers) -> Customers {
    // Get the max value
    let max = input.iter().fold(0, |a, b| a.max(b.num_purchases)) as usize;

    let mut counts = vec![0i32; max + 1];
    let mut output = vec![
        Customer {
            id: "".to_string(),
            num_purchases: 0
        };
        input.len()
    ];

    // Build the counts vec first.
    input
        .iter()
        .for_each(|elem| counts[elem.num_purchases as usize] += 1);

    // Accumulate positions in the vec for populating the output vec.
    // We start at one and add any zeros in the zero place.
    // Spent an embarassing amount of time after setting 0 to 0 and starting at one.
    // 🤦‍
    for idx in 1..counts.len() {
        counts[idx as usize] += counts[idx as usize - 1]
    }

    // Place the elements in order in the output array.
    input.iter().rev().for_each(|k| {
        counts[k.num_purchases as usize] -= 1;
        output[counts[k.num_purchases as usize] as usize] = k.clone();
    });

    Customers(output)
}

struct Customers(pub Vec<Customer>);
impl fmt::Display for Customers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.iter().fold(Ok(()), |result, customer| {
            result.and_then(|_| writeln!(f, "{}", customer))
        })
    }
}
impl Customers {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn iter(&self) -> Iter<'_, Customer> {
        self.0.iter()
    }

    fn into_iter(self) -> IntoIter<Customer> {
        self.0.into_iter()
    }
}

#[derive(Clone)]
struct Customer {
    id: String,
    num_purchases: i32,
}

impl fmt::Display for Customer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Customer{{id: {}, num_purchases: {}}})",
            self.id, self.num_purchases
        )
    }
}

/// Check that a vec is sorted.
fn sorted_customers(v: &Customers) -> bool {
    if v.len() == 0 {
        return true;
    }

    let mut prev = -1;

    for c in &v.0 {
        if c.num_purchases < prev {
            return false;
        }
        prev = c.num_purchases;
    }

    true
}

/// Makes a vec of len `num_items` of customers with a max value of max.
fn make_customers(num_items: usize, max: usize) -> Vec<Customer> {
    let mut prng = Prng::new();

    let mut v: Vec<Customer> = Vec::with_capacity(num_items);
    for i in 0..num_items {
        v.push(Customer {
            id: format!("C{i}"),
            num_purchases: prng.next_i32(0, max as i32),
        });
    }

    v
}

#[cfg(test)]
mod unit {
    use super::*;

    #[test]
    fn test_counting_sort() {
        for i in 5..1000 {
            let v = make_customers(i, i * 2);
            let got = counting_sort(Customers(v));
            assert!(sorted_customers(&got));
        }
    }
}
