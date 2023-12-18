pub fn max_vowels(s: String, k: i32) -> i32 {
    // Convert the input string to a vector of ASCII values and subtract 'a' to get the index in the frequency array
    let s = s
        .bytes()
        .map(|b| (b - b'a') as usize)
        .collect::<Vec<_>>();
    let k = k as usize;

    // Initialize an array to mark vowels as 1 and consonants as 0
    let is_vowels = "aeiou".bytes().fold([0; (b'z' - b'a' + 1) as usize], |mut is_vowels, c| {
        is_vowels[(c - b'a') as usize] = 1;
        is_vowels
    });

    // Initialize the number of vowels in the first window of length k-1
    let mut n_vowels_in_window = s
        .iter()
        .take(k - 1)
        .map(|c| is_vowels[*c])
        .sum::<i32>();
    let mut max_vowels = n_vowels_in_window;

    // Iterate over each window of length k in the input string
    for (right, right_c) in s
        .iter()
        .enumerate()
        .skip(k - 1) {
        n_vowels_in_window += is_vowels[*right_c]; // Add the next character to the window
        max_vowels = max_vowels.max(n_vowels_in_window); // Update the maximum number of vowels seen so far

        let left = (right as i32) - (k as i32) + 1; // Calculate the index of the leftmost character in the current window
        let left_c = s[left as usize]; // Get the leftmost character in the current window
        n_vowels_in_window -= is_vowels[left_c]; // Remove the leftmost character from the window
    }

    max_vowels // Return the maximum number of vowels seen in any window of length k
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_max_vowels() {
        let input = String::from("abciiidef");

        let k = 3;

        assert_eq!(max_vowels(input, k), 3)
    }

    #[test]
    fn test_max_vowels_two() {
        let input = String::from("aeiou");

        let k = 2;

        assert_eq!(max_vowels(input, k), 2)
    }
}
