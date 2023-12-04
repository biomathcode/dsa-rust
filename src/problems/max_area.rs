pub fn max_area(height: Vec<i32>) -> i32 {
    let mut max_area = 0;
    let mut left = 0;
    let mut right = height.len() - 1;

    while left < right {
        let h_left = height[left];
        let h_right = height[right];

        let h = h_left.min(h_right);

        let w = (right - left) as i32;

        let area = h * w;

        max_area = max_area.max(area);

        if h_left < h_right {
            left += 1;
        } else {
            right -= 1;
        }
    }

    max_area
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_max_area() {
        let input = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        let output = 49;

        assert_eq!(max_area(input), output, "this is waht we are expecting ")
    }
}
