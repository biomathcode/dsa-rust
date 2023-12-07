pub fn total_money(n: i32) -> i32 {
    let mut amount = 0;
    let mut curr_money = 1;
    let mut weeks = 0;

    for i in 0..n {
        amount += curr_money;
        curr_money += 1;

        if i % 7 == 6 {
            weeks += 1;
            curr_money = 1;
            curr_money += weeks;
        }
    }

    amount
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_() {
        let days = 10;

        assert_eq!(total_money(days), 37)
    }
}
