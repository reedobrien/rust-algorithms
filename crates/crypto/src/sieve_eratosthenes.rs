//! Run `cargo r --release --example sieve`
//! to for an interactive env.

pub fn sieve_of_eratosthenes(max: usize) -> Vec<bool> {
    assert!(max > 1);

    let mut primes = vec![true; max + 1];
    primes[0] = false;
    primes[1] = false;

    let mut start = 2;
    let end = (max as f64).sqrt() as usize;

    while start <= end {
        // Only start if next prime.
        if primes[start] {
            // from start to max+1.
            (start..max + 1)
                // skip to the next multiple.
                .skip(start)
                // step by our current start value.
                .step_by(start)
                // Set each multiple to false.
                .for_each(|i| primes[i] = false);
        }
        // Next start value
        start += 1;
    }

    primes
}

// Benchmarks run with divan, see `benches/sieve.rs`.
// Most of the time is probably printing to stdout.
//
// ```
// sieve           fastest       │ slowest       │ median        │ mean          │ samples │ iters
// ├─ print_sieve_opt
//                 59.12 µs      │ 81.95 µs      │ 62.93 µs      │ 63.49 µs      │ 100     │ 100
// ╰─ print_sieve_unopt
//                 61.2 µs       │ 96.95 µs      │ 64.64 µs      │ 65.64 µs      │ 100     │ 100
// ```

pub fn print_sieve(sieve: &[bool]) {
    sieve
        .iter()
        .enumerate()
        .filter(|x| *x.1)
        .for_each(|b| print!("{} ", b.0));

    println!();
}

pub fn print_sieve_optimized(sieve: &[bool]) {
    print!("2 ");

    (3..sieve.len()).step_by(2).for_each(|i| {
        if sieve[i] {
            print!("{i} ",);
        }
    });

    println!();
}

// Not needed
// pub fn print_numbers(primes: &[usize]) {
//     primes.iter().for_each(|prime| print!("{prime} "));
// }
