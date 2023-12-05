pub fn largest_good_integer(num: String) -> String {
    // return the larget good integer
    // if no 3 digit same integer than return
    // ""
    // algo
    // split the string in 3
    // return the max of them

    num.to_string();

    let mut sub_string: Vec<i32> = vec![];

    for i in 0..num.len() - 2 {
        let sub = &num[i..i + 3];

        if
            sub.chars().nth(0).unwrap() == sub.chars().nth(1).unwrap() &&
            sub.chars().nth(1).unwrap() == sub.chars().nth(2).unwrap()
        {
            let number: i32 = sub.parse().unwrap();

            sub_string.push(number);
        }
    }

    if sub_string.is_empty() {
        return String::new();
    } else {
        let mut el: String = sub_string.iter().max().unwrap().to_string();

        if el == "0" {
            el = String::from("000");
        }

        return el;
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_largest_good_integer() {
        let input = String::from("6777133339");

        let output = String::from("777");

        assert_eq!(largest_good_integer(input), output)
    }
}
