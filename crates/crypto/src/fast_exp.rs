//! Run `cargo r --release --example fastexp` to run
//! a looping program to interactively test inputs.
pub fn fast_exp(mut num: usize, mut pow: usize) -> usize {
    assert!(num >= 1);

    let mut result = 1;

    while pow > 0 {
        // if pow & 1 == 1
        if pow % 2 == 1 {
            result *= num;
        }

        pow /= 2; // pow >>= 1;
        num *= num;
    }

    result
}

pub fn fast_exp_mod(mut num: usize, mut pow: usize, modulus: usize) -> usize {
    assert!(num >= 1);
    let mut result = 1;
    while pow > 0 {
        if pow % 2 == 1 {
            result = (result * num) % modulus;
        }
        pow /= 2;

        num = (num * num) % modulus;
    }

    result
}

#[cfg(test)]
mod unit {
    use super::*;

    #[test]
    fn test_fast_exp() {
        assert_eq!(fast_exp(8, 6), 8usize.pow(6));
        assert_eq!(fast_exp(8, 6), 262144);
        assert_eq!(fast_exp(7, 10), 7usize.pow(10));
        assert_eq!(fast_exp(7, 10), 282475249);

        assert_eq!(fast_exp(9, 13), 9usize.pow(13));
        assert_eq!(fast_exp(9, 13), 2541865828329);

        assert_eq!(fast_exp(213, 5), 213usize.pow(5));
        assert_eq!(fast_exp(213, 5), 438427732293);
    }
    #[test]
    fn test_fast_exp_mod() {
        assert_eq!(fast_exp_mod(8, 6, 10), 8usize.pow(6) % 10);
        assert_eq!(fast_exp_mod(8, 6, 10), 4);
        assert_eq!(fast_exp_mod(7, 10, 101), 7usize.pow(10) % 101);
        assert_eq!(fast_exp_mod(7, 10, 101), 65);

        assert_eq!(fast_exp_mod(9, 13, 283), 9usize.pow(13) % 283);
        assert_eq!(fast_exp_mod(9, 13, 283), 179);

        assert_eq!(fast_exp_mod(213, 5, 1000), 213usize.pow(5) % 1000);
        assert_eq!(fast_exp_mod(213, 5, 1000), 293);
    }
}
