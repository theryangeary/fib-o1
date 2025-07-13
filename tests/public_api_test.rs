use fib_o1::Fib;
#[cfg(feature = "bigint")]
use num_bigint::{BigInt, ToBigInt};

const FIB93_U64: u64 = 12200160415121876738;


#[test]
fn test_fib_u64_from_u16() {
    assert_eq!(u64::fib(2u16).unwrap(), 1);
    assert_eq!(u64::fib(3u16).unwrap(), 2);
    assert_eq!(u64::fib(4u16).unwrap(), 3);
    assert_eq!(u64::fib(93u16).unwrap(), FIB93_U64);

    assert!(u64::fib(94_u16).is_err());
}

#[test]
fn test_fib_u64_from_u32() {
    assert_eq!(u64::fib(2u32).unwrap(), 1);
    assert_eq!(u64::fib(3u32).unwrap(), 2);
    assert_eq!(u64::fib(4u32).unwrap(), 3);
    assert_eq!(u64::fib(93u32).unwrap(), FIB93_U64);

    assert!(u64::fib(94_u32).is_err());
}

#[test]
fn test_fib_u64_from_u64() {
    assert_eq!(u64::fib(0u64).unwrap(), 0);
    assert_eq!(u64::fib(1u64).unwrap(), 1);
    assert_eq!(u64::fib(2u64).unwrap(), 1);
    assert_eq!(u64::fib(3u64).unwrap(), 2);
    assert_eq!(u64::fib(4u64).unwrap(), 3);
    assert_eq!(u64::fib(5u64).unwrap(), 5);
    assert_eq!(u64::fib(6u64).unwrap(), 8);
    assert_eq!(u64::fib(7u64).unwrap(), 13);
    assert_eq!(u64::fib(8u64).unwrap(), 21);
    assert_eq!(u64::fib(9u64).unwrap(), 34);
    assert_eq!(u64::fib(10u64).unwrap(), 55);
    assert_eq!(u64::fib(20u64).unwrap(), 6765);
    assert_eq!(u64::fib(30u64).unwrap(), 832040);
    assert_eq!(u64::fib(40u64).unwrap(), 102334155);
    assert_eq!(u64::fib(93u64).unwrap(), FIB93_U64);
}


#[test]
fn test_fib_u64_from_u128() {
    assert_eq!(u64::fib(2u128).unwrap(), 1);
    assert_eq!(u64::fib(3u128).unwrap(), 2);
    assert_eq!(u64::fib(4u128).unwrap(), 3);
    assert_eq!(u64::fib(93u128).unwrap(), FIB93_U64);

    assert!(u64::fib(94_u128).is_err());
}

#[test]
fn test_fib_u64_from_usize() {
    assert_eq!(u64::fib(2usize).unwrap(), 1);
    assert_eq!(u64::fib(3usize).unwrap(), 2);
    assert_eq!(u64::fib(4usize).unwrap(), 3);
    assert_eq!(u64::fib(93usize).unwrap(), FIB93_U64);

    assert!(u64::fib(94_usize).is_err());
}

#[test]
fn test_fib_err() {
    #[cfg(not(feature = "bigint"))]
    assert!(u64::fib(94u64).is_err());
}

#[cfg(feature = "bigint")]
#[test]
fn test_fib_bigint() {
    assert_eq!(BigInt::fib(0).unwrap(), 0.to_bigint().unwrap());
    assert_eq!(BigInt::fib(1).unwrap(), 1.to_bigint().unwrap());
    assert_eq!(BigInt::fib(2).unwrap(), 1.to_bigint().unwrap());
    assert_eq!(BigInt::fib(3).unwrap(), 2.to_bigint().unwrap());
    assert_eq!(BigInt::fib(4).unwrap(), 3.to_bigint().unwrap());
    assert_eq!(BigInt::fib(5).unwrap(), 5.to_bigint().unwrap());
    assert_eq!(BigInt::fib(6).unwrap(), 8.to_bigint().unwrap());
    assert_eq!(BigInt::fib(7).unwrap(), 13.to_bigint().unwrap());
    assert_eq!(BigInt::fib(8).unwrap(), 21.to_bigint().unwrap());
    assert_eq!(BigInt::fib(9).unwrap(), 34.to_bigint().unwrap());
    assert_eq!(BigInt::fib(10).unwrap(), 55.to_bigint().unwrap());
    assert_eq!(BigInt::fib(20).unwrap(), 6765.to_bigint().unwrap());
    assert_eq!(BigInt::fib(30).unwrap(), 832040.to_bigint().unwrap());
    assert_eq!(BigInt::fib(40).unwrap(), 102334155.to_bigint().unwrap());
    assert_eq!(BigInt::fib(93).unwrap(), 12200160415121876738u128.to_bigint().unwrap());
    assert_eq!(BigInt::fib(99).unwrap(), 218922995834555169026u128.to_bigint().unwrap());
}
