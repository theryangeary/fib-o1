include!(concat!(env!("OUT_DIR"), "/fib.rs"));
mod error;

pub use error::OutOfBoundsError;
