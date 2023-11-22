use std::time::{SystemTime, UNIX_EPOCH};

pub struct Prng {
    seed: u32,
}

impl Default for Prng {
    fn default() -> Self {
        Self::new()
    }
}

impl Prng {
    pub fn new() -> Self {
        let mut prng = Self { seed: 0 };
        prng.randomize();

        prng
    }

    pub fn randomize(&mut self) {
        self.seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards")
            .as_millis() as u32;
    }

    pub fn next_u32(&mut self) -> u32 {
        self.seed = self.seed.wrapping_mul(1_103_515_245).wrapping_add(12_345);
        self.seed %= 1 << 31;

        self.seed
    }

    pub fn next_f64(&mut self) -> f64 {
        self.next_u32() as f64 / (2147483647.0 + 1.0)
    }

    pub fn next_i32(&mut self, min: i32, max: i32) -> i32 {
        let range = (max - min) as f64;

        (min as f64 + range * self.next_f64()) as i32
    }
}

#[cfg(test)]
mod common {
    use super::*;

    #[test]
    fn test_new() {
        let mut tut = Prng::new();

        let mut prev: i32 = 0;
        for _ in 0..=100 {
            let got = tut.next_i32(1, 100000);
            assert_ne!(prev, got);
            prev = got;
        }

        let mut prev: u32 = 0;
        for _ in 0..=100 {
            let got = tut.next_u32();
            assert_ne!(prev, got);
            prev = got;
        }

        let mut prev: f64 = 0.0;
        for _ in 0..=100 {
            let got = tut.next_f64();
            assert_ne!(prev, got);
            prev = got;
        }
    }

    #[test]
    fn test_prng() {
        let mut rng1 = Prng { seed: 3 };
        let mut rng2 = Prng { seed: 3 };

        for _ in 1..=100 {
            let v1 = rng1.next_i32(1, 1000);
            let v2 = rng2.next_i32(1, 1000);
            assert_eq!(v1, v2);
        }
    }
}
