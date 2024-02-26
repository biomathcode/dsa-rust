use std::cmp::min;

pub fn erase_overlap_intervals(intervals: &mut Vec<Vec<i32>>) -> i32 {
    let mut res = 0;
    intervals.sort_by_key(|x| x[0]);
    let mut end = intervals[0][1];
    for i in 1..intervals.len() {
        if intervals[i][0] >= end {
            end = intervals[i][1];
        } else {
            res += 1;
            end = min(end, intervals[i][1]);
        }
    }
    res
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_() {
        let mut res = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]];

        assert_eq!(erase_overlap_intervals(&mut res), 1);
    }
}
