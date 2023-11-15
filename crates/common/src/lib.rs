use std::io;
use std::io::Write;

use anyhow::{anyhow, Result};

mod prng;

fn get_count(prompt: &str) -> Result<i32> {
    println!("{prompt}");
    io::stdout().flush()?;

    let mut val = String::new();
    io::stdin().read_line(&mut val)?;

    val.trim().parse().map_err(|e| anyhow!("{e}"))
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
