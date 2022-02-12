use std::collections::{HashSet, VecDeque};

struct Solution;

const NROW: usize = 2;
const NCOL: usize = 3;

impl Solution {
    pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
        let mut queue = VecDeque::new();
        let mut used = HashSet::new();
        let mut board_arr = [0; NROW * NCOL];
        for i in 0..NROW {
            for j in 0..NCOL {
                board_arr[i * NCOL + j] = board[i][j];
            }
        }
        queue.push_back((0, board_arr));
        while let Some((d, b)) = queue.pop_front() {
            if b == [1, 2, 3, 4, 5, 0] {
                return d;
            }
            if used.contains(&b) {
                continue;
            }
            used.insert(b.clone());

            let (i, j) = Self::find_empty(&b);
            for b2 in Self::find_next_poss(b, i, j) {
                if !used.contains(&b2) {
                    queue.push_back((d + 1, b2));
                }
            }
        }
        return -1;
    }

    fn find_empty(board: &[i32]) -> (usize, usize) {
        for i in 0..NROW {
            for j in 0..NCOL {
                if board[i * NCOL + j] == 0 {
                    return (i, j);
                }
            }
        }
        unreachable!()
    }

    fn find_next_poss(board: [i32; NROW * NCOL], i: usize, j: usize) -> Vec<[i32; NROW * NCOL]> {
        let mut adj = vec![];
        match i {
            0 => adj.push((1, j)),
            _ => adj.push((0, j)),
        }
        match j {
            1 => {
                adj.push((i, 0));
                adj.push((i, 2));
            }
            _ => adj.push((i, 1)),
        }

        let ij = i * NCOL + j;

        adj.into_iter()
            .map(|(ii, jj)| {
                let mut poss = board.clone();
                let ij2 = ii * NCOL + jj;
                poss[ij] = board[ij2];
                poss[ij2] = 0;
                poss
            })
            .collect()
    }
}

#[test]
fn test_773() {
    assert_eq!(
        1,
        Solution::sliding_puzzle(vec![vec![1, 2, 3], vec![4, 0, 5]])
    );
    assert_eq!(
        -1,
        Solution::sliding_puzzle(vec![vec![1, 2, 3], vec![5, 4, 0]])
    );
    assert_eq!(
        5,
        Solution::sliding_puzzle(vec![vec![4, 1, 2], vec![5, 0, 3]])
    );
}
