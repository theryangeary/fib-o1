use num::bigint::BigInt;
use std::{fmt::Display, fs::File, io::Write, path::Path};

macro_rules! impl_fib {
    ($file:ident, $input_ty:ty, $output_ty:ty) => {
        implement_fib_for_type::<$input_ty, $output_ty>(&mut $file, stringify!($input_ty), stringify!($output_ty), <$input_ty>::MAX)?;
    };
    ( $file:ident, $($input_ty:ty),+ => $output_ty:ty) => {
        $(
            impl_fib!($file, $input_ty, $output_ty);
        )+
    };
}

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

    impl_fib!(file, u8, u16, u32, u64, u128, usize => u8);
    impl_fib!(file, u8, u16, u32, u64, u128, usize => u16);
    impl_fib!(file, u8, u16, u32, u64, u128, usize => u32);
    impl_fib!(file, u8, u16, u32, u64, u128, usize => u64);
    impl_fib!(file, u8, u16, u32, u64, u128, usize => u128);
    impl_fib!(file, u8, u16, u32, u64, u128, usize => usize);

    #[cfg(feature = "bigint")]
    implement_fib_for_type::<u64, BigInt>(&mut file, "u64", "BigInt", 100)?;

    println!("cargo::rerun-if-changed=build.rs");

    Ok(())
}

fn implement_fib_for_type<I, O>(
    file: &mut File,
    input_ty: &str,
    output_ty: &str,
    limit: I,
) -> Result<(), std::io::Error>
where
    I: From<u8> + num::CheckedAdd + Display + std::ops::AddAssign<I> + Clone + std::cmp::PartialOrd,
    O: From<u8> + num::CheckedAdd + Display + std::ops::AddAssign<O> + Clone + std::cmp::PartialOrd,
{
    let result0 = get_result_ok_internal(&0u64, &0u64, output_ty);
    let result1 = get_result_ok_internal(&0u64, &1u64, output_ty);
    file.write_all(
        format!(
            "impl Fib<{input_ty}, {output_ty}> for {output_ty} {{
    fn fib(n: {input_ty}) -> Result<{output_ty}, crate::OutOfBoundsError<{input_ty}>> {{
        match n {{
            0 => Ok({result0}),
            1 => Ok({result1}),\n"
        )
        .as_bytes(),
    )?;
    let mut a = O::from(0);
    let mut b = O::from(1);
    let mut i = I::from(2);
    while i < limit && a.checked_add(&b).is_some() {
        let result = get_result_ok_internal(&a, &b, output_ty);
        file.write_all(format!("\t\t\t{i} => Ok({result}),\n",).as_bytes())?;
        let c = b.clone();
        b = a + b;
        a = c.clone();
        i += I::from(1);
    }
    file.write_all(
        b"\t\t\t_ => Err(crate::OutOfBoundsError::from(n)),
        }
    }
}\n\n",
    )?;
    Ok(())
}

fn get_result_ok_internal<I>(a: &I, b: &I, output_ty: &str) -> String
where
    I: num::CheckedAdd + Display + std::ops::AddAssign<I> + Clone + std::cmp::PartialOrd,
{
    match output_ty {
        "BigInt" => format!("{}u128.to_bigint().unwrap()", a.clone() + b.clone()),
        _ => format!("{}_{output_ty}", a.clone() + b.clone()),
    }
}
