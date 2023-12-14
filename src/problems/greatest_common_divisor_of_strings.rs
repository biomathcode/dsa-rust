// Solution by GCD

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

pub fn gcd_of_strings(str1: String, str2: String) -> String {
    if str1.clone() + &str2 != str2.clone() + &str1 {
        return String::from("");
    }

    let gcd_len = gcd(str1.len(), str2.len());

    str1[..gcd_len].to_string()
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
