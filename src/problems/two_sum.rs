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

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let ve = vec![2, 7, 11, 15];
        let result = two_sum(ve, 9);
        assert_eq!(result, [0, 1], "we are testing addition with {:?} and ", result);
    }
    #[test]
    fn test_two() {
        let ve = vec![3, 2, 4];

        let res = two_sum(ve, 6);

        assert_eq!(res, [1, 2], "we are testing addition with {:?} and ", res);
    }
}
