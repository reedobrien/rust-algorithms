use anyhow::Result;

use crate::item::{sum_values, Item};

/// Run `cargo r --release --example rod` to execute the example
/// and view output in a terminal.
pub fn rods_technique(
    items: &mut [Item],
    allowed_weight: usize,
) -> Result<(Vec<Item>, isize, usize)> {
    make_block_lists(items);

    let best_value: usize = 0;
    let current_value: usize = 0;
    let current_weight: usize = 0;
    let remaining_value: usize = sum_values(items);

    do_rod(
        items,
        allowed_weight,
        best_value,
        current_value,
        current_weight,
        remaining_value,
        0,
    )
}

pub fn rods_technique_sorted(
    items: &mut [Item],
    allowed_weight: usize,
) -> Result<(Vec<Item>, isize, usize)> {
    make_block_lists(items);

    items.sort_by(|a, b| b.block_list.len().cmp(&a.block_list.len()));
    items.iter_mut().enumerate().for_each(|(idx, it)| {
        it.id = idx;
    });

    make_block_lists(items);

    let best_value: usize = 0;
    let current_value: usize = 0;
    let current_weight: usize = 0;
    let remaining_value: usize = sum_values(items);

    do_rod(
        items,
        allowed_weight,
        best_value,
        current_value,
        current_weight,
        remaining_value,
        0,
    )
}

fn do_rod(
    items: &mut [Item],
    allowed_weight: usize,
    // a. Add the new parameters
    // best_value, current_value,
    // current_weight, and remaining_value.
    mut best_value: usize,
    current_value: usize,
    current_weight: usize,
    remaining_value: usize,
    next_idx: usize,
) -> Result<(Vec<Item>, isize, usize)> {
    // Base case.
    if next_idx >= items.len() {
        return Ok((
            // Items vec.
            items.to_vec(),
            // Sum of values of selected items.
            current_value as isize,
            // Num calls made in this call.
            1,
        ));
    }

    if current_value + remaining_value <= best_value {
        return Ok((vec![], 0, 1)); // solution_value(items, allowed_weight)? as isize, 1));
    }

    let mut sol: Vec<Item> = vec![];
    let mut total_val: isize = 0;
    let mut fcalls: usize = 1;
    if items[next_idx].blocked_by.is_none()
        && current_weight + items[next_idx].weight <= allowed_weight
    {
        items[next_idx].selected = true;
        (sol, total_val, fcalls) = do_rod(
            items,
            allowed_weight,
            best_value,
            current_value + items[next_idx].value,
            current_weight + items[next_idx].weight,
            remaining_value - items[next_idx].value,
            next_idx + 1,
        )?;
        if total_val > best_value as isize {
            best_value = total_val as usize;
        }
    }

    let current_block_list = items[next_idx].block_list.clone();
    current_block_list.iter().for_each(|i| {
        if items[*i].blocked_by.is_none() {
            items[*i].blocked_by = Some(items[next_idx].id);
        }
    });

    items[next_idx].selected = false;

    let sol2: Vec<Item>;
    let total_val2: isize;
    let fcalls2: usize;

    (sol2, total_val2, fcalls2) = do_rod(
        items,
        allowed_weight,
        best_value,
        current_value,
        current_weight,
        remaining_value - items[next_idx].value,
        next_idx + 1,
    )?;

    current_block_list.iter().for_each(|i| {
        if items[*i].blocked_by == Some(items[next_idx].id) {
            items[*i].blocked_by = None;
        }
    });

    // h. Return the better of the two recursive solutions as before.
    // v. Compare the values of the two test solutions and return the one
    //    that is better, its total value, and the sum of the test function
    //    call counts plus one (for the current call).
    if total_val >= total_val2 {
        Ok((sol, total_val, fcalls + fcalls2 + 1))
    } else {
        Ok((sol2, total_val2, fcalls2 + fcalls + 1))
    }
}

pub fn make_block_lists(items: &mut [Item]) {
    for i in 0..items.len() {
        items[i].block_list.clear();
        for j in 0..items.len() {
            if i != j && items[i].value >= items[j].value && items[i].weight <= items[j].weight {
                items[i].block_list.insert(items[j].id);
            }
        }
    }
}

#[cfg(test)]
mod unit {
    use super::*;

    use rand::{rngs::SmallRng, SeedableRng};

    use std::collections::BTreeSet;

    use crate::item::make_items;

    #[test]
    fn test_make_block_items() {
        let mut rng = SmallRng::seed_from_u64(1337);
        let mut items = make_items(&mut rng, 3, 1, 10, 4, 10);

        make_block_lists(&mut items);

        assert_eq!(
            items[0],
            Item {
                id: 0,
                value: 10,
                weight: 9,
                selected: false,
                blocked_by: None,
                block_list: BTreeSet::from([1]),
            }
        );

        assert_eq!(
            items[1],
            Item {
                id: 1,
                value: 1,
                weight: 9,
                selected: false,
                blocked_by: None,
                block_list: BTreeSet::new(),
            }
        );
        assert_eq!(
            items[2],
            Item {
                id: 2,
                value: 5,
                weight: 7,
                selected: false,
                blocked_by: None,
                block_list: BTreeSet::from([1]),
            }
        );
    }
}
