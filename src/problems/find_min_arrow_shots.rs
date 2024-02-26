use std::cmp::min;

pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
    let mut res = 1;
    let mut points = points;
    points.sort_by_key(|x| x[0]);
    let mut end = points[0][1];
    for i in 1..points.len() {
        if points[i][0] > end {
            res += 1;
            end = points[i][1];
        } else {
            end = min(end, points[i][1]);
        }
    }

    res
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_() {
        let vec = vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]];

        let res = 2;

        assert_eq!(find_min_arrow_shots(vec), res);
    }
}
