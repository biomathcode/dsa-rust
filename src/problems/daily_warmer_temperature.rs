use std::convert::TryInto;

pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut result = vec![0; temperatures.len()];
    let mut stack: Vec<usize> = Vec::new(); // pair: [temp, index]

    for (i, &num) in temperatures.iter().enumerate() {
        while let Some(&top) = stack.last() {
            if num > temperatures[top as usize] {
                let index = stack.pop().unwrap() as i32;
                result[index as usize] = (i as i32) - index;
            } else {
                break;
            }
        }

        stack.push(i.try_into().unwrap());
    }

    result
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_daily_temperatur() {
        let temperatures = Vec::from([73, 74, 75, 71, 69, 72, 76, 73]);
        let output = [1, 1, 4, 2, 1, 1, 0, 0];

        assert_eq!(daily_temperatures(temperatures), output);
    }

    #[test]
    fn test_daily_temperatur_two() {
        let temperatures = Vec::from([30, 40, 50, 60]);
        let output = [1, 1, 1, 0];

        assert_eq!(daily_temperatures(temperatures), output);
    }
}
