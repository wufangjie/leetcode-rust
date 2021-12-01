struct Solution;

impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        let mut j_i_plus_20 = vec![0; 40];
        let n = matrix[0].len();
        for j in 0..n {
            j_i_plus_20[j + 20] = matrix[0][j];
        }
        for i in 1..matrix.len() {
            j_i_plus_20[20 - i] = matrix[i][0];
            for j in 1..n {
                if matrix[i][j] != j_i_plus_20[20 + j - i] {
                    return false;
                }
            }
        }
        true
    }
}

#[test]
fn test_766() {
    assert_eq!(
        true,
        Solution::is_toeplitz_matrix(vec![vec![1, 2, 3, 4], vec![5, 1, 2, 3], vec![9, 5, 1, 2]])
    );
    assert_eq!(
        false,
        Solution::is_toeplitz_matrix(vec![vec![1, 2], vec![2, 2]])
    );
}
