// We will have three points i, k, n
// we will use j for the repeated char

pub fn compress(chars: &mut Vec<char>) -> i32 {
    let (mut i, mut k, n) = (0, 0, chars.len());

    while i < n {
        let mut j = i + 1;

        while j < n && chars[j] == chars[i] {
            j += 1;
        }

        chars[k] = chars[i];
        k += 1;

        if j - i > 1 {
            let cnt = (j - i).to_string();

            for c in cnt.chars() {
                chars[k] = c;
                k += 1;
            }
        }

        println!("{:?}", chars);

        i = j;
    }

    println!("{:?}", chars);

    k as i32
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
