use anyhow::Result;

use crypto::{get_number, print_sieve_optimized, sieve_of_eratosthenes, sieve_to_primes};

fn main() -> Result<()> {
    let max = get_number("Enter max prime value:")?;
    let primes = sieve_of_eratosthenes(max);

    sieve_to_primes(&primes).iter().for_each(|p| print!("{p} "));
    println!();

    print_sieve_optimized(&primes);

    Ok(())
}
