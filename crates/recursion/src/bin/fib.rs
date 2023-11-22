use anyhow::Result;
use recursion::get_count;

fn main() -> Result<()> {
    loop {
        let n: i128 = get_count("Calculate fib sequence for?\n(enter negative number to exit):")?;
        if n < 0 {
            break;
        }
        println!("fib sequence for {n} is {:#?}", fibr(n as u64));
    }

    println!("Here's the first 22 in the fib sequence:");
    for n in 0..=22 {
        println!("fib({n}) = {}", fibr(n as u64));
    }

    Ok(())
}

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

    #[test]
    fn test_fib_recursive() {
        let fibseq: Vec<u64> = vec![
            0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181,
            6765, 10946, 17711, 28657, 46368, 75025, 121393, 196418, 317811, 514229, 832040,
            1346269, 2178309, 3524578, 5702887, 9227465, 14930352, 24157817, 39088169, 63245986,
            102334155,
        ];

        for (n, want) in fibseq.iter().enumerate() {
            assert_eq!(fibr(n as u64), *want);
        }
    }
}
