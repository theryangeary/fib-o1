/*! O(1) generation of arbitrary Fibonacci sequence values

This crate allows constant time Fibonacci lookup by front-loading the sequence calculation to compile time.

# Usage

## Cargo

Add to Cargo.toml:
```toml
[dependencies]
fib-o1 = {version = "0.1.1", features = ["bigint", "pow12"]}
```

Choose a `pow<N>` between `pow10` and `pow14`. 2^N represents the highest input `fib(n)` can handle at runtime, and comes at the expense of compile time.

## Code

```rust
use fib_o1::Fib;

assert_eq!(u8::fib(0u8).unwrap(), 0);
assert_eq!(u8::fib(1u8).unwrap(), 1);
assert_eq!(u8::fib(2u8).unwrap(), 1);
assert_eq!(u8::fib(3u8).unwrap(), 2);
assert_eq!(u8::fib(4u8).unwrap(), 3);
assert_eq!(u8::fib(5u8).unwrap(), 5);
assert_eq!(u8::fib(6u8).unwrap(), 8);
assert_eq!(u8::fib(7u8).unwrap(), 13);
```

## OutOfBoundsError

Based on the maximum value a number type can represent, and based on your selected `pow<N>` feature, certain inputs will produce error results.
```rust
use fib_o1::Fib;

// max value for a u8 is 255, but fib(20) > 255, so we get an error
assert!(u8::fib(20u8).is_err())
```

## Generics

Any unsigned integer types work with [`Fib`], and in larger programs the compiler should be able to determine the necessary types, but if you need to specify, the format is like `<output_type>::fib(n)` and if n is a literal, its type can be specified with a type suffix, for example `3u8` is the value `3` with type `u8`.

## Arbitrary sized integers (BigInts)

By using feature `bigint`, you can enable arbitrary sized integers which can be helpful for large values of `n`.

```rust
use fib_o1::Fib;
use num_bigint::BigUint;

assert_eq!(BigUint::fib(2048u16).unwrap(), BigUint::parse_bytes(b"45415304437437894250455714462906892027009082612936444289511823902789714525092834356843497180347717304332077420750102996639625006407838018797363807741815915794968069489957662592260489596860563484362187663942834824730009793065752175759244081518806465182648002219755758995565516482064617351513826704211517343602925990599710229276939710372081414109914714493582044185153918055170241694035610145547104337536614028338983073680262684101", 10).unwrap());
```
*/

mod error;

#[cfg(feature = "codegen-inplace")]
mod fib;
#[cfg(not(feature = "codegen-inplace"))]
include!(concat!(env!("OUT_DIR"), "/fib.rs"));


pub use error::OutOfBoundsError;

#[cfg(feature = "codegen-inplace")]
pub use fib::MAX_INPUT;

/// 
pub trait Fib<I, O> {
    fn fib(t: I) -> Result<O, crate::OutOfBoundsError<I>>;
}
