use anyhow::Result;

use crate::item::{sum_values, Item};

/// Run `cargo r --release --example branch_bound` to execute the example
/// and view output in a terminal.
pub fn branch_and_bound(
    items: &mut [Item],
    allowed_weight: usize,
) -> Result<(Vec<Item>, isize, usize)> {
    let best_value: usize = 0;
    let current_value: usize = 0;
    let current_weight: usize = 0;
    let remaining_value: usize = sum_values(items);

    do_bnb(
        items,
        allowed_weight,
        best_value,
        current_value,
        current_weight,
        remaining_value,
        0,
    )
}

fn do_bnb(
    items: &mut [Item],
    allowed_weight: usize,
    // a. Add the new parameters
    // best_value, current_value,
    // current_weight, and remaining_value.
    best_value: usize,
    current_value: usize,
    current_weight: usize,
    remaining_value: usize,
    next_idx: usize,
) -> Result<(Vec<Item>, isize, usize)> {
    // b. If we have a full assignment, return it as in the previous milestone.
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

    // c. If current_value + remaining_value <= best_value,
    //    then return a blank solution vector
    //    (so we continue to use the previous best solution).
    if current_value + remaining_value <= best_value {
        return Ok((vec![], 0, 1)); // solution_value(items, allowed_weight)? as isize, 1));
    }

    // d. If current_weight + items[next_index].weight <= allowed_weight,
    //    then recursively try adding the next item to the solution.
    //    Pass in values for the current_value, current_weight,
    //    and remaining_value parameters that are adjusted for adding the
    //    next item to the solution. Save the results of the recursive call
    //   in variables test1_solution, test1_value, and test1_calls.
    let sol: Vec<Item>;
    let total_val: isize;
    let fcalls: usize;
    if current_weight + items[next_idx].weight <= allowed_weight {
        items[next_idx].selected = true;
        (sol, total_val, fcalls) = do_bnb(
            items,
            allowed_weight,
            best_value,
            current_value + items[next_idx].value,
            current_weight + items[next_idx].weight,
            remaining_value - items[next_idx].value,
            // sum_values(&items[next_idx..]),
            next_idx + 1,
        )?;
    } else {
        // e. If current_weight + items[next_index].weight <= allowed_weight is
        //    not true, then set the variables test1_solution, test1_value,
        //    and test1_calls to represent a blank solution with 0 value and 1
        //    function call.
        (sol, total_val, fcalls) = (vec![], 0, 1);
    }

    // f. If current_value + remaining_value - items[next_index].value > best_value,
    //    then recursively try not adding the next item to the solution.
    //    Don’t change current_value or current_weight,
    //    but subtract the omitted item’s value from the remaining_value
    //    parameter that you pass into the recursive call.
    //    Save the results of the recursive call in variables test2_solution,
    //    test2_value, and test2_calls.
    items[next_idx].selected = false;

    let sol2: Vec<Item>;
    let total_val2: isize;
    let fcalls2: usize;

    (sol2, total_val2, fcalls2) = do_bnb(
        items,
        allowed_weight,
        best_value,
        current_value,
        current_weight,
        remaining_value - items[next_idx].value,
        next_idx + 1,
    )?;

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
