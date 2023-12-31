use std::time::Instant;

use anyhow::Result;

pub mod item;

mod exhaustive;
pub use exhaustive::exhaustive_search;

use item::Item;

use crate::item::{selected_items, sum_weights, Items};

type Algorithm<'a> = &'a dyn Fn(&mut [Item], usize) -> Result<(Vec<Item>, isize, usize)>;

pub fn run_algorithm(alg: Algorithm, items: &mut [Item], allowed_weight: usize) {
    // let mut test_items = items.to_vec();

    let start = Instant::now();

    let solution: Vec<Item>;
    let total_value: isize;
    let function_calls: usize;

    (solution, total_value, function_calls) =
        alg(items, allowed_weight).expect("failed to run algorithm");

    let duration = start.elapsed();
    println!("Elapsed: {:?}", duration);

    let selected = selected_items(&solution);

    println!("{}", Items(selected.clone()));
    println!(
        r#"Value:  {total_value}
Weight: {}
Calls:  {function_calls}  
"#,
        sum_weights(&selected),
    );
    println!();
}
