use super::Pi;

pub struct Collider {
    pub digits: i32,
}

impl Pi<usize> for Collider {
    fn calculate(&self) -> usize {
        let mut clacks = 0;
        let mut little_block = Block::new(1.0, 0.0);
        let mut big_block = Block::new(100f64.powi(self.digits - 1), -1.);

        while big_block.velocity <= 0. || little_block.velocity.abs() > big_block.velocity.abs() {
            let v1 = little_block.collide(&big_block);
            let v2 = big_block.collide(&little_block);
            little_block.velocity = v1;
            big_block.velocity = v2;
            clacks += 1;
            if little_block.velocity < 0. {
                clacks += 1;
                little_block.velocity *= -1.0;
            }
        }
        clacks
    }
}

struct Block {
    mass: f64,
    velocity: f64,
}

impl Block {
    fn new(mass: f64, velocity: f64) -> Self {
        Self { mass, velocity }
    }

    fn collide(&self, other: &Self) -> f64 {
        let m1 = self.mass;
        let m2 = other.mass;
        let u1 = self.velocity;
        let u2 = other.velocity;
        (m1 - m2) / (m1 + m2) * u1 + (2.0 * m2) / (m1 + m2) * u2
    }
}

#[cfg(test)]
mod tests {
    use super::{Collider, Pi};
    #[test]
    fn one_digit() {
        assert_eq!(Collider { digits: 1 }.calculate(), 3);
    }
    #[test]
    fn two_digits() {
        assert_eq!(Collider { digits: 2 }.calculate(), 31);
    }
    #[test]
    fn three_digits() {
        assert_eq!(Collider { digits: 3 }.calculate(), 314);
    }
    #[test]
    fn nine_digits() {
        let expected = 314159265;
        assert_eq!(Collider { digits: 9 }.calculate(), expected);
    }
}
