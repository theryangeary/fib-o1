include!(concat!(env!("OUT_DIR"), "/fib.rs"));
mod error;

pub use error::OutOfBoundsError;

pub trait Fib<T> {
    fn fib(t: T) -> Result<T, crate::OutOfBoundsError>;
}
