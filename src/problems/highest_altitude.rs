pub fn largest_altitude(gain: Vec<i32>) -> i32 {
    let mut altitude: Vec<i32> = vec![];

    let mut sum = 0;
    altitude.push(sum);

    for &element in &gain {
        sum += element;
        altitude.push(sum);
    }
    let max = altitude.iter().max().unwrap().to_owned();

    return max;
}

pub fn other_solution(gain: Vec<i32>) -> i32 {
    let mut h = 0;
    let mut a = 0;
    for i in gain {
        a += i;
        h = a.max(h);
        // if a > h {
        //     h = a;
        // }
    }
    h
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_largest_altitude() {
        let gain = vec![-5, 1, 5, 0, -7];

        assert_eq!(other_solution(gain), 1);
    }
}
