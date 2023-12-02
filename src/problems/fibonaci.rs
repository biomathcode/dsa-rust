pub fn fibonacci_reccursive(n: i32) -> i32 {
    match n {
        0 => panic!("No Fibonacci for zero"),
        1 | 2 => 1,
        3 => 2,
        _ => fibonacci_reccursive(n - 1) + fibonacci_reccursive(n - 2),
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_fibonaci() {
        assert_eq!(fibonacci_reccursive(4), 3)
    }
}
