use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct OutOfBoundsError {
    n: u64
}

impl Display for OutOfBoundsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fib_o1 not compiled with support for fib({})", self.n)
    }
}

impl From<u64> for OutOfBoundsError {
    fn from(n: u64) -> Self {
        Self{n}
    }
}
