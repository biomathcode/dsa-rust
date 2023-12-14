pub fn compress(chars: &mut Vec<char>) -> i32 {
    todo!()
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_compress() {
        let mut input: Vec<char> = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];

        let result = 6;

        assert_eq!(compress(&mut input), result);
    }

    #[test]
    fn test_compress_two() {
        let mut input: Vec<char> = vec!['a'];

        let result = 1;

        assert_eq!(compress(&mut input), result);
    }
}
