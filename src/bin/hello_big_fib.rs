use fib_o1::{Fib, OutOfBoundsError};
use num_bigint::BigInt;
fn main() -> Result<(), OutOfBoundsError<u32>> {
    let mut i = 0u32;
    while let Ok(out) = BigInt::fib(i) {
        println!("{out}");
        i += 1;
    }
    Ok(())
}
