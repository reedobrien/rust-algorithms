use std::fmt::{Display, Error, Formatter};

use anyhow::{anyhow, Result};
use rand::{Rng, RngCore};

pub struct Items(pub Vec<Item>);

/// Example
///
/// ```
/// println!("{}", Items(items.clone()));
/// ```
impl Display for Items {
    fn fmt(&self, f: &mut Formatter) -> std::result::Result<(), Error> {
        let mut output = String::new();
        for (i, it) in self.0.iter().enumerate() {
            output.push_str(&format!("{i}:({}, {}) ", it.value, it.weight));

            if i == 100 {
                output.push_str("...");
                break;
            }
        }

        write!(f, "{}", output)
    }
}

/// Items are placed into knapsacks.
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Item {
    pub value: usize,
    pub weight: usize,
    pub selected: bool,
}

/// Example
///
/// ```
/// println!(
///     "[ {} ]",
///     items
///         .iter()
///         .fold(String::new(), |acc, item| acc + format!("{item} ").as_str())
/// );
/// ```
impl Display for Item {
    fn fmt(&self, f: &mut Formatter) -> std::result::Result<(), Error> {
        write!(
            f,
            "Item[ value: {}, weight: {}, selected: {} ]",
            self.value, self.weight, self.selected,
        )
    }
}

pub fn make_items(
    rng: &mut impl RngCore,
    num_items: usize,
    min_val: usize,
    max_val: usize,
    min_weight: usize,
    max_weight: usize,
) -> Vec<Item> {
    let mut items = Vec::with_capacity(num_items);

    for _ in 0..num_items {
        items.push(Item {
            value: rng.gen_range(min_val..max_val + 1),
            weight: rng.gen_range(min_weight..max_weight + 1),
            ..Default::default()
        });
    }

    items
}

pub fn duplicate(items: &[Item]) -> Vec<Item> {
    items.to_vec()
}
pub fn sum_values(items: &[Item]) -> usize {
    items.iter().map(|i| i.value).sum()
}

pub fn sum_weights(items: &[Item]) -> usize {
    items.iter().map(|i| i.weight).sum()
}

pub fn selected_items(items: &[Item]) -> Vec<Item> {
    items.iter().filter(|i| i.selected).cloned().collect()
}

pub fn solution_value(items: &[Item], max_weight: usize) -> Result<usize> {
    let selected = selected_items(items);
    if sum_weights(&selected) > max_weight {
        return Err(anyhow!("too heavy"));
    }

    Ok(sum_values(&selected))
}

#[cfg(test)]
mod unit {
    use super::*;

    use rand::rngs::SmallRng;
    use rand::SeedableRng;

    fn make_test_items() -> Vec<Item> {
        let mut items = Vec::with_capacity(10);
        for i in 0..10 {
            items.push(Item {
                value: i + 1,
                weight: i + 1,
                // 1, 3, 5, 7, 9 are selected.
                selected: i % 2 == 0,
            })
        }

        items
    }

    #[test]
    fn test_deterministic_make_items() {
        let num_items = 10;
        let min_val = 1;
        let max_val = 5;
        let min_weight = 2;
        let max_weight = 10;

        let mut rng = SmallRng::seed_from_u64(1337);
        let mut want = Vec::with_capacity(num_items);

        for _ in 0..num_items {
            want.push(Item {
                value: rng.gen_range(min_val..max_val + 1),
                weight: rng.gen_range(min_weight..max_weight + 1),
                ..Default::default()
            });
        }

        // Restart the RNG.
        let mut rng = SmallRng::seed_from_u64(1337);

        let got = make_items(
            &mut rng, num_items, min_val, max_val, min_weight, max_weight,
        );

        assert_eq!(got, want);
    }

    #[test]
    fn test_copy_items() {
        let want = make_test_items();
        assert_eq!(duplicate(&want), want);
    }

    #[test]
    fn test_sums() {
        let items = make_test_items();
        assert_eq!(sum_values(&items), 55);
        assert_eq!(sum_weights(&items), 55);
    }

    #[test]
    fn test_selected_items() {
        assert_eq!(selected_items(&make_test_items()).len(), 5);
    }

    #[test]
    fn test_solution_value() {
        let items = make_test_items();
        let max_weight = sum_weights(&items) / 2;
        assert_eq!(
            solution_value(&items, max_weight).expect("failed to calculate value"),
            25
        );
    }
}
