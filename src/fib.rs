pub fn fib(n: u64) -> u64 {
    if n < 2 {
        return n;
    }
    return fib(n-1) + fib(n-2);
}