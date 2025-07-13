mod error;

#[cfg(feature = "codegen-inplace")]
mod fib;
#[cfg(not(feature = "codegen-inplace"))]
include!(concat!(env!("OUT_DIR"), "/fib.rs"));


pub use error::OutOfBoundsError;

pub trait Fib<I, O> {
    fn fib(t: I) -> Result<O, crate::OutOfBoundsError<I>>;
}
