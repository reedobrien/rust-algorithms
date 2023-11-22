use std::{
    io::{self, Write},
    str::FromStr,
};

use anyhow::{anyhow, Result};

/// Get's a number of elements to sort from the user.
pub fn get_count<T>(prompt: &str) -> Result<T>
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

// #[cfg(test)]
// mod tests {
//     use super::*;

// #[test]
// fn it_works() {
//     let result = add(2, 2);
//     assert_eq!(result, 4);
// }
// }
