use std::sync::OnceLock;

use crate::{sieve_of_eratosthenes, sieve_to_primes};

static BUNCHA_PRIMES: OnceLock<Vec<usize>> = OnceLock::new();

pub fn init_prime_sieve() {
    buncha_primes();
}

fn buncha_primes() -> &'static Vec<usize> {
    // For 100_000_000:
    // the sieve of bools takes about 96.4MB
    // the actual primes takes about 37.1MB
    // This takes about 6.2 seconds to initialize a vec of usize
    // for the primes up to 100M.
    // Trading time for space here.
    BUNCHA_PRIMES.get_or_init(|| sieve_to_primes(&sieve_of_eratosthenes(100_000_000)))
}

// Run `cargo r --release --example factor` to
// interactively test in terminal.
pub fn find_primes(mut num: usize) -> Vec<usize> {
    let mut i = 2;
    let mut factors: Vec<usize> = Vec::new();

    while i * i <= num {
        // doesn't divide evenly
        if num % i != 0 {
            // it isn't 2.
            // if i != 2 {
            //     i += 1;
            // }
            i += 1;
        } else {
            num /= i;
            factors.push(i)
        }
    }

    if num > 1 {
        // divides by itself.
        factors.push(num);
    }

    factors
}

pub fn find_primes_sieve(mut num: usize) -> Vec<usize> {
    let mut factors: Vec<usize> = Vec::new();

    for i in buncha_primes() {
        if i * i <= num {
            // doesn't divide evenly
            if num % i == 0 {
                // it isn't 2.
                // if i != 2 {
                //     i += 1;
                // }
                factors.push(*i);
                num /= i;
            }
        }
    }
    if num > 1 {
        // divides by itself.
        factors.push(num);
    }

    factors
}

// // Just use the iter().product method.
// fn multiply_vector(v: &[usize]) -> usize {
//     v.iter().product()
// }

#[cfg(test)]
mod unit {
    use super::*;

    #[test]
    fn finds_factors() {
        assert_eq!(find_primes(25), vec![5, 5]);
        assert_eq!(find_primes(11), vec![11]);
        assert_eq!(find_primes(714), vec![2, 3, 7, 17]);
        assert_eq!(find_primes(33), vec![3, 11]);
        assert_eq!(find_primes(147), vec![3, 7, 7]);
        assert_eq!(find_primes(17), vec![17]);
        assert_eq!(find_primes(330), vec![2, 3, 5, 11]);
        assert_eq!(find_primes(312680865509917), vec![7791799, 40129483]);
        assert_eq!(find_primes(1819448968910731), vec![40129483, 45339457]);
        assert_eq!(find_primes(12345678901234), vec![2, 7, 73, 12079920647]);
        assert_eq!(find_primes(6795742697625173), vec![6880691, 987654103]);
        assert_eq!(find_primes(64374108854777), vec![64374108854777]);
    }

    #[test]
    fn sieves_factors() {
        assert_eq!(find_primes_sieve(25), vec![5, 5]);
        assert_eq!(find_primes_sieve(11), vec![11]);
        assert_eq!(find_primes_sieve(714), vec![2, 3, 7, 17]);
        assert_eq!(find_primes_sieve(33), vec![3, 11]);
        assert_eq!(find_primes_sieve(147), vec![3, 7, 7]);
        assert_eq!(find_primes_sieve(17), vec![17]);
        assert_eq!(find_primes_sieve(330), vec![2, 3, 5, 11]);
        assert_eq!(find_primes_sieve(312680865509917), vec![7791799, 40129483]);
        assert_eq!(
            find_primes_sieve(1819448968910731),
            vec![40129483, 45339457]
        );
        assert_eq!(
            find_primes_sieve(12345678901234),
            vec![2, 7, 73, 12079920647]
        );
        assert_eq!(
            find_primes_sieve(6795742697625173),
            vec![6880691, 987654103]
        );
        assert_eq!(find_primes_sieve(64374108854777), vec![64374108854777]);
    }
}
