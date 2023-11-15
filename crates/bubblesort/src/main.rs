use std::io;
use std::io::Write;

use anyhow::{anyhow, Result};

fn main() {
    println!("Hello, world!");
}

fn get_count(prompt: &str) -> Result<i32> {
    println!("{prompt}");
    io::stdout().flush()?;

    let mut val = String::new();
    io::stdin().read_line(&mut val)?;

    val.trim().parse().map_err(|e| anyhow!("{e}"))
}
