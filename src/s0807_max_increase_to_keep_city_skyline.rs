struct Solution;

/// NOTE: it's easy that the skyline in each direction is a number
/// rather than a increasing stack, otherwise it should implement by dfs + process(edge==0)
impl Solution {
    pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let row_max: Vec<i32> = grid
            .iter()
            .map(|row| row.iter().max().unwrap())
            .cloned()
            .collect();
        let col_max: Vec<i32> = (0..n)
            .map(|j| (0..m).fold(0, |acc, x| acc.max(grid[x][j])))
            .collect();
        grid.into_iter()
            .enumerate()
            .map(|(i, row)| {
                row.into_iter()
                    .enumerate()
                    .map(|(j, cur_h)| row_max[i].min(col_max[j]) - cur_h)
                    .sum::<i32>()
            })
            .sum()
    }
}

#[test]
fn test_0807() {
    assert_eq!(
        35,
        Solution::max_increase_keeping_skyline(vec![
            vec![3, 0, 8, 4],
            vec![2, 4, 5, 7],
            vec![9, 2, 6, 3],
            vec![0, 3, 1, 0],
        ])
    );

    assert_eq!(
        0,
        Solution::max_increase_keeping_skyline(vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0],])
    );
}
