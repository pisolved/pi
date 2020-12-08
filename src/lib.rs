mod collisions;
mod coprime;
mod dartboard;

pub trait Pi<T> {
    fn calculate(&self) -> T;
}

pub fn rounded_to(input: f64, digit: i32) -> f64 {
    let pow = 10f64.powi(digit);
    (input * pow).round() / pow
}
