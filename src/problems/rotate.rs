pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let n = matrix.len();

    let mt = matrix.clone();

    for i in 0..n {
        for j in 0..n {
            matrix[i][j] = mt[j][i];
        }
    }

    for rows in matrix.iter_mut() {
        rows.reverse();
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_rotate() {
        let mut input = vec![[1, 2, 3].to_vec(), [4, 5, 6].to_vec(), [7, 8, 9].to_vec()];

        let output = vec![[7, 4, 1].to_vec(), [8, 5, 2].to_vec(), [9, 6, 3].to_vec()];

        rotate(&mut input);

        println!("matrix after this{:?}", input);

        let e = input == output;

        assert!(e)
    }
}
