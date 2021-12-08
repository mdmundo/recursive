fn fib(n: usize) -> usize {
    match n {
        0 | 1 => n,
        _ => fib(n - 1) + fib(n - 2),
    }
}

#[cfg(test)]
mod fib_tests {
    use super::*;

    #[test]
    fn fib_0() {
        let result = fib(0);
        assert_eq!(result, 0);
    }

    #[test]
    fn fib_1() {
        let result = fib(1);
        assert_eq!(result, 1);
    }

    #[test]
    fn fib_2() {
        let result = fib(2);
        assert_eq!(result, 1);
    }

    #[test]
    fn fib_3() {
        let result = fib(3);
        assert_eq!(result, 2);
    }

    #[test]
    fn fib_4() {
        let result = fib(4);
        assert_eq!(result, 3);
    }

    #[test]
    fn fib_5() {
        let result = fib(5);
        assert_eq!(result, 5);
    }

    #[test]
    fn fib_6() {
        let result = fib(6);
        assert_eq!(result, 8);
    }
}
