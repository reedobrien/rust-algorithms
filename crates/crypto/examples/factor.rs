use std::{process::exit, time::SystemTime};

use anyhow::Result;

use crypto::{find_primes, find_primes_sieve, get_number, init_prime_sieve};

fn main() -> Result<()> {
    init_prime_sieve();
    loop {
        let num = get_number("Find prime factors of:")?;

        let start = SystemTime::now();
        let primes = find_primes(num);
        let time = SystemTime::now().duration_since(start)?;

        println!("Found primes in:{:?}\n{:?}", time, primes);
        println!(
            "num: {} == product of primes: {}",
            num,
            primes.iter().product::<usize>()
        );

        let start = SystemTime::now();
        let primes = find_primes_sieve(num);
        let time = SystemTime::now().duration_since(start)?;

        println!("Sieved primes in:{:?}\n{:?}", time, primes);
        println!(
            "num: {} == product of primes: {}",
            num,
            primes.iter().product::<usize>()
        );
    }
}
