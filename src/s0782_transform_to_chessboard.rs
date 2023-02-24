use std::cmp::Ordering;

struct Solution;

impl Solution {
    pub fn moves_to_chessboard(board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();
        let mut a1 = Arranger::new(n);
        for i in 0..n {
            let (valid, key) = Solution::process_row(&board, i);
            if !valid || !a1.push(i, key) {
                return -1;
            }
        }

        let mut a2 = Arranger::new(n);
        for i in 0..n {
            // will split loop be faster? seems not
            let (valid, key) = Solution::process_col(&board, i);
            if !valid || !a2.push(i, key) {
                return -1;
            }
        }
        a1.count_min_rearrange_step() + a2.count_min_rearrange_step()
    }

    fn process_row(board: &[Vec<i32>], i: usize) -> (bool, i32) {
        let n = board.len();
        let mut count_0: i32 = 0;
        let mut count_1: i32 = 0;
        let mut key: i32 = 0;
        for (i, &v) in board[i].iter().enumerate() {
            if v == 0 {
                count_0 += 1;
            } else {
                count_1 += 1;
                key ^= 1 << i;
            }
        }
        (
            if (count_0 - count_1).abs() < 2 {
                true
            } else {
                false
            },
            key,
        )
    }

    fn process_col(board: &[Vec<i32>], j: usize) -> (bool, i32) {
        let n = board.len();
        let mut count_0: i32 = 0;
        let mut count_1: i32 = 0;
        let mut key: i32 = 0;
        for (i, row) in board.iter().enumerate() {
            if row[j] == 0 {
                count_0 += 1;
            } else {
                count_1 += 1;
                key ^= 1 << i;
            }
        }
        (
            if (count_0 - count_1).abs() < 2 {
                true
            } else {
                false
            },
            key,
        )
    }
}

#[derive(Debug)]
struct Arranger {
    half: usize,
    type1: i32,
    type2: i32,
    idxs1: Vec<usize>,
    idxs2: Vec<usize>,
}

impl Arranger {
    fn new(n: usize) -> Self {
        let half = (n + 1) >> 1;
        Self {
            half,
            type1: -1,
            type2: -1,
            idxs1: Vec::with_capacity(half),
            idxs2: Vec::with_capacity(half),
        }
    }

    /// is the pushed one not break arranger's contract:
    /// 1. > 2 types
    /// 2. count(one_type) > half
    fn push(&mut self, i: usize, t: i32) -> bool {
        if self.type1 == -1 {
            self.type1 = t;
            self.idxs1.push(i);
        } else if self.type1 == t {
            if self.idxs1.len() == self.half {
                return false;
            }
            self.idxs1.push(i);
        } else if self.type2 == -1 {
            self.type2 = t;
            self.idxs2.push(i);
        } else if self.type2 == t {
            if self.idxs2.len() == self.half {
                return false;
            }
            self.idxs2.push(i);
        } else {
            return false;
        }
        true
    }

    fn count_min_rearrange_step(&self) -> i32 {
        match self.idxs1.len().cmp(&self.idxs2.len()) {
            Ordering::Equal => {
                Self::count_not_at_odd(&self.idxs2).min(Self::count_not_at_odd(&self.idxs1))
            }
            Ordering::Less => Self::count_not_at_odd(&self.idxs2),
            Ordering::Greater => Self::count_not_at_odd(&self.idxs1),
        }
    }

    #[inline]
    fn count_not_at_odd(idxs: &[usize]) -> i32 {
        idxs.iter().map(|&i| (i & 1 == 1) as i32).sum::<i32>()
    }
}

#[test]
fn test_0782() {
    assert_eq!(
        2,
        Solution::moves_to_chessboard(vec![
            vec![0, 1, 1, 0],
            vec![0, 1, 1, 0],
            vec![1, 0, 0, 1],
            vec![1, 0, 0, 1]
        ])
    );
    assert_eq!(
        0,
        Solution::moves_to_chessboard(vec![vec![0, 1], vec![1, 0]])
    );
    assert_eq!(
        -1,
        Solution::moves_to_chessboard(vec![vec![1, 0], vec![1, 0]])
    );
    assert_eq!(
        1,
        Solution::moves_to_chessboard(vec![vec![1, 0, 0], vec![0, 1, 1], vec![1, 0, 0]])
    );
}
