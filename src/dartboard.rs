use rand::Rng;

use super::*;

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
    use super::{rounded_to, DartBoard, Pi};
    #[test]
    fn thousand_iters() {
        let actual = rounded_to(DartBoard { iters: 1_000 }.calculate(), 0);
        assert_eq!(actual, 3.)
    }
    #[test]
    fn million_iters() {
        let actual = rounded_to(DartBoard { iters: 1_000_000 }.calculate(), 1);
        assert_eq!(actual, 3.1)
    }

    #[test]
    fn ten_million_iters() {
        let actual = rounded_to(DartBoard { iters: 10_000_000 }.calculate(), 2);
        assert_eq!(actual, 3.14)
    }
}
