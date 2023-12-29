use anyhow::{anyhow, Result};
use rand::Rng;

use crate::{gcd, lcm};

/// Pick a random exponent _e_ in the range [3,ln) such that
/// gcd(e, ln) = 1.
/// To do that, use the Prng struct next_i64 function in a loop and repeat
/// until you pick a random value e such that gcd(e, λ_n) = 1.
pub fn random_exponent(rng: &mut impl Rng, ln: usize) -> usize {
    loop {
        let e = rng.gen_range(2..ln);
        if gcd(e, ln) == 1 {
            return e;
        }
    }
}

/// Write an inverse_mod function that calculates the inverse of a number a in a modulus.
/// For example, the inverse of 3 mod 7 is 5 because 3 * 5 = 15 ≡ 1 mod 7.
/// To learn how to find the inverse,
/// see the section [Computing multiplicative inverses in modular structures](https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Computing_multiplicative_inverses_in_modular_structures)
/// in the Wikipedia article "Extended Euclidean algorithm."
/// Implement the inverse function shown in the "Modular integers" subsection,
/// but call it inverse_mod.
/// (Just implementing the function is enough for this project,
/// but more power to you if you read the rest of the page to figure out how it works!)
pub fn inverse_mod(e: usize, m: usize) -> Result<usize> {
    assert!(m > 1);

    let mut t: i128 = 0;
    let mut new_t: i128 = 1;
    let mut r = m as i128;
    let mut new_r = e as i128;

    while new_r != 0 {
        let quotient = r / new_r;
        (t, new_t) = (new_t, t - quotient * new_t);
        (r, new_r) = (new_r, r - quotient * new_r);
    }

    if r > 1 {
        return Err(anyhow!("{} is not invertable", e));
    }

    if t < 0 {
        t += m as i128;
    }

    Ok(t as usize)
}

/// https://en.wikipedia.org/wiki/RSA_(cryptosystem)#Example
/// Show's we calculate `λ(n)` as `lcm(p-1, q-1...)`.
/// So  `λ(3233) = totient(61,53) = 780`
/// So we are calculating it from the known q and q inputs to
/// `λ(3233)` or 61, and 53.
pub fn totient(p: usize, q: usize) -> usize {
    lcm(p - 1, q - 1)
}

#[cfg(test)]
mod unit {
    use super::*;

    #[test]
    fn test_totient() {
        assert_eq!(totient(61, 53), 780);
        assert_eq!(totient(53, 61), 780);
        assert_eq!(totient(3449, 5009), 2158448);
        assert_eq!(totient(5009, 3449), 2158448);
    }

    #[test]
    fn test_random_exponenet() {
        let mut rng = rand::thread_rng();
        assert!(random_exponent(&mut rng, 780) > 0);
    }

    #[test]
    fn test_inverse_mod() {
        assert_eq!(inverse_mod(3, 7).expect("failed to invert"), 5);
        assert_eq!(inverse_mod(3, 26).expect("failed to invert"), 9);
    }
}
