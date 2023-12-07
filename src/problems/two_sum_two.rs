// TODO: Improve the speed of this solution
pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let len = numbers.len();
    let mut el = Vec::new();

    for i in 0..len {
        for j in i + 1..len {
            if numbers[i] + numbers[j] == target {
                el.push((i as i32) + 1);
                el.push((j as i32) + 1);
            }
        }
    }
    return el;
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_two_sum_II() {
        let ve = vec![2, 7, 11, 15];
        let target = 9;

        let output = vec![1, 2];

        assert_eq!(two_sum(ve, target), output, "They are equal")
    }
}
