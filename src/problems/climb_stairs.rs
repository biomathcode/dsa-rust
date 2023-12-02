pub fn climb_stairs(n: i32) -> i32 {
    if n == 1 {
        return 1;
    }
    if n == 2 {
        return 2;
    }
    let mut prev2 = 1;
    let mut prev1 = 2;
    for _i in 3..n {
        let temp = prev1 + prev2;
        prev2 = prev1;
        prev1 = temp;
    }
    prev1 + prev2
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_climb_stairs() {
        assert_eq!(climb_stairs(2), 2)
    }
}
