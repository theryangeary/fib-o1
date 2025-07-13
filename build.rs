use num::bigint::{BigInt, BigUint};
use paste::paste;
use std::{fmt::Debug, fmt::Display, fs::File, io::Write, path::Path};

macro_rules! impl_fib {
    ($file:ident, $input_ty:ty, $output_ty:ty, $limit:expr) => {
        implement_fib_for_type::<$input_ty, $output_ty>(&mut $file, stringify!($input_ty), stringify!($output_ty), $limit)?;
    };
    ($file:ident, $input_ty:ty, $output_ty:ty) => {
        impl_fib!($file, $input_ty, $output_ty, <$input_ty>::MAX.try_into().unwrap());
    };
    ( $file:ident, $($input_ty:ty),+ => $output_ty:ty) => {
        $(
            impl_fib!($file, $input_ty, $output_ty);
        )+
    };
    ( $file:ident, $($input_ty:ty),+ => $output_ty:ty, $limit:expr) => {
        $(
            impl_fib!($file, $input_ty, $output_ty, $limit);
        )+
    };
}

macro_rules! limit {
    ($limiter:expr, $input_ty:expr, $limit:expr) => {
        match $input_ty {
            "u8" => $limiter <= u8::MAX as u128 && $limiter < $limit,
            "u16" => $limiter <= u16::MAX as u128 && $limiter < $limit,
            "u32" => $limiter <= u32::MAX as u128 && $limiter < $limit,
            "u64" => $limiter <= u64::MAX as u128 && $limiter < $limit,
            "u128" => $limiter <= u128::MAX as u128 && $limiter < $limit,
            "usize" => $limiter <= usize::MAX as u128 && $limiter < $limit,
            _ => panic!("Unsupported type: {}", $input_ty),
        }
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
    file.write_all(b"use num_bigint::{BigInt,BigUint,ToBigUint,ToBigInt};\n")?;

    impl_fib!(file, u8, u16, u32, u64, u128, usize => u8);
    impl_fib!(file, u8, u16, u32, u64, u128, usize => u16);
    impl_fib!(file, u8, u16, u32, u64, u128, usize => u32);
    impl_fib!(file, u8, u16, u32, u64, u128, usize => u64);
    impl_fib!(file, u8, u16, u32, u64, u128, usize => u128);
    impl_fib!(file, u8, u16, u32, u64, u128, usize => usize);

    let big_int_limit = u16::MAX / 16;

    #[cfg(feature = "bigint")]
    impl_fib!(file, u8, u16, u32, u64, u128, usize => BigInt, big_int_limit.try_into().unwrap());
    #[cfg(feature = "bigint")]
    impl_fib!(file, u8, u16, u32, u64, u128, usize => BigUint, big_int_limit.try_into().unwrap());

    println!("cargo::rerun-if-changed=build.rs");

    Ok(())
}

fn implement_fib_for_type<I, O>(
    file: &mut File,
    input_ty: &str,
    output_ty: &str,
    limit: u128,
) -> Result<(), std::io::Error>
where
    I: TryFrom<u16>
        + num::CheckedAdd
        + Display
        + std::ops::AddAssign<I>
        + Clone
        + std::cmp::PartialOrd,
    <I as TryFrom<u16>>::Error: Debug,
    O: TryFrom<u64>
        + num::CheckedAdd
        + Display
        + std::ops::AddAssign<O>
        + Clone
        + std::cmp::PartialOrd,
    <O as TryFrom<u64>>::Error: Debug,
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
    let mut a = O::try_from(0).unwrap();
    let mut b = O::try_from(1).unwrap();
    let mut limiter = 2;
    while limit!(limiter, input_ty, limit) && a.checked_add(&b).is_some() {
        let result = get_result_ok_internal(&a, &b, output_ty);
        file.write_all(format!("\t\t\t{limiter} => Ok({result}),\n",).as_bytes())?;
        let c = b.clone();
        b = a + b;
        a = c.clone();
        limiter += 1;
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
    I: num::CheckedAdd
        + Display
        + std::ops::AddAssign<I>
        + Clone
        + std::cmp::PartialOrd
        + TryFrom<u64>,
    <I as TryFrom<u64>>::Error: Debug,
{
    match output_ty {
        "BigInt" => {
            if a.clone() + b.clone()
                > I::try_from(u64::MAX).expect(&format!(
                    "expected <I> big enough to house u64::MAX, found: {}",
                    std::any::type_name::<I>()
                ))
            {
                format!(
                    "BigInt::parse_bytes(b\"{}\", 10).unwrap()",
                    a.clone() + b.clone()
                )
            } else {
                format!("{}u128.to_bigint().unwrap()", a.clone() + b.clone())
            }
        }
        "BigUint" => {
            if a.clone() + b.clone()
                > I::try_from(u64::MAX).expect(&format!(
                    "expected <I> big enough to house u64::MAX, found: {}",
                    std::any::type_name::<I>()
                ))
            {
                format!(
                    "BigUint::parse_bytes(b\"{}\", 10).unwrap()",
                    a.clone() + b.clone()
                )
            } else {
                format!("{}u128.to_biguint().unwrap()", a.clone() + b.clone())
            }
        }
        _ => format!("{}_{output_ty}", a.clone() + b.clone()),
    }
}
