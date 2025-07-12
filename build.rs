use std::{env, fs::File, io::Write, path::Path};


fn main() -> std::io::Result<()> {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir).join("fib.rs");
    let mut file = File::create(out_path)?;

    file.write_all(b"pub fn fib(n: u64) -> Result<u64, crate::OutOfBoundsError> {
    match n {
        0 => Ok(0),
        1 => Ok(1),")?;

    let mut a = 0u64;
    let mut b = 1;
    let mut i = 2;

    while a.checked_add(b).is_some() {
        file.write_all(format!("\t\t{i} => Ok({}),\n", a+b).as_bytes())?;
        let c = b;
        b = a + b;
        a = c;
        i += 1;
    }

    file.write_all(b"
        _ => Err(crate::OutOfBoundsError::from(n)),
    }
}")?;

    println!("cargo::rerun-if-changed=build.rs");
    
    Ok(())
}
