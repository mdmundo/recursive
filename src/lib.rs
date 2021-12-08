#[cfg(test)]
mod fact_tests;
#[cfg(test)]
mod fib_tests;

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
