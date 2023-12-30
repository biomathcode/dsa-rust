pub fn predict_party_victory(senate: String) -> String {
    let mut stack: Vec<char> = vec![];
    let mut queue: Vec<char> = senate.chars().collect();

    while !queue.is_empty() {
        let top = queue.remove(0);

        if stack.is_empty() || *stack.last().unwrap() == top {
            stack.push(top);
        } else {
            queue.push(stack.pop().unwrap());
        }
    }

    match stack.pop().unwrap() {
        'R' => "Radiant".to_string(),
        _ => "Dire".to_string(),
    }
}

/*
Vecque solution
use std::collections::VecDeque;
impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
    // Count of each party's senators
    let mut r_count = senate.chars().filter(|&c| c == 'R').count();
    let mut d_count = senate.len() - r_count;

    // Floating ban count for each party
    let (mut d_floating_ban,mut r_floating_ban) = (0u16,0u16);

    // Queue of senators
    let mut q: VecDeque<char> = senate.chars().collect();

    // While both parties have senators
    while r_count > 0 && d_count > 0 {
        // Pop the senator with turn
        if let Some(curr) = q.pop_front() {
            // Logic for each party's actions
            match (curr, d_floating_ban, r_floating_ban) {
                ('D', 0, _) => {
                    r_floating_ban += 1;
                    q.push_back(curr);
                }
                ('D', _, _) => {
                    d_floating_ban -= 1;
                    d_count -= 1;
                }
                ('R', _, 0) => {
                    d_floating_ban += 1;
                    q.push_back(curr);
                }
                ('R', _, _) => {
                    r_floating_ban -= 1;
                    r_count -= 1;
                }
                _ => {}
            }
        }
    }
    // Determine the winner
    if r_count == 0 { "Dire" } else { "Radiant" }.to_string()
}
}

*/

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_party_test() {
        let senate = String::from("RD");
        let output = String::from("Radiant");

        assert_eq!(predict_party_victory(senate), output)
    }
}
