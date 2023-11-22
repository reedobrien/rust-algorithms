use anyhow::Result;

use sorting::{check_sorted, get_count, make_one, quicksort};

fn main() -> Result<()> {
    let count = get_count("How many to sort?")?;
    let max = get_count("Max value: ")?;
    let mut v = make_one(count, max);

    quicksort(&mut v);

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
