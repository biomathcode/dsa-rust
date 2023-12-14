use std::collections::{ HashMap, HashSet };

pub fn unique_occurrences(arr: Vec<i32>) -> bool {
    let mut occurrences_cound: HashMap<i32, i32> = HashMap::new();

    for &num in &arr {
        *occurrences_cound.entry(num).or_insert(0) += 1;
    }

    let mut unique_occurrences: HashSet<_> = HashSet::new();

    for &count in occurrences_cound.values() {
        if !unique_occurrences.insert(count) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_unique_occurrences() {
        let input = vec![1, 2, 2, 1, 1, 3];

        assert_eq!(unique_occurrences(input), true);
    }
}
