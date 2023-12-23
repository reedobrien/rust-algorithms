use anyhow::Result;

use crypto::{gcd, get_number, lcm};

fn main() -> Result<()> {
    loop {
        let a = get_number("First Value?:")?;
        let b = get_number("Second Value?:")?;

        println!("gcd({a}, {b})={}", gcd(a, b));
        println!("lcm({a}, {b})={}", lcm(a, b));
    }
}
