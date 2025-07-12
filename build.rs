use std::{fs::File, io::Write};

fn main() -> std::io::Result<()> {
    let mut file = File::create("src/fib.rs")?;
    file.write_all(b"pub fn fib(n: u64) -> u64 {
    if n < 2 {
        return n;
    }
    return fib(n-1) + fib(n-2);
}")?;
    Ok(())
}
