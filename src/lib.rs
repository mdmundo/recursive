#[cfg(test)]
mod fact_tests;
#[cfg(test)]
mod fib_tests;

fn fib(n: usize) -> usize {
    match n {
        0 | 1 => n,
        _ => fib(n - 1) + fib(n - 2),
    }
}

fn fact(n: usize) -> usize {
    match n {
        0 => 1,
        _ => n * fact(n - 1),
    }
}
