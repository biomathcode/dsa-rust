pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    let mut a = cost[0];
    let mut b = cost[1];

    match cost.len() {
        2 => a.min(b),
        _ => {
            for i in 2..cost.len() as usize {
                let c = cost[i] + a.min(b);
                a = b;
                b = c;
            }
            a.min(b)
        }
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_min_cost_test() {
        let cost = vec![10, 15, 20];

        assert_eq!(min_cost_climbing_stairs(cost), 15)
    }
}
