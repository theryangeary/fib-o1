use num::bigint::BigInt;
use std::{fmt::Display, fs::File, io::Write, path::Path};

fn main() -> std::io::Result<()> {
    #[cfg(feature = "codegen-inplace")]
    let out_dir = "./src";
    #[cfg(not(feature = "codegen-inplace"))]
    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir).join("fib.rs");
    let mut file = File::create(out_path)?;


    #[cfg(feature = "codegen-inplace")]
    file.write_all(b"pub use crate::Fib;\n")?;

    #[cfg(feature = "bigint")]
    file.write_all(b"use num_bigint::{BigInt,ToBigInt};\n")?;

    implement_fib_for_type::<u64>(&mut file, "u64", "u64", 100)?;

    #[cfg(feature = "bigint")]
    implement_fib_for_type::<BigInt>(&mut file, "BigInt", 100)?;

    println!("cargo::rerun-if-changed=build.rs");

    Ok(())
}

fn implement_fib_for_type<InputT>(file: &mut File, input_ty: &str, output_ty: &str, limit: u64) -> Result<(), std::io::Error>
where
    InputT: From<u64>
        + num::CheckedAdd
        + Display
        + std::ops::AddAssign<InputT>
        + Clone
        + std::cmp::PartialOrd,
{
    let result0 = get_result_ok_internal(&0u64,& 0u64, output_ty);
    let result1 = get_result_ok_internal(&0u64, &1u64, output_ty);
    file.write_all(
        format!(
            "impl Fib<{input_ty}, {output_ty}> for {output_ty} {{
    fn fib(n: {input_ty}) -> Result<{output_ty}, crate::OutOfBoundsError<u64>> {{
        match n {{
            0 => Ok({result0}),
            1 => Ok({result1}),\n"
        )
        .as_bytes(),
    )?;
    let mut a = InputT::from(0);
    let mut b = InputT::from(1);
    let mut i = 2u64;
    while i < limit && a.checked_add(&b).is_some() {
        let result = get_result_ok_internal(&a, &b, output_ty);
        file.write_all(format!("\t\t{i} => Ok({result}),\n",).as_bytes())?;
        let c = b.clone();
        b = a + b;
        a = c.clone();
        i += 1;
    }
    file.write_all(
        b"\t\t_ => Err(crate::OutOfBoundsError::from(n)),
        }
    }
}\n\n",
    )?;
    Ok(())
}

fn get_result_ok_internal<I>(a: &I, b: &I, ty: &str) -> String
where
    I: num::CheckedAdd
        + Display
        + std::ops::AddAssign<I>
        + Clone
        + std::cmp::PartialOrd,
{
    match ty {
        "BigInt" => format!("{}u128.to_bigint().unwrap()", a.clone() + b.clone()),
        _ => format!("{}_{ty}", a.clone() + b.clone()),
    }
}
