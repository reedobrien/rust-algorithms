use std::time::SystemTime;

use anyhow::Result;

use rand::thread_rng;

use crypto::{find_prime, get_number};

const NUM_TESTS: usize = 20;

fn main() -> Result<()> {
    // Prepare a Prng.
    let mut prng = thread_rng();

    // Display the probability that a number is prime
    // if it passes all NUM_TESTS tests.
    let probability = 100f64 - (0.5f64.powi(NUM_TESTS as i32));

    println!("Probability: {}%\n", probability);

    // Generate random primes.
    loop {
        // Get the number of digits.
        let num_digits: usize = get_number("# Digits (max 9): ")?;
        if num_digits < 1 {
            return Ok(());
        }

        let start = SystemTime::now();
        // Calculate minimum and maximum values.
        let mut min = 10usize.pow((num_digits - 1) as u32);
        let max = 10 * min;
        if min == 1 {
            min = 2;
        } // 1 is not prime.

        // Find a prime.
        println!("Prime: {}", find_prime(&mut prng, min, max, NUM_TESTS));
        println!("Found in {:?}", SystemTime::now().duration_since(start)?);
    }
}
