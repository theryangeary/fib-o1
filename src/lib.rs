mod error;

#[cfg(feature = "codegen-inplace")]
mod fib;
#[cfg(not(feature = "codegen-inplace"))]
include!(concat!(env!("OUT_DIR"), "/fib.rs"));


pub use error::OutOfBoundsError;

pub trait Fib<T> {
    fn fib(t: u64) -> Result<T, crate::OutOfBoundsError<u64>>;
}
