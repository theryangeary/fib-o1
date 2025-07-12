use fib_o1::fib;

#[test]
fn test_fib() {
    assert_eq!(fib(0).unwrap(), 0);
    assert_eq!(fib(1).unwrap(), 1);
    assert_eq!(fib(2).unwrap(), 1);
    assert_eq!(fib(3).unwrap(), 2);
    assert_eq!(fib(4).unwrap(), 3);
    assert_eq!(fib(5).unwrap(), 5);
    assert_eq!(fib(6).unwrap(), 8);
    assert_eq!(fib(7).unwrap(), 13);
    assert_eq!(fib(8).unwrap(), 21);
    assert_eq!(fib(9).unwrap(), 34);
    assert_eq!(fib(10).unwrap(), 55);
    assert_eq!(fib(20).unwrap(), 6765);
    assert_eq!(fib(30).unwrap(), 832040);
    assert_eq!(fib(40).unwrap(), 102334155);
    assert_eq!(fib(93).unwrap(), 12200160415121876738);

}

#[test]
fn test_fib_err() {
    #[cfg(not(feature = "big-int"))]
    assert!(fib(94).is_err());
}
