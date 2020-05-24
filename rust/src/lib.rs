mod collisions;
mod dartboard;

pub trait Pi<T> {
    fn calculate(&self) -> T;
}
