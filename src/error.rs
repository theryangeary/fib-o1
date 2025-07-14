use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct OutOfBoundsError<T> {
    #[allow(dead_code)]
    n: T
}

impl<T> Display for OutOfBoundsError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fib_o1 not compiled with support for fib()")
    }
}

impl<T> From<T> for OutOfBoundsError<T> {
    fn from(n: T) -> Self {
        Self{n}
    }
}
