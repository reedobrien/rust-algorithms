use anyhow::Result;

use crate::item::{selected_items, solution_value, Item};

pub fn exhaustive_search(
    items: &mut [Item],
    allowed_weight: usize,
) -> Result<(Vec<Item>, isize, usize)> {
    do_exhaustive_search(items, allowed_weight, 0)
}

fn do_exhaustive_search(
    items: &mut [Item],
    allowed_weight: usize,
    next_idx: usize,
) -> Result<(Vec<Item>, isize, usize)> {
    // Base case.
    if next_idx >= items.len() {
        match solution_value(&selected_items(items), allowed_weight) {
            Ok(val) => {
                return Ok((
                    // Items vec.
                    items.to_vec(),
                    // Sum of values of selected items.
                    val as isize,
                    // Num calls made in this call.
                    1,
                ));
            }
            Err(e) => {
                //  Too heavy not really possible on the first call.
                if e.to_string() != "too heavy" {
                    return Err(e);
                }

                return Ok((vec![], 0, 1));
            }
        }
    }

    // Continue recursing.
    // i. Try adding item number next_index to the solution.
    //    To do that, set its is_selected value to true.
    items[next_idx].selected = true;

    // ii. Then recursively call do_exhaustive_search to examine the next item
    //     (the next level of the tree). To do that,
    //     pass the recursive function the value next_index + 1 as the index
    //     that it should examine. Save the result solution, total value,
    //     and number of function calls in variables.

    let sol: Vec<Item>;
    let total_val: isize;
    let mut fcalls: usize = 0;
    match do_exhaustive_search(items, allowed_weight, next_idx + 1) {
        Ok(r) => {
            sol = r.0;
            total_val = r.1;
            fcalls += r.2;
        }
        Err(e) => {
            if e.to_string() != "too heavy" {
                return Err(e);
            }
            sol = vec![];
            total_val = -1;
            fcalls += 1;
        }
    }
    // iii. Try removing item number next_index from the solution.
    //      To do that, set its is_selected value to false.
    items[next_idx].selected = false;

    // iv. Then recursively call do_exhaustive_search to examine the next
    //     item/level in the tree. Save the result solution, total value,
    //     and number of function calls in variables.

    let sol2: Vec<Item>;
    let total_val2: isize;
    let mut fcalls2: usize = 0;
    match do_exhaustive_search(items, allowed_weight, next_idx + 1) {
        Ok(r) => {
            sol2 = r.0;
            total_val2 = r.1;
            fcalls2 += r.2;
        }
        Err(e) => {
            if e.to_string() != "too heavy" {
                return Err(e);
            }
            sol2 = vec![];
            total_val2 = -1;
            fcalls2 += 1;
        }
    }
    // v. Compare the values of the two test solutions and return the one
    //    that is better, its total value, and the sum of the test function
    //    call counts plus one (for the current call).
    if total_val > total_val2 {
        Ok((sol, total_val, fcalls + fcalls2 + 1))
    } else {
        Ok((sol2, total_val2, fcalls2 + fcalls + 1))
    }
}
