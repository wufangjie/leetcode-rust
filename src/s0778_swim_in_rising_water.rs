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
        let mut used = vec![vec![0u8; n]; n]; // 0: never, 1: met, 2: popped
        let n_1 = n - 1;
        heap.push((Reverse(grid[0][0]), 0, 0));
        let mut res = 0;

        while let Some((t, x, y)) = heap.pop() {
            if used[x][y] != 2 {
                used[x][y] = 2;
                if t.0 > res {
                    res = t.0;
                }
                if x == n_1 && y == n_1 {
                    return res;
                }

                if x > 0 && used[x - 1][y] == 0 {
                    Self::memo_push(&mut heap, &grid, &mut used, x - 1, y);
                }
                if x < n_1 && used[x + 1][y] == 0 {
                    Self::memo_push(&mut heap, &grid, &mut used, x + 1, y);
                }
                if y > 0 && used[x][y - 1] == 0 {
                    Self::memo_push(&mut heap, &grid, &mut used, x, y - 1);
                }
                if y < n_1 && used[x][y + 1] == 0 {
                    Self::memo_push(&mut heap, &grid, &mut used, x, y + 1);
                }
            }
        }
        unreachable!();
    }

    #[inline(always)]
    fn memo_push(
        heap: &mut BinaryHeap<(Reverse<i32>, usize, usize)>,
        grid: &[Vec<i32>],
        used: &mut [Vec<u8>],
        x: usize,
        y: usize,
    ) {
        heap.push((Reverse(grid[x][y]), x, y));
        used[x][y] = 1;
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
