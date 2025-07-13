use fib_o1::Fib;
#[cfg(feature = "bigint")]
use num_bigint::{BigInt, ToBigInt};

#[test]
fn test_fib() {
    assert_eq!(u64::fib(0).unwrap(), 0);
    assert_eq!(u64::fib(1).unwrap(), 1);
    assert_eq!(u64::fib(2).unwrap(), 1);
    assert_eq!(u64::fib(3).unwrap(), 2);
    assert_eq!(u64::fib(4).unwrap(), 3);
    assert_eq!(u64::fib(5).unwrap(), 5);
    assert_eq!(u64::fib(6).unwrap(), 8);
    assert_eq!(u64::fib(7).unwrap(), 13);
    assert_eq!(u64::fib(8).unwrap(), 21);
    assert_eq!(u64::fib(9).unwrap(), 34);
    assert_eq!(u64::fib(10).unwrap(), 55);
    assert_eq!(u64::fib(20).unwrap(), 6765);
    assert_eq!(u64::fib(30).unwrap(), 832040);
    assert_eq!(u64::fib(40).unwrap(), 102334155);
    assert_eq!(u64::fib(93).unwrap(), 12200160415121876738);
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

#[test]
fn test_fib_err() {
    #[cfg(not(feature = "bigint"))]
    assert!(u64::fib(94).is_err());
}
