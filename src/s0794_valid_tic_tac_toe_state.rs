struct Solution;

impl Solution {
    pub fn valid_tic_tac_toe(board: Vec<String>) -> bool {
        let mut count_diff = 0;
        let mut acc_row = [0; 3];
        let mut acc_col = [0; 3];
        let mut diag_1 = 0;
        let mut diag_2 = 0;
        for (i, row) in board.iter().enumerate() {
            for (j, elem) in row.bytes().enumerate() {
                match elem {
                    b'X' => {
                        count_diff += 1;
                        acc_row[i] += 1;
                        acc_col[j] += 1;
                        diag_1 += (i == j) as i32;
                        diag_2 += (i + j == 2) as i32;
                    }
                    b'O' => {
                        count_diff -= 1;
                        acc_row[i] -= 1;
                        acc_col[j] -= 1;
                        diag_1 -= (i == j) as i32;
                        diag_2 -= (i + j == 2) as i32;
                    }
                    _ => (),
                }
            }
        }
        if !(0..=1).contains(&count_diff) {
            false
        } else {
            acc_row.sort_unstable();
            acc_col.sort_unstable();
            let win_x = acc_row[2] == 3 || acc_col[2] == 3 || diag_1 == 3 || diag_2 == 3;
            let win_o = acc_row[0] == -3 || acc_col[0] == -3 || diag_1 == -3 || diag_2 == -3;
            if win_x && win_o {
                false
            } else if win_x {
                count_diff == 1
            } else if win_o {
                count_diff == 0
            } else {
                true
            }
        }
    }
}

#[test]
fn test_0794() {
    dbg!(Solution::valid_tic_tac_toe(vec![
        "XXX".to_string(),
        "OOX".to_string(),
        "OOX".to_string()
    ]));
}
