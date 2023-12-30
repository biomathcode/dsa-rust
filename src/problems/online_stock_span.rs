struct StockSpanner {
    prices: Vec<i32>,
    spans: Vec<i32>,
}

impl StockSpanner {
    fn new() -> Self {
        StockSpanner {
            prices: Vec::new(),
            spans: Vec::new(),
        }
    }

    fn next(&mut self, price: i32) -> i32 {
        self.prices.push(price);
        let mut span = 1;
        let mut i = (self.prices.len() as i32) - 2;

        while i >= 0 && self.prices[i as usize] <= price {
            span += self.spans[i as usize];
            i -= self.spans[i as usize];
        }

        self.spans.push(span);
        span
    }
}
