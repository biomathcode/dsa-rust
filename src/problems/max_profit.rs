pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut max = 0;

    let mut first = 0;

    let mut last = prices.len();

    // while
    // todo:

    todo!()
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_max_profit() {
        let input = vec![7, 1, 5, 3, 6, 4];
        // You must buy at a lower price
        // sell at a higher price

        assert_eq!(max_profit(input), 5)
    }
}
