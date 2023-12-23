//! Run `cargo r --release --example gcdlcm` to run
//! a looping program to test inputs.
pub fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}

pub fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

#[cfg(test)]
mod unit {
    use super::*;

    #[test]
    fn gcd_works() {
        assert_eq!(gcd(0, 2), 2);
        assert_eq!(gcd(2, 0), 2);
        assert_eq!(gcd(21, 110), 1);
        assert_eq!(gcd(110, 21), 1);
        assert_eq!(gcd(5, 10), 5);
        assert_eq!(gcd(10, 5), 5);
        assert_eq!(gcd(270, 192), 6);
        assert_eq!(gcd(192, 270), 6);
        assert_eq!(gcd(12, 18), 6);
        assert_eq!(gcd(270, 192), 6);
        assert_eq!(gcd(7469, 2464), 77);
        assert_eq!(gcd(55290, 115430), 970);
    }

    #[test]
    fn lcm_works() {
        assert_eq!(lcm(12, 18), 36);
        assert_eq!(lcm(270, 192), 8640);
        assert_eq!(lcm(7469, 2464), 239008);
        assert_eq!(lcm(55290, 115430), 6579510);
    }
}
