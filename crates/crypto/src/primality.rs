use rand::Rng;

use crate::fast_exp_mod;

// Run `cargo r --release --example primality` to
// manually test in the terminal.
pub fn find_prime(rng: &mut impl Rng, min: usize, max: usize, tests: usize) -> usize {
    loop {
        let n = rand_odd_num(rng, min, max);
        if is_prime(rng, n, tests) {
            return n;
        }
    }
}

fn is_prime(rng: &mut impl Rng, candidate: usize, num_tests: usize) -> bool {
    for _ in 0..num_tests {
        let n = rng.gen_range(1..candidate);

        if fast_exp_mod(n, candidate - 1, candidate) != 1 {
            // Definitely not prime.
            return false;
        }
    }

    // Probably prime.
    true
}

fn rand_odd_num(rng: &mut impl Rng, min: usize, max: usize) -> usize {
    let mut n: usize = 2;
    while n % 2 == 0 {
        // gen_range is half open [min, max)
        n = rng.gen_range(min..max + 1);
    }

    n
}

#[cfg(test)]
mod unit {
    use super::*;

    #[test]
    fn gets_rand_odd_num() {
        let mut rng = rand::thread_rng();
        for i in 1..1000 {
            assert!(rand_odd_num(&mut rng, i * 100, i * 1000) % 2 != 0);
        }
    }
}
