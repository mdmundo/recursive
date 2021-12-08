// About tests, check: https://doc.rust-lang.org/stable/cargo/reference/cargo-targets.html#tests

pub fn fib(n: usize) -> usize {
    match n {
        0 | 1 => n,
        _ => fib(n - 1) + fib(n - 2),
    }
}

pub fn fact(n: usize) -> usize {
    match n {
        0 => 1,
        _ => n * fact(n - 1),
    }
}
