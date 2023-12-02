pub fn length_of_last_word(s: String) -> i32 {
    let e = s.trim();

    let words: Vec<&str> = e.split(" ").collect();

    let res = words.last().unwrap();

    res.len() as i32

    //
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_length_of_last_word() {
        let s = "Hello World";
        let output = 5;

        assert_eq!(output, length_of_last_word(String::from("Hello World")))
    }
}
