pub fn gcd_of_strings(str1: String, str2: String) -> String {
    "".to_string()
}
// TODO
#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_greatest_common_divisor() {
        let str1 = String::from("ABCABC");
        let str2 = String::from("ABC");

        assert_eq!(gcd_of_strings(str1, str2), String::from("ABC"));
    }
}
