use rand::Rng;

use super::Pi;

pub struct DartBoard {
    iters: usize,
}

impl Pi<f64> for DartBoard {
    fn calculate(&self) -> f64 {
        let mut circle_points = 0;
        for _ in 0..self.iters {
            if dart_did_land_on_board() {
                circle_points += 1;
            }
        }
        4.0 * ((circle_points as f64) / (self.iters as f64))
    }
}

fn dart_did_land_on_board() -> bool {
    let mut rng = rand::thread_rng();
    let x: f64 = rng.gen();
    let y: f64 = rng.gen();
    x * x + y * y <= 1.0
}

#[cfg(test)]
mod tests {
    fn rounded_to(input: f64, digit: i32) -> f64 {
        let pow = 10f64.powi(digit);
        (input * pow).round() / pow
    }
    use super::{DartBoard, Pi};
    #[test]
    fn thousand_iters() {
        let actual = rounded_to(DartBoard { iters: 1_000 }.calculate(), 0);
        assert_eq!(actual, 3.)
    }
    #[test]
    fn million_iters() {
        let expected = 3.1;
        let actual = rounded_to(DartBoard { iters: 1_000_000 }.calculate(), 1);
        assert_eq!(actual, expected)
    }

    #[test]
    fn ten_million_iters() {
        let actual = rounded_to(DartBoard { iters: 10_000_000 }.calculate(), 2);
        assert_eq!(actual, 3.14)
    }
}
