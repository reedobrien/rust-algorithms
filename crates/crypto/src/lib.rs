use std::{
    io::{self, Write},
    str::FromStr,
};

use anyhow::{anyhow, Result};

mod gcd_lcm;
pub use gcd_lcm::{gcd, lcm};

/// Get's a number of elements to sort from the user.
pub fn get_number<T>(prompt: &str) -> Result<T>
where
    T: FromStr,
{
    println!("{prompt}");
    io::stdout().flush()?;

    let mut val = String::new();
    io::stdin().read_line(&mut val)?;

    val.trim()
        .parse::<T>()
        .map_err(|_| anyhow!(format!("failed to parse input: {prompt}")))
}
