pub fn can_place_flowers(mut flowerbed: Vec<i32>, n: i32) -> bool {
    flowerbed.push(0);
    flowerbed.insert(0, 0);
    flowerbed
        .split(|&k| k == 1)
        .map(|v| (v.len() - 1) / 2)
        .sum::<usize>() >= (n as usize)
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_can_place_flowers() {
        let i = vec![1, 0, 0, 0, 1];

        assert_eq!(can_place_flowers(i, 1), true);
    }

    #[test]
    fn test_can_place_flowers_two() {
        let i = vec![1, 0, 0, 0, 1];

        assert_eq!(can_place_flowers(i, 2), false);
    }

    #[test]
    fn test_can_place_flowers_three() {
        let i = vec![0, 1, 0];

        assert_eq!(can_place_flowers(i, 1), false);
    }
}
