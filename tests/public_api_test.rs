use fib_o1::fib;

#[test]
fn test_fib() {
    assert_eq!(fib(0), 0);
    assert_eq!(fib(1), 1);
    assert_eq!(fib(2), 1);
    assert_eq!(fib(3), 2);
    assert_eq!(fib(4), 3);
    assert_eq!(fib(5), 5);
    assert_eq!(fib(6), 8);
    assert_eq!(fib(7), 13);
    assert_eq!(fib(8), 21);
    assert_eq!(fib(9), 34);
    assert_eq!(fib(10), 55);
    assert_eq!(fib(20), 6765);
    assert_eq!(fib(30), 832040);
    assert_eq!(fib(40), 102334155);
}
