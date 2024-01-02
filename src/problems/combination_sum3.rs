pub fn backtrack(result: &mut Vec<Vec<i32>>, cur: &mut Vec<i32>, remain: i32, k: i32, i: i32) {
    if remain == 0 && (cur.len() as i32) == k {
        result.push(cur.to_vec());
        return;
    } else if remain < 0 || (cur.len() as i32) == k {
        return;
    }
    for num in i..=9 {
        cur.push(num);
        backtrack(result, cur, remain - num, k, num + 1);
        cur.pop();
    }
}

pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
    let mut result = vec![];
    backtrack(&mut result, &mut vec![], n, k, 1);
    result
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_combination_sum3() {
        let k = 3;
        let n = 7;

        let output = vec![vec![1, 2, 4]];

        assert_eq!(combination_sum3(k, n), output)
    }
}
