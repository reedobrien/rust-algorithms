use std::collections::HashMap;

use anyhow::Result;
use recursion::get_count;

fn main() -> Result<()> {
    let mut fibg = Fibonacci::default();
    loop {
        let n: i128 = get_count("Calculate fib sequence for?\n(enter negative number to exit):")?;
        if n < 0 {
            break;
        }
        println!("fib sequence for {n} is {:#?}", fibg.nth(n as usize));
    }

    println!("Here's the first 22 in the fib sequence:");
    for n in 0..=22 {
        println!("fib({n}) = {}", fibr(n as u64));
    }

    Ok(())
}

// use a generator
struct Fibonacci {
    a: u128,
    b: u128,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { a: 1, b: 0 }
    }
}

impl Default for Fibonacci {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for Fibonacci {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.b;

        self.b = self.a;
        self.a += ret;

        Some(ret)
    }
}

// memoized in a vector.
#[allow(dead_code)]
fn fibv(n: u128) -> u128 {
    fn fib_memo(n: u128, memo: &mut [Option<u128>]) -> u128 {
        memo[n as usize].unwrap_or_else(|| match n {
            0 => 0,
            1 | 2 => 1,
            _ => {
                let val = fib_memo(n - 1, memo) + fib_memo(n - 2, memo);

                memo[n as usize] = Some(val);
                val
            }
        })
    }

    fib_memo(n, &mut vec![None; n as usize + 1])
}

// memoized in a map.
#[allow(dead_code)]
fn fibd(n: u128) -> u128 {
    fn fib_memo(n: u128, memo: &mut HashMap<u128, u128>) -> u128 {
        match n {
            0 => {
                memo.insert(0, 0);
                return 0;
            }
            1 => {
                memo.insert(1, 1);
                return 1;
            }
            2 => {
                memo.insert(2, 1);
                return 1;
            }
            _ => (),
        }

        if !memo.contains_key(&n) {
            let one = fib_memo(n - 1, memo);
            let two = fib_memo(n - 2, memo);
            memo.entry(n).or_insert(one + two);
        }

        *memo.get(&n).unwrap()
    }

    let mut memo: HashMap<u128, u128> = HashMap::with_capacity(n as usize);

    fib_memo(n, &mut memo)
}

#[allow(dead_code)]
fn fibr(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        n => fibr(n - 1) + fibr(n - 2),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fib_seq_u128() -> Vec<u128> {
        vec![
            0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181,
            6765, 10946, 17711, 28657, 46368, 75025, 121393, 196418, 317811, 514229, 832040,
            1346269, 2178309, 3524578, 5702887, 9227465, 14930352, 24157817, 39088169, 63245986,
            102334155,
        ]
    }
    fn fib_seq_u64() -> Vec<u64> {
        vec![
            0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181,
            6765, 10946, 17711, 28657, 46368, 75025, 121393, 196418, 317811, 514229, 832040,
            1346269, 2178309, 3524578, 5702887, 9227465, 14930352, 24157817, 39088169, 63245986,
            102334155,
        ]
    }

    #[test]
    fn test_fib_recursive() {
        let fibseq = fib_seq_u64();
        for (n, want) in fibseq.iter().take(20).enumerate() {
            assert_eq!(fibr(n as u64), *want);
        }
    }

    #[test]
    fn test_fibd() {
        let fibseq = fib_seq_u128();
        for (n, want) in fibseq.iter().enumerate() {
            assert_eq!(fibd(n as u128), *want);
        }
    }

    #[test]
    fn test_fibv() {
        let fibseq = fib_seq_u128();
        for (n, want) in fibseq.iter().enumerate() {
            assert_eq!(fibv(n as u128), *want);
        }
    }

    #[test]
    fn test_fibiter() {
        let mut fibg = Fibonacci::default();
        for (_, want) in fib_seq_u128().iter().enumerate() {
            assert_eq!(fibg.next(), Some(*want));
        }
    }
}
