//use utils::Heap;
use std::cmp::Reverse; // learned Reverse + Binaryheap
use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        if n == 1 {
            return grid[0][0];
        }

        let mut heap = BinaryHeap::new();
        let mut used = vec![vec![false; n]; n];
        heap.push((Reverse(grid[0][0]), 0, 0));
        used[0][0] = true;
        let n_1 = n - 1;
        let mut res = 0;

        while let Some((t, x, y)) = heap.pop() {
            if t.0 > res {
                res = t.0;
            }
            if x == n_1 && y == n_1 {
                return res;
            }

            if x > 0 && !used[x - 1][y] {
                Self::memo_push(&mut heap, &grid, &mut used, x - 1, y);
            }
            if x < n_1 && !used[x + 1][y] {
                Self::memo_push(&mut heap, &grid, &mut used, x + 1, y);
            }
            if y > 0 && !used[x][y - 1] {
                Self::memo_push(&mut heap, &grid, &mut used, x, y - 1);
            }
            if y < n_1 && !used[x][y + 1] {
                Self::memo_push(&mut heap, &grid, &mut used, x, y + 1);
            }
        }
        unreachable!();
    }

    #[inline(always)]
    fn memo_push(
        heap: &mut BinaryHeap<(Reverse<i32>, usize, usize)>,
        grid: &[Vec<i32>],
        used: &mut [Vec<bool>],
        x: usize,
        y: usize,
    ) {
        heap.push((Reverse(grid[x][y]), x, y));
        used[x][y] = true;
    }
}

#[test]
fn test_778() {
    assert_eq!(3, Solution::swim_in_water(vec![vec![0, 1], vec![2, 3]]));
    assert_eq!(
        16,
        Solution::swim_in_water(vec![
            vec![0, 1, 2, 3, 4],
            vec![24, 23, 22, 21, 5],
            vec![12, 13, 14, 15, 16],
            vec![11, 17, 18, 19, 20],
            vec![10, 9, 8, 7, 6]
        ])
    );
}
