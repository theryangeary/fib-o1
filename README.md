# Constant Time Fibonacci Sequence Values

`fib_o1` provides a `fn fib(n)` which produces a member of the Fibonacci
sequence in constant (O(1)) time, up to a compile-time defined upper limit (or
limits imposed by your chosen datatype).

`fn fib(n)` works will all rust primitive unsigned integer types as well as with
`num::BigInt` and `num::BigUint` from
[`num_bigint`](https://docs.rs/num-bigint/latest/num_bigint/).

# Why?

To flex about how fast it is. It's almost as fast as [writing to
/dev/null](https://www.youtube.com/watch?v=b2F-DItXtZs&t=102s).

Realistically it is mostly pointless. Ask your local mathmatician about
practical applications.

# Performance

`fib_o1` will *always* give the requested result in constant O(1) time.

Comparison of alternative algorithms:

1. Naive, recursive calculations take O(2^n) time.
2. Recursive with memoization takes O(n) time.
3. Sequential generation (i.e. calculating all values in the sequence up to n) also takes O(n) time but improves upon recursive with memoization by using O(1) memory as well.
4. Using matrix multiplication can yield an option that performs O(log n) time.
5. And finally, there seems to be [a formula](https://r-knott.surrey.ac.uk/Fibonacci/fibFormula.html) which achieves O(1) time but thats not a stupid and funny as just generating all the values at compile time, so we'll choose to ignore that. Also, it relies on an irrational number constant and this may lead to rounding errors.

# Tradeoffs

## Compile Time

Producing O(1) Fibonacci sequence members does require some leg work[^1] at
compile time, which can impact build times depending on what maximum sequence
value you require. Just remember how much time you will save at runtime and
[have a sword fight](https://xkcd.com/303/).

On my 2021 MacBook Pro with M1 Pro chip, compile times for fib() up to:

| 2^ | time |
|-----------|------|
|12|negligible|
|13|~10s|
|14|~1m20s|
|15|the rust compiler complains that files larger than a certain size are unsupported|

## Binary Size

You may think encoding many Fibonacci sequence numbers into your binary will
increase its size. However, when feature `bigint` is off, the bin size of
`bin/hello_fib.rs` is the same as `bin/hello_world.rs` (around 400kb).

When feature `bigint` is on, however, the binary size will start to be
impacted[^2].

| features | bin size (kb) |
|----------|---------------|
| bigint,pow10 | 678 |
| bigint,pow11 | 1,090 |
| bigint,pow12 | 2,577 |
| bigint,pow13 | 8,306 |
| bigint,pow14 | 30,267 |


# Footnotes

[^1]: read: generating a static data lookup table
[^2]: as measured by `cargo build --release --no-default-features --features
    pow13,bigint --bin hello_big_fib 2&>/dev/null && ls -la
    target/release/hello_big_fib`. Please go ahead and roast my methodology
    [here](https://github.com/theryangeary/fib-o1/issues/new)
