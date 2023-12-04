/**
pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut cols = HashSet::new();
    let mut rows = HashSet::new();
    let mut squares = HashSet::new();
    for row in 0..9 {
        for col in 0..9 {
            let chr = board[row][col];
            if chr.is_numeric() {
                if !cols.insert((col, chr)) {
                    return false;
                }
                if !rows.insert((row, chr)) {
                    return false;
                }
                if !squares.insert((row / 3, col / 3, chr)) {
                    return false;
                }
            }
        }
    }
    true
}

*/

pub fn is_valid_columns(board: &Vec<Vec<char>>, column: usize) -> bool {
    let mut seen = [false; 10];

    for i in 0..9 {
        let digit = board[i][column];

        println!("this is the digit: {:?}", digit);

        if digit == '.' {
            continue;
        }

        let digit = digit.to_digit(10).unwrap() as usize;

        if seen[digit] {
            return false;
        }
        seen[digit] = true;
    }
    true
}

pub fn is_valid_rows(arr: Vec<char>) -> bool {
    let mut seen = [false; 10];

    for i in 0..9 {
        let digit = arr[i];

        if digit == '.' {
            continue;
        }

        let digit = digit.to_digit(10).unwrap() as usize;

        if seen[digit] {
            return false;
        }
        seen[digit] = true;
    }

    true
}

pub fn is_valid_subgrid(board: &Vec<Vec<char>>, start_row: usize, start_col: usize) -> bool {
    let mut seen = [false; 10]; // Array to keep track of seen digits (1 to 9)

    for i in 0..3 {
        for j in 0..3 {
            let digit = board[start_row + i][start_col + j];

            // Skip empty cells (you might want to adjust this based on your actual representation)
            if digit == '.' {
                continue;
            }

            let digit = digit.to_digit(10).unwrap() as usize;

            // Check if the digit is already seen in the subgrid
            if seen[digit] {
                return false; // Digit is repeated
            }

            seen[digit] = true;
        }
    }

    true // Subgrid is valid
}

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    // each row should contain the digits 1-9 without repetition
    // Each column must contain the digits 1-9 without repetition.
    //Each of the nine 3 x 3 sub-boxes of the grid must contain the digits 1-9 without repetition.

    for i in 0..board.len() {
        if !is_valid_rows(board[i].clone()) {
            return false;
        }

        if !is_valid_columns(&board, i) {
            return false;
        }
    }

    for i in (0..9).step_by(3) {
        for j in (0..9).step_by(3) {
            if !is_valid_subgrid(&board, i, j) {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_valid_sudoku() {
        let board: Vec<Vec<char>> = vec![
            ['5', '3', '.', '.', '7', '.', '.', '.', '.'].to_vec(),
            ['6', '.', '.', '1', '9', '5', '.', '.', '.'].to_vec(),
            ['.', '9', '8', '.', '.', '.', '.', '6', '.'].to_vec(),
            ['8', '.', '.', '.', '6', '.', '.', '.', '3'].to_vec(),
            ['4', '.', '.', '8', '.', '3', '.', '.', '1'].to_vec(),
            ['7', '.', '.', '.', '2', '.', '.', '.', '6'].to_vec(),
            ['.', '6', '.', '.', '.', '.', '2', '8', '.'].to_vec(),
            ['.', '.', '.', '4', '1', '9', '.', '.', '5'].to_vec(),
            ['.', '.', '.', '.', '8', '.', '.', '7', '9'].to_vec()
        ];

        assert_eq!(is_valid_sudoku(board), true)
    }

    #[test]
    fn test_valid_sudoku_2() {
        let board = vec![
            ['8', '3', '.', '.', '7', '.', '.', '.', '.'].to_vec(),
            ['6', '.', '.', '1', '9', '5', '.', '.', '.'].to_vec(),
            ['.', '9', '8', '.', '.', '.', '.', '6', '.'].to_vec(),
            ['8', '.', '.', '.', '6', '.', '.', '.', '3'].to_vec(),
            ['4', '.', '.', '8', '.', '3', '.', '.', '1'].to_vec(),
            ['7', '.', '.', '.', '2', '.', '.', '.', '6'].to_vec(),
            ['.', '6', '.', '.', '.', '.', '2', '8', '.'].to_vec(),
            ['.', '.', '.', '4', '1', '9', '.', '.', '5'].to_vec(),
            ['.', '.', '.', '.', '8', '.', '.', '7', '9'].to_vec()
        ];

        assert_eq!(is_valid_sudoku(board), false)
    }

    #[test]
    fn test_row() {
        let row = vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'];

        assert_eq!(is_valid_rows(row), true)
    }

    #[test]
    fn wrong_test() {
        let board = vec![
            ['.', '.', '4', '.', '.', '.', '6', '3', '.'].to_vec(),
            ['.', '.', '.', '.', '.', '.', '.', '.', '.'].to_vec(),
            ['5', '.', '.', '.', '.', '.', '.', '9', '.'].to_vec(),
            ['.', '.', '.', '5', '6', '.', '.', '.', '.'].to_vec(),
            ['4', '.', '3', '.', '.', '.', '.', '.', '1'].to_vec(),
            ['.', '.', '.', '7', '.', '.', '.', '.', '.'].to_vec(),
            ['.', '.', '.', '5', '.', '.', '.', '.', '.'].to_vec(),
            ['.', '.', '.', '.', '.', '.', '.', '.', '.'].to_vec(),
            ['.', '.', '.', '.', '.', '.', '.', '.', '.'].to_vec()
        ];

        assert_eq!(is_valid_sudoku(board), false)
    }
}
