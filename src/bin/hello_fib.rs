use fib_o1::{Fib, OutOfBoundsError};
fn main() -> Result<(), OutOfBoundsError<u32>> {
    let mut i = 0u32;
    while let Ok(out) = u32::fib(i) {
        println!("{out}");
        i += 1;
    }
    Ok(())
}
