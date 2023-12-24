use crypto::{print_sieve, print_sieve_optimized, sieve_of_eratosthenes};
use divan::{black_box, Bencher};

fn main() {
    divan::main();
}

#[divan::bench]
fn print_sieve_unopt(bencher: Bencher) {
    let primes = sieve_of_eratosthenes(10000);

    bencher.bench_local(move || {
        black_box(print_sieve(&primes));
    });
}
#[divan::bench]
fn print_sieve_opt(bencher: Bencher) {
    let primes = sieve_of_eratosthenes(10000);

    bencher.bench_local(move || {
        black_box(print_sieve_optimized(&primes));
    });
}
