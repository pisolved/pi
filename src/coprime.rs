use super::*;

use gcd::Gcd;
use rand::Rng;

pub struct Coprime {
    pub iters: u32,
}

impl Pi<f64> for Coprime {
    fn calculate(&self) -> f64 {
        let mut coprimes = 0;
        let mut rng = rand::thread_rng();
        for _ in 0..self.iters {
            let a: u64 = rng.gen();
            let b: u64 = rng.gen();
            if a.gcd(b) == 1 {
                coprimes += 1;
            }
        }
        (6. / (coprimes as f64 / self.iters as f64)).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_digit() {
        let actual = Coprime { iters: 1_000_000 }.calculate();
        assert_eq!(rounded_to(actual, 1), 3.1);
    }
    #[test]
    fn two_digits() {
        let actual = Coprime { iters: 2_000_000 }.calculate();
        assert_eq!(rounded_to(actual, 2), 3.14);
    }
}
