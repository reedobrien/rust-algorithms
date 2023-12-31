use std::process;

use anyhow::Result;

const EINVAL: i32 = 22;
/// Number of items available for packing.
const NUM_ITEMS: usize = 20;

const MIN_VALUE: usize = 1;
const MAX_VALUE: usize = 10;
const MIN_WEIGHT: usize = 4;
const MAX_WEIGHT: usize = 10;

use dynprog::item::{make_items, sum_values, sum_weights};
use dynprog::{exhaustive_search, run_algorithm};
use rand::rngs::SmallRng;
use rand::SeedableRng;

/// The number of calls may be off :shrug:.
fn main() -> Result<()> {
    let mut rng = SmallRng::seed_from_u64(1337);
    let mut items = make_items(
        &mut rng, NUM_ITEMS, MIN_VALUE, MAX_VALUE, MIN_WEIGHT, MAX_WEIGHT,
    );

    let allowed_weight: usize = sum_weights(&items) / 2;

    println!(
        r#"*** Parameters
# items:         {NUM_ITEMS}
Total value:     {}
Total weight:    {}
Allowed weight:  {allowed_weight}
"#,
        sum_values(&items),
        sum_weights(&items),
    );

    if NUM_ITEMS > 23 {
        eprintln!("{NUM_ITEMS} is too many items for exhaustive search.");
        process::exit(EINVAL)
    }

    println!("*** Exhaustive Search ***");
    run_algorithm(&exhaustive_search, &mut items, allowed_weight);

    Ok(())
}
