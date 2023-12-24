use anyhow::Result;

use crypto::{get_number, print_sieve, print_sieve_optimized, sieve_of_eratosthenes};

fn main() -> Result<()> {
    let max = get_number("Enter max prime value:")?;
    let primes = sieve_of_eratosthenes(max);

    print_sieve(&primes);
    print_sieve_optimized(&primes);
    Ok(())
}
