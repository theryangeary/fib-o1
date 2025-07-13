mod error;

#[cfg(feature = "codegen-inplace")]
mod fib;
#[cfg(not(feature = "codegen-inplace"))]
include!(concat!(env!("OUT_DIR"), "/fib.rs"));


pub use error::OutOfBoundsError;

#[cfg(feature = "codegen-inplace")]
pub use fib::MAX_INPUT;

pub trait Fib<I, O> {
    fn fib(t: I) -> Result<O, crate::OutOfBoundsError<I>>;
}
